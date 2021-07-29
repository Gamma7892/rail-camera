//use log::debug;
use std::error::Error;
use std::{thread, time::Duration};

mod l239d_motor_dsriver;
use crate::l239d_motor_driver::Motor;

//pins connected to L293D
const PIN_MOTOR_EN: u8 = 13;
const PIN_MOTOR_1A: u8 = 6;
const PIN_MOTOR_2A: u8 = 5;

fn main() -> Result<(), Box<dyn Error>>  {

    let mut motor = Motor::new(PIN_MOTOR_EN, PIN_MOTOR_1A, PIN_MOTOR_2A)?;
    
    
    ctrlc::set_handler(move || {
        let temp_motor = Motor::new(PIN_MOTOR_EN, PIN_MOTOR_1A, PIN_MOTOR_2A);
        let mut temp_motor = match temp_motor {
            Ok(motor) => motor,
            Err(_e) => {std::process::exit(1);},
        };
        temp_motor.off();        
    })
    .expect("Error setting Ctrl-C handler"); 
    
    
    loop {
        motor.spin_forward();
        
        println!("spinning left?");
        thread::sleep(Duration::from_secs(1));

        motor.spin_backward();

        println!("spinning right?");
        thread::sleep(Duration::from_secs(1));

        motor.brake();

        println!("stopping.");
        thread::sleep(Duration::from_secs(1));
    }
}


