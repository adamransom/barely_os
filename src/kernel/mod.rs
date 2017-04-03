use time;

#[no_mangle]
pub extern "C" fn kernel_main() {
    // Wait 10 seconds, then return back to assembly
    hang!(10 * time::MICROS_PER_SEC);
}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    hang!();
}
