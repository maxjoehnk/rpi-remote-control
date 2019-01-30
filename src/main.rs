extern crate ssd1306;
extern crate embedded_hal;
extern crate linux_embedded_hal as hal;
extern crate homeassistant;
extern crate dotenv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::sync::mpsc;
use dotenv::dotenv;
use embedded_hal::prelude::*;

use state::ApplicationState;
use input::*;
use output::*;
use state::Command;
use std::thread;
use std::time::Duration;

const LED_PIN: u64 = 25;

mod input;
mod state;
mod output;
mod error;

fn main() -> error::Result<()> {
    println!("starting...");
    dotenv().ok();

    let ha_url = std::env::var("HOME_ASSISTANT_URL").unwrap();
    let ha_token = std::env::var("HOME_ASSISTANT_TOKEN").unwrap();
    let avr_entity = std::env::var("AVR_ENTITY").unwrap();

    println!("configuration loaded");

    let mut state = ApplicationState::new(ha_url, ha_token, avr_entity);

    let mut led = setup_output(LED_PIN).expect("led");
    let mut display = setup_display()?;

    let (tx, rx) = mpsc::channel();

    encoder_thread(tx.clone());
    button_thread(tx.clone());
    refresh_thread(tx);

    display.render(&state)?;

    for cmd in rx {
        println!("command: {:?}", cmd);

        state.run(cmd)?;

        display.render(&state)?;

        if state.avr.mute {
            led.set_high();
        }else {
            led.set_low();
        }
    }

    Ok(())
}

pub fn refresh_thread(sender: mpsc::Sender<Command>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            sender.send(Command::Refresh).unwrap();
            thread::sleep(Duration::from_secs(5))
        }
    })
}
