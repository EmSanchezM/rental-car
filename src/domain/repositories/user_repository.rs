use async_trait::async_trait;
use crate::domain::entities::user::User;
use crate::presentation::dtos::users_dto::NewUser;

#[async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: String) -> Option<User>;

    async fn save(&self, user: &NewUser) -> Result<(), diesel::result::Error>;
}