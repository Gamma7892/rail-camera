//use log::debug;
use std::{error::Error, sync::mpsc};
use std::{io, thread};

mod l239d_driver;
use crate::l239d_driver::four_pin_stepper_motor::Motor;

/* Driver code written for parts no longer in use
mod a3144_hall_sensor_driver;
use crate::a3144_hall_sensor_driver::HallSensor;
*/

//pins connected to L293D
const PIN_MOTOR_EN1: u8 = 13;
const PIN_MOTOR_EN2: u8 = 25;
const PIN_MOTOR_1A: u8 = 6;
const PIN_MOTOR_2A: u8 = 5;
const PIN_MOTOR_3A: u8 = 23;
const PIN_MOTOR_4A: u8 = 24;

// fields describing motor behavior
const RPM: u16 = 250; //TODO make a more educated guess
const STEPS_PER_REV: u16 = 200;
const WHEEL_CIRCUMFERENCE: u16 = 150; // in mm

//pin connected to A3144 Hall sensor
//const PIN_HALL_IN: u8 = 26;

const ACCEPTABLE_ERROR: f32 = 0.5; // in mm

// used to determine which thread we're reciving from.
enum MessageType {
    Command(Command),
    Location(f32),
}

/// Each station is a printer's distance from home in mm, values after that are special cmds who don't use their assigned nums
#[derive(Debug)]
enum Command {
    Home = 0,
    Station1 = 10,
    Station2 = 50,
    Station3 = 150,
    On = 1,  // arbitrary val
    Off = 2, // arbitrary val
    Status = 3, // arbitrary val
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
    let mut target = Command::Home as i32;
    let mut nav_flag = false;
    let mut err_flag = false; //if this is ever set to true we'll need to restart after fixing the problem

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

        println!("Greetings from the Camera Carriage!");
        print_cmds();

        loop {
            let mut desired_state = Command::Off; //i32 = 0;
            let mut cmd = String::new();
            
            io.read_line(&mut cmd).expect("problems taking input.");
            cmd.pop();
            match &cmd as &str {
                "0" => desired_state = Command::Home,
                "1" => desired_state = Command::Station1,
                "2" => desired_state = Command::Station2,
                "3" => desired_state = Command::Station3,
                "4" => desired_state = Command::On,
                "5" => desired_state = Command::Off,
                "6" => desired_state = Command::Status,
                "7" => print_cmds(),
                _ => println!("error parsing your input of \"{}\", try again.", cmd),
            }

            if &cmd != "7" { println!("relaying command: {:?}",desired_state) }

            //TODO properly handle Result instead of using unwrap
            tx_input.send(MessageType::Command(desired_state)).unwrap();
        }
    });

    //main thread manages motor driving
    //TODO write out blocking version of HOME cmd that uses a button to reset step count
    loop {
        match rx.try_recv() {
            Ok(received) => match received {
                MessageType::Command(state) => handle_state_change(state, &mut target, &mut nav_flag, &mut motor),
                MessageType::Location(dist) => err_flag = handle_location_message(dist, motor.dist_from_home(), &mut motor),
            },
            Err(_) => (),
        }
        // We use nav_flag to only goto the target once instead of calling repeatedly
        if nav_flag && !err_flag {
            // non blocking motor driving for normal use
            let range_to_target = target as f32 - motor.dist_from_home();
            if range_to_target > ACCEPTABLE_ERROR || range_to_target < (-1.0 * ACCEPTABLE_ERROR) {
                if range_to_target > 0.0 { motor.step(1) } else { motor.step(-1) };
            }
        }
    }
}

/// handles user input from user IO thread
fn handle_state_change(new_state: Command, old_target: &mut i32, nav_flag: &mut bool, motor: &mut Motor) {
    use crate::Command::*;

    match new_state {
        Home => *old_target = Home as i32,
        Station1 => *old_target = Station1 as i32,
        Station2 => *old_target = Station2 as i32,
        Station3 => *old_target = Station3 as i32,
        On =>  { motor.set_power(true); *nav_flag = true; }
        Off => { motor.set_power(false); *nav_flag = false; }
        Status => motor.status(),
    }
}

/// uses encoder reading to verify motor position. 
fn handle_location_message(encoder_dist_from_home: f32, stepper_dist_from_home: f32, motor: &mut Motor) -> bool {
    let mut err_flag = false;
    let difference = stepper_dist_from_home - encoder_dist_from_home;
    if difference > ACCEPTABLE_ERROR {
        motor.set_power(false);
        err_flag = true;
        println!("encoder detected motor fault. please diagnose and restart.");
    }
    err_flag
}

/// helper function to get this block out of the match statement
fn print_cmds() {
    println!(
        " Cmds: 
         0: HOME the gondola
         1: move to printer 1 
         2: move to printer 2 
         3: move to printer 3 
         4: enable motor 
         5: disable motor 
         6: print status info 
         7: print these commands"
    );
}



