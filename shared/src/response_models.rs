use domain::models::User;
use rocket::serde::Serialize;
#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    User(User),
    Users(Vec<User>),
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
