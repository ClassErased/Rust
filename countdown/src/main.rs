use std::io;

fn main() {
    println!("Welcome to countdown timer! \nEnter how many seconds for countdown: ");
    let mut timer = String::new();
    io::stdin()
        .read_line(&mut timer)
        .expect("failed to read from stdin");

    /*let trimmed = timer.trim();
    let trimmed: i32 = match timer.trim().parse() {
        Ok(num) => num,
        Err(..) => println!("This was not an integer: {}", timer),
    //             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `()`
    };*/
}  
        
        
        
/* Well ive had my first wrestle with the compiler trying to make
        A simple countdown program with a for loop, haven't enjoyed it.
        I tried using what I learned in the guessing game
        However it is not working

    
            Ok(i) => for number in (1..trimmed).rev() {
            println!(" {} seconds remaining", number)
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
   };
} */

