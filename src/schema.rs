// @generated automatically by Diesel CLI.

diesel::table! {
    cars (id) {
        id -> Int4,
        car_number -> Int4,
        car_model -> Varchar,
        car_color -> Varchar,
        car_status -> Varchar,
        rent_prize -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rentals (id) {
        id -> Int4,
        rental_number -> Nullable<Varchar>,
        rental_date -> Nullable<Timestamp>,
        departure_time -> Nullable<Timestamp>,
        arrival_time -> Nullable<Timestamp>,
        accommodation_date -> Nullable<Timestamp>,
        return_date -> Nullable<Timestamp>,
        capacity -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        transaction_name -> Varchar,
        transaction_date -> Nullable<Timestamp>,
        transaction_status -> Varchar,
        card_id -> Int4,
        rental_id -> Int4,
        user_id -> Int4,
        payment_transaction_id -> Varchar,
        payment_amount -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        phone_number -> Varchar,
        address -> Varchar,
    }
}

diesel::joinable!(transactions -> cars (card_id));
diesel::joinable!(transactions -> rentals (rental_id));
diesel::joinable!(transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cars,
    rentals,
    transactions,
    users,
);
