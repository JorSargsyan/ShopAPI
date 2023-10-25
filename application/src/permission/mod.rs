use diesel::prelude::*;
use domain::permissions::{NewPermission, Permission};
use infrastructure::establish_connection;
use rocket::{response::status::Created, serde::json::Json};

pub fn list_permissions() -> Vec<Permission> {
    use domain::schema::permissions;

    match permissions::table
        .select(permissions::all_columns)
        .load::<Permission>(&mut establish_connection())
    {
        Ok(perm) => perm,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

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
