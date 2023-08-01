extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {

// Tell the Pi which GPIO pin you are using
let mut led = LED::new(2);

loop{
    // Make the led switch on
    println!("on");
    led.on();

    // Let the LED stay on for one second
    sleep(Duration::from_secs(1));

    // Make the led switch off
    println!("off");
    led.off();

    // Let the LED stay off for one second
    sleep(Duration::from_secs(1));
    }

    // You can also use led.blink()
}
