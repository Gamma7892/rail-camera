use log::debug;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, IoPin, Mode};

//pins connected to L293D
const PIN_MOTOR_EN: u8 = 13;
const PIN_MOTOR_1A: u8 = 6;
const PIN_MOTOR_2A: u8 = 5;

fn main() -> Result<(), Box<dyn Error>> {
    //setup section
    let gpio = Gpio::new()?;

    let mut motor_en = gpio.get(PIN_MOTOR_EN)?.into_output();
    let mut motor_1a = gpio.get(PIN_MOTOR_1A)?.into_output();
    let mut motor_2a = gpio.get(PIN_MOTOR_2A)?.into_output();

    loop {
        //spin left
        motor_en.set_high();
        motor_1a.set_high();
        motor_2a.set_low();
        
        println!("spinning left?");
        thread::sleep(Duration::from_secs(1));

        //spin right
        motor_en.set_high();
        motor_1a.set_low();
        motor_2a.set_high();

        println!("spinning right?");
        thread::sleep(Duration::from_secs(1));

        //fast stop
        motor_en.set_high();
        motor_1a.set_high();
        motor_2a.set_high();

        println!("stopping.");
        thread::sleep(Duration::from_secs(1));
    }
}
