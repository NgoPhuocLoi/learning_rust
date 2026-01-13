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

fn find_longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() >= str2.len() {
        return str1;
    }
    str2
}

fn main() {
    let str1 = String::from("abcd");
    let str2 = "xyz";

    let l = find_longest(str1.as_str(), str2);

    println!("the longest string is {l}");
}
