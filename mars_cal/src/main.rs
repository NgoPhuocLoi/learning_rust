use std::io;

fn get_mars_weight(earth_weight: f32) -> f32 {
    earth_weight * 0.38
}

fn main() {
    let mut weight_input = String::new();
    println!("Enter your weight on Earth: ");

    io::stdin()
        .read_line(&mut weight_input)
        .expect("Failed to read!");
    println!("Weight input: {}", weight_input);
    let weight: f32 = weight_input.trim().parse().expect("Please input a number");

    let mars_weight = get_mars_weight(weight);

    println!("Your Mars weight is {}", mars_weight)
}
