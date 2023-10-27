#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::{Identifiable, Insertable, Queryable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::permissions;

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize, Insertable)]
#[diesel(primary_key(id))]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = permissions)]
pub struct Permission {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub title: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct CreatePermissionRequest {
    pub title: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct UpdatePermissionRequest {
    pub title: String,
    pub id: i32,
}
