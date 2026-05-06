use crate::{Env, FromJsValue, Result, ToJsValue, Value};

macro_rules! either_from_js_value {
    ($env:expr, $value:expr, $parameter:ident) => {
        $parameter::from_js_value($env, $value).map(Self::$parameter)
    };
    ($env:expr, $value:expr, $parameter:ident, $($rest:ident),+ $(,)?) => {
        match $parameter::from_js_value($env, $value) {
            Ok(value) => Ok(Self::$parameter(value)),
            Err(_) => either_from_js_value!($env, $value, $($rest),+),
        }
    };
}

macro_rules! either_n {
    ($either_name:ident, $($parameter:ident),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $either_name<$($parameter),+> {
            $($parameter($parameter)),+
        }

        impl<$($parameter),+> ToJsValue for $either_name<$($parameter),+>
        where
            $($parameter: ToJsValue),+
        {
            fn to_js_value(&self, env: &Env) -> Result<Value> {
                match self {
                    $(Self::$parameter(value) => value.to_js_value(env)),+
                }
            }
        }

        impl<$($parameter),+> FromJsValue for $either_name<$($parameter),+>
        where
            $($parameter: FromJsValue),+
        {
            fn from_js_value(env: &Env, value: Value) -> Result<Self> {
                either_from_js_value!(env, value, $($parameter),+)
            }
        }
    };
}

either_n!(Either, A, B);
either_n!(Either3, A, B, C);
either_n!(Either4, A, B, C, D);
either_n!(Either5, A, B, C, D, E);
either_n!(Either6, A, B, C, D, E, F);
either_n!(Either7, A, B, C, D, E, F, G);
either_n!(Either8, A, B, C, D, E, F, G, H);
either_n!(Either9, A, B, C, D, E, F, G, H, I);
either_n!(Either10, A, B, C, D, E, F, G, H, I, J);
either_n!(Either11, A, B, C, D, E, F, G, H, I, J, K);
either_n!(Either12, A, B, C, D, E, F, G, H, I, J, K, L);
either_n!(Either13, A, B, C, D, E, F, G, H, I, J, K, L, M);
either_n!(Either14, A, B, C, D, E, F, G, H, I, J, K, L, M, N);
either_n!(Either15, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
either_n!(Either16, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
either_n!(Either17, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
either_n!(Either18, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
either_n!(Either19, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
either_n!(Either20, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
either_n!(Either21, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
either_n!(Either22, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
either_n!(Either23, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
either_n!(Either24, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
either_n!(Either25, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
either_n!(Either26, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_from_js_value<T: FromJsValue>() {}
    fn assert_to_js_value<T: ToJsValue>() {}

    #[test]
    fn either_26_exposes_all_variants() {
        type E = Either26<
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
        >;

        assert_eq!(E::A(1), E::A(1));
        assert_eq!(E::Z(26), E::Z(26));
    }

    #[test]
    fn either_n_implements_js_value_traits() {
        assert_from_js_value::<Either3<i32, String, bool>>();
        assert_to_js_value::<Either3<i32, String, bool>>();
        assert_from_js_value::<
            Either26<
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
            >,
        >();
        assert_to_js_value::<
            Either26<
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
            >,
        >();
    }
}
