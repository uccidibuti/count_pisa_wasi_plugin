use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub fn wasm_malloc(size: u32) -> u32 {
    // println!("Allocating {} bytes", size);
    let mut memory: Vec<u8> = vec![0; size as usize];
    // println!("wasm_malloc = {:?}", memory);
    let memory_ptr = memory.as_mut_ptr();
    std::mem::forget(memory_ptr);
    memory_ptr as u32
}

#[no_mangle]
pub fn wasm_dealloc(memory_ptr: *mut u8, size: usize) {
    let memory = unsafe { Vec::from_raw_parts(memory_ptr, size, size) };
    // println!("wasm_dealloc = {:?}", memory);
    std::mem::drop(memory);
}

#[link(wasm_import_module = "host")]
extern "C" {
    fn get_tuscany_city() -> u32;
}

fn get_cstr_from_memory_ptr<'a>(memory_ptr: *const u8) -> &'a CStr {
    char_pointer_to_cstr(memory_ptr as *const c_char)
}

fn char_pointer_to_cstr<'a>(p: *const c_char) -> &'a CStr {
    unsafe {
	return CStr::from_ptr(p);
    }
}

#[no_mangle]
pub fn check_if_pisa() -> u32 {
    let city_pointer = unsafe { get_tuscany_city() };
    let city = get_cstr_from_memory_ptr(city_pointer as *const u8);
    let city_0 = get_cstr_from_memory_ptr(city_pointer as *const u8);
    // println!("city = {:?}, buf_len = {}", city, city.to_bytes().len());
    let mut res = 0;
    if city.to_bytes() == b"PISA" && city_0.to_bytes() == b"PISA" {
	res = 1;
    }
    // wasm_dealloc(city.as_ptr() as *mut u8, city.to_bytes_with_nul().len());
    res
}


