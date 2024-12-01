use std::io;

fn main() {
    println!("Enter the weight in kgs:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 =input.trim().parse().unwrap();
    println!("input {}",weight);

    let mars_weight = weight_on_mars(weight);
    println!("the weight on the mars is {} kg",mars_weight);
}

fn weight_on_mars(weight: f32) -> f32 {
    (weight/9.81)*3.711
}
