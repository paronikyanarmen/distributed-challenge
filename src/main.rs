use echo_challenge::message::{Message, MessageBody};
use echo_challenge::node::Node;
use std::io;
use echo_challenge::handlers::{handle_echo, handle_init};

fn main() -> io::Result<()> {
    let deserializer = serde_json::Deserializer::from_reader(io::stdin());
    let de_iter = deserializer.into_iter::<Message>();

    let mut node = Node { id: None, message_id: 1 };

    // let lines = io::stdin().lines();
    for message in de_iter {
        let message = message?;

        let res = match message.body {
            MessageBody::Init { .. } => handle_init(&message, &mut node),
            MessageBody::Echo { .. } => handle_echo(&message, &mut node),
            _ => message
        };

        println!("{}", serde_json::to_string(&res)?);
    }
    Ok(())
}
