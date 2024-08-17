use diesel::Insertable;
use serde::Deserialize;
use crate::schema::users;

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub address: String
}
