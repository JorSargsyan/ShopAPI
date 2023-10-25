use application::permission::{create, read};
use domain::permissions::{NewPermission, Permission};
use revolt_rocket_okapi::openapi;
use rocket::{get, post, response::status::Created, serde::json::Json};
use shared::response_models::{Response, ResponseBody};

#[openapi]
#[get("/permissions")]
pub fn list_permissions() -> String {
    let permissions: Vec<Permission> = read::list_permissions();
    let response: Response = Response {
        body: ResponseBody::Permissions(permissions),
    };

    serde_json::to_string(&response).unwrap()
}

#[openapi]
#[post("/permission", format = "json", data = "<permission>")]
pub fn create_permission(permission: Json<NewPermission>) -> Created<String> {
    create::create_permission(permission)
}
