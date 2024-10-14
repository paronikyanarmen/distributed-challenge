use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: MessageBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MessageBody {
    #[serde(rename = "init")]
    Init {
        node_id: Option<String>,

        #[serde(flatten)]
        meta: Option<MessageMeta>,
    },

    #[serde(rename = "init_ok")]
    InitOk {
        #[serde(flatten)]
        meta: Option<MessageMeta>,
    },
    #[serde(rename = "echo")]
    Echo {
        echo: String,

        #[serde(flatten)]
        meta: Option<MessageMeta>,
    },

    #[serde(rename = "echo_ok")]
    EchoOk {
        echo: String,

        #[serde(flatten)]
        meta: Option<MessageMeta>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageMeta {
    pub msg_id: Option<u64>,
    pub in_reply_to: Option<u64>,
}
