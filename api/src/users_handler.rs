use application::user;
use domain::users::{NewUser, User};
use revolt_rocket_okapi::openapi;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::response_models::{Response, ResponseBody};

#[openapi]
#[get("/users")]
pub fn list_users() -> String {
    let users: Vec<User> = user::list_users();
    let response = Response {
        body: ResponseBody::Users(users),
    };

    serde_json::to_string(&response).unwrap()
}

#[openapi]
#[get("/users/<user_id>")]
pub fn list_user_by_id(user_id: i32) -> Result<String, NotFound<String>> {
    let user: User = user::list_user_by_id(user_id)?;
    let response = Response {
        body: ResponseBody::User(user),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[openapi]
#[post("/user", format = "json", data = "<user>")]
pub fn create_user(user: Json<NewUser>) -> Created<String> {
    user::create_user(user)
}
