//! Fake implementation of the LEDs API, documented here:
//! https://github.com/tock/tock/blob/master/doc/syscalls/00002_leds.md
//!
//! Like the real API, `Leds` controls a set of fake LEDs. It provides
//! a function `get_led` used to retrieve the state of an LED.

use core::cell::Cell;
use libtock_platform::{CommandReturn, ErrorCode};

pub struct Leds<const LEDS_COUNT: usize> {
    leds: [Cell<bool>; LEDS_COUNT],
}

impl<const LEDS_COUNT: usize> Leds<LEDS_COUNT> {
    pub fn new() -> std::rc::Rc<Leds<LEDS_COUNT>> {
        #[allow(clippy::declare_interior_mutable_const)]
        const OFF: Cell<bool> = Cell::new(false);
        std::rc::Rc::new(Leds {
            leds: [OFF; LEDS_COUNT],
        })
    }
}

impl<const LEDS_COUNT: usize> crate::fake::SyscallDriver for Leds<LEDS_COUNT> {
    fn id(&self) -> u32 {
        DRIVER_NUMBER
    }
    fn num_upcalls(&self) -> u32 {
        0
    }

    fn command(&self, command_number: u32, argument0: u32, _argument1: u32) -> CommandReturn {
        match command_number {
            DRIVER_CHECK => crate::command_return::success_u32(LEDS_COUNT as u32),
            LED_ON => {
                if argument0 < LEDS_COUNT as u32 {
                    self.leds[argument0 as usize].set(true);
                    crate::command_return::success()
                } else {
                    crate::command_return::failure(ErrorCode::Invalid)
                }
            }
            LED_OFF => {
                if argument0 < LEDS_COUNT as u32 {
                    self.leds[argument0 as usize].set(false);
                    crate::command_return::success()
                } else {
                    crate::command_return::failure(ErrorCode::Invalid)
                }
            }
            LED_TOGGLE => {
                if argument0 < LEDS_COUNT as u32 {
                    self.leds[argument0 as usize].set(!self.leds[argument0 as usize].get());
                    crate::command_return::success()
                } else {
                    crate::command_return::failure(ErrorCode::Invalid)
                }
            }
            _ => crate::command_return::failure(ErrorCode::NoSupport),
        }
    }
}

// -----------------------------------------------------------------------------
// Implementation details below
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests;

const DRIVER_NUMBER: u32 = 2;

// Command numbers
const DRIVER_CHECK: u32 = 0;
const LED_ON: u32 = 1;
const LED_OFF: u32 = 2;
const LED_TOGGLE: u32 = 3;

impl<const NUM_LEDS: usize> Leds<NUM_LEDS> {
    pub fn get_led(&self, led: usize) -> Option<bool> {
        if led < NUM_LEDS {
            Some(self.leds[led].get())
        } else {
            None
        }
    }
}
