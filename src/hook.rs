#[macro_export]
macro_rules! hook {
    ($function_name:ident($($parameter_name:ident: $parameter_type:ty),*) -> $return_type:ty => $new_function_name:ident $body:block) => {
        #[allow(non_upper_case_globals)]
        static $new_function_name: std::sync::LazyLock<fn($($parameter_type),*) -> $return_type> = std::sync::LazyLock::new(|| {
            let function_name = stringify!($function_name);
            let function_name_cstring = std::ffi::CString::new(function_name).unwrap();
            let function_pointer = unsafe { libc::dlsym(libc::RTLD_NEXT, function_name_cstring.as_ptr()) };
            if function_pointer.is_null() {
                panic!("Unable to find function {function_name:?}");
            }
            unsafe { std::mem::transmute(function_pointer) }
        });

        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $function_name($($parameter_name: $parameter_type),*) -> $return_type {
            $body
        }
    }
}
