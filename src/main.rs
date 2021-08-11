//use log::debug;
use std::{error::Error, sync::mpsc};
use std::{io, thread};

mod l239d_4pin_stepper_motor_driver;
use crate::l239d_4pin_stepper_motor_driver::Motor;

/* Driver code written for parts no longer in use

mod l239d_bidirectional_dc_motor_driver;
use crate::l239d_bidirectional_dc_motor_driver::Motor;
mod a3144_hall_sensor_driver;
use crate::a3144_hall_sensor_driver::HallSensor;
*/

//pins connected to L293D
const PIN_MOTOR_EN: u8 = 13;
const PIN_MOTOR_1A: u8 = 6;
const PIN_MOTOR_2A: u8 = 5;
const PIN_MOTOR_3A: u8 = 7; //TODO get real numbers 
const PIN_MOTOR_4A: u8 = 8;
const RPM: u16 = 60; //TODO make a more educated guess

//pin connected to A3144
const PIN_HALL_IN: u8 = 26;

const RANGE: f32 = 3.0; // acceptable +/- distance from target Station

enum MessageType {
    State(State),
    Location(f32)
}

/// Each station is a printer's distance from home, values after that are special cmds
enum State {
    Home = 0,
    Station1 = 30,
    Station2 = 50,
    Station3 = 100,
    Off = 1, // arbitrary, not called.
}


fn main() -> Result<(), Box<dyn Error>>  {

    //device setup
    let mut motor = Motor::new(PIN_MOTOR_EN, PIN_MOTOR_1A, PIN_MOTOR_2A,
                                             PIN_MOTOR_3A, PIN_MOTOR_4A,
                                             RPM);
    //let mut motor = Motor::new(PIN_MOTOR_EN, PIN_MOTOR_1A, PIN_MOTOR_2A)?;
    //let mut encoder = HallSensor::new(PIN_HALL_IN)?;

    //thread setup
    let (tx_encoder, rx) = mpsc::channel();
    let tx_input = tx_encoder.clone();

    // variable declaration for main thread
    let mut target = State::Home as i32;
    let mut nav_flag = false;
    //if dist were 0 on the first loop it'd never start because it's not updated till
    //encoder reads the magnet which can't happen till the motor spins
    let mut distance: f32 = 99.0;

    //thread for maintaining distance 
    // thread::spawn(move || {
    //     loop {
    //         encoder.update();
    //         //encoder.fake_tick();
    //         tx_encoder.send(MessageType::Location(encoder.dist_from_home() as f32)).unwrap();
    //         //this ^ was somehow interpreting as a f32 despite the fn signature
    //         //clearly stating it was an i32, so it's an explicit cast now.
    //     }
    // });

    //thread for tracking user input (and converting it to target distance)
    thread::spawn(move || {
        let io = io::stdin();
        
        
        loop {
            let mut desired_state = State::Off; //i32 = 0;
            let mut cmd = String::new();
            println!(" Cmds: 
                 0: HOME the gondola
                 1: move to printer 1 
                 2: move to printer 2 
                 3: move to printer 3 
                 4: disable motor"
            );
            io.read_line(&mut cmd).expect("problems taking input.");
            cmd.pop();
            match &cmd as &str {
                "0" => desired_state = State::Home,
                "1" => desired_state = State::Station1,
                "2" => desired_state = State::Station2,
                "3" => desired_state = State::Station3,
                "4" => desired_state = State::Off,
                _ => println!("error parsing your input {}, try again.", cmd), 
            }

            //TODO properly handle Result instead of using unwrap
            tx_input.send(MessageType::State(desired_state)).unwrap();
        }
    });

    //main thread manages motor driving
    //TODO revise HOME cmd to use a button sensor to reset the encoder somehow
    loop {
        match rx.try_recv() {
            Ok(received) => {
                match received {
                    MessageType::State(state) => handle_state_change(state, &mut target, &mut nav_flag),
                    MessageType::Location(dist) => distance = handle_location_message(dist, target),
                }
            },
            Err(_) => (),
        }
        // We use nav_flag to decide if we're interested in moving rn.
        if nav_flag {
            //check if we're close enough to target
            if distance < RANGE && distance > (-1.0 * RANGE) {
               // motor.brake();
            }
            else if distance > 0.0 {
               // motor.forward();
            }
            else {
                //motor.backward();
            }
        }
        else {
           // motor.off();
        }
    }
}

/// handles user input from user IO thread
fn handle_state_change(new_state: State, old_target: &mut i32, nav_flag: &mut bool) {
    use crate::State::*;

    match new_state {
        Home => *old_target = Home as i32,
        Station1 => *old_target = Station1 as i32,
        Station2 => *old_target = Station3 as i32,
        Station3 => *old_target = Station3 as i32,
        Off => *nav_flag = false,
    }
}


/// handles location message sent from encoder reading
fn handle_location_message(dist_from_home: f32, target: i32) -> f32 {
    target as f32 - dist_from_home
}