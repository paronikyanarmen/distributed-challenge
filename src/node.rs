use crate::message::Message;

pub struct Node {
    pub id: Option<String>,
    pub message_id: usize,
}

impl Node {
    pub fn reply_to(&mut self, message: &Message) -> Message {

        let mut body = message.body.clone();

        body.in_reply_to = body.msg_id;

        body.msg_id = Some(self.message_id as u64);

        let mut res = Message {
            src: self.id.clone().unwrap(),
            dest: message.src.clone(),
            body: message.body.clone(),
        };

        self.message_id += 1;

        res
    }
}