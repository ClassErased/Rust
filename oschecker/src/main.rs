use std::env;

/* does a check of arch and system files
documentation: https://doc.rust-lang.org/src/std/env.rs.html#874 */

fn main() {
    
println!("{}", env::consts::OS); // Prints the current OS.

}
