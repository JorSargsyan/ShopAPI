use diesel::prelude::*;
use domain::models::{NewUser, User};
use infrastructure::establish_connection;
use rocket::{response::status::Created, serde::json::Json};
use shared::response_models::{Response, ResponseBody};

pub fn create_user(form_data: Json<NewUser>) -> Created<String> {
    use domain::schema::users;
    let new_user = form_data.into_inner();

    let user = User {
        name: new_user.name,
        lastname: new_user.lastname,
        email: new_user.email,
        role_id: new_user.role_id,
        id: 10,
        created_at: String::from("10"),
        credential_id: 10,
    };

    match diesel::insert_into(users::table)
        .values(&user)
        .get_result::<User>(&mut establish_connection())
    {
        Ok(user) => {
            let response = Response {
                body: ResponseBody::User(user),
            };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
