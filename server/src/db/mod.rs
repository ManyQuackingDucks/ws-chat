pub mod model;
pub mod scheme;

use diesel::prelude::*;
use bb8_diesel::DieselConnection;
use bb8::Pool;
use model::User;
use tokio::task;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use self::model::InsertUser;
use self::scheme::users;

pub type ConnType<'a> = bb8::PooledConnection<'a, bb8_diesel::DieselConnectionManager<diesel::SqliteConnection>>;
pub type PoolType = Pool<bb8_diesel::DieselConnectionManager<diesel::SqliteConnection>>;
pub async fn establish_connection(db_conn: &str) -> Pool<bb8_diesel::DieselConnectionManager<diesel::SqliteConnection>> {
    let mgr = bb8_diesel::DieselConnectionManager::<diesel::SqliteConnection>::new(db_conn);
    bb8::Pool::builder().build(mgr).await.unwrap()
}
#[allow(dead_code)]
pub async fn create_user<'a>(conn: ConnType<'a>, user: &str, pass: String, admin: bool) -> anyhow::Result<()> {
    let pass_hash = tokio::task::spawn_blocking(move ||create_hash(pass.to_string()).unwrap()).await?;
    let new_user = InsertUser {
        username: user,
        pass_hash:  &pass_hash,
        admin,
    };
    diesel::insert_into(scheme::users::table)
        .values(&new_user)
        .execute(&*conn)?;
    Ok(())
}
fn create_hash(pass: String) -> anyhow::Result<String>{
    Ok(Argon2::default().hash_password(pass.as_bytes(), &SaltString::generate(&mut OsRng)).unwrap().to_string())

}
#[allow(dead_code)]
pub fn delete_user<'a>(conn: ConnType<'a>, user: &str) -> anyhow::Result<()> {
    use scheme::users::dsl::{username, users};
    diesel::delete(users.filter(username.eq(user))).execute(&*conn)?;
    Ok(())
}
pub async fn auth_user<'a>(conn: ConnType<'a>, user: crate::types::LoginUser) -> Option<crate::types::LoggedInUser> {
    use crate::db::users::dsl::users;
    let user_struct: User = match users.find(user.username).first(&*conn) {
        Ok(e) => e,
        Err(_) => return None,
    };
    tokio::task::spawn_blocking(|| _auth_user(user_struct, user.password)).await.unwrap()
}
fn _auth_user(
    user_struct: User,
    user_pass: String
) -> Option<crate::types::LoggedInUser> {
    if valid_pass(user_pass, user_struct.pass_hash) {
        Some(crate::types::LoggedInUser {
            username: user_struct.username,
            admin: user_struct.admin,
        })
    } else {
        None
    }
}

fn valid_pass(pass: String, hash: String) -> bool {
    let hash = PasswordHash::new(&hash).unwrap();
    Argon2::default().verify_password(pass.as_bytes(), &hash).is_ok()
}
