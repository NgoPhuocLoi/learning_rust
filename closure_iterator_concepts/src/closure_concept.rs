use std::thread;

#[derive(Debug)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.get_most_stocked())
    }

    fn get_most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for s in &self.shirts {
            match s {
                ShirtColor::Blue => {
                    blue_count += 1;
                }
                ShirtColor::Red => red_count += 1,
            }
        }

        if blue_count >= red_count {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn test_inventory() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_preference_1 = Some(ShirtColor::Blue);
    let gift_1 = inventory.giveaway(user_preference_1);

    println!("The gift_1 is {:?}", gift_1);

    let user_preference_2 = None;
    let gift_2 = inventory.giveaway(user_preference_2);

    println!("The gift_1 is {:?}", gift_2);
}

fn closure_inferring_type() {
    let example_closure = |x| x;
    let x1 = example_closure("Hello");
    println!("{x1}");
    // let x2 = example_closure(5); => Err
}

fn take_ownership(v: Vec<i32>) {
    println!("Taked ownership: {v:?}");
}

fn capturing_references_and_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("The list in closure {list:?}");

    println!("Before calling closure: {list:?}");
    // take_ownership(list); => Err since the list was borrowed by 'only_borrows'
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list2 = vec![1, 2, 3];

    let mut borrow_and_mutate = || list2.push(10);

    // println!("Before calling borrow_and_mutate: {list2:?}"); => Error
    borrow_and_mutate();
    println!("After calling borrow_and_mutate: {list2:?}");

    // force to move ownership to the new thread by using 'move' keywork
    thread::spawn(move || println!("List2 in new thread: {list2:?}"))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn closure_traits() {
    let some_number = Some(10);
    let number = some_number.unwrap_or_else(|| 10);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
