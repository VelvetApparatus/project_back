use serde::{Serialize, Deserialize};
use uuid::Uuid;

/*
struct for Response to Front
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub body: String,
    pub sender: String,
    pub channel_id: Uuid
}