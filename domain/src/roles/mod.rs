#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::{Identifiable, Insertable, Queryable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::{role_permissions, roles};

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

#[derive(Queryable, Debug)]
pub struct RolePermission {
    pub id: i32,
    pub role_id: Option<i32>,
    pub permission_id: Option<i32>,
}
