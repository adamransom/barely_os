//! Contains functionality particular to the BCM2837 board.
//!
//! There is no published documenation for the BCM2837 board in particular, but a lot of the
//! specifications are similar to the [BCM2835] and [BCM2836].
//!
//! [BCM2835]: https://www.raspberrypi.org/documentation/hardware/raspberrypi/bcm2835/BCM2835-ARM-Peripherals.pdf
//! [BCM2836]: https://www.raspberrypi.org/documentation/hardware/raspberrypi/bcm2836/QA7_rev3.4.pdf

pub mod mem;
pub mod mailbox;
pub mod timers;

/// Macro which hangs the CPU, either indefinitely or for a specific number of microseconds.
#[macro_export]
macro_rules! hang {
    () => (
        loop {}
    );
    ($us:expr) => (
        ::board::hang($us);
    );
}

/// Hang the CPU for a specific number of microseconds.
pub fn hang(duration: u32) {
    use self::timers::system_timer as timer;
    use self::mem;

    let start = timer::read();
    let mut now = timer::read();

    while now - start < duration {
        now = timer::read();
    }

    // Need a data memory barrier after reading a peripheral's memory due to a feature of the
    // processor which might allow data to arrive out of order.
    //
    // https://www.raspberrypi.org/documentation/hardware/raspberrypi/bcm2835/BCM2835-ARM-Peripherals.pdf#page=7
    mem::read_barrier();
}
