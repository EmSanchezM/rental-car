use diesel::prelude::Queryable;
use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Rental {
    pub id: i32,
    pub rental_number: String,
    pub rental_date: NaiveDateTime,
    pub departure_time: NaiveDateTime,
    pub arrival_time: NaiveDateTime,
    accommodation_date: NaiveDateTime,
    return_date: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}