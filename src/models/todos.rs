use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Todos {
    pub todo: i32,
    pub name: String,
    pub is_done: bool
}