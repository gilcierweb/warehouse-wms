// -- API Types
export type { ApiFetchFunction, ApiFetchOptions } from './api'

// -- Route Meta for Authentication

export interface AuthRouteMeta {
  /** Requires authentication to access this route */
  requiresAuth?: boolean
  /** Redirect authenticated users away from this route (e.g., login page) */
  guestOnly?: boolean
  /** Required roles to access this route */
  requiredRoles?: string[]
  /** Route name to redirect when unauthenticated (default: /auth/login) */
  loginRedirect?: string
  /** Route name to redirect when authenticated but on guest-only route (default: /chat) */
  homeRedirect?: string
}

// -- Auth 

export interface User {
  id: string
  email: string
  confirmed_at: string | null
  totp_enabled: boolean
  created_at: string
}

export interface AuthResponse {
  access_token: string
  refresh_token: string
  token_type: string
  user: User
}

// -- Profile 

export interface Profile {
  id: string
  user_id: string
  first_name: string | null
  last_name: string | null
  nickname: string | null
  bio: string | null
  phone: string | null
  birthday: string | null
  avatar_url: string | null
  status: boolean
  social_network: Record<string, string>
}

// -- Keys 

export interface PrekeyBundle {
  user_id: string
  identity_key: string
  signed_prekey: string
  signed_prekey_signature: string
  one_time_prekey: string | null
  one_time_prekey_id: string | null
}

export interface LocalKeyPair {
  publicKey: CryptoKey
  privateKey: CryptoKey
}

export interface StoredKeys {
  identityKeyPair: CryptoKeyPair
  signedPreKeyPair: CryptoKeyPair
  oneTimePreKeys: CryptoKeyPair[]
}

// -- Conversation 

export interface Conversation {
  id: string
  conversation_type: 'direct' | 'group'
  name: string | null
  avatar_url: string | null
  created_by: string
  created_at: string
  updated_at: string
  // Populated client-side
  last_message?: Message | null
  unread_count?: number
  members?: ConversationMember[]
}

export interface ConversationMember {
  conversation_id: string
  user_id: string
  role: 'owner' | 'admin' | 'member'
  joined_at: string
  last_read_at: string | null
}

// -- Message 

export interface Message {
  id: string
  conversation_id: string
  sender_id: string
  /** Base64-encoded ciphertext as returned from server */
  ciphertext: string
  iv: string
  message_type: 'text' | 'image' | 'file' | 'audio'
  reply_to_id: string | null
  created_at: string
  deleted_at: string | null
  // Populated client-side after decryption
  plaintext?: string
  // Frontend-only marker to prevent persisting/reusing fallback placeholders
  is_placeholder?: boolean
  status?: 'sending' | 'sent' | 'delivered' | 'read' | 'error'
}

export interface SendMessagePayload {
  ciphertext: string   // base64
  iv: string           // base64
  message_type: string
  reply_to_id?: string | null
}

// -- WebSocket 

export type WsIncomingMessage =
  | { type: 'authenticated'; user_id: string }
  | { type: 'new_message'; conversation_id: string; message_id: string; sender_id: string; ciphertext: string; iv: string; message_type: string; reply_to_id: string | null; created_at: string }
  | { type: 'typing'; conversation_id: string; user_id: string }
  | { type: 'presence_update'; user_id: string; online: boolean }
  | { type: 'pong' }
  | { type: 'error'; message: string }
  // Auction events
  | { type: 'new_bid'; auction_id: string; bid_id: string; bidder_id: string; bidder_name: string; amount: string; auction_title: string; is_owner?: boolean; owner_id?: string }
  | { type: 'new_bid_owner'; auction_id: string; bid_id: string; bidder_id: string; bidder_name: string; amount: string; auction_title: string }
  | { type: 'outbid'; auction_id: string; auction_title: string; new_amount: string; user_id?: string }
  | { type: 'auction_ended'; auction_id: string; auction_title: string; winner_id?: string; winning_amount?: string }
  | { type: 'auction_won'; auction_id: string; auction_title: string; winning_amount?: string; winner_id?: string }
  | { type: 'auction_ended_owner'; auction_id: string; auction_title: string; has_winner?: boolean; winning_amount?: string }
