//! Defines functionality for the mailbox system of the BCM2837.
//!
//! There is some documentation regarding the use of the mailbox on the [Raspberry Pi Wiki] which
//! defines most of the implementation used in this module.
//!
//! # Notes
//!
//! A lot of implementations seem to check the wrong status registers when reading and writing.
//! This usually does not matter though, as in normal use the mailboxes will rarely become full.
//! However, [testing has shown][2] that it is correct to read mailbox 1's status when writing and
//! mailbox 2's status when reading.
//!
//! [Raspberry Pi Wiki]: https://github.com/raspberrypi/firmware/wiki/Mailboxes
//! [2]: https://www.raspberrypi.org/forums/viewtopic.php?f=72&t=165529

pub mod property_tags;

use mmio;

/// List of channels used by the mailbox.
///
/// # Note
///
/// Currently only the frame buffer and property tags (ARM to VC) channels are documented and
/// therefore are the only ones supported.
#[derive(Copy, Clone)]
pub enum Channel {
    /// Frame buffer interface channel.
    #[allow(dead_code)]
    FrameBuffer = 1,
    /// Property tag interface channel for ARM to VC communication.
    PropertyTags = 8,
}

const MAILBOX_BASE: usize = 0x3f00b880;

const MB_0_READ: usize = 0x0;
const MB_0_STATUS: usize = 0x18;
const MB_1_WRITE: usize = 0x20;
const MB_1_STATUS: usize = 0x28;

const STATUS_FULL: u32 = 0x80000000;
const STATUS_EMPTY: u32 = 0x40000000;

/// Sends a message to the mailbox.
///
/// The process is as follows:
///     1. Read the status of mailbox 1 until it is NOT full.
///     2. Create the message by combining the data (shifted into the upper 28 bits) with the
///        channel you want to send the message to.
///     3. Write the message to mailbox 1's write register.
///
/// In all documented mailbox interfaces, the data is a memory address of a 16-byte aligned buffer
/// (the lower 4 bits are all 0).
fn write(data: u32, channel: Channel) {
    unsafe {
        let mut status: u32 = mmio::read(MAILBOX_BASE + MB_1_STATUS);

        while (status & STATUS_FULL) != 0 {
            status = mmio::read(MAILBOX_BASE + MB_1_STATUS);
        }

        let message = data + (channel as u32);

        mmio::write(MAILBOX_BASE + MB_1_WRITE, message);
    }
}

/// Reads a message from the mailbox.
///
/// The process is as follows:
///     1. Read the status of mailbox 0 until it is NOT empty.
///     2. Read the data from mailbox 0's read register.
///     3. If the lower 4 bits don't match the requested channel, repeat from 1.
///     4. Otherwise, the upper 28 bits are the returned data.
///
/// In all documented mailbox interfaces, the data is a memory address of a 16-byte aligned buffer
/// (the lower 4 bits are all 0).
fn read(channel: Channel) -> u32 {
    unsafe {
        let mut status: u32 = mmio::read(MAILBOX_BASE + MB_0_STATUS);
        let channel = channel as u32;

        loop {
            while (status & STATUS_EMPTY) != 0 {
                status = mmio::read(MAILBOX_BASE + MB_0_STATUS);
            }

            let data = mmio::read(MAILBOX_BASE + MB_0_READ);

            if data & 0b1111 == channel {
                return data;
            }
        }
    }
}
