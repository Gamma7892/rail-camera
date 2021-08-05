//use log::debug;
use std::{error::Error, time::Duration, sync::mpsc};
use std::{io, thread};
    
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

    let (tx_encoder, rx) = mpsc::channel();

    let tx_input = tx_encoder.clone();

    //thread for maintaining distance 
    thread::spawn(move || {
        encoder.dist_from_home();

        //once written properly, should only send message when halts are needed.
        tx_encoder.send("1").unwrap(); //rehandle the Result properly
    });

    //thread for tracking user input (and converting it to target distance)
    thread::spawn(move || {
        let io = io::stdin();
        let mut cmd = String::new();
        let mut desired_state = "1"; //temp declaration

        io.read_line(&mut cmd).expect("problems taking input.");

        //TODO convert raw input into desired state
        tx_input.send(desired_state).unwrap();
    });

    //main thread manages motor driving
    for received in rx {
        
    }

    //old test code
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


