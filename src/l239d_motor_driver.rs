
use std::error::Error;
use rppal::gpio::{Gpio, OutputPin};

pub struct Motor {

    motor_en: OutputPin,
    motor_1a: OutputPin,
    motor_2a: OutputPin,
    state: State,
}
pub enum State {
    Forward,
    Backward,
    Stopped,
    Off,
}
impl Motor {
    
    #[allow(non_snake_case)] //pins should be constant, names reflect that
    pub fn new(PIN_MOTOR_EN: u8, PIN_MOTOR_1A: u8, PIN_MOTOR_2A: u8) -> Result<Motor, Box<dyn Error>>{
        let gpio = Gpio::new()?;
        let motor = Motor {
            motor_en: gpio.get(PIN_MOTOR_EN)?.into_output(),
            motor_1a: gpio.get(PIN_MOTOR_1A)?.into_output(),
            motor_2a: gpio.get(PIN_MOTOR_2A)?.into_output(),
            state: State::Stopped,
        };
        Ok(motor)
    }
    //pin combonations pulled dirctly from datasheet @
    // https://www.ti.com/lit/ds/symlink/l293.pdf
    pub fn forward(&mut self) {
        self.motor_en.set_high();
        self.motor_1a.set_high();
        self.motor_2a.set_low();
        self.state = State::Forward;
    }

    pub fn backward(&mut self) {
        self.motor_en.set_high();
        self.motor_1a.set_low();
        self.motor_2a.set_high();
        self.state = State::Backward;
    }

    pub fn brake(&mut self) {
        self.motor_en.set_high();
        self.motor_1a.set_high();
        self.motor_2a.set_high();
        self.state = State::Stopped;
    }

    pub fn off(&mut self) {
        self.motor_en.set_low();
        self.motor_1a.set_low();
        self.motor_2a.set_low();
        self.state = State::Off;
    }

}