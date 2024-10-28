use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: MessageBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageBody {
    pub msg_id: Option<u64>,
    pub in_reply_to: Option<u64>,

    #[serde(flatten)]
    pub type_specific: MessageTypeData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MessageTypeData {
    #[serde(rename = "init")]
    Init {
        node_id: Option<String>,
    },

    #[serde(rename = "init_ok")]
    InitOk {},

    #[serde(rename = "echo")]
    Echo {
        echo: String,
    },

    #[serde(rename = "echo_ok")]
    EchoOk {
        echo: String
    },

    #[serde(rename = "generate")]
    Generate {},

    #[serde(rename = "generate_ok")]
    GenerateOk {
        id: String,
    },

    #[serde(rename = "broadcast")]
    Broadcast {
        message: u64
    },

    #[serde(rename = "broadcast_ok")]
    BroadcastOk {},

    #[serde(rename = "read")]
    Read {},

    #[serde(rename = "read_ok")]
    ReadOk {
        messages: Vec<u64>,
    },

    #[serde(rename = "topology")]
    Topology {
        
    }
}

