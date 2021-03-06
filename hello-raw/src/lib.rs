


pub extern "C" fn greet(name: &str) -> String{
    format!("Hello {}!", name )
}
// "C" modifier makes sure we're calling right convention
#[export_name = "greet"]
pub extern "C" fn __greet_wrapper(
    arg0_ptr: *const u8, 
    arg0_len: usize, 
) -> *mut String {
    let arg0 = unsafe {
        let slice = 
            std::slice::from_raw_parts(arg0_ptr, arg0_len);
            std::str::from_utf8_unchecked(slice)
    };
    let _ret = greet(arg0);
    Box::into_raw(Box::new(_ret))
}