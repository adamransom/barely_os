use board::mailbox::property_tags;
use time;

#[no_mangle]
pub extern "C" fn kernel_main() {
    let on_tag = property_tags::SetGpioState::new(130, 1);
    let off_tag = property_tags::SetGpioState::new(130, 0);

    loop {
        property_tags::send(&on_tag);
        hang!(1 * time::MICROS_PER_SEC);
        property_tags::send(&off_tag);
        hang!(1 * time::MICROS_PER_SEC);
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    hang!();
}
