extern crate ssd1306;
extern crate embedded_hal;
extern crate linux_embedded_hal as hal;

use ssd1306::prelude::*;
use ssd1306::Builder;

use embedded_hal::prelude::*;
use hal::spidev::SpidevOptions;
use hal::{Pin, Spidev};
use hal::sysfs_gpio::{Direction, Edge};
use std::io;
use std::fmt::Write;
use std::sync::mpsc;
use std::thread;

const RST_PIN: u64 = 23;
const DC_PIN: u64 = 24;
const LED_PIN: u64 = 25;

const BTN_PIN: u64 = 17;
const ENCODER_CLK_PIN: u64 = 27;
const ENCODER_DAT_PIN: u64 = 22;

fn main() {
    let mut led = setup_output(LED_PIN).expect("led");
    led.set_high();
    setup_display().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let clk = setup_input(ENCODER_CLK_PIN).unwrap();
        let dat = setup_input(ENCODER_DAT_PIN).unwrap();

        let mut last_clk_state = 1;

        loop {
            match clk.get_value() {
                Ok(1) => if last_clk_state == 0 {
                    if let Ok(0) = dat.get_value() {
                        tx.send(1).unwrap();
                    }else {
                        tx.send(-1).unwrap();
                    }

                    last_clk_state = 1;
                },
                Ok(state) => {
                    last_clk_state = state;
                },
                Err(_) => {}
            }

            thread::sleep_ms(1);
        }
    });

    thread::spawn(move || {
        let btn = setup_input(BTN_PIN).unwrap();

        btn.set_edge(Edge::RisingEdge);

        let mut poller = btn.get_poller().unwrap();

        loop {
            println!("button {:?}", poller.poll(isize::max_value()).unwrap());
        }
    });

    let mut count = 0;

    for received in rx {
        count += received;

        println!("counter: {}", count);
    }
}

fn setup_display() -> io::Result<()> {
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

fn setup_output(pin_number: u64) -> hal::sysfs_gpio::Result<Pin> {
    let pin = Pin::new(pin_number);
    pin.export()?;

    while !pin.is_exported() {}
    pin.set_direction(Direction::Out)?;

    Ok(pin)
}

fn setup_input(pin_number: u64) -> hal::sysfs_gpio::Result<Pin> {
    let pin = Pin::new(pin_number);
    pin.export()?;

    while !pin.is_exported() {}
    pin.set_direction(Direction::In)?;

    Ok(pin)
}