// blank the display -- after all, OLEDs fade

extern crate embedded_graphics;
extern crate linux_embedded_hal as hal;
extern crate machine_ip;
extern crate ssd1306;

use hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut disp = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_terminal_mode();
    disp.init().unwrap();
    disp.clear().unwrap();
}
