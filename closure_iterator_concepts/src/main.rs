fn iterator_in_for_loop() {
    let v = vec![1, 2, 3, 4];

    let v_iter = v.iter();

    println!("=== Iterator in for loop");
    for value in v_iter {
        println!("\t {value}");
    }
}

fn comsume_iterator() {
    let v = vec![5, 6, 7, 8];
    let mut v_iter = v.iter();

    println!("=== Consume iterator");
    loop {
        if let Some(value) = v_iter.next() {
            println!("\t {value}");
        } else {
            break;
        }
    }
}

fn methods_consume_iterator() {
    let v = vec![5, 6, 7, 8];
    let v_iter = v.iter();

    let sum: i32 = v_iter.sum();

    println!("=== methods_consume_iterator");
    println!("Sum: {sum}");

    // for i in v_iter {} => Error since .sum() took the ownership
}

fn methods_produce_iterator() {
    let v = vec![5, 6, 7, 8];
    let plus_one = |x: &i32| x + 1;

    let v_plus_one: Vec<i32> = v.iter().map(plus_one).collect();
    println!("=== methods_produce_iterator");
    dbg!(v_plus_one);
}

fn main() {
    iterator_in_for_loop();
    comsume_iterator();
    methods_consume_iterator();
    methods_produce_iterator();
}
