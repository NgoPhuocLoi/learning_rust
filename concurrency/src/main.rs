use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(5);

    {
        let mut m = mutex.lock().unwrap();
        *m = 6;
    }

    println!("The value of mutex is {:?}", mutex);
}
