use std::{io, thread};
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();


    let tx_1 = tx.clone();
    thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(1000));
            tx_1.send(i.to_string()).unwrap();
        }
    });

    thread::spawn(move || {
        let lines = io::stdin().lines();
        for line in lines {
            // thread::sleep(Duration::from_millis(1));
            tx.send(line.unwrap()).unwrap();
        }
    });


    for line in rx {
        println!("Got {}", line);
    }
}
