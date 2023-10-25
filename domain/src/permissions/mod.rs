#![allow(unused)]
#![allow(clippy::all)]

use diesel::prelude::{Identifiable, Insertable, Queryable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::permissions;

#[derive(Queryable, Debug)]
pub struct Permission {
    pub id: i32,
    pub title: String,
}
