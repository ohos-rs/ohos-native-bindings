use std::ffi::c_void;
use std::marker::PhantomData;
use std::mem;
use std::slice;

use ohos_jsvm_sys as sys;

use crate::error::{check_status_with_env, type_mismatch, JsvmError, Result};
use crate::{Env, ToJsValue, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypedArrayType {
    Int8,
    Uint8,
    Uint8Clamped,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Float32,
    Float64,
    BigInt64,
    BigUint64,
}

impl TypedArrayType {
    pub fn from_raw(raw: sys::JSVM_TypedarrayType) -> Option<Self> {
        match raw {
            sys::JSVM_TypedarrayType_JSVM_INT8_ARRAY => Some(Self::Int8),
            sys::JSVM_TypedarrayType_JSVM_UINT8_ARRAY => Some(Self::Uint8),
            sys::JSVM_TypedarrayType_JSVM_UINT8_CLAMPED_ARRAY => Some(Self::Uint8Clamped),
            sys::JSVM_TypedarrayType_JSVM_INT16_ARRAY => Some(Self::Int16),
            sys::JSVM_TypedarrayType_JSVM_UINT16_ARRAY => Some(Self::Uint16),
            sys::JSVM_TypedarrayType_JSVM_INT32_ARRAY => Some(Self::Int32),
            sys::JSVM_TypedarrayType_JSVM_UINT32_ARRAY => Some(Self::Uint32),
            sys::JSVM_TypedarrayType_JSVM_FLOAT32_ARRAY => Some(Self::Float32),
            sys::JSVM_TypedarrayType_JSVM_FLOAT64_ARRAY => Some(Self::Float64),
            sys::JSVM_TypedarrayType_JSVM_BIGINT64_ARRAY => Some(Self::BigInt64),
            sys::JSVM_TypedarrayType_JSVM_BIGUINT64_ARRAY => Some(Self::BigUint64),
            _ => None,
        }
    }

    pub fn as_raw(self) -> sys::JSVM_TypedarrayType {
        match self {
            Self::Int8 => sys::JSVM_TypedarrayType_JSVM_INT8_ARRAY,
            Self::Uint8 => sys::JSVM_TypedarrayType_JSVM_UINT8_ARRAY,
            Self::Uint8Clamped => sys::JSVM_TypedarrayType_JSVM_UINT8_CLAMPED_ARRAY,
            Self::Int16 => sys::JSVM_TypedarrayType_JSVM_INT16_ARRAY,
            Self::Uint16 => sys::JSVM_TypedarrayType_JSVM_UINT16_ARRAY,
            Self::Int32 => sys::JSVM_TypedarrayType_JSVM_INT32_ARRAY,
            Self::Uint32 => sys::JSVM_TypedarrayType_JSVM_UINT32_ARRAY,
            Self::Float32 => sys::JSVM_TypedarrayType_JSVM_FLOAT32_ARRAY,
            Self::Float64 => sys::JSVM_TypedarrayType_JSVM_FLOAT64_ARRAY,
            Self::BigInt64 => sys::JSVM_TypedarrayType_JSVM_BIGINT64_ARRAY,
            Self::BigUint64 => sys::JSVM_TypedarrayType_JSVM_BIGUINT64_ARRAY,
        }
    }

    pub fn element_size(self) -> usize {
        match self {
            Self::Int8 | Self::Uint8 | Self::Uint8Clamped => 1,
            Self::Int16 | Self::Uint16 => 2,
            Self::Int32 | Self::Uint32 | Self::Float32 => 4,
            Self::Float64 | Self::BigInt64 | Self::BigUint64 => 8,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArrayBuffer {
    value: Value,
}

impl ArrayBuffer {
    pub fn from_value(env: &Env, value: Value) -> Result<Self> {
        if value.is_arraybuffer(env)? {
            Ok(Self { value })
        } else {
            Err(type_mismatch("ArrayBuffer", Some(value.value_type(env)?)))
        }
    }

    pub fn as_value(self) -> Value {
        self.value
    }

    pub fn info(self, env: &Env) -> Result<ArrayBufferInfo> {
        let mut data = std::ptr::null_mut::<c_void>();
        let mut byte_length = 0usize;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetArraybufferInfo(
                env.as_raw(),
                self.value.as_raw(),
                &mut data,
                &mut byte_length,
            )
        })?;
        Ok(ArrayBufferInfo { data, byte_length })
    }

    pub fn bytes(self, env: &Env) -> Result<Vec<u8>> {
        let info = self.info(env)?;
        if info.byte_length == 0 {
            return Ok(Vec::new());
        }
        Ok(unsafe { slice::from_raw_parts(info.data.cast::<u8>(), info.byte_length) }.to_vec())
    }

    pub fn detach(self, env: &Env) -> Result<()> {
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_DetachArraybuffer(env.as_raw(), self.value.as_raw())
        })
    }

    pub fn is_detached(self, env: &Env) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_IsDetachedArraybuffer(env.as_raw(), self.value.as_raw(), &mut result)
        })?;
        Ok(result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayBufferInfo {
    pub data: *mut c_void,
    pub byte_length: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypedArray {
    value: Value,
}

impl TypedArray {
    pub fn from_value(env: &Env, value: Value) -> Result<Self> {
        if value.is_typedarray(env)? {
            Ok(Self { value })
        } else {
            Err(type_mismatch("TypedArray", Some(value.value_type(env)?)))
        }
    }

    pub fn as_value(self) -> Value {
        self.value
    }

    pub fn info(self, env: &Env) -> Result<TypedArrayInfo> {
        let mut raw_type = 0;
        let mut length = 0usize;
        let mut data = std::ptr::null_mut::<c_void>();
        let mut arraybuffer = std::ptr::null_mut();
        let mut byte_offset = 0usize;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetTypedarrayInfo(
                env.as_raw(),
                self.value.as_raw(),
                &mut raw_type,
                &mut length,
                &mut data,
                &mut arraybuffer,
                &mut byte_offset,
            )
        })?;

        let array_type = TypedArrayType::from_raw(raw_type)
            .ok_or(JsvmError::Status(sys::JSVM_Status_JSVM_INVALID_ARG))?;
        Ok(TypedArrayInfo {
            array_type,
            length,
            data,
            arraybuffer: ArrayBuffer::from_value(env, Value::from_raw(arraybuffer)?)?,
            byte_offset,
        })
    }

    pub fn bytes(self, env: &Env) -> Result<Vec<u8>> {
        let info = self.info(env)?;
        let byte_length = info.length * info.array_type.element_size();
        if byte_length == 0 {
            return Ok(Vec::new());
        }
        Ok(unsafe { slice::from_raw_parts(info.data.cast::<u8>(), byte_length) }.to_vec())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypedArrayInfo {
    pub array_type: TypedArrayType,
    pub length: usize,
    pub data: *mut c_void,
    pub arraybuffer: ArrayBuffer,
    pub byte_offset: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataView {
    value: Value,
}

impl DataView {
    pub fn from_value(env: &Env, value: Value) -> Result<Self> {
        if value.is_dataview(env)? {
            Ok(Self { value })
        } else {
            Err(type_mismatch("DataView", Some(value.value_type(env)?)))
        }
    }

    pub fn as_value(self) -> Value {
        self.value
    }

    pub fn info(self, env: &Env) -> Result<DataViewInfo> {
        let mut byte_length = 0usize;
        let mut data = std::ptr::null_mut::<c_void>();
        let mut arraybuffer = std::ptr::null_mut();
        let mut byte_offset = 0usize;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetDataviewInfo(
                env.as_raw(),
                self.value.as_raw(),
                &mut byte_length,
                &mut data,
                &mut arraybuffer,
                &mut byte_offset,
            )
        })?;
        Ok(DataViewInfo {
            byte_length,
            data,
            arraybuffer: ArrayBuffer::from_value(env, Value::from_raw(arraybuffer)?)?,
            byte_offset,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataViewInfo {
    pub byte_length: usize,
    pub data: *mut c_void,
    pub arraybuffer: ArrayBuffer,
    pub byte_offset: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Buffer(pub Vec<u8>);

impl Buffer {
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Buffer {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl From<Buffer> for Vec<u8> {
    fn from(value: Buffer) -> Self {
        value.0
    }
}

impl crate::ToJsValue for Buffer {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.create_arraybuffer(&self.0)
    }
}

impl crate::FromJsValue for Buffer {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        Ok(Self(value.bytes(env)?))
    }
}

pub trait TypedArrayElement: Copy + 'static {
    const ARRAY_TYPE: TypedArrayType;
}

macro_rules! impl_typed_array_element {
    ($ty:ty, $array_type:expr) => {
        impl TypedArrayElement for $ty {
            const ARRAY_TYPE: TypedArrayType = $array_type;
        }
    };
}

impl_typed_array_element!(i8, TypedArrayType::Int8);
impl_typed_array_element!(u8, TypedArrayType::Uint8);
impl_typed_array_element!(i16, TypedArrayType::Int16);
impl_typed_array_element!(u16, TypedArrayType::Uint16);
impl_typed_array_element!(i32, TypedArrayType::Int32);
impl_typed_array_element!(u32, TypedArrayType::Uint32);
impl_typed_array_element!(f32, TypedArrayType::Float32);
impl_typed_array_element!(f64, TypedArrayType::Float64);

#[derive(Clone, Debug, PartialEq)]
pub struct TypedArrayValue<T: TypedArrayElement> {
    values: Vec<T>,
    _marker: PhantomData<T>,
}

impl<T: TypedArrayElement> TypedArrayValue<T> {
    pub fn new(values: Vec<T>) -> Self {
        Self {
            values,
            _marker: PhantomData,
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.values
    }
}

impl<T: TypedArrayElement> From<Vec<T>> for TypedArrayValue<T> {
    fn from(values: Vec<T>) -> Self {
        Self::new(values)
    }
}

impl<T: TypedArrayElement> ToJsValue for TypedArrayValue<T> {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        let byte_length = self.values.len() * mem::size_of::<T>();
        let bytes =
            unsafe { slice::from_raw_parts(self.values.as_ptr().cast::<u8>(), byte_length) };
        let arraybuffer = env.create_arraybuffer(bytes)?;
        env.create_typedarray(T::ARRAY_TYPE, self.values.len(), arraybuffer, 0)
    }
}

impl<T: TypedArrayElement> crate::FromJsValue for TypedArrayValue<T> {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let typed_array = TypedArray::from_value(env, value)?;
        let info = typed_array.info(env)?;
        if info.array_type != T::ARRAY_TYPE {
            return Err(type_mismatch(
                "matching TypedArray",
                Some(value.value_type(env)?),
            ));
        }
        if info.length == 0 {
            return Ok(Self::new(Vec::new()));
        }
        let slice = unsafe { slice::from_raw_parts(info.data.cast::<T>(), info.length) };
        Ok(Self::new(slice.to_vec()))
    }
}
