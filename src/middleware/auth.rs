use crate::config::Config;
use crate::utils::jwt::verify_jwt;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct AuthenticatedUser {
    pub user_id: String,
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let config = match request.rocket().state::<Config>() {
            Some(config) => config,
            None => return Outcome::Error((Status::InternalServerError, "Config not found")),
        };

        let auth_header = match request.headers().get_one("Authorization") {
            Some(header) => header,
            None => return Outcome::Error((Status::Unauthorized, "Missing Authorization header")),
        };

        if !auth_header.starts_with("Bearer ") {
            return Outcome::Error((Status::Unauthorized, "Invalid Authorization header format"));
        }

        let token = &auth_header[7..]; // Remove "Bearer " prefix

        match verify_jwt(token, &config.jwt_secret) {
            Ok(claims) => Outcome::Success(AuthenticatedUser {
                user_id: claims.sub,
                email: claims.email,
            }),
            Err(_) => Outcome::Error((Status::Unauthorized, "Invalid or expired token")),
        }
    }
}
