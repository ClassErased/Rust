use std::io;

//just playing around, worked out why it works, trying to test input
fn main() {
    let mut print_labeled_measurement = String::new();
    io::stdin().read_line(&mut print_labeled_measurement).expect("Failed to read line");
}

fn print_labeled_measurement(value: i32, value2: i32) {
    println!("The measurement is: {}", value * value2);
}

