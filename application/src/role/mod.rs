use core::panic;
use diesel::prelude::*;
use domain::{
    roles::{CreateRoleRequest, Role, RolePermission, UpdateRoleRequest},
    schema::{
        role_permissions::{permission_id, role_id},
        roles::{id, title},
    },
};
use infrastructure::establish_connection;
use rocket::{response::status::Created, serde::json::Json};
use std::collections::HashSet;

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
        Ok(role) => {
            let current_permissions = role_permissions::table
                .filter(role_permissions::role_id.eq(role.id.unwrap()))
                .select(role_permissions::permission_id)
                .get_results::<i32>(&mut establish_connection());

            match current_permissions {
                Ok(perms) => {
                    let new_permissions_set: HashSet<i32> =
                        updated_role.permission_ids.iter().cloned().collect();
                    let current_permissions_set: HashSet<i32> = perms.iter().cloned().collect();
                    let permissions_to_add: Vec<i32> = new_permissions_set
                        .difference(&current_permissions_set)
                        .cloned()
                        .collect();
                    let permissions_to_remove: Vec<i32> = current_permissions_set
                        .difference(&new_permissions_set)
                        .cloned()
                        .collect();

                    //insert new permissions
                    for permission in permissions_to_add.iter() {
                        let _ = diesel::insert_into(role_permissions::table)
                            .values((role_id.eq(role.id.unwrap()), permission_id.eq(*permission)))
                            .execute(&mut establish_connection());
                    }

                    // Remove permissions no longer associated
                    for permission in permissions_to_remove.iter() {
                        let _ = diesel::delete(
                            role_permissions::table.filter(
                                role_id
                                    .eq(role.id.unwrap())
                                    .and(permission_id.eq(*permission)),
                            ),
                        )
                        .execute(&mut establish_connection());
                    }

                    role
                }
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

pub fn remove_role(delete_id: i32) -> Vec<Role> {
    use domain::schema::roles;

    let delete_query =
        diesel::delete(roles::table.filter(id.eq(delete_id))).execute(&mut establish_connection());

    match delete_query {
        Ok(_) => {
            let roles_list = roles::table
                .select(roles::all_columns)
                .get_results::<Role>(&mut establish_connection());

            match roles_list {
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
