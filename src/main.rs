//use log::debug;
use std::{error::Error, sync::mpsc};
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

//constant distances from HOME to each printer (in mm)
const HOME: i32 = 0;
const STATION1: i32 = 20;
const STATION2: i32 = 30;
const STATION3: i32 = 60;

const TARGET_FLAG: i32 = -0b10000000000000000000000000000000;
const RANGE: i32 = 3; // acceptable +/- distance from target STATION

fn main() -> Result<(), Box<dyn Error>>  {

    //device setup
    let mut motor = Motor::new(PIN_MOTOR_EN, PIN_MOTOR_1A, PIN_MOTOR_2A)?;
    let mut encoder = HallSensor::new(PIN_HALL_IN)?;

    //thread setup
    let (tx_encoder, rx) = mpsc::channel();
    let tx_input = tx_encoder.clone();

    // variable declaration for main thread
    let mut target = HOME as i32;
    //if dist were 0 on the first loop it'd never start because it's not updated till
    //encoder reads the magnet which can't happen till the motor spins
    let mut distance: i32 = 99; 

    //thread for maintaining distance 
    thread::spawn(move || {
        loop {
            encoder.update();
            tx_encoder.send(encoder.dist_from_home() as i32).unwrap();
            //this ^ was somehow interpreting as a f32 despite the fn signature
            //clearly stating it was an i32, so it's an explicit cast now.
        }
    });

    //thread for tracking user input (and converting it to target distance)
    thread::spawn(move || {
        let io = io::stdin();
        let mut desired_state: i32 = 0;
        
        loop {
            let mut cmd = String::new();
            println!("Cmds: 
                 0: HOME the gondola
                 1: move to printer 1 
                 2: move to printer 2 
                 3: move to printer 3"
            );
            io.read_line(&mut cmd).expect("problems taking input.");
            cmd.pop();
            match &cmd as &str {
                "0" => desired_state = HOME,
                "1" => desired_state = STATION1,
                "2" => desired_state = STATION2,
                "3" => desired_state = STATION3,
                _ => println!("error parsing your input {}, try again.", cmd), 
            }
            println!("Target: {}", desired_state);
            //we'll check for this later to know this is a target
            desired_state = desired_state | TARGET_FLAG;

            //TODO properly handle Result instead of using unwrap
            tx_input.send(desired_state as i32).unwrap();
        }
    });

    //main thread manages motor driving
    //TODO revise HOME cmd to use a button sensor to reset the encoder somehow
    loop {
        match rx.try_recv() {
            Ok(received) => {
                if (received & TARGET_FLAG) == TARGET_FLAG {
                    //this msg is from our user input so we take the flag out and store
                    target = received & (!TARGET_FLAG);
                }
                else {
                    //we have an encoder distance
                    //so calculate distance & direction to target
                    distance = target - received;
                }
            },
            Err(_) => (),
        }
        //check if we're close enough to target
        if distance < RANGE && distance > (-1 * RANGE) {
            motor.brake();
        }
        else if distance > 0 {
            motor.forward();
        }
        else {
            motor.backward();
        }
    }
}