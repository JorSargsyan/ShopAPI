#[macro_use]
extern crate rocket;
use api::users_handler;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![users_handler::list_users, users_handler::list_user_by_id],
    )
}
