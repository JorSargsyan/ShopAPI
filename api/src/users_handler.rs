use domain::models::UserResponse;
use rocket::get;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};

#[get("/")]
pub fn list_users() -> String {
    let users: Vec<User> = read::list_users();
}

#[get("/users/<user_id>")]
pub fn list_user_by_id(user_id: i32) -> Result<String, NotFound<String>> {
    todo!()
}
