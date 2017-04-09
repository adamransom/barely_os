use board::mailbox::property_tags;
use core::intrinsics;
use time;

#[no_mangle]
pub extern "C" fn kernel_main() {
    let on_buffer: [u8; 8] = unsafe { intrinsics::transmute([130u32, 1u32]) };
    let off_buffer: [u8; 8] = unsafe { intrinsics::transmute([130u32, 0u32]) };
    let on_tag = property_tags::PropertyTag::new(0x00038041, &on_buffer);
    let off_tag = property_tags::PropertyTag::new(0x00038041, &off_buffer);

    loop {
        property_tags::send(&on_tag);
        hang!(1 * time::MICROS_PER_SEC);
        property_tags::send(&off_tag);
        hang!(1 * time::MICROS_PER_SEC);
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    hang!();
}
