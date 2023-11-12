use crate::entity::BaseEntity;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{Thing, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub email_id: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlimUser {
    pub id: Option<Thing>,
    pub name: String,
}

impl User {
    pub fn new(name: String, email_id: String, age: u8) -> Self {
        Self {
            id: None,
            name,
            email_id,
            age,
        }
    }
}
impl From<User> for Value {
    fn from(user: User) -> Value {
        let mut data: BTreeMap<String, Value> = BTreeMap::new();

        data.insert("name".into(), user.name.clone().into());
        data.insert("email_id".into(), user.email_id.into());
        data.insert("age".into(), user.age.into());

        if let Some(id) = user.id {
            data.insert("id".into(), id.clone().into());
        }
        data.into()
    }
}

impl BaseEntity<User> for User {
    const TABLE_NAME: &'static str = "user";
}
