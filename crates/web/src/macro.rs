#[macro_export]
macro_rules! ark_web_member_exists {
    ($s:expr, $f:ident) => {{
        let s_ptr = $s as *const _ as *const u8;
        let f_ptr = &((*$s).$f) as *const _ as *const u8;
        let f_size = std::mem::size_of_val(&((*$s).$f));
        let offset = f_ptr as usize - s_ptr as usize;

        // 访问结构体的size字段，假设它是第一个字段
        let struct_size = *(s_ptr as *const usize);

        (offset + f_size <= struct_size)
    }};
}

#[macro_export]
macro_rules! ark_web_member_missing {
    ($s:expr, $f:ident) => {
        (!$crate::ark_web_member_exists!($s, $f) || !((*$s).$f).is_some())
    };
}
