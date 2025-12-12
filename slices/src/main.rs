fn find_first_word(str: &String) -> usize {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}

fn find_first_word_using_slice(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }
    &str[..]
}

fn main() {
    let mut s = String::from("Hello world");

    println!("Index: {}", find_first_word(&s));

    let word = find_first_word_using_slice(&s);
    // s.clear(); => error
    println!("Actual work: {}", word);
}
