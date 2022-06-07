fn five() -> i32 {
    5 * 60
}

fn six() -> i32 {
    6 / 2
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);

    let x = six();

    println!("The value is: {}", x);
}
