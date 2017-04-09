//! Defines functionality for the property tag interface of the BCM2837 mailbox system.
//!
//! There is some documentation on the various tags that are supported by this interface on the
//! [Raspberry Pi Wiki], along with information which defines most of the implementation used in
//! this module.
//!
//! Currently only sending a single tag at a time is supported.
//!
//! [Raspberry Pi Wiki]: https://github.com/raspberrypi/firmware/wiki/Mailbox-property-interface

use board::mem;
use core::intrinsics;
use mmio;
use super::Channel;

/// A struct defining a single property tag.
pub struct PropertyTag<'a> {
    /// The tag identifier.
    pub tag_id: u32,
    /// The value buffer.
    pub value: &'a [u8],
}

impl<'a> PropertyTag<'a> {
    /// Create a new property tag.
    pub fn new(tag_id: u32, value: &'a [u8]) -> PropertyTag {
        PropertyTag {
            tag_id: tag_id,
            value: value,
        }
    }
}

const PT_PROCESS_REQUEST: u32 = 0;
const PT_END_TAG: u32 = 0;

extern "C" {
    /// A 16-byte aligned buffer, defined in assembly (`property_tags.s`)
    static __property_tags: *mut u8;
}

/// Sends a property tag request to the VideoCore via the mailbox.
pub fn send(tag: &PropertyTag) {
    // Currently we use a single static buffer, defined in assembly, as Rust doesn't have a nice
    // way to align stack-allocated buffers. This means that you can only send property tags
    // synchronously at the moment, whereas the actual interface allows asynchronous requests.
    unsafe {
        let addr = __property_tags as usize;

        // Buffer size in bytes (including the header values, the end tag and padding)
        mmio::write(addr + 0, 24 + tag.value.len());
        // Buffer request code
        mmio::write(addr + 4, PT_PROCESS_REQUEST);
        // Tag identifier
        mmio::write(addr + 8, tag.tag_id);
        // Tag buffer size in bytes
        mmio::write(addr + 12, tag.value.len());
        // Tag request code
        mmio::write(addr + 16, PT_PROCESS_REQUEST);
        // End tag
        mmio::write(addr + 16 + tag.value.len(), PT_END_TAG);

        // Copy the tag data into the buffer
        intrinsics::volatile_copy_nonoverlapping_memory(__property_tags.offset(20),
                                                        tag.value.as_ptr(),
                                                        tag.value.len());

        // Send the buffer address to the mailbox
        super::write(addr as u32, Channel::PropertyTags);
        mem::write_barrier();
        // Read the value received from the mailbox. Without this read, the mailbox would
        // eventually become full and no more messages could be send. Currently the data returned
        // by the mailbox is ignored.
        let _ = super::read(Channel::PropertyTags);
        mem::read_barrier();
    }
}
