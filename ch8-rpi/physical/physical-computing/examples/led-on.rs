extern crate rust_gpiozero;
use rust_gpiozero::*;

fn main() {
    let led = LED::new(2);

    led.on();
}
