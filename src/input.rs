use hal;
use hal::Pin;
use hal::sysfs_gpio::{Direction, Edge};
use std::sync::mpsc;
use std::thread;
use state::Command;

const BTN_PIN: u64 = 17;
const ENCODER_CLK_PIN: u64 = 27;
const ENCODER_DAT_PIN: u64 = 22;

fn setup_input(pin_number: u64) -> hal::sysfs_gpio::Result<Pin> {
    let pin = Pin::new(pin_number);
    pin.export()?;

    while !pin.is_exported() {}
    pin.set_direction(Direction::In)?;

    Ok(pin)
}

pub fn button_thread(sender: mpsc::Sender<Command>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let btn = setup_input(BTN_PIN).unwrap();

        btn.set_edge(Edge::RisingEdge).unwrap();

        let mut poller = btn.get_poller().unwrap();

        loop {
            match poller.poll(isize::max_value()) {
                Ok(Some(1)) => sender.send(Command::ToggleMute).unwrap(),
                Ok(_) => {},
                Err(err) => println!("Error {:?}", err)
            }
        }
    })
}

pub fn encoder_thread(sender: mpsc::Sender<Command>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let clk = setup_input(ENCODER_CLK_PIN).unwrap();
        let dat = setup_input(ENCODER_DAT_PIN).unwrap();

        clk.set_edge(Edge::RisingEdge).unwrap();

        let mut poller = clk.get_poller().unwrap();

        loop {
            match poller.poll(isize::max_value()) {
                Ok(Some(1)) => {
                    if let Ok(0) = dat.get_value() {
                        sender.send(Command::IncreaseVolume).unwrap();
                    }else {
                        sender.send(Command::DecreaseVolume).unwrap();
                    }
                },
                Ok(_) => {},
                Err(err) => {
                    println!("err {:?}", err);
                }
            }

            thread::sleep_ms(1);
        }
    })
}
