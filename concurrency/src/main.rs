use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("Hi!");
        thread::sleep(Duration::from_secs(3));
        tx.send(s).unwrap();
    });

    // rx.revc() -> block the current thread, wait until there is some data
    // rx.try_recv() -> does not block the current thread, return immediately Result<T, E>
    let receive = rx.recv().unwrap();
    println!("Got: {receive}");
}
