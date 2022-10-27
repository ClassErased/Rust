//A Vector is basically a dynamically sized array, values can be pushed and changed.
//Unlike arrays where they only be replaced.

pub fn run() {
    let numbers: Vec<i32> = vec![2, 4, 6, 8 ,10];

    // use debug trait to print vector
    println!("{:?}", numbers);
}

fn main() {
    run()
}
