// #![feature(static_nobundle)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::{c_short};

    #[test]
    fn test_valid_aoflagger_version() {
        unsafe {
            let mut major: c_short = -1;
            let mut minor: c_short = -1;
            let mut subMinor: c_short = -1;
            aoflagger_AOFlagger_GetVersion(&mut major, &mut minor, &mut subMinor);
            assert!(major != -1);
            assert!(minor != -1);
            assert!(subMinor != -1);
        }
    }
}
