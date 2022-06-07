use std::io;

//just playing around, worked out why it works, trying to test input
fn main() {
    print_labeled_measurement(5, 4)
}

fn print_labeled_measurement(labeled_measurement: i32, value2: i32) {
    println!("The measurement is: {}", labeled_measurement * value2);

}

