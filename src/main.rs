//use log::debug;
use std::error::Error;
use std::{thread, time::Duration};

mod l239d_motor_driver;
use crate::l239d_motor_driver::Motor;

mod a3144_hall_sensor_driver;
use crate::a3144_hall_sensor_driver::HallSensor;

//pins connected to L293D
const PIN_MOTOR_EN: u8 = 13;
const PIN_MOTOR_1A: u8 = 6;
const PIN_MOTOR_2A: u8 = 5;

const PIN_HALL_IN: u8 = 26;

fn main() -> Result<(), Box<dyn Error>>  {

    let mut motor = Motor::new(PIN_MOTOR_EN, PIN_MOTOR_1A, PIN_MOTOR_2A)?;
    let mut encoder = HallSensor::new(PIN_HALL_IN)?;

    loop {
        motor.spin_forward();
        
        println!("spinning left?");
        thread::sleep(Duration::from_secs(1));

        motor.spin_backward();

        println!("spinning right?");
        thread::sleep(Duration::from_secs(1));

        motor.off();

        println!("stopping.");
        thread::sleep(Duration::from_secs(1));
    }
}


