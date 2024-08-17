use diesel::prelude::Queryable;
use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct Car {
    pub id: i32,
    pub car_number: String,
    pub car_model: String,
    pub car_color: String,
    pub car_status: String,
    rent_prize:    f64,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}