#![feature(asm,concat_idents,const_fn)]
#![no_std]

#[allow(unused_imports)]
#[macro_use(debug)]
extern crate kernel;

extern "C" {
    pub fn init();
}

mod peripheral_interrupts;
mod peripheral_registers;
mod nvic;


pub mod chip;
pub mod clock;
pub mod gpio;
pub mod timer;
pub mod rtc;
pub mod uart;
pub mod pinmux;
pub use chip::NRF52;