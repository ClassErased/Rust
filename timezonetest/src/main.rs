use std::time::SystemTime;

pub fn now() -> SystemTime {
    let sys_time = SystemTime::now();
    println!("({})", sys_time);
}

fn main() {
    now()
}

