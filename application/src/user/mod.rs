use diesel::prelude::*;
use domain::users::{NewUser, User};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use rocket::{response::status::Created, serde::json::Json};
use shared::response_models::{Response, ResponseBody};

pub fn list_user_by_id(user_id: i32) -> Result<User, NotFound<String>> {
    use domain::schema::users;

    match users::table
        .find(user_id)
        .first::<User>(&mut establish_connection())
    {
        Ok(user) => Ok(user),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting user with id {} - {}",
                        user_id, err
                    )),
                };
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_users() -> Vec<User> {
    use domain::schema::users;

    match users::table
        .select(users::all_columns)
        .load::<User>(&mut establish_connection())
    {
        Ok(users) => users,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn create_user(form_data: Json<NewUser>) -> Created<String> {
    use domain::schema::users;
    let new_user = form_data.into_inner();

    let user = User {
        id: None,
        name: new_user.name,
        lastname: new_user.lastname,
        email: new_user.email,
        role_id: new_user.role_id,
        credential_id: 10,
        created_at: String::from("10"),
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
