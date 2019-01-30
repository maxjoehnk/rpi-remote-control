use std::fmt::Write;

use hal;
use hal::{Delay, Pin, Spidev};
use hal::spidev::SpidevOptions;
use hal::sysfs_gpio::Direction;
use ssd1306::Builder;
use ssd1306::prelude::*;
use error::Result;
use state::ApplicationState;

const RST_PIN: u64 = 23;
const DC_PIN: u64 = 24;

pub struct Display {
    display: TerminalMode<SpiInterface<Spidev, Pin>>
}

impl Display {
    pub fn render(&mut self, state: &ApplicationState) -> Result<()> {
        self.display.clear()?;
        write!(self.display, "Volume: {}", state.avr.volume)?;
        Ok(())
    }
}

pub fn setup_display() -> Result<Display> {
    println!("setting up display...");
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new().max_speed_hz(50_000).build();

    spi.configure(&options)?;

    // Setup Reset Pin
    let mut reset = setup_output(RST_PIN).unwrap();

    // Setup DC Pin
    let dc = setup_output(DC_PIN).unwrap();

    let mut delay = Delay {};

    let mut display: TerminalMode<_> = Builder::new().with_size(DisplaySize::Display128x32).connect_spi(spi, dc).into();
    display.reset(&mut reset, &mut delay);
    display.init()?;
    display.clear()?;

    println!("done");

    Ok(Display {
        display
    })
}

pub fn setup_output(pin_number: u64) -> hal::sysfs_gpio::Result<Pin> {
    let pin = Pin::new(pin_number);
    pin.export()?;

    while !pin.is_exported() {}
    pin.set_direction(Direction::Out)?;

    Ok(pin)
}
