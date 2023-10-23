use application::user::{create, read};
use domain::models::{NewUser, User};
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::response_models::{Response, ResponseBody};

#[get("/")]
pub fn list_users() -> String {
    let users: Vec<User> = read::list_users();
    let response = Response {
        body: ResponseBody::Users(users),
    };

    serde_json::to_string(&response).unwrap()
}

#[get("/users/<user_id>")]
pub fn list_user_by_id(user_id: i32) -> Result<String, NotFound<String>> {
    let user: User = read::list_user_by_id(user_id)?;
    let response = Response {
        body: ResponseBody::User(user),
    };

    Ok(serde_json::to_string(&response).unwrap())
}

#[post("/user", format = "json", data = "<user>")]
pub fn create_user(user: Json<NewUser>) -> Created<String> {
    create::create_user(user)
}
