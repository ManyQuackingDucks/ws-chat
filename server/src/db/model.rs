use diesel::{Insertable, Queryable};

use super::scheme::users;


#[derive(Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub username: String,
    pub pass_hash: String,
    pub admin: bool,
}