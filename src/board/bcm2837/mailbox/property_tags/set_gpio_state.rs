use core::intrinsics;
use super::PropertyTag;

/// The SET_GPIO_STATE property tag.
///
/// # Notes
///
/// This is not an officially documented tag, but it appeared in a [comment][1] on the Raspberry Pi
/// forums from the firmware developers.
///
/// # Examples
///
/// To turn on the activity LED on the Raspberry Pi use `130` as the pin number:
///
/// ```rust
/// let tag = SetGpioState::new(130, 1);
/// ```
pub struct SetGpioState {
    buffer: [u8; 8],
}

impl SetGpioState {
    /// Create a new SetGpioState property tag.
    pub fn new(pin: u32, state: u32) -> SetGpioState {
        SetGpioState { buffer: unsafe { intrinsics::transmute([pin, state]) } }
    }
}

impl PropertyTag for SetGpioState {
    fn id(&self) -> u32 {
        0x00038041
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }
}
