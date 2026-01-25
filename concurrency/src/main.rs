use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("Hi!");
        thread::sleep(Duration::from_secs(1));
        tx.send(s).unwrap();

        let s = String::from("World!");
        thread::sleep(Duration::from_secs(1));
        tx.send(s).unwrap();

        let s = String::from("12345!");
        thread::sleep(Duration::from_secs(1));
        tx.send(s).unwrap();
    });

    for receive in rx {
        println!("Got: {receive}");
    }
}
