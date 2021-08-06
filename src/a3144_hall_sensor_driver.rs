use std::error::Error;
use rppal::gpio::{Gpio, InputPin, Trigger, Level};

pub struct HallSensor {
    sens_in: InputPin,
    step_count: u32,
}

impl HallSensor {

    #[allow(non_snake_case)] //pins should be constant, names reflect that
    pub fn new(PIN_HALL_IN: u8) -> Result<HallSensor, Box<dyn Error>>{
        let mut hall_sensor = HallSensor {
            sens_in: Gpio::new()?.get(PIN_HALL_IN)?.into_input_pullup(),
            step_count: 0,
        };
        hall_sensor.sens_in.set_interrupt(Trigger::FallingEdge)?;
        Ok(hall_sensor)
    }
    //TODO change this to track both peaks so ik if i'm moving backwards
    pub fn update(&mut self) {
        match self.sens_in.poll_interrupt(true, None) {
            Ok(Some(Level::Low)) => self.step_count += 1,
            _ => (),
        }
    }

    pub fn dist_from_home(&self) -> u32 {
        // math to convert step_count to distance in mm
        0
    }
}