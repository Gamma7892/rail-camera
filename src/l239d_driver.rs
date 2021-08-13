pub mod four_pin_stepper_motor;

pub mod bidirectional_dc_motor;

pub enum State {
    Forward,
    Backward,
    Stopped,
    Off,
}