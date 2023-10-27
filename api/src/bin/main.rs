#[macro_use]
extern crate rocket;
use api::permissions_handler;
use api::roles_handler;
use api::users_handler;
use revolt_rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/api/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            openapi_get_routes![
                users_handler::list_users,
                users_handler::list_user_by_id,
                users_handler::create_user,
                roles_handler::list_roles,
                roles_handler::update_role,
                roles_handler::create_role,
                roles_handler::remove_role,
                permissions_handler::create_permission,
                permissions_handler::list_permissions,
                permissions_handler::update_permission,
                permissions_handler::remove_permission
            ],
        )
        .mount("/swagger", make_swagger_ui(&get_docs()))
}
