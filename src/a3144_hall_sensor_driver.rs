use std::error::Error;
use rppal::gpio::{Gpio, InputPin, Trigger, Level};

pub struct HallSensor {
    sens_in: InputPin,
    step_count: i32,
}

impl HallSensor {

    #[allow(non_snake_case)] //pin should be constant, name reflect that
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

    pub fn dist_from_home(&self) -> f32 {
        // assume 4 ticks = 1 revolution of drive
        // and 1 drive revolve = 1/40 output revolve
        // then 4 ticks * 40 = 1 output revolve
        // then 4 ticks * 40 = perimeter of output wheel
        // assume perimeter of output wheel = 100 mm

        //(self.step_count as f32 * 100.0/160.0)
        0.0
    }

    pub fn fake_tick(&mut self) {
        self.step_count += 1;
    }
}