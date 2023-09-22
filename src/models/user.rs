use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String
}


impl User {
    pub fn new_message(&self, body: String) -> String {
        format!("{}: {}", self.name, body)
    }

    pub fn enter_channel(self, group: &str) -> String {
        format!("{} is now in {} channel", self.name, group)
    }

    pub fn leave_channel(&self, group: &str) -> String {
        format!("{} is no longer in {} channel", self.name, group)
    }
}