#[macro_export]
macro_rules! ark_web_member_exists {
    ($struct:expr, $field:ident) => {
        unsafe {
            let s_ptr = $struct as *const _ as *const u8;
            let f_ptr = &(*$struct).$field as *const _ as *const u8;
            let offset = f_ptr.offset_from(s_ptr) as usize;
            let size = std::mem::size_of_val(&(*$struct).$field);

            let s_size_ptr = $struct as *const _ as *const usize;
            let max_size = *s_size_ptr;

            offset + size <= max_size
        }
    };
}

#[macro_export]
macro_rules! ark_web_member_missing {
    ($struct:expr, $field:ident) => {{
        !crate::ark_web_member_exists!($struct, $field) || {
            let field_value = unsafe { &(*$struct).$field };
            field_value.is_none()
        }
    }};
}
