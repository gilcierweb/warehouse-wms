pub mod app_config;
pub use app_config::AppConfig;
// use std::env;

// #[derive(Debug, Clone)]
// pub struct AppConfig {
//     pub jwt_secret: String,
//     pub jwt_expiry_hours: i64,
//     pub database_url: String,
//     pub host: String,
//     pub port: u16,
// }

// impl AppConfig {
//     pub fn from_env() -> Self {
//         Self {
//             jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET environment variable must be set"),
//             jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
//                 .unwrap_or_else(|_| "8".to_string())
//                 .parse()
//                 .expect("JWT_EXPIRY_HOURS must be a valid number"),
//             database_url: env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set"),
//             host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
//             port: env::var("PORT")
//                 .unwrap_or_else(|_| "8080".to_string())
//                 .parse()
//                 .expect("PORT must be a valid number"),
//         }
//     }
// }
