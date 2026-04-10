/**
 * useKeyStore — manages E2E key pairs using Web Crypto API + IndexedDB.
 *
 * Key hierarchy:
 *  - Identity key pair   (X25519, long-lived)
 *  - Signed prekey pair  (X25519, rotated weekly)
 *  - One-time prekeys    (X25519, consumed per session, batch of 100)
 *
 * Private keys are stored in IndexedDB as non-extractable CryptoKey objects.
 * Only public keys are uploaded to the server.
 */

import { openDB, type IDBPDatabase } from 'idb'

const DB_NAME = 'simple-chat-keys'
const DB_VERSION = 1
const STORE_KEYS = 'keys'

let _db: IDBPDatabase | null = null

async function getDb(): Promise<IDBPDatabase> {
  if (_db) return _db
  _db = await openDB(DB_NAME, DB_VERSION, {
    upgrade(db) {
      if (!db.objectStoreNames.contains(STORE_KEYS)) {
        db.createObjectStore(STORE_KEYS)
      }
    },
  })
  return _db
}

export const useKeyStore = () => {
  const config = useRuntimeConfig()
  const authStore = useAuthStore()

  /**
   * Generate X25519 ECDH key pair.
   * Private key is non-extractable (can only be used for derivation).
   */
  async function generateKeyPair(): Promise<CryptoKeyPair> {
    return crypto.subtle.generateKey(
      { name: 'ECDH', namedCurve: 'P-256' }, // Use P-256 (widely supported); X25519 available in modern browsers
      false, // non-extractable private key
      ['deriveKey', 'deriveBits'],
    )
  }

  /**
   * Export a public key as base64 string (for uploading to server).
   */
  async function exportPublicKey(key: CryptoKey): Promise<string> {
    const raw = await crypto.subtle.exportKey('spki', key)
    return btoa(String.fromCharCode(...new Uint8Array(raw)))
  }

  /**
   * Import a peer's public key from base64.
   */
  async function importPublicKey(b64: string): Promise<CryptoKey> {
    const raw = Uint8Array.from(atob(b64), c => c.charCodeAt(0))
    return crypto.subtle.importKey(
      'spki',
      raw,
      { name: 'ECDH', namedCurve: 'P-256' },
      true,
      [],
    )
  }

  /**
   * Derive a shared AES-256-GCM key from our private key + peer's public key (ECDH).
   */
  async function deriveSharedKey(
    ourPrivateKey: CryptoKey,
    peerPublicKey: CryptoKey,
  ): Promise<CryptoKey> {
    return crypto.subtle.deriveKey(
      { name: 'ECDH', public: peerPublicKey },
      ourPrivateKey,
      { name: 'AES-GCM', length: 256 },
      false,
      ['encrypt', 'decrypt'],
    )
  }

  /**
   * Encrypt plaintext with AES-256-GCM.
   * Returns { ciphertext: base64, iv: base64 }
   */
  async function encrypt(key: CryptoKey, plaintext: string): Promise<{ ciphertext: string; iv: string }> {
    const iv = crypto.getRandomValues(new Uint8Array(12))
    const encoded = new TextEncoder().encode(plaintext)
    const ct = await crypto.subtle.encrypt({ name: 'AES-GCM', iv }, key, encoded)
    
    // Convert Uint8Array to base64 properly
    const ctBytes = new Uint8Array(ct)
    const ctString = String.fromCharCode.apply(null, Array.from(ctBytes))
    const ivString = String.fromCharCode.apply(null, Array.from(iv))
    
    return {
      ciphertext: btoa(ctString),
      iv: btoa(ivString),
    }
  }

  /**
   * Decrypt ciphertext with AES-256-GCM.
   */
  async function decrypt(key: CryptoKey, ciphertextInput: string | number[], iv: string): Promise<string> {
    try {
      // 1. Prepare Ciphertext
      let ct: Uint8Array
      if (Array.isArray(ciphertextInput)) {
        ct = new Uint8Array(ciphertextInput)
      } else {
        const ctString = atob(ciphertextInput)
        ct = new Uint8Array(ctString.length)
        for (let i = 0; i < ctString.length; i++) {
          ct[i] = ctString.charCodeAt(i)
        }
      }
      
      // 2. Prepare IV
      const ivString = atob(iv)
      const ivBytes = new Uint8Array(ivString.length)
      for (let i = 0; i < ivString.length; i++) {
        ivBytes[i] = ivString.charCodeAt(i)
      }
      
      console.log(`[Decryption] Attempting with ct-len: ${ct.length}, iv-len: ${ivBytes.length}`)
      const plain = await crypto.subtle.decrypt({ name: 'AES-GCM', iv: ivBytes }, key, ct)
      return new TextDecoder().decode(plain)
    } catch (error) {
      console.error('[Decryption] DOMException details:', error)
      throw error
    }
  }

  /**
   * Store a CryptoKey in IndexedDB by name.
   */
  async function storeKey(name: string, key: CryptoKey): Promise<void> {
    const db = await getDb()
    await db.put(STORE_KEYS, key, name)
  }

  /**
   * Retrieve a CryptoKey from IndexedDB by name.
   */
  async function loadKey(name: string): Promise<CryptoKey | null> {
    const db = await getDb()
    return db.get(STORE_KEYS, name) ?? null
  }

  /**
   * Store a full key pair.
   */
  async function storeKeyPair(prefix: string, pair: CryptoKeyPair): Promise<void> {
    await storeKey(`${prefix}:pub`, pair.publicKey)
    await storeKey(`${prefix}:priv`, pair.privateKey)
  }

  /**
   * Load a full key pair.
   */
  async function loadKeyPair(prefix: string): Promise<CryptoKeyPair | null> {
    const pub = await loadKey(`${prefix}:pub`)
    const priv = await loadKey(`${prefix}:priv`)
    if (!pub || !priv) return null
    return { publicKey: pub, privateKey: priv }
  }

  /**
   * Ensure keys exist locally and on the server.
    * Called after login — generates if missing, uploads public keys.
    */
  async function ensureKeys(accessToken: string): Promise<void> {
    console.log('ensureKeys called')
    const db = await getDb()

    let identityPair = await loadKeyPair('identity')
    let signedPreKeyPair = await loadKeyPair('spk')
    console.log('identityPair:', !!identityPair, 'signedPreKeyPair:', !!signedPreKeyPair)

    if (!identityPair) {
      identityPair = await generateKeyPair()
      await storeKeyPair('identity', identityPair)
      console.log('Generated identity key pair')
    }

    if (!signedPreKeyPair) {
      signedPreKeyPair = await generateKeyPair()
      await storeKeyPair('spk', signedPreKeyPair)
      console.log('Generated signed prekey pair')
    }

    // Generate batch of one-time prekeys (only if none exist)
    const existingOtpks = []
    for (let i = 0; i < 20; i++) {
      const pair = await loadKeyPair(`otpk:${i}`)
      if (pair) existingOtpks.push(pair)
    }
    console.log('Existing OTPKs:', existingOtpks.length)
    
    if (existingOtpks.length < 20) {
      for (let i = 0; i < 20; i++) {
        const pair = await generateKeyPair()
        await storeKeyPair(`otpk:${i}`, pair)
      }
      console.log('Generated 20 OTPKs')
    }

    // Always upload keys to ensure they're on server
    console.log('Uploading keys to server...')
    try {
      await uploadKeysToServer(accessToken)
      console.log('Keys uploaded successfully')
    } catch (e) {
      console.error('Failed to upload keys:', e)
    }
  }

  /**
   * Establish a shared key with a peer given their prekey bundle.
   * Returns the derived AES-256-GCM key for message encryption.
   * 
   * Uses ECDH with identity keys for symmetric key derivation:
   * - Both sides use: our identity private + peer's identity public
   * This ensures both derive the SAME shared secret.
   */
  async function establishSession(peerBundle: {
    identity_key: string
    signed_prekey: string
    one_time_prekey: string | null
  }): Promise<CryptoKey> {
    // Load our identity key pair
    const identityPair = await loadKeyPair('identity')
    if (!identityPair) throw new Error('No identity key found — please log in again')

    // Import peer's identity public key
    const peerIdentityKey = await importPublicKey(peerBundle.identity_key)
    
    // Derive shared secret using our identity private + peer's identity public
    // This is symmetric - both sides do the same and get the same result
    const sharedKey = await deriveSharedKey(identityPair.privateKey, peerIdentityKey)
    return sharedKey
  }

  /**
   * Get or establish session key for a conversation.
   * Session keys are cached in IndexedDB.
   */
  async function getSessionKey(conversationId: string, peerUserId: string, peerBundle?: {
    identity_key: string
    signed_prekey: string
    one_time_prekey: string | null
  }): Promise<CryptoKey> {
    // v3 cache key is scoped by conversation + peer user.
    // This prevents stale/mismatched session reuse across peers.
    const cacheKey = `session:v3:${conversationId}:${peerUserId}`

    // If a fresh peer bundle is provided, always rebuild and overwrite cache.
    // This protects against stale sessions when peer keys were rotated.
    if (peerBundle) {
      const key = await establishSession(peerBundle)
      await storeKey(cacheKey, key)
      return key
    }

    const cached = await loadKey(cacheKey)
    if (cached) return cached

    if (!peerBundle) {
      // Try to fetch peer bundle from server and establish session
      const bundle = await fetchPeerBundle(peerUserId)
      if (bundle) {
        return await getSessionKey(conversationId, peerUserId, bundle)
      }
      throw new Error('Need peer bundle to establish new session')
    }

    throw new Error('Need peer bundle to establish new session')
  }

  /**
   * Fetch peer's prekey bundle from server.
   */
  async function fetchPeerBundle(peerUserId: string): Promise<{
    identity_key: string
    signed_prekey: string
    one_time_prekey: string | null
  } | null> {
    const token = authStore.accessToken
    
    if (!token) {
      console.error('[fetchPeerBundle] No access token available')
      return null
    }
    
    try {
      const response = await $fetch<{
        status: string
        data: {
          identity_key: string
          signed_prekey: string
          one_time_prekey: string | null
        }
      }>(`${config.public.apiBase}/keys/${peerUserId}`, {
        headers: {
          'Content-Type': 'application/json',
          'X-API-Key': config.public.apiKey,
          Authorization: `Bearer ${token}`,
        },
      })
      return response.data || null
    } catch (e: any) {
      console.error('[fetchPeerBundle] Failed:', e?.statusCode, e?.message)
      return null
    }
  }

  /**
   * Remove a session key from cache.
   */
  async function invalidateSession(conversationId: string, peerUserId: string): Promise<void> {
    const cacheKey = `session:v3:${conversationId}:${peerUserId}`
    const db = await getDb()
    await db.delete(STORE_KEYS, cacheKey)
  }

  /**
   * Ensure a session exists for a conversation.
   * Creates session if not exists.
   */
  async function ensureSession(conversationId: string, peerUserId: string): Promise<void> {
    const cacheKey = `session:v3:${conversationId}:${peerUserId}`
    const cached = await loadKey(cacheKey)
    if (cached) return

    const bundle = await fetchPeerBundle(peerUserId)
    if (bundle) {
      const key = await establishSession(bundle)
      await storeKey(cacheKey, key)
    }
  }

  /**
   * Upload keys to server.
   */
  async function uploadKeysToServer(accessToken: string): Promise<void> {
    console.log('uploadKeysToServer called')
    const identityPair = await loadKeyPair('identity')
    const signedPreKeyPair = await loadKeyPair('spk')
    if (!identityPair || !signedPreKeyPair) {
      console.log('Missing key pairs, cannot upload')
      return
    }

    const identityPubB64 = await exportPublicKey(identityPair.publicKey)
    const spkPubB64 = await exportPublicKey(signedPreKeyPair.publicKey)

    const otpks: string[] = []
    for (let i = 0; i < 20; i++) {
      const pair = await loadKeyPair(`otpk:${i}`)
      if (pair) {
        otpks.push(await exportPublicKey(pair.publicKey))
      }
    }
    console.log('OTPKs to upload:', otpks.length)

    const spkSignature = btoa('self-signed:' + spkPubB64.substring(0, 32))

    // POST /api/v1/keys/upload (matches backend keys_controller)
    const response = await $fetch(`${config.public.apiBase}/keys/upload`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-API-Key': config.public.apiKey,
        Authorization: `Bearer ${accessToken}`,
      },
      body: {
        identity_key: identityPubB64,
        signed_prekey: spkPubB64,
        signed_signature: spkSignature,
        one_time_prekeys: otpks,
      },
    })
    console.log('Upload response:', response)
  }

  return {
    generateKeyPair,
    exportPublicKey,
    importPublicKey,
    deriveSharedKey,
    encrypt,
    decrypt,
    storeKey,
    loadKey,
    storeKeyPair,
    loadKeyPair,
    ensureKeys,
    establishSession,
    getSessionKey,
    ensureSession,
    uploadKeysToServer,
    fetchPeerBundle,
    invalidateSession,
  }
}
