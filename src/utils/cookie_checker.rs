use actix_web::{web::Data, HttpRequest};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::chat::user::User;


pub enum CheckResult {
    Unauthorized,
    Success(User),
    BadGateway
}




pub async fn check(
    pool: &Data<PgPool>,
    request: &HttpRequest
) -> CheckResult {
    match request.cookie("auth") {
        // If cookie is empty => user is not allowed to create
        None => CheckResult::Unauthorized,

        Some(cookie) => {
            let user_id = cookie.value();
            match Uuid::parse_str(user_id) {
                // If we can't convert cookie value to Uuid => user is unathorized
                Err(_) => CheckResult::Unauthorized,
        
                Ok(user_id) => {
                     match User::get_by_user_id(&user_id, &pool).await {
                        // Some problems with Database
                        Err(_) => CheckResult::BadGateway,
        
                        Ok(res) => {
                            match res.first() {
                                // If we can't find current user => user is unauthorized
                                None => CheckResult::Unauthorized,
                                
                                Some(user) => {
                                    CheckResult::Success(user.clone())
                                }
                            }
                        }
                     }
                }
                
            }

        }
    }
}