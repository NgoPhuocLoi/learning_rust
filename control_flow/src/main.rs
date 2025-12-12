fn main() {
    let number = 5;

    if number < 10 {
        println!("Less than 10");
    } else {
        println!("Greater than or equal 10");
    }

    loop {
        println!("In loop!");
        break;
    }
    let mut counter = 1;
    let value = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    println!("Counter: {}", value);

    let array: [u32; 5] = [1, 2, 3, 4, 5];

    let mut index = 0;

    while index < array.len() {
        println!("Element value: {}", array[index]);
        index += 1;
    }

    for e in array {
        println!("Element: {}", e);
    }

    for e in (4..9).rev() {
        println!("Element: {}", e);
    }
}
