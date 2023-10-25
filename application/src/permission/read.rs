use diesel::prelude::*;
use domain::permissions::Permission;
use infrastructure::establish_connection;

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
