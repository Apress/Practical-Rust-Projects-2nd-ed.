extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {

// Tell the Pi which GPIO pin you are using
let mut led = LED::new(2);
let mut button = Button::new(4);

loop{
    println!("wait for button");
    button.wait_for_press(None);
    // Make the led switch on
    println!("button pressed!");
    led.toggle();

    }
}
