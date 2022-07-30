// dumb ass countdown script
// Adapted by nomo and classerased
use std::io;
use std::{thread, time::Duration};
use chrono::prelude::*;
use std::time::SystemTime;

// Function that counts the clock down from the number that user inputs, additional functionality explained.
fn countdown(mut input: i32) {
  let sys_time = SystemTime::now(); // Read the time of the system to allow for diff later
  let start = input;
  thread::sleep(Duration::from_millis(1000));
  while input > 1 {
    
    if start != input {
      println!("{} seconds remaining", input);
      thread::sleep(Duration::from_millis(1000));
    } 
    input-=1; 
  }
  if input ==  1 {
    thread::sleep(Duration::from_millis(1000));
    
    // Creates two time variables, one from system clock and another from system local timezone
    let new_sys_time = SystemTime::now();
    let timenow: DateTime<Local> = Local::now();
    
    let difference = new_sys_time.duration_since(sys_time) // do diff on system clock readings
        .expect("Clock may have gone backwards");
    println!("{difference:?} have passed since execution and the time is now {}", timenow);
  }
}

// Beginning of program and input handling
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

// Main function
fn main() {
  countdown(input_control());
}