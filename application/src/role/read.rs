use diesel::prelude::*;
use domain::models::Role;
use infrastructure::establish_connection;

pub fn list_roles() -> Vec<Role> {
    use domain::schema::roles;

    match roles::table
        .select(roles::all_columns)
        .load::<Role>(&mut establish_connection())
    {
        Ok(roles) => roles,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
