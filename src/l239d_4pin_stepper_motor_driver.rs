/* datasheet for L239D can be found here:
   https://www.ti.com/lit/ds/symlink/l293.pdf

   Much of this code is refactored C pulled from this arduino lib:
   https://github.com/arduino-libraries/Stepper
*/
use std::{error::Error, time::{Instant, Duration}};
use rppal::gpio::{Gpio, OutputPin};

pub struct Motor {
    motor_en: OutputPin,
    motor_1a: OutputPin,
    motor_2a: OutputPin,
    motor_3a: OutputPin,
    motor_4a: OutputPin,
    steps_taken: i32,
    steps_per_rev: u16,
    step_delay: Duration,
    last_step_time: Instant,
    direction: State,
}
pub enum State {
    Forward,
    Backward,
    Stopped,
    Off,
}
impl Motor {
    
    #[allow(non_snake_case)] //pins should be constant, names reflect that
    pub fn new(PIN_MOTOR_EN: u8, PIN_MOTOR_1A: u8, PIN_MOTOR_2A: u8,
                                 PIN_MOTOR_3A: u8, PIN_MOTOR_4A: u8,
                                 steps_per_rev: u16)
                                 -> Result<Motor, Box<dyn Error>> {
        
        let gpio = Gpio::new()?;
        let mut motor = Motor {
            motor_en: gpio.get(PIN_MOTOR_EN)?.into_output(),
            motor_1a: gpio.get(PIN_MOTOR_1A)?.into_output(),
            motor_2a: gpio.get(PIN_MOTOR_2A)?.into_output(),
            motor_3a: gpio.get(PIN_MOTOR_3A)?.into_output(),
            motor_4a: gpio.get(PIN_MOTOR_4A)?.into_output(),
            steps_taken: 0,
            steps_per_rev: steps_per_rev,
            step_delay: Duration::from_millis(0),
            last_step_time: Instant::now(),
            direction: State::Off,
        };

        //TODO refactor enabling/disabling to pub fns
        motor.motor_en.set_high();

        Ok(motor)
    }
    
    /// Converts int RPM into Duration between steps 
    pub fn setSpeed(&mut self, rpm: u16) {
        self.step_delay = Duration::from_millis(60 * 1000 * 1000 / self.steps_per_rev as u64 / rpm as u64);
    }
    
    /// wrapper over step that provides conversion from distance to step count
    pub fn goto(&mut self, distance: f32) {

    }

    /// converts steps_taken to a measurement from start
    pub fn dist_from_home(&mut self) -> f32 {
        0.0
    }

    /// moves the motor a given amount of steps
    pub fn step(&mut self, steps: i32) {
        let mut remaining_steps = steps.abs();
        
        if steps > 0 { self.direction = State::Forward}
        if steps < 0 { self.direction = State::Backward}


        while remaining_steps > 0 {
            let now = Instant::now();

            if now - self.last_step_time >= self.step_delay {
                self.last_step_time = now;

                match self.direction {
                    State::Forward => self.steps_taken += 1,
                    State::Backward => self.steps_taken -= 1,
                    _ => () //TODO throw error here i guess?
                }
                
                self.drive_motor();

                remaining_steps -= 1;
            }

            
        }
    }

    /// handles each individual motor step
    fn drive_motor(&mut self) {
        let this_step = self.steps_taken % 4;

        match this_step {
            0 => {
                self.motor_2a.set_high();
                self.motor_1a.set_low();
                self.motor_3a.set_high();
                self.motor_4a.set_low();
            },
            1 => {
                self.motor_2a.set_low();
                self.motor_1a.set_high();
                self.motor_3a.set_high();
                self.motor_4a.set_low();
            },
            2 => {
                self.motor_2a.set_low();
                self.motor_1a.set_high();
                self.motor_3a.set_low();
                self.motor_4a.set_high();
            },
            3 => {
                self.motor_2a.set_high();
                self.motor_1a.set_low();
                self.motor_3a.set_low();
                self.motor_4a.set_high();
            },
            _ => () //we never get here anyway but it makes compiler happy
        }
    }

}