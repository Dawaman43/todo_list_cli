use serde::{Deserialize, Serialize};
use tabled::Tabled;
#[derive(Deserialize, Serialize, Tabled, Debug)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}
