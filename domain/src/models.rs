use crate::schema::{credentials, permissions, roles, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub role_id: i32,
    pub credential_id: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub role_id: i32,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Role {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub title: String,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Permission {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = permissions)]
pub struct NewPermission {
    pub title: String,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct Credential {
    pub id: i32,
    pub password: String,
}
