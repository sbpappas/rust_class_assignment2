//I used rust analyzer, rustfmt, clippy
use libc::c_char;
use std::ffi::CStr;

extern "C" {
    fn hello_world() -> *const c_char;
}

fn call_c() -> &'static str {
    unsafe {
        CStr::from_ptr(hello_world())
            .to_str()
            .expect("String conversion failure")
    }
}

fn main() {
    println!("{}", subproj_lib::hello_world());
    println!("{}", call_c());
    println!("This is a test so that the rustfmt.toml file will chop off part of the line because it exceeds the max width.");
}

#[cfg(test)]
mod tests {
    use crate::call_c;

    #[test]
    fn it_works() {
        let result = call_c();
        assert_eq!(result, "Hello from the C Library!");
    }
}
