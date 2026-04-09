use std::env;

/// All configuration values loaded from environment variables.
/// In production, use Docker secrets or a secrets manager like Vault.
#[derive(Debug, Clone)]
pub struct AppConfig {
    // Server
    pub host: String,
    pub port: u16,
    pub frontend_url: String,
    pub environment: Environment,

    // API Key for frontend access
    pub api_key: String,

    // Database
    pub database_url: String,
    pub db_pool_size: u32,

    // Redis
    pub redis_url: String,

    // JWT
    pub jwt_secret: String,
    pub jwt_access_expiry_secs: i64,  // 15 minutes
    pub jwt_refresh_expiry_secs: i64, // 30 days

    // Email (Resend)
    pub resend_api_key: String,
    pub email_from: String,
    pub email_from_name: String,

    // Bunny.net CDN / Storage
    pub bunny_storage_zone: String,
    pub bunny_storage_key: String,
    pub bunny_cdn_url: String,
    pub bunny_token_key: String, // for URL signing

    // Bunny.net Stream
    pub bunny_stream_library_id: String,
    pub bunny_stream_key: String,
    pub bunny_stream_webhook_secret: String,

    // Backblaze B2
    pub b2_key_id: String,
    pub b2_application_key: String,
    pub b2_bucket_id: String,
    pub b2_bucket_name: String,
    pub b2_endpoint: String,

    // Stripe
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
    pub stripe_publishable_key: String,

    // Platform settings
    pub platform_commission_percent: f64,  // e.g. 20.0
    pub min_subscription_price_cents: i64, // e.g. 500 = $5.00
    pub max_subscription_price_cents: i64, // e.g. 50000 = $500.00
    pub min_withdrawal_amount_cents: i64,  // e.g. 2000 = $20.00

    // TOTP (2FA)
    pub totp_issuer: String,

    // File upload limits
    pub max_video_size_bytes: u64, // 10 GB
    pub max_photo_size_bytes: u64, // 50 MB
    pub max_audio_size_bytes: u64, // 500 MB
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Production,
    Test,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            environment: match env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string())
                .as_str()
            {
                "production" => Environment::Production,
                "test" => Environment::Test,
                _ => Environment::Development,
            },

            api_key: env::var("API_KEY")
                .unwrap_or_else(|_| "dev-api-key-change-in-production".to_string()),

            database_url: env::var("DATABASE_URL")?,
            db_pool_size: env::var("DB_POOL_SIZE")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),

            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),

            jwt_secret: env::var("JWT_SECRET")?,
            jwt_access_expiry_secs: 2 * 60 * 60, // 2 hours (was 15 min)
            jwt_refresh_expiry_secs: 30 * 24 * 3600, // 30 days

            resend_api_key: env::var("RESEND_API_KEY").unwrap_or_default(),
            email_from: env::var("EMAIL_FROM")
                .unwrap_or_else(|_| "noreply@justfans.com".to_string()),
            email_from_name: env::var("EMAIL_FROM_NAME").unwrap_or_else(|_| "JustFans".to_string()),

            bunny_storage_zone: env::var("BUNNY_STORAGE_ZONE").unwrap_or_default(),
            bunny_storage_key: env::var("BUNNY_STORAGE_KEY").unwrap_or_default(),
            bunny_cdn_url: env::var("BUNNY_CDN_URL")
                .unwrap_or_else(|_| "https://cdn.justfans.com".to_string()),
            bunny_token_key: env::var("BUNNY_TOKEN_KEY").unwrap_or_default(),

            bunny_stream_library_id: env::var("BUNNY_STREAM_LIBRARY_ID").unwrap_or_default(),
            bunny_stream_key: env::var("BUNNY_STREAM_KEY").unwrap_or_default(),
            bunny_stream_webhook_secret: env::var("BUNNY_STREAM_WEBHOOK_SECRET")
                .unwrap_or_default(),

            b2_key_id: env::var("B2_KEY_ID").unwrap_or_default(),
            b2_application_key: env::var("B2_APPLICATION_KEY").unwrap_or_default(),
            b2_bucket_id: env::var("B2_BUCKET_ID").unwrap_or_default(),
            b2_bucket_name: env::var("B2_BUCKET_NAME").unwrap_or_default(),
            b2_endpoint: env::var("B2_ENDPOINT")
                .unwrap_or_else(|_| "https://s3.us-west-004.backblazeb2.com".to_string()),

            stripe_secret_key: env::var("STRIPE_SECRET_KEY").unwrap_or_default(),
            stripe_webhook_secret: env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default(),
            stripe_publishable_key: env::var("STRIPE_PUBLISHABLE_KEY").unwrap_or_default(),

            platform_commission_percent: env::var("PLATFORM_COMMISSION_PERCENT")
                .unwrap_or_else(|_| "20.0".to_string())
                .parse()
                .unwrap_or(20.0),
            min_subscription_price_cents: 500,
            max_subscription_price_cents: 50_000,
            min_withdrawal_amount_cents: 2_000,

            totp_issuer: env::var("TOTP_ISSUER").unwrap_or_else(|_| "JustFans".to_string()),

            max_video_size_bytes: 10 * 1024 * 1024 * 1024, // 10 GB
            max_photo_size_bytes: 50 * 1024 * 1024,        // 50 MB
            max_audio_size_bytes: 500 * 1024 * 1024,       // 500 MB
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }
}
