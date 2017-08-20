extern crate lightctrl;

use lightctrl::*;

fn main() {
    let leds = LEDDevice::connect("172.22.99.133:1234", 4).expect("Failed to connect");
    for _ in 0..100 {
        leds.update(&[Color::new(1.0, 0.3, 0.5), Color::new(0.3, 0.1, 1.0), Color::new(0.0, 0.4, 0.8), Color::new(0.1, 0.3, 0.8)]).expect("Failed to send");
    }
    println!("{:?}", leds);
}