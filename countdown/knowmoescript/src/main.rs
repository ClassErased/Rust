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
  let sys_time = SystemTime::now();
  let start = input;
  thread::sleep(Duration::from_millis(1000));
  while input > 0 {
    
    if start != input {
      println!("{}", input);
      thread::sleep(Duration::from_millis(1000));
    } 
    
    input-=1; 

  }
  if input ==  1 {
    println!("time zone verify testing");
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time.duration_since(sys_time)
        .expect("Clock may have gone backwards");
    println!("{difference:?} have passed since execution!");
  // Add function to check local time and print the time +/- 1
  // when I work out how to do it lol
  // Currently this attempts to display how long (in seconds) since the script was run
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