use diesel::prelude::*;
use domain::permissions::{NewPermission, Permission};
use infrastructure::establish_connection;
use rocket::{response::status::Created, serde::json::Json};

pub fn create_permission(form_data: Json<NewPermission>) -> Created<String> {
    use domain::schema::permissions;

    let new_perm = form_data.into_inner();

    let perm = Permission {
        title: new_perm.title,
        id: None,
    };

    match diesel::insert_into(permissions::table)
        .values(&perm)
        .get_result::<Permission>(&mut establish_connection())
    {
        Ok(_) => Created::new(""),
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
