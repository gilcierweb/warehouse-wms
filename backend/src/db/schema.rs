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
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        #[max_length = 255]
        full_name -> Nullable<Varchar>,
        #[max_length = 255]
        nickname -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        birthday -> Nullable<Date>,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        phone -> Nullable<Int8>,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        encrypted_password -> Varchar,
        #[max_length = 255]
        reset_password_token -> Nullable<Varchar>,
        reset_password_sent_at -> Nullable<Timestamp>,
        remember_created_at -> Nullable<Timestamp>,
        sign_in_count -> Int4,
        current_sign_in_at -> Nullable<Timestamp>,
        last_sign_in_at -> Nullable<Timestamp>,
        current_sign_in_ip -> Nullable<Varchar>,
        last_sign_in_ip -> Nullable<Varchar>,
        role -> Int4,
        status -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(movements -> slots (slot_id));
diesel::joinable!(movements -> users (operator_id));
diesel::joinable!(profiles -> users (user_id));
diesel::joinable!(slots -> users (updated_by));

diesel::allow_tables_to_appear_in_same_query!(
    alert_configs,
    movements,
    profiles,
    slots,
    users,
);
