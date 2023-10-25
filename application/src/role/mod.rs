use diesel::prelude::*;
use domain::{
    roles::{CreateRoleRequest, Role, RolePermission, UpdateRoleRequest},
    schema::{roles::*, users::role_id},
};
use infrastructure::establish_connection;
use rocket::{response::status::Created, serde::json::Json};

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

pub fn create_role(form_data: Json<CreateRoleRequest>) -> Created<String> {
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
                let role_perm: RolePermission = RolePermission {
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

pub fn update_role(form_data: Json<UpdateRoleRequest>) -> Role {
    use domain::schema::role_permissions;
    use domain::schema::roles;
    let updated_role: UpdateRoleRequest = form_data.into_inner();

    match diesel::update(roles::table)
        .filter(id.eq(updated_role.id))
        .set(title.eq(updated_role.title))
        .get_result::<Role>(&mut establish_connection())
    {
        Ok(role) => role,
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}
