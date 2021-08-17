pub mod four_pin_stepper_motor;

#[allow(dead_code)] // not using this driver rn.
pub mod bidirectional_dc_motor;

#[derive(Debug)]
pub enum State {
    Forward,
    Backward,
    Stopped,
    Off,
}