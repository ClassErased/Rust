//A Vector is basically a dynamically sized array, new values can be pushed and added.
//Unlike arrays where they may only be replaced.

pub fn run() {
    let mut numbers: Vec<i32> = vec![2, 4, 6, 8 ,10];

    // Vec::insert 
    // use debug trait to print vector
    println!("{:?}", numbers);

    //assert_eq!(numbers, [2, 3, 4]);
}

fn main() {
    run()
}
