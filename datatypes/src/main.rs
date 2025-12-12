use std::io;

fn main() {
    let arrays: [u32; 5] = [1, 2, 3, 4, 5];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read a value!");

    let index: usize = index.trim().parse().expect("Expect a number");

    let value = arrays[index];

    println!(
        "The value at index {} is {}. The length is {}",
        index,
        value,
        arrays.len()
    );
}
