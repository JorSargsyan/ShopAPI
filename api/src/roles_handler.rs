use application::role::{create, read};
use domain::models::{NewRole, Role};
use revolt_rocket_okapi::openapi;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{get, post};
use shared::response_models::{Response, ResponseBody};

#[openapi]
#[get("/roles")]
pub fn list_roles() -> String {
    let roles: Vec<Role> = read::list_roles();
    let response: Response = Response {
        body: ResponseBody::Roles(roles),
    };

    serde_json::to_string(&response).unwrap()
}

#[openapi]
#[post("/role", format = "json", data = "<role>")]
pub fn create_role(role: Json<NewRole>) -> Created<String> {
    create::create_role(role)
}
