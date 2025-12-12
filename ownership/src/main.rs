fn main() {
    let mut s = String::from("Hello World ");
    // let _s2 = s;
    print_str(&mut s);
    // s.push_str("New value");

    println!("String: {}", s);
}

fn print_str(s: &mut String) {
    println!("Print string: {}", s);
    s.push_str(" modified!");
}
