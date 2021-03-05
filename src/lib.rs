#[cxx::bridge]
#[allow(dead_code)]
mod ffi {

    unsafe extern "C++" {
        include!("birli/include/cxx_aoflagger.h");

        fn aoflagger_GetVersion(major: &mut i16, minor: &mut i16, subMinor: &mut i16);

        type CxxImageSet;
        type CxxAOFlagger;
        unsafe fn cxx_aoflagger_new() -> UniquePtr<CxxAOFlagger>;

        // CxxAOFlagger methods
        fn GetVersion(self: &CxxAOFlagger, major: &mut i16, minor: &mut i16, subMinor: &mut i16);
        unsafe fn MakeImageSet(
            self: &CxxAOFlagger,
            width: usize,
            height: usize,
            count: usize,
            initialValue: f32,
            widthCapacity: usize,
        ) -> UniquePtr<CxxImageSet>;

        // CxxImageSet methods
        fn Width(self: &CxxImageSet) -> usize;
        fn Height(self: &CxxImageSet) -> usize;
        fn ImageCount(self: &CxxImageSet) -> usize;
        fn HorizontalStride(self: &CxxImageSet) -> usize;
        fn ImageBuffer(self: &CxxImageSet, imageIndex: usize) -> &mut [f32];
    }
}

#[cfg(test)]
mod tests {
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
        unsafe {
            let aoflagger = cxx_aoflagger_new();
            aoflagger.GetVersion(&mut major, &mut minor, &mut sub_minor);
        }
        assert!(major != -1);
        assert!(minor != -1);
        assert!(sub_minor != -1);
    }

    #[test]
    fn test_valid_image_set_init() {
        let width = 2 as usize;
        let height = 3 as usize;
        let count = 4 as usize;
        let initial_value = 5 as f32;
        let width_capacity = 6 as usize;
        unsafe {
            let aoflagger = cxx_aoflagger_new();
            let image_set =
                aoflagger.MakeImageSet(width, height, count, initial_value, width_capacity);
            assert_eq!(image_set.Width(), width);
            assert_eq!(image_set.Height(), height);
            assert_eq!(image_set.ImageCount(), count);
            let fist_buffer = image_set.ImageBuffer(0);
            assert_eq!(fist_buffer[0], 5 as f32);
            // aoflagger_ImageSet_ImageSet_destructor(&mut image_set);
        }
    }

    #[test]
    fn test_valid_image_set_rw() {
        let width = 2 as usize;
        let height = 3 as usize;
        let count = 4 as usize;
        let initial_value = 5 as f32;
        let width_capacity = 6 as usize;
        unsafe {
            let aoflagger = cxx_aoflagger_new();
            let image_set =
                aoflagger.MakeImageSet(width, height, count, initial_value, width_capacity);
            let first_buffer_write = image_set.ImageBuffer(0);
            first_buffer_write[0] = 7 as f32;
            let first_buffer_read = image_set.ImageBuffer(0);
            assert_eq!(first_buffer_read[0], 7 as f32);
        }
    }
}
