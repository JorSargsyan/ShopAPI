use diesel::prelude::*;
use domain::roles::{NewRole, Role};
use infrastructure::establish_connection;
use rocket::{response::status::Created, serde::json::Json};

pub fn create_role(form_data: Json<NewRole>) -> Created<String> {
    use domain::schema::roles;
    let new_role = form_data.into_inner();

    let role = Role {
        title: new_role.title,
        id: None,
    };

    match diesel::insert_into(roles::table)
        .values(&role)
        .get_result::<Role>(&mut establish_connection())
    {
        Ok(role) => Created::new(""),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
