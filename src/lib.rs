mod hashable;
mod hasher;
mod hashmap;
pub mod ruby; // To ignore unused function

mod util {
    use std::ffi;

    pub fn cstring(str: &str) -> ffi::CString {
        ffi::CString::new(str).unwrap()
    }
}

#[export_name = "Init_rust_hashmap"]
pub extern "C" fn init() {
    let module = ruby::define_module(&util::cstring("Rust"));
    hashmap::define_ruby_class(&util::cstring("HashMap"), Some(module));
}
