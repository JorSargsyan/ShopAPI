#![allow(unused)]
#![allow(clippy::all)]

use diesel::{
    prelude::{Associations, Identifiable, Insertable, Queryable},
    query_builder::QueryId,
    Selectable,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::{role_permissions, roles};

use crate::permissions::Permission;

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize, Insertable)]
#[diesel(primary_key(id))]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = roles)]
pub struct Role {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub title: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct NewRole {
    pub permission_ids: Vec<i32>,
    pub title: String,
}

#[derive(Queryable, Debug, Associations, Identifiable, Selectable, Insertable)]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(Permission))]
#[diesel(table_name= role_permissions)]
#[diesel(primary_key(id))]
pub struct RolePermission {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub permission_id: i32,
    pub role_id: i32,
}
