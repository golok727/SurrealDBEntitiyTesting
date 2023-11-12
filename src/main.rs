use std::sync::Arc;

use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

mod entity;
use entity::BaseEntity;
mod user;
use user::User;
mod lib;
static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    DB.connect::<Ws>("127.0.0.1:8000").await?;

    // db.signin(Root {
    //     username: "root",
    //     password: "root",
    // })
    // .await?;

    DB.use_ns("development").use_db("test").await?;

    // let new_user = User::new("Radha".into(), "radha_krsna@golok.vrindavan".into(), 19);
    // dbg!(&new_user);

    // let db = DB.clone();
    // let db = Arc::new(db.clone());
    // let data: Value = new_user.clone().into();
    // let created_user = new_user.create(db.clone(), data).await?;
    // dbg!(&created_user);

    // let db = DB.clone();
    // let db = Arc::new(db.clone());
    // let users = User::get_all(db).await?;
    // dbg!(users);

    println!("Radhey Shyam");
    Ok(())
}
