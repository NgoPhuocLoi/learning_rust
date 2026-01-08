fn test_vec() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    let item2 = &mut v[1];

    *item2 = 10;

    for (i, &item) in v.iter().enumerate() {
        println!("index: {} = item {}", i, item);
    }

    for i in &mut v {
        println!("Item: {}", *i);
    }
}

fn test_string() {
    let mut s1 = String::from("New string");
    let s2 = String::from("S2");

    s1.push('C');

    s1.push_str(" Hehehe");

    let mut s3 = s1 + &s2;

    // println!("{s1}"); s1 has been moved

    s3 = format!("{s3}-new--hehe");

    println!("{s3}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}

fn test_hash_map() {
    use std::collections::HashMap;

    let mut m = HashMap::new();

    m.insert("Yellow", 30);
    m.insert("blue", 50);

    println!("{:?}", m);

    let blue_ccore = m.get("blue").copied().unwrap();

    println!("blue score is: {}", blue_ccore);

    for (key, value) in &m {
        println!("Key {} = {}", key, value);
    }

    // Replace existing key
    m.insert("blue", 100);

    // Add if not exist
    m.entry("blue").or_insert(2000);

    let bc = m.entry("blue").or_default();

    *bc += 1000;

    println!("{:?}", m);
}
fn main() {
    test_hash_map();
}
