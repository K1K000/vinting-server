use argon2::Argon2;
use argon2::PasswordVerifier;
use dtos::user::{get::UserGetDto, post::UserPostDto};
use entity::prelude::*;
use entity::user;
use rocket::FromForm;
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::response::status::NoContent;
use rocket::{State, http::uri::Host, post, response::status::Created, serde::json::Json};
use sea_orm::DbConn;
use sea_orm::SelectExt;
use services::service_trait::ServiceFilter;
use services::user_service::UserService;

use crate::responder::Responder;
use crate::routes::users::jwt::JWT_KEY;
use crate::routes::users::jwt::JWT_STR;
use crate::routes::users::jwt::make_jwt;

#[derive(Debug, Clone, FromForm)]
pub struct LoginDetails<'a> {
    email: &'a str,
    password: &'a str,
}

#[post("/signup", format = "application/json", data = "<data>")]
pub async fn signup(
    host: &Host<'_>,
    db: &State<DbConn>,
    data: Json<UserPostDto>,
    jar: &CookieJar<'_>,
) -> Result<Created<Json<UserGetDto>>, Responder> {
    let db = db.inner();
    let user = data.into_inner();

    if User::find_by_email(user.email.clone())
        .service_filter::<UserService>()
        .exists(db)
        .await?
    {
        return Err(Responder::bad_request(
            "A user with that email already exists",
        ));
    }

    let am = user::ActiveModelEx::from(user);
    let user = am.insert(db).await?;

    add_jwt_to_jar(user.id, jar)?;

    Ok(Created::new(format!("{host}/api/users/{}", user.id)).body(Json(user.into())))
}

#[post("/login", data = "<data>")]
pub async fn login(
    db: &State<DbConn>,
    data: Form<LoginDetails<'_>>,
    jar: &CookieJar<'_>,
) -> Result<NoContent, Responder> {
    let db = db.inner();

    let user = User::find_by_email(data.email)
        .service_filter::<UserService>()
        .one(db)
        .await?
        .ok_or(Responder::not_found(
            "There is no user with the given email",
        ))?;

    let argon2 = Argon2::default();

    if argon2
        .verify_password(data.password.as_bytes(), user.password_hash.as_str())
        .is_err()
    {
        return Err(Responder::unauhorized("Wrong password"));
    }

    add_jwt_to_jar(user.id, jar)?;

    Ok(NoContent)
}

fn add_jwt_to_jar(uid: i32, jar: &CookieJar<'_>) -> Result<(), jsonwebtoken::errors::Error> {
    // TODO: key
    let jwt = make_jwt(uid, JWT_KEY.to_string())?;

    jar.add((JWT_STR, jwt));

    Ok(())
}
