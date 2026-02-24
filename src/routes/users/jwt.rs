use std::sync::LazyLock;

use chrono::{Duration, Local};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use rocket::{
    Request, async_trait,
    http::Status,
    request::{FromRequest, Outcome},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const JWT_STR: &str = "JWT";

pub static JWT_KEY: LazyLock<&str> = LazyLock::new(|| "");

/// This is the struct used inside the JWT
/// It implements FromRequest, so you can check if a user is signed in with the following:
/// ```no_run
/// use rocket::get;
/// use vinting_server::routes::users::jwt::JwtClaims;
///
///
/// #[get("/")]
/// fn a_route(
///     user_claims: JwtClaims
/// ) -> &'static str {
///     "You are logged in"
/// }
/// ```
/// But since the db is not accessable from within the request,
/// you still need to verify if the user exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    exp: i64,
    iat: i64,
    uid: i32,
}

#[derive(Debug, Clone, Error)]
pub enum JwtError {
    #[error("Could not find a jwt")]
    Missing,
    #[error("There was an error during jwt parsing: {0}")]
    Decoding(String),
    #[error("The jwt has expired")]
    Expired,
}

#[async_trait]
impl<'a> FromRequest<'a> for JwtClaims {
    type Error = JwtError;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let jar = request.cookies();
        let Some(jwt) = jar.get("jwt") else {
            return Outcome::Error((Status::Unauthorized, JwtError::Missing));
        };

        let res = decode::<JwtClaims>(
            jwt.to_string(),
            &DecodingKey::from_rsa_der(&[]),
            &Validation::default(),
        )
        .map_err(|err| match err.clone().into_kind() {
            ErrorKind::ExpiredSignature => (Status::Unauthorized, JwtError::Expired),
            _ => (Status::Unauthorized, JwtError::Decoding(err.to_string())),
        });

        let jwt = match res {
            Ok(val) => val,
            Err(err) => return Outcome::Error(err),
        };

        Outcome::Success(jwt.claims)
    }
}

pub fn make_jwt(uid: i32, _secret: String) -> Result<String, jsonwebtoken::errors::Error> {
    let mut header = Header::new(Algorithm::HS256);
    header.typ = Some(JWT_STR.to_string());

    let now = Local::now();
    let iat = now.timestamp();
    let exp = (now + Duration::days(30)).timestamp();

    let claims = JwtClaims { exp, iat, uid };

    // TODO: key
    encode(&header, &claims, &EncodingKey::from_rsa_der(&[]))
}
