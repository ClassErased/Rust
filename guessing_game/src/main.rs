use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    /* call random number generator from rand 
    save the number as variable */
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    // start loop
    loop{
        println!("Guess the number!");
        println!("Please input your guess.");
        
        // create a mutable variable for a new string input
        let mut guess = String::new();

        /* call stdin 
        read the input and save it to our variable
        if input fails display "failed to read line" */
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        /* convert a string to an unsigned 32-bit integer
        trim blankspace either side of the character
        parse the integer,continue if input = number
        continue to the previous input until its valid */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        /* take the guess and compare to secret_number
        if equal then print winning message and escape
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win! Nice");
                break;
            }
        }
    }
}
//fn main();