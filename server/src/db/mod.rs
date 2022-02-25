pub mod scheme;
pub mod model;

use diesel::r2d2::Pool;
use model::User;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Builder;


use self::scheme::users;
pub type DbConnType =  diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    Builder::new().build( ConnectionManager::new("file:db.sqlite")).expect("Could not create pool")
}
#[allow(dead_code)]
pub fn create_user(conn: &DbConnType , user: String, pass: String, admin: bool) -> anyhow::Result<()>{
    let pass_hash = pass; //Replace with hashing function
    let new_user = User {username: user, pass_hash, admin};

    diesel::insert_into(scheme::users::table)
        .values(&new_user)
        .execute(conn)?;
    Ok(())
}
#[allow(dead_code)]
pub fn delete_user(conn: &DbConnType, user: String) -> anyhow::Result<()>{
    use scheme::users::dsl::*;
    diesel::delete(users.filter(username.eq(user)))
        .execute(conn)?;
    Ok(())
}


pub fn auth_user(conn: &DbConnType, user: crate::types::LoginUser) -> Option<crate::types::LoggedInUser>{
    use crate::db::users::dsl::*;
    let user_struct: User = match users.find(user.username).first(conn){
        Ok(e) => e,
        Err(_) => return None,
    };
    if user_struct.pass_hash == hash(user.password){
        Some(crate::types::LoggedInUser {username: user_struct.username, admin: user_struct.admin})
    } else {
        None
    }
    
}

fn hash(pass: String) -> String{
    //Hashing will be implemented later
    pass
}