extern crate ssd1306;
extern crate embedded_hal;
extern crate linux_embedded_hal as hal;
extern crate homeassistant;
extern crate dotenv;

use std::sync::mpsc;
use dotenv::dotenv;
use std::env;

use state::ApplicationState;
use input::*;
use output::*;

const LED_PIN: u64 = 25;

mod input;
mod state;
mod output;

fn main() {
    dotenv().ok();

    let ha_url = std::env::var("HOME_ASSISTANT_URL").unwrap();
    let ha_token = std::env::var("HOME_ASSISTANT_TOKEN").unwrap();

    let mut state = ApplicationState::new(ha_url, ha_token);

    let mut led = setup_output(LED_PIN).expect("led");
    setup_display().unwrap();

    let (tx, rx) = mpsc::channel();

    encoder_thread(tx.clone());
    button_thread(tx.clone());

    for cmd in rx {
        println!("command: {:?}", cmd);

        state.run(cmd);

        if state.avr.mute {
            led.set_high();
        }else {
            led.set_low();
        }
    }
}
