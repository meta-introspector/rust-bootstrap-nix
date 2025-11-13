extern crate openssl_sys;

fn main() {
    unsafe {
        let version = openssl_sys::OpenSSL_version(0);
        let c_str = std::ffi::CStr::from_ptr(version);
        let r_str = c_str.to_str().unwrap();
        println!("OpenSSL version: {}", r_str);
    }
}