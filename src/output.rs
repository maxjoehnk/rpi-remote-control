use ssd1306::prelude::*;
use ssd1306::Builder;

use embedded_hal::prelude::*;
use hal::spidev::SpidevOptions;
use hal::{Pin, Spidev};
use hal::sysfs_gpio::{Direction, Edge};
use std::io;
use std::fmt::Write;

const RST_PIN: u64 = 23;
const DC_PIN: u64 = 24;

pub fn setup_display() -> io::Result<()> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new().max_speed_hz(50_000).build();

    spi.configure(&options)?;

    // Setup Reset Pin
    let reset = setup_output(RST_PIN).unwrap();

    // Setup DC Pin
    let dc = setup_output(DC_PIN).unwrap();

    let mut disp: TerminalMode<_> = Builder::new().connect_spi(spi, dc).into();
    disp.init().unwrap();
    disp.clear().unwrap();

    disp.write_str("Hello World").unwrap();

    Ok(())
}

pub fn setup_output(pin_number: u64) -> hal::sysfs_gpio::Result<Pin> {
    let pin = Pin::new(pin_number);
    pin.export()?;

    while !pin.is_exported() {}
    pin.set_direction(Direction::Out)?;

    Ok(pin)
}
