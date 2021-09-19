#![no_std]

const ERRNO_BADF: u16 = 8;
const ERRNO_INTVAL: u16 = 28;

extern "C" {
    fn wavm_caller_module_memory_load8(ptr: usize) -> u8;
    fn wavm_caller_module_memory_load32(ptr: usize) -> u32;
    fn wavm_caller_module_memory_store8(ptr: usize, val: u8);
    fn wavm_caller_module_memory_store32(ptr: usize, val: u32);
}

#[panic_handler]
unsafe fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[no_mangle]
pub unsafe extern "C" fn wasi_snapshot_preview1__proc_exit(_: u32) {
    core::arch::wasm32::unreachable()
}

#[no_mangle]
pub unsafe extern "C" fn env__exit(_: u32) {
    core::arch::wasm32::unreachable()
}

#[no_mangle]
pub unsafe extern "C" fn wasi_snapshot_preview1__environ_sizes_get(
    length_ptr: usize,
    data_size_ptr: usize,
) -> u16 {
    wavm_caller_module_memory_store32(length_ptr, 0);
    wavm_caller_module_memory_store32(data_size_ptr, 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn wasi_snapshot_preview1__environ_get(_: usize, _: usize) -> u16 {
    ERRNO_INTVAL
}

#[no_mangle]
pub unsafe extern "C" fn wasi_snapshot_preview1__fd_close(_: usize) -> u16 {
    ERRNO_BADF
}

#[no_mangle]
pub unsafe extern "C" fn wasi_snapshot_preview1__fd_read(
    _: usize,
    _: usize,
    _: usize,
    _: usize,
) -> u16 {
    ERRNO_BADF
}

#[no_mangle]
pub unsafe extern "C" fn wasi_snapshot_preview1__path_open(
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: u64,
    _: u64,
    _: usize,
    _: usize,
) -> u16 {
    ERRNO_BADF
}

#[no_mangle]
pub unsafe extern "C" fn wasi_snapshot_preview1__fd_prestat_get(_: usize, _: usize) -> u16 {
    ERRNO_BADF
}

#[no_mangle]
pub unsafe extern "C" fn wasi_snapshot_preview1__fd_prestat_dir_name(
    _: usize,
    _: usize,
    _: usize,
) -> u16 {
    ERRNO_BADF
}
