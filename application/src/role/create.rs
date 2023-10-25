use diesel::prelude::*;
use domain::roles::{NewRole, Role, RolePermission};
use infrastructure::establish_connection;
use rocket::{response::status::Created, serde::json::Json};

pub fn create_role(form_data: Json<NewRole>) -> Created<String> {
    use domain::schema::role_permissions;
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
        Ok(role) => {
            let mut role_perms: Vec<RolePermission> = vec![];

            for item in new_role.permission_ids.into_iter() {
                let role_perm = RolePermission {
                    id: None,
                    role_id: role.id.unwrap(),
                    permission_id: item,
                };

                role_perms.push(role_perm)
            }

            match diesel::insert_into(role_permissions::table)
                .values(&role_perms)
                .execute(&mut establish_connection())
            {
                Ok(_) => Created::new(""),
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
