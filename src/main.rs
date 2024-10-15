use echo_challenge::handlers::{handle_echo, handle_init};
use echo_challenge::message::{Message, MessageTypeData};
use echo_challenge::node::Node;
use std::io;

fn main() -> io::Result<()> {
    let deserializer = serde_json::Deserializer::from_reader(io::stdin());
    let de_iter = deserializer.into_iter::<Message>();

    let mut node = Node { id: None, message_id: 1 };

    // let lines = io::stdin().lines();
    for message in de_iter {
        let message = message?;

        let res = match message.body.type_specific {
            MessageTypeData::Init { .. } => handle_init(&message, &mut node),
            MessageTypeData::Echo { .. } => handle_echo(&message, &mut node),
            _ => message
        };

        println!("{}", serde_json::to_string(&res)?);
    }
    Ok(())
}
