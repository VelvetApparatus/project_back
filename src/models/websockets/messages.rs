use actix::prelude::{Message, Recipient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::chat;


#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}

#[derive(Message, Clone, Serialize, Deserialize, Debug)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Vec<Uuid>,
    pub msg: chat::message::Message,
}
/*
{
    "id": [
        "51e3a4ef-dc75-4534-9efe-ceb5cce8a994"
    ],
    "msg": {
        "message_id": "51e3a4ef-dc75-4534-9efe-ceb5cce8a994",
        "user_id": "51e3a4ef-dc75-4534-9efe-ceb5cce8a994",
        "channel_id": "51e3a4ef-dc75-4534-9efe-ceb5cce8a994",
        "body": "some_message_body_here",
        "is_reply": true,
        "created_at": "2023-12-10T12:34:56.789012345"

    }
}
*/