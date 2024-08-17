use diesel::prelude::*;
use serde::Serialize;
use chrono::NaiveDateTime;

use crate::schema::transactions;

use crate::domain::entities::rentals::Rental;
use crate::domain::entities::user::User;
use crate::domain::entities::car::Car;

#[derive(Queryable, Selectable, Serialize, Identifiable, Associations, Debug, Clone, PartialEq)]
#[diesel(table_name = transactions)]
#[diesel(belongs_to(Rental))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Car))]
pub struct Transaction {
    pub id: i32,
    pub transaction_name: String,
    pub transaction_status: String,
    pub transaction_date: NaiveDateTime,
    pub car_id: i32,
    pub rental_id: i32,
    pub user_id: i32,
    pub payment_transaction_id: String,
    pub payment_amount: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}