//use log::debug;
use std::{error::Error, sync::mpsc};
use std::{io, thread};

mod l239d_driver;
use crate::l239d_driver::four_pin_stepper_motor::Motor;

/* Driver code written for parts no longer in use

mod l239d_bidirectional_dc_motor_driver;
use crate::l239d_bidirectional_dc_motor_driver::Motor;
mod a3144_hall_sensor_driver;
use crate::a3144_hall_sensor_driver::HallSensor;
*/

//pins connected to L293D
const PIN_MOTOR_EN1: u8 = 13;
const PIN_MOTOR_EN2: u8 = 25;
const PIN_MOTOR_1A: u8 = 6;
const PIN_MOTOR_2A: u8 = 5;
const PIN_MOTOR_3A: u8 = 23; //TODO get real numbers
const PIN_MOTOR_4A: u8 = 24;
const RPM: u16 = 250; //TODO make a more educated guess
const STEPS_PER_REV: u16 = 200;
const WHEEL_CIRCUMFERENCE: u16 = 150; // in mm

//pin connected to A3144
//const PIN_HALL_IN: u8 = 26;

enum MessageType {
    State(State),
    Location(f32),
}

/// Each station is a printer's distance from home in mm, values after that are special cmds who don't use their assigned nums
enum State {
    Home = 0,
    Station1 = 10,
    Station2 = 50,
    Station3 = 150,
    On = 1,  // arbitrary val
    Off = 2, // arbitrary val
}

fn main() -> Result<(), Box<dyn Error>> {
    //device setup
    let mut motor = Motor::new(
        PIN_MOTOR_EN1,
        PIN_MOTOR_EN2,
        PIN_MOTOR_1A,
        PIN_MOTOR_2A,
        PIN_MOTOR_3A,
        PIN_MOTOR_4A,
        STEPS_PER_REV,
        WHEEL_CIRCUMFERENCE,
    )?;
    motor.set_speed(RPM);

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
            println!(
                " Cmds: 
                 0: HOME the gondola
                 1: move to printer 1 
                 2: move to printer 2 
                 3: move to printer 3 
                 4: enable motor 
                 5: disable motor"
            );
            io.read_line(&mut cmd).expect("problems taking input.");
            cmd.pop();
            match &cmd as &str {
                "0" => desired_state = State::Home,
                "1" => desired_state = State::Station1,
                "2" => desired_state = State::Station2,
                "3" => desired_state = State::Station3,
                "4" => desired_state = State::On,
                "5" => desired_state = State::Off,
                _ => println!("error parsing your input of \"{}\", try again.", cmd),
            }

            //TODO properly handle Result instead of using unwrap
            tx_input.send(MessageType::State(desired_state)).unwrap();
        }
    });

    //main thread manages motor driving\
    //TODO revise HOME cmd to use a button sensor to reset the encoder somehow
    loop {
        match rx.try_recv() {
            Ok(received) => match received {
                MessageType::State(state) => handle_state_change(state, &mut target, &mut nav_flag, &mut motor),
                MessageType::Location(dist) => distance = handle_location_message(dist, target),
            },
            Err(_) => continue,
        }
        // We use nav_flag to only goto the target once instead of calling repeatedly
        if nav_flag {
            motor.goto(target as f32);
        }
    }
}

/// handles user input from user IO thread
fn handle_state_change(new_state: State, old_target: &mut i32, nav_flag: &mut bool, motor: &mut Motor) {
    use crate::State::*;

    match new_state {
        Home => *old_target = Home as i32,
        Station1 => *old_target = Station1 as i32,
        Station2 => *old_target = Station2 as i32,
        Station3 => *old_target = Station3 as i32,
        On =>  { motor.set_power(true); *nav_flag = true; }
        Off => { motor.set_power(false); *nav_flag = false; }
    }
}

/// handles location message sent from encoder reading
fn handle_location_message(dist_from_home: f32, target: i32) -> f32 {
    target as f32 - dist_from_home
}
