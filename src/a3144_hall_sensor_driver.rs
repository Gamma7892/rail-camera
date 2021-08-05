use std::error::Error;
use rppal::gpio::{Gpio, InputPin};

pub struct HallSensor {
    sens_in: InputPin,
    step_count: u32,
}

impl HallSensor {

    #[allow(non_snake_case)] //pins should be constant, names reflect that
    pub fn new(PIN_HALL_IN: u8) -> Result<HallSensor, Box<dyn Error>>{
        let hall_sensor = HallSensor {
            sens_in: Gpio::new()?.get(PIN_HALL_IN)?.into_input_pullup(),
            step_count: 0,
        };
        Ok(hall_sensor)
    }
    //TODO make this only update on 
    pub fn update(&mut self) {
        if self.sens_in.is_low() {
            self.step_count += 1;
        }
    }

    pub fn dist_from_home(&self) -> f32 {
        // math to convert step_count to distance
        0.0
    }
}