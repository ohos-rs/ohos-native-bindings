#[macro_export]
macro_rules! member_exists {
    ($struct:expr, $field:ident) => {
        {
            use std::mem;
            let struct_ptr = $struct as *const _;
            let field_ptr = &(*struct_ptr).$field as *const _;
            let offset = field_ptr as usize - struct_ptr as usize;
            offset + mem::size_of_val(&(*struct_ptr).$field) <= mem::size_of_val($struct)
        }
    };
}

#[macro_export]
macro_rules! member_missing {
    ($struct:expr, $field:ident) => {
        {
            !member_exists!($struct, $field) || {
                let field_value = &(*$struct).$field;
                match field_value {
                    // For numeric types, check if zero
                    n if std::any::type_name::<T>() == "i32" || std::any::type_name::<T>() == "u32" => *n == 0,
                    // For boolean, check if false
                    b if std::any::type_name::<T>() == "bool" => !*b,
                    // For Option types, check if None
                    o if std::any::type_name::<T>() == "Option<T>" => o.is_none(),
                    // For references, check if null
                    r if std::any::type_name::<T>() == "&T" => r.is_null(),
                    // For other types, consider them as missing if they're default
                    _ => field_value == &Default::default()
                }
            }
        }
    };
} 