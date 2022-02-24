#[macro_use]
extern crate diesel;
use diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection {

    SqliteConnection::establish("/db.sqlite")
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str) -> usize {
    use schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}