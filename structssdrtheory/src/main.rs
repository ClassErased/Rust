// idk lets try to write a tuner and amp theory code for an SDR in rust, cpp can do it.

struct Tune {
    rx_active: bool,
    tx_active: bool,
    zero_freq: isize,
    offset_freq: isize,
}

struct Amp {
    decibels: usize,
    multiplier: isize,
}

fn main() {
    println!("I hate java");
}
