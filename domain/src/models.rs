// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Debug)]
pub struct Credential {
    pub id: i32,
    pub password: String,
}

#[derive(Queryable, Debug)]
pub struct Permission {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Debug)]
pub struct RolePermission {
    pub id: i32,
    pub role_id: Option<i32>,
    pub permission_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct Role {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub role_id: i32,
    pub credential_id: i32,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub role_id: i32,
}
