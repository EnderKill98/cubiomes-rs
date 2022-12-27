#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // Disable u128/i128 not FFI-safe warning

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use std::ffi::{c_int, CStr, CString};

    /// Do some random function to proove the lib was built and linked properly
    #[test]
    fn test_lib_linked() {
        unsafe {
            assert_eq!(
                CStr::from_ptr(super::biome2str(
                    super::MCVersion_MC_1_19 as c_int,
                    super::BiomeID_badlands
                )),
                CString::new("badlands").unwrap().as_c_str()
            );
        }
    }
}
