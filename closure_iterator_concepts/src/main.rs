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

fn main() {
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
