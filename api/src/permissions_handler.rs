use application::permission;
use domain::permissions::{CreatePermissionRequest, Permission, UpdatePermissionRequest};
use revolt_rocket_okapi::openapi;
use rocket::{delete, get, post, put, response::status::Created, serde::json::Json};
use shared::response_models::{Response, ResponseBody};

#[openapi]
#[get("/permissions")]
pub fn list_permissions() -> String {
    let permissions: Vec<Permission> = permission::list_permissions();
    let response: Response = Response {
        body: ResponseBody::Permissions(permissions),
    };

    serde_json::to_string(&response).unwrap()
}

#[openapi]
#[post("/permission", format = "json", data = "<permission>")]
pub fn create_permission(permission: Json<CreatePermissionRequest>) -> Created<String> {
    permission::create_permission(permission)
}

#[openapi]
#[put("/permission", format = "json", data = "<permission>")]
pub fn update_permission(permission: Json<UpdatePermissionRequest>) -> String {
    let perm = permission::update_permission(permission);

    let response: Response = Response {
        body: ResponseBody::Permission(perm),
    };

    serde_json::to_string(&response).unwrap()
}

#[openapi]
#[delete("/permission/<id>")]
pub fn remove_permission(id: i32) -> String {
    let permissions = permission::remove_permission(id);
    let response: Response = Response {
        body: ResponseBody::Permissions(permissions),
    };

    serde_json::to_string(&response).unwrap()
}
