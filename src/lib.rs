#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn create_context() {
        unsafe {
            let context = mlirContextCreate();

            assert!(!context.ptr.is_null());

            mlirContextDestroy(context);
        };

    }
}
