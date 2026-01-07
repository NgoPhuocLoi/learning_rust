fn main() {
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
