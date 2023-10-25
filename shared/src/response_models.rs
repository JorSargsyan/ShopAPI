use domain::roles::Role;
use domain::users::User;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    User(User),
    Roles(Vec<Role>),
    Users(Vec<User>),
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
