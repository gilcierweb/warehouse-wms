#[path = "../auth/mod.rs"]
mod auth;
#[path = "../db/schema.rs"]
mod schema;
#[path = "../utils/mod.rs"]
mod utils;

use chrono::Utc;
use diesel::prelude::*;
use diesel::{Connection, PgConnection};

use auth::password::password_hash;
use uuid::Uuid;

use schema::{
    alert_configs, profiles, roles, slots, user_roles, users,
};

// ========== ROLES ==========

#[derive(Insertable)]
#[diesel(table_name = roles)]
struct NewRole {
    id: Uuid,
    name: String,
    resource_type: Option<String>,
    resource_id: Option<Uuid>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

// ========== USERS ==========

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    id: Uuid,
    email: String,
    password_hash: String,
    confirmation_token: Option<String>,
    confirmed_at: Option<chrono::DateTime<Utc>>,
    sign_in_count: i32,
    failed_attempts: i32,
    totp_enabled: bool,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

// ========== USER ROLES ==========

#[derive(Insertable)]
#[diesel(table_name = user_roles)]
struct NewUserRoleAssignment {
    user_id: Uuid,
    role_id: Uuid,
}

// ========== PROFILES ==========

#[derive(Insertable)]
#[diesel(table_name = profiles)]
struct NewProfile {
    id: Uuid,
    first_name_enc: Option<Vec<u8>>,
    last_name_enc: Option<Vec<u8>>,
    phone_enc: Option<Vec<u8>>,
    full_name: Option<Vec<u8>>,
    nickname: Option<String>,
    bio: Option<String>,
    birthday: Option<chrono::NaiveDate>,
    avatar: Option<String>,
    phone: Option<i64>,
    social_network: serde_json::Value,
    status: bool,
    user_id: Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

// ========== SLOTS ==========

#[derive(Insertable)]
#[diesel(table_name = slots)]
struct NewSlot {
    id: Uuid,
    address: String,
    street: String,
    position: i16,
    lane: String,
    status: String,
    sku: Option<String>,
    updated_by: Option<Uuid>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

// ========== ALERT CONFIG ==========

#[derive(Insertable)]
#[diesel(table_name = alert_configs)]
struct NewAlertConfig {
    id: Uuid,
    threshold_pct: i16,
    notify_browser: bool,
    notify_email: bool,
    email_recipient: Option<String>,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}

// ========== MAIN ==========

fn main() {
    println!("Starting database seed...");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let mut conn = PgConnection::establish(&database_url)
        .expect("Failed to connect to database");

    println!("Connected to database");

    let now = Utc::now();

    // ========== ROLES ==========
    println!("Creating roles...");

    let role_admin = create_role(&mut conn, "admin", None, None, now);
    let role_moderator = create_role(&mut conn, "moderator", None, None, now);
    let role_seller = create_role(&mut conn, "seller", None, None, now);
    let role_bidder = create_role(&mut conn, "bidder", None, None, now);

    println!("  - admin role: {}", role_admin);
    println!("  - moderator role: {}", role_moderator);
    println!("  - seller role: {}", role_seller);
    println!("  - bidder role: {}", role_bidder);

    // ========== USERS ==========
    println!("Creating users...");

    let admin_user = create_user(&mut conn, "admin@warehouse.com", "admin123", None, now);
    println!("  - admin user: {} (admin@warehouse.com)", admin_user);

    let operator1 = create_user(&mut conn, "operator1@warehouse.com", "operator123", None, now);
    println!("  - operator1: {} (operator1@warehouse.com)", operator1);

    let operator2 = create_user(&mut conn, "operator2@warehouse.com", "operator123", None, now);
    println!("  - operator2: {} (operator2@warehouse.com)", operator2);

    let viewer1 = create_user(&mut conn, "viewer@warehouse.com", "viewer123", None, now);
    println!("  - viewer: {} (viewer@warehouse.com)", viewer1);

    // ========== USER ROLES ==========
    println!("Assigning roles to users...");

    assign_role(&mut conn, admin_user, role_admin);
    println!("  - admin@warehouse.com -> admin");

    assign_role(&mut conn, operator1, role_moderator);
    println!("  - operator1@warehouse.com -> moderator");

    assign_role(&mut conn, operator2, role_moderator);
    println!("  - operator2@warehouse.com -> moderator");

    assign_role(&mut conn, viewer1, role_bidder);
    println!("  - viewer@warehouse.com -> bidder");

    // ========== PROFILES ==========
    println!("Creating profiles...");

    create_profile(&mut conn, admin_user, now);
    println!("  - admin profile created");

    create_profile(&mut conn, operator1, now);
    println!("  - operator1 profile created");

    create_profile(&mut conn, operator2, now);
    println!("  - operator2 profile created");

    create_profile(&mut conn, viewer1, now);
    println!("  - viewer profile created");

    // ========== SLOTS ==========
    println!("Creating warehouse slots...");

    let streets = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    let lanes = ["N1", "N2", "N3"];
    let positions = 1..=30;

    let mut slot_count = 0;
    let mut occupied_count = 0;

    for street in streets {
        for lane in lanes {
            for position in positions.clone() {
                let address = format!("{}-{}-{}", street, position, lane);
                
                let is_occupied = rand_bool(0.30);
                let status = if is_occupied { "occupied" } else { "free" };
                let sku = if is_occupied {
                    occupied_count += 1;
                    Some(generate_sku())
                } else {
                    None
                };

                create_slot(&mut conn, &address, &street.to_string(), position as i16, lane, status, sku, None, now);
                slot_count += 1;
            }
        }
    }

    println!("  - Created {} slots ({} occupied, {} free)", slot_count, occupied_count, slot_count - occupied_count);

    // ========== ALERT CONFIG ==========
    println!("Creating alert config...");

    diesel::insert_into(alert_configs::table)
        .values(NewAlertConfig {
            id: Uuid::new_v4(),
            threshold_pct: 80,
            notify_browser: true,
            notify_email: true,
            email_recipient: Some("admin@warehouse.com".to_string()),
            created_at: now.naive_utc(),
            updated_at: now.naive_utc(),
        })
        .execute(&mut conn)
        .expect("Failed to insert alert config");

    println!("  - Alert config created (threshold: 80%)");

    println!("\n✅ Database seed completed successfully!");
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    TEST CREDENTIALS                           ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Admin:    admin@warehouse.com     /     admin123           ║");
    println!("║  Operator: operator1@warehouse.com /     operator123         ║");
    println!("║  Operator: operator2@warehouse.com /     operator123         ║");
    println!("║  Viewer:   viewer@warehouse.com   /     viewer123           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}

// ========== HELPER FUNCTIONS ==========

fn create_role(
    conn: &mut PgConnection,
    name: &str,
    resource_type: Option<String>,
    resource_id: Option<Uuid>,
    now: chrono::DateTime<Utc>,
) -> Uuid {
    let id = Uuid::new_v4();
    
    diesel::insert_into(roles::table)
        .values(NewRole {
            id,
            name: name.to_string(),
            resource_type,
            resource_id,
            created_at: now,
            updated_at: now,
        })
        .execute(conn)
        .expect("Failed to insert role");

    id
}

fn create_user(
    conn: &mut PgConnection,
    email: &str,
    password: &str,
    confirmation_token: Option<String>,
    now: chrono::DateTime<Utc>,
) -> Uuid {
    let new_id = Uuid::new_v4();
    let hash = password_hash(password.to_string());

    diesel::insert_into(users::table)
        .values(NewUser {
            id: new_id,
            email: email.to_string(),
            password_hash: hash,
            confirmation_token: None,
            confirmed_at: Some(now),
            sign_in_count: 0,
            failed_attempts: 0,
            totp_enabled: false,
            created_at: now,
            updated_at: now,
        })
        .execute(conn)
        .expect(&format!("Failed to insert user {}", email));

    new_id
}

fn assign_role(conn: &mut PgConnection, user_id: Uuid, role_id: Uuid) {
    diesel::insert_into(user_roles::table)
        .values(NewUserRoleAssignment { user_id, role_id })
        .execute(conn)
        .expect("Failed to insert user role");
}

fn create_profile(conn: &mut PgConnection, user_id: Uuid, now: chrono::DateTime<Utc>) {
    diesel::insert_into(profiles::table)
        .values(NewProfile {
            id: Uuid::new_v4(),
            first_name_enc: None,
            last_name_enc: None,
            phone_enc: None,
            full_name: None,
            nickname: None,
            bio: None,
            birthday: None,
            avatar: None,
            phone: None,
            social_network: serde_json::json!({}),
            status: true,
            user_id,
            created_at: now.naive_utc(),
            updated_at: now.naive_utc(),
        })
        .execute(conn)
        .expect("Failed to insert profile");
}

fn create_slot(
    conn: &mut PgConnection,
    address: &str,
    street: &str,
    position: i16,
    lane: &str,
    status: &str,
    sku: Option<String>,
    updated_by: Option<Uuid>,
    now: chrono::DateTime<Utc>,
) {
    let _ = diesel::insert_into(slots::table)
        .values(NewSlot {
            id: Uuid::new_v4(),
            address: address.to_string(),
            street: street.to_string(),
            position,
            lane: lane.to_string(),
            status: status.to_string(),
            sku,
            updated_by,
            created_at: now.naive_utc(),
            updated_at: now.naive_utc(),
        })
        .execute(conn);
}

fn rand_bool(probability: f64) -> bool {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 / u32::MAX as f64) < probability
}

fn generate_sku() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    
    let prefix = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
        .into_iter()
        .nth((nanos as usize) % 26)
        .unwrap();
    
    let number = (nanos % 10000) as i32;
    let suffix = ['A', 'B', 'C', 'D', 'E'][((nanos / 10000) as usize) % 5];
    
    format!("{}-{:04}-{}", prefix, number, suffix)
}
