use crate::message::Message;

pub struct Node {
    pub id: Option<String>,
    pub message_id: usize,
}

impl Node {
    pub fn reply_to(&mut self, message: &Message) -> Message {
        let mut body = message.body.clone();

        // println!("{:?}", message);

        body.in_reply_to = body.msg_id;

        body.msg_id = Some(self.message_id as u64);

        let res = Message {
            src: self.id.clone().unwrap(),
            dest: message.src.clone(),
            body,
        };

        self.message_id += 1;

        res
    }
}