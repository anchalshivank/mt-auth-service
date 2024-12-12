// @generated automatically by Diesel CLI.

diesel::table! {
    machines (id) {
        id -> Int4,
        under_maintenance -> Bool,
        eligible_for_use -> Nullable<Bool>,
        last_service -> Timestamp,
        next_service -> Timestamp,
        last_serviced_by -> Nullable<Int4>,
    }
}

diesel::table! {
    reporting (id) {
        id -> Int4,
        #[max_length = 50]
        flight_number -> Varchar,
        time_of_reporting -> Timestamp,
        ba_reading -> Float8,
        #[max_length = 50]
        medical_personal_id -> Varchar,
        remarks -> Nullable<Text>,
        user_id -> Int4,
    }
}

diesel::table! {
    service_history (id) {
        id -> Int4,
        machine_id -> Int4,
        user_id -> Int4,
        service_date -> Timestamp,
        service_notes -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 150]
        username -> Varchar,
        #[max_length = 150]
        email -> Varchar,
        password -> Text,
        #[max_length = 150]
        staff_no -> Varchar,
        #[max_length = 150]
        license_no -> Varchar,
        digi_signature -> Text,
    }
}

diesel::joinable!(machines -> users (last_serviced_by));
diesel::joinable!(reporting -> users (user_id));
diesel::joinable!(service_history -> machines (machine_id));
diesel::joinable!(service_history -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    machines,
    reporting,
    service_history,
    users,
);
