use domain::models::UserResponse;
use rocket::serde::Serialize;
#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    User(UserResponse),
    Users(Vec<UserResponse>),
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
