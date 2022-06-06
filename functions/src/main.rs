use std::io;

//just playing around, worked out why it works, trying to test input
fn main() {
    let mut labeled_measurement = String::new();
    io::stdin().read_line(&mut labeled_measurement).expect("Failed to read line");
    
}

fn statics(){
    let value2 = 1.3;
    


}

fn print_labeled_measurement(labeled_measurement: f32, value2: f32) {
    println!("The measurement is: {}", labeled_measurement * value2);
}

