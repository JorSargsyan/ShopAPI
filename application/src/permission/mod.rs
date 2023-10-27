use diesel::prelude::*;
use domain::{
    permissions::{CreatePermissionRequest, Permission, UpdatePermissionRequest},
    schema::permissions::{id, title},
};
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

pub fn create_permission(form_data: Json<CreatePermissionRequest>) -> Created<String> {
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

pub fn update_permission(form_data: Json<UpdatePermissionRequest>) -> Permission {
    use domain::schema::permissions;
    let new_perm = form_data.into_inner();

    match diesel::update(permissions::table)
        .filter(id.eq(new_perm.id))
        .set(title.eq(new_perm.title))
        .get_result::<Permission>(&mut establish_connection())
    {
        Ok(perm) => perm,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn remove_permission(delete_id: i32) -> Vec<Permission> {
    use domain::schema::permissions;

    let delete_query = diesel::delete(permissions::table.filter(id.eq(delete_id)))
        .execute(&mut establish_connection());

    match delete_query {
        Ok(_) => {
            let perm_list = permissions::table
                .select(permissions::all_columns)
                .get_results::<Permission>(&mut establish_connection());

            match perm_list {
                Ok(list) => list,
                Err(err) => match err {
                    _ => {
                        panic!("Database error - {}", err);
                    }
                },
            }
        }
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
