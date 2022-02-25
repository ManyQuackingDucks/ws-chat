use diesel::{Insertable, Queryable};

use super::scheme::users;

#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub pass_hash: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub(super) struct InsertUser<'a> {
    pub username: &'a str,
    pub pass_hash: &'a str,
    pub admin: bool,
}