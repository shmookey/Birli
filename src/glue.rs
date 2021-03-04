#![allow(non_camel_case_types, non_snake_case, unused)]

extern "C" {
    #[link_name = "\u{1}__ZN9aoflagger9AOFlagger10GetVersionERsS1_S1_"]
    pub fn aoflagger_AOFlagger_GetVersion(
        major: *mut ::std::os::raw::c_short,
        minor: *mut ::std::os::raw::c_short,
        subMinor: *mut ::std::os::raw::c_short,
    );
}
