// #![feature(static_nobundle)]
// mod aoflagger_bindings {
//     #![allow(non_upper_case_globals)]
//     #![allow(non_camel_case_types)]
//     #![allow(non_snake_case)]
//     #![allow(clippy::all)]
//     include!(concat!(env!("OUT_DIR"), "/aoflagger_bindings.rs"));
// }

// mod glue;

#[cxx::bridge]
mod ffi {


    unsafe extern "C++" {
        include!("birli/include/cxx_aoflagger.h");

        // type ImageSet;

        fn aoflagger_GetVersion(
            major: &mut i16,
            minor: &mut i16,
            subMinor: &mut i16,
        );

        type CxxAOFlagger;
        fn cxx_aoflagger_new() -> UniquePtr<CxxAOFlagger>;
        fn GetVersion(
            self: &CxxAOFlagger,
            major: &mut i16,
            minor: &mut i16,
            subMinor: &mut i16,
        );

        // fn cxx_aoflagger_MakeImageSet(
        //     width: usize,
        //     height: usize,
        //     count: usize,
        // ) -> ImageSet;
    }
}

#[cfg(test)]
mod tests {
    // use super::glue::{
    //     glue_aoflagger_AOFlagger_MakeImageSet, aoflagger_ImageSet, aoflagger_ImageSet_Height,
    //     aoflagger_ImageSet_ImageCount, aoflagger_ImageSet_ImageSet_destructor,
    //     aoflagger_ImageSet_Width, glue_aoflagger_AOFlagger_GetVersion,
    // };
    use super::ffi::{aoflagger_GetVersion, cxx_aoflagger_new};
    use std::os::raw::c_short;

    #[test]
    fn test_valid_aoflagger_version() {
        let mut major: c_short = -1;
        let mut minor: c_short = -1;
        let mut sub_minor: c_short = -1;
        aoflagger_GetVersion(&mut major, &mut minor, &mut sub_minor);
        assert!(major != -1);
        assert!(minor != -1);
        assert!(sub_minor != -1);
    }

    #[test]
    fn test_valid_cxx_aoflagger_version() {
        let mut major: c_short = -1;
        let mut minor: c_short = -1;
        let mut sub_minor: c_short = -1;
        let aoflagger = cxx_aoflagger_new();
        aoflagger.GetVersion(&mut major, &mut minor, &mut sub_minor);
        assert!(major != -1);
        assert!(minor != -1);
        assert!(sub_minor != -1);
    }

    // #[test]
    // fn test_valid_image_set() {
    //     let width = 2;
    //     let height = 3;
    //     let count = 4;
    //     unsafe {
    //         let mut image_set: aoflagger_ImageSet =
    //             glue_aoflagger_AOFlagger_MakeImageSet(width, height, count);
    //         assert_eq!(aoflagger_ImageSet_Width(&mut image_set), width);
    //         assert_eq!(aoflagger_ImageSet_Height(&mut image_set), height);
    //         assert_eq!(aoflagger_ImageSet_ImageCount(&mut image_set), count);
    //         aoflagger_ImageSet_ImageSet_destructor(&mut image_set);
    //     }
    // }
}
