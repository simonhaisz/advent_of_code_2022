mod packet;
mod signal;

use util::Timer;

fn main() -> std::io::Result<()> {
    let _timer = Timer::new();

    println!("Hello, world!");

    Ok(())
}
