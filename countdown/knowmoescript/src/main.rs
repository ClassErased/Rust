// dumb ass countdown script
// Adapted by nomo and classerased


// Using this as my base to build a countdown program I can pull for Windows and Linux - ab
// Which is probably going to require an OS check, unless I find a solution that works on both
// Or make two scripts


use std::io;
use std::{thread, time::Duration};
// use chrono::prelude::*; dependency used for time reading and printing [chrono = "0.4"]
use std::time::SystemTime;

fn countdown(mut input: i32) {
  let sys_time = SystemTime::now(); // Read the time of the system to allow for diff later
  let start = input;
  thread::sleep(Duration::from_millis(993));
  while input > 1 {
    
    if start != input {
      println!("{} seconds remaining", input);
      thread::sleep(Duration::from_millis(993));
    } 
    
    input-=1; 

  }
  if input ==  1 {
    thread::sleep(Duration::from_millis(993));
   
    // println!("ADD TIME ZONE READING AND PRINT CURRENT TIME dd/mm/yyyy hh:mm:ss");
     
    let new_sys_time = SystemTime::now(); // Take the second system time reading to diff
    let difference = new_sys_time.duration_since(sys_time) // Compare two readings of sys_time to get difference
    // Ideally need to round this output down as computer seems to lose a minimum of 0.007s each cycle, or work out why its losing this time and optimize
        .expect("Clock may have gone backwards");
    println!("{difference:?} have passed since execution!");
  // Add function to check local time and print the time +/- 1
  // when I work out how to do it lol
  // Currently this displays how long (in seconds) since the script was run
  }
}

fn input_control() -> i32 {
    let reader = io::stdin();
    let mut input_text = String::new();

    println!("Enter a number to start the countdown:");

    reader.read_line(&mut input_text).expect("failed to read line");

    let input_opt = input_text.trim().parse::<i32>();

    let input_int = match input_opt {
        Ok(input_int) => input_int,
        Err(e) => {
            println!("Please enter a number ({})", e);
            return 0;
        }
    };
  return input_int;
}


fn main() {
  countdown(input_control());
}