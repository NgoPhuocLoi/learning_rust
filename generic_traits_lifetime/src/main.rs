use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(nums: &[T]) -> &T {
    let mut largest = &nums[0];

    for num in nums {
        if num > largest {
            largest = num;
        }
    }
    largest
}

struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> Point<X, Y> {
    fn get_x(&self) -> &X {
        &self.x
    }
}

fn main() {
    let numbers = vec![10, 2, 3, 4, 5];

    let largest_num = largest(&numbers);

    println!("The largest number is {}", largest_num);

    let chars = vec!['a', 'c', 'A', 'X'];

    let largest_char = largest(&chars);

    println!("The largest char is {}", largest_char);

    let p1 = Point { x: 10, y: 2.0 };

    println!("{} {}", p1.get_x(), p1.y);
}
