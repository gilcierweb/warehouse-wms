// @generated automatically by Diesel CLI.

diesel::table! {
    alert_configs (id) {
        id -> Uuid,
        threshold_pct -> Int2,
        notify_browser -> Bool,
        notify_email -> Bool,
        #[max_length = 200]
        email_recipient -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    audit_logs (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        #[max_length = 100]
        action -> Varchar,
        #[max_length = 100]
        resource_type -> Nullable<Varchar>,
        resource_id -> Nullable<Uuid>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    movements (id) {
        id -> Uuid,
        slot_id -> Nullable<Uuid>,
        movement_type -> Int4,
        operator_id -> Nullable<Uuid>,
        #[max_length = 80]
        operator_name -> Nullable<Varchar>,
        #[max_length = 100]
        sku -> Nullable<Varchar>,
        note -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    profiles (id) {
        id -> Uuid,
        first_name_enc -> Nullable<Bytea>,
        last_name_enc -> Nullable<Bytea>,
        phone_enc -> Nullable<Bytea>,
        full_name -> Nullable<Bytea>,
        #[max_length = 255]
        nickname -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        birthday -> Nullable<Date>,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        phone -> Nullable<Int8>,
        social_network -> Jsonb,
        status -> Bool,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        token_hash -> Varchar,
        device_info -> Nullable<Text>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        expires_at -> Timestamptz,
        revoked_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    roles (id) {
        id -> Uuid,
        name -> Varchar,
        resource_type -> Nullable<Varchar>,
        resource_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    slots (id) {
        id -> Uuid,
        #[max_length = 12]
        address -> Varchar,
        #[max_length = 1]
        street -> Bpchar,
        position -> Int2,
        #[max_length = 3]
        lane -> Varchar,
        #[max_length = 10]
        status -> Varchar,
        #[max_length = 100]
        sku -> Nullable<Varchar>,
        updated_by -> Nullable<Uuid>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        reset_password_token -> Nullable<Varchar>,
        reset_password_sent_at -> Nullable<Timestamptz>,
        remember_created_at -> Nullable<Timestamptz>,
        sign_in_count -> Int4,
        current_sign_in_at -> Nullable<Timestamptz>,
        last_sign_in_at -> Nullable<Timestamptz>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_ip -> Nullable<Varchar>,
        confirmation_token -> Nullable<Varchar>,
        confirmed_at -> Nullable<Timestamptz>,
        confirmation_sent_at -> Nullable<Timestamptz>,
        unconfirmed_email -> Nullable<Varchar>,
        failed_attempts -> Int4,
        unlock_token -> Nullable<Varchar>,
        locked_at -> Nullable<Timestamptz>,
        totp_secret -> Nullable<Varchar>,
        totp_enabled -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(audit_logs -> users (user_id));
diesel::joinable!(movements -> slots (slot_id));
diesel::joinable!(movements -> users (operator_id));
diesel::joinable!(profiles -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(slots -> users (updated_by));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    alert_configs,
    audit_logs,
    movements,
    profiles,
    refresh_tokens,
    roles,
    slots,
    user_roles,
    users,
);
