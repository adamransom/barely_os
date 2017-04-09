use core::intrinsics;

/// Performs a write to memory which can not be optimised away.
pub unsafe fn write<T>(address: usize, data: T) {
    intrinsics::volatile_store(address as *mut T, data);
}

/// Performs a read from memory which can not be optimised away.
pub unsafe fn read<T>(address: usize) -> T {
    intrinsics::volatile_load(address as *const T)
}
