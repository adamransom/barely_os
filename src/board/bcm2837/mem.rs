/// Inserts a data memory barrier, useful when reading data from a CPU peripheral.
pub fn read_barrier() {
    unsafe { asm!("dmb") }
}

/// Inserts a data sychronisation (or write) barrier, useful when writing data from a CPU peripheral.
#[allow(dead_code)]
pub fn write_barrier() {
    unsafe { asm!("dsb") }
}
