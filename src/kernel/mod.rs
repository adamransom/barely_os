#[no_mangle]
pub extern "C" fn kernel_main() {
    // Just return here back to assembly
}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop {}
}
