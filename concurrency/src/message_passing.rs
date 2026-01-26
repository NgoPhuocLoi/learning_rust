use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn message_passing() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("Hi!");
        thread::sleep(Duration::from_secs(1));
        tx.send(s).unwrap();
    });

    let receive = rx.recv().unwrap();
    println!("Got: {receive}");
}

fn send_and_receive_multiple_values() {
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

fn multiple_senders() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
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

    thread::spawn(move || {
        let s = String::from("Hi! from another sender");
        thread::sleep(Duration::from_secs(1));
        tx2.send(s).unwrap();

        let s = String::from("World! another sender");
        thread::sleep(Duration::from_secs(1));
        tx2.send(s).unwrap();

        let s = String::from("12345! another sender");
        thread::sleep(Duration::from_secs(1));
        tx2.send(s).unwrap();
    });

    for receive in rx {
        println!("Got: {receive}");
    }
}
