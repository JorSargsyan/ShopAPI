use application::role;
use domain::roles::{CreateRoleRequest, Role, UpdateRoleRequest};
use revolt_rocket_okapi::openapi;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use shared::response_models::{Response, ResponseBody};

#[openapi]
#[get("/roles")]
pub fn list_roles() -> String {
    let roles: Vec<Role> = role::list_roles();
    let response: Response = Response {
        body: ResponseBody::Roles(roles),
    };

    serde_json::to_string(&response).unwrap()
}

#[openapi]
#[post("/role", format = "json", data = "<role>")]
pub fn create_role(role: Json<CreateRoleRequest>) -> Created<String> {
    role::create_role(role)
}

#[openapi]
#[put("/role", format = "json", data = "<role>")]
pub fn update_role(role: Json<UpdateRoleRequest>) -> String {
    let new_role = role::update_role(role);
    let response: Response = Response {
        body: ResponseBody::Role(new_role),
    };

    serde_json::to_string(&response).unwrap()
}

#[openapi]
#[delete("/role/<id>")]
pub fn remove_role(id: i32) -> String {
    let roles = role::remove_role(id);
    let response: Response = Response {
        body: ResponseBody::Roles(roles),
    };

    serde_json::to_string(&response).unwrap()
}
