use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    thread::spawn(move || {
        println!("The vector in main thread id  {:?}", v);
    })
    .join()
    .unwrap();

    // println!("{:?}", v); => v no longer can be used
}
