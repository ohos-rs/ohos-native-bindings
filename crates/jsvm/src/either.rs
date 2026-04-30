use crate::{Env, FromJsValue, Result, ToJsValue, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> ToJsValue for Either<A, B>
where
    A: ToJsValue,
    B: ToJsValue,
{
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        match self {
            Self::A(value) => value.to_js_value(env),
            Self::B(value) => value.to_js_value(env),
        }
    }
}

impl<A, B> FromJsValue for Either<A, B>
where
    A: FromJsValue,
    B: FromJsValue,
{
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        match A::from_js_value(env, value) {
            Ok(value) => Ok(Self::A(value)),
            Err(_) => B::from_js_value(env, value).map(Self::B),
        }
    }
}
