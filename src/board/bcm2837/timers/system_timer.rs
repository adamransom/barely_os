//! Contains functions for working with the system timer on the BCM2837 board.
//!
//! All of the constants used here (as well as information on the system timer in general) can be
//! found in [BCM2835 ARM Peripherals.pdf][1].
//!
//! # Notes
//!
//! The system timer [runs at 1Mhz][2], though this is not documented anywhere officially.
//!
//! [1]: https://www.raspberrypi.org/documentation/hardware/raspberrypi/bcm2835/BCM2835-ARM-Peripherals.pdf#page=172
//! [2]: http://xinu.mscs.mu.edu/BCM2835_System_Timer

use mmio;

const SYS_TIMER_BASE: usize = 0x3f003000;

const CLO: usize = 0x004; // System Timer Counter Lower 32 bits
#[allow(dead_code)]
const CHI: usize = 0x008; // System Timer Counter Higher 32 bits

/// Read the current timer value
///
/// # Notes
///
/// The timer uses a 32-bit unsigned integer and therefore wraps every 4295 seconds.
pub fn read() -> u32 {
    unsafe {
        mmio::read(SYS_TIMER_BASE + CLO)
    }
}
