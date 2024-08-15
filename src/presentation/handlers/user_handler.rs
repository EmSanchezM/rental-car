use actix_web::{get, post, web, HttpResponse};
use diesel::Insertable;
use serde::Deserialize;
use log::error;

use crate::schema::users;

use crate::{application::use_cases::{get_user::GetUserUseCase, register_user::RegisterUserUseCase}, infrastructure::repositories::postgres_user_repository::PostgresUserRepository};

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub address: String
}

#[post("/")]
pub async fn register_user_handler(repo: web::Data<PostgresUserRepository>, input: web::Json<NewUser>) -> HttpResponse {
    match RegisterUserUseCase::new(repo.into_inner())
        .execute(input.into_inner()).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(ex) => {
                error!("Error registering user! {:?}", ex);
                HttpResponse::InternalServerError().body("Please try again... ")
            }
        }
}

#[get("/{email}")]
pub async fn get_by_email_handler(repo: web::Data<PostgresUserRepository>, path: web::Path<(String, )>) -> HttpResponse {
    match GetUserUseCase::new(repo.into_inner()).get(path.into_inner().0).await {
        Some(customer) => HttpResponse::Ok().json(customer),
        None => {
            error!("Error obtain user!");
            HttpResponse::InternalServerError().body("Please try again...")
        }
    }
}