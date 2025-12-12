fn main() {
    println!("Hello, world!");
    println!("Fibonacii: {}", fibonacci(22));
}

fn fibonacci(i: u32) -> u32 {
    if i == 0 {
        return 0;
    }
    if i == 1 {
        return 1;
    }

    return fibonacci(i - 1) + fibonacci(i - 2);
}
