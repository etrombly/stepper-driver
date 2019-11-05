//! Crate to interface stepper motor drivers

#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]

pub mod ic;

use core::marker::PhantomData;

use embedded_hal::digital::v2::OutputPin;

enum State {
    HIGH,
    LOW,
}

/// Stepper direction
pub enum Direction {
    /// Clockwise
    CW,
    /// Counter Clockwise
    CCW,
}

const STEPS: [[State; 4]; 4] = [
    [State::HIGH, State::HIGH, State::LOW, State::LOW],
    [State::LOW, State::HIGH, State::HIGH, State::LOW],
    [State::LOW, State::LOW, State::HIGH, State::HIGH],
    [State::HIGH, State::LOW, State::LOW, State::HIGH],
];

fn digital_write<ERROR>(state: &State, pin: &mut dyn OutputPin<Error = ERROR>) -> Result<(), ERROR>{
    match state {
        &State::HIGH => pin.set_high(),
        &State::LOW => pin.set_low(),
    }
}

/// A stepper motor driver
pub struct Stepper<IN1, IN2, IN3, IN4, IC, ERROR>
where
    IN1: OutputPin<Error = ERROR>,
    IN2: OutputPin<Error = ERROR>,
    IN3: OutputPin<Error = ERROR>,
    IN4: OutputPin<Error = ERROR>,
{
    index: usize,
    /// clockwise or counterclockwise rotation
    pub direction: Direction,
    in1: IN1,
    in2: IN2,
    in3: IN3,
    in4: IN4,
    _ic: PhantomData<IC>,
}

impl<IN1, IN2, IN3, IN4, IC, ERROR> Stepper<IN1, IN2, IN3, IN4, IC, ERROR>
where
    IN1: OutputPin<Error = ERROR>,
    IN2: OutputPin<Error = ERROR>,
    IN3: OutputPin<Error = ERROR>,
    IN4: OutputPin<Error = ERROR>,
{
    /// Change the stepper direction
    pub fn direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    /// Make the stepper move one step
    pub fn step(&mut self) -> Result<&mut Self, ERROR> {
        digital_write(&STEPS[self.index][0], &mut self.in1)?;
        digital_write(&STEPS[self.index][1], &mut self.in2)?;
        digital_write(&STEPS[self.index][2], &mut self.in3)?;
        digital_write(&STEPS[self.index][3], &mut self.in4)?;

        self.index = match self.direction {
            Direction::CW => {
                if self.index < 3 {
                    self.index + 1
                } else {
                    0
                }
            }
            Direction::CCW => {
                if self.index > 0 {
                    self.index - 1
                } else {
                    3
                }
            }
        };
        Ok(self)
    }

    /// Disable stepper
    pub fn disable(&mut self) -> Result<&mut Self, ERROR> {
        self.in1.set_low()?;
        self.in2.set_low()?;
        self.in3.set_low()?;
        self.in4.set_low()?;
        Ok(self)
    }
}

impl<IN1, IN2, IN3, IN4, ERROR> Stepper<IN1, IN2, IN3, IN4, ic::ULN2003, ERROR>
where
    IN1: OutputPin<Error = ERROR>,
    IN2: OutputPin<Error = ERROR>,
    IN3: OutputPin<Error = ERROR>,
    IN4: OutputPin<Error = ERROR>,
{
    /// Creates a new `Stepper`
    pub fn uln2003(direction: Direction, in1: IN1, in2: IN2, in3: IN3, in4: IN4) -> Self {
        Stepper {
            index: 0,
            direction,
            in1,
            in2,
            in3,
            in4,
            _ic: PhantomData,
        }
    }
}