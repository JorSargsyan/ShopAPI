#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::{Identifiable, Insertable, Queryable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::{credentials, users};

#[derive(Queryable, Debug, Identifiable, Serialize, Deserialize, Insertable)]
#[diesel(primary_key(id))]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct User {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub role_id: i32,
    pub credential_id: i32,
    pub created_at: String,
}

#[derive(Queryable, Debug)]
pub struct Credential {
    pub id: i32,
    pub password: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct NewUser {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub role_id: i32,
}
