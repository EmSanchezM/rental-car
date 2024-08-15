use diesel::prelude::Queryable;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub address: String,
}
