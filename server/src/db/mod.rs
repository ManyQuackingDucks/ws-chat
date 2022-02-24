pub mod scheme;
pub mod model;
use model::User;

use diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection {

    SqliteConnection::establish("file:db.sqlite")
        .unwrap_or_else(|_| panic!("Error connecting to {}", "file:db.sqlite"))
}

pub fn create_user(conn: &SqliteConnection, user: String, pass: String) -> usize {
    let pass_hash = pass; //Replace with hashing function
    let new_user = User {username: user, pass_hash};

    diesel::insert_into(scheme::users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new post")
}