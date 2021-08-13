/* datasheet for L239D can be found here:
   https://www.ti.com/lit/ds/symlink/l293.pdf

   Much of this code is refactored C pulled from this arduino lib:
   https://github.com/arduino-libraries/Stepper
*/
use rppal::gpio::{Gpio, OutputPin};
use std::{
    error::Error,
    time::{Duration, Instant},
};
use super::State;

pub struct Motor {
    motor_en1: OutputPin,
    motor_en2: OutputPin,
    motor_1a: OutputPin,
    motor_2a: OutputPin,
    motor_3a: OutputPin,
    motor_4a: OutputPin,
    steps_taken: i32,
    steps_per_rev: u16,
    step_delay: Duration,
    last_step_time: Instant,
    direction: State,
    wheel_circumference: u16, // in mm
}

impl Motor {
    #[allow(non_snake_case)] //pins should be constant, names reflect that
    pub fn new(
        PIN_M_EN1: u8,
        PIN_M_EN2: u8,
        PIN_M_1A: u8,
        PIN_M_2A: u8,
        PIN_M_3A: u8,
        PIN_M_4A: u8,
        steps_per_rev: u16,
        wheel_circumference: u16,
    ) -> Result<Motor, Box<dyn Error>> {
        
        let gpio = Gpio::new()?;
        let motor = Motor {
            motor_en1: gpio.get(PIN_M_EN1)?.into_output(),
            motor_en2: gpio.get(PIN_M_EN2)?.into_output(),
            motor_1a: gpio.get(PIN_M_1A)?.into_output(),
            motor_2a: gpio.get(PIN_M_2A)?.into_output(),
            motor_3a: gpio.get(PIN_M_3A)?.into_output(),
            motor_4a: gpio.get(PIN_M_4A)?.into_output(),
            steps_taken: 0,
            steps_per_rev: steps_per_rev,
            step_delay: Duration::from_millis(0),
            last_step_time: Instant::now(),
            direction: State::Off,
            wheel_circumference: wheel_circumference,
        };
        Ok(motor)
    }

    /// Enables / Disables stepper motor. TRUE -> on, FALSE -> off
    pub fn set_power(&mut self, new_state: bool) {
        if new_state {
            self.motor_en1.set_high();
            self.motor_en2.set_high();
        }
        else {
            self.motor_en1.set_low();
            self.motor_en2.set_low();
        }
    }

    /// Converts int RPM into Duration between steps
    pub fn set_speed(&mut self, rpm: u16) {
        self.step_delay =
            Duration::from_millis(60 * 10000 / self.steps_per_rev as u64 / rpm as u64);
    }

    /// wrapper over step that provides conversion from distance to step count
    pub fn goto(&mut self, location: f32) {
        let difference: f32 = location - self.dist_from_home();
        self.step((difference / self.wheel_circumference as f32 * self.steps_per_rev as f32) as i32);
    }

    /// converts steps_taken to a measurement from start
    pub fn dist_from_home(&mut self) -> f32 {
        self.steps_taken as f32 / self.steps_per_rev as f32 * self.wheel_circumference as f32
    }

    /// moves the motor a given amount of steps
    pub fn step(&mut self, steps: i32) {
        
        if self.step_delay == Duration::from_millis(0) {
            println!("speed not set call set_speed() first.");
            return;
        }

        let mut remaining_steps = steps.abs();

        if steps > 0 {
            self.direction = State::Forward
        }
        else if steps < 0 {
            self.direction = State::Backward
        }
        else {
            self.direction = State::Stopped
        }

        while remaining_steps > 0 {
            let now = Instant::now();

            if now - self.last_step_time >= self.step_delay {
                self.last_step_time = now;

                match self.direction {
                    State::Forward => self.steps_taken += 1,
                    State::Backward => self.steps_taken -= 1,
                    _ => (), //TODO throw error here i guess?
                }

                self.drive_motor();

                remaining_steps -= 1;
            }
        }
        println!("current location is: {}", self.dist_from_home());
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
            }
            1 => {
                self.motor_2a.set_low();
                self.motor_1a.set_high();
                self.motor_3a.set_high();
                self.motor_4a.set_low();
            }
            2 => {
                self.motor_2a.set_low();
                self.motor_1a.set_high();
                self.motor_3a.set_low();
                self.motor_4a.set_high();
            }
            3 => {
                self.motor_2a.set_high();
                self.motor_1a.set_low();
                self.motor_3a.set_low();
                self.motor_4a.set_high();
            }
            _ => (), //we never get here anyway but it makes compiler happy
        }
    }
}
