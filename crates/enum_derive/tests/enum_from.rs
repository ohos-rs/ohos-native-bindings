#![allow(non_upper_case_globals)]

use ohos_enum_derive::EnumFrom;

type RawEnum = isize;

const RawEnum_PREFIX_DEFAULT: RawEnum = 1;
const RawEnum_CUSTOM: RawEnum = 2;
const RawEnum_PREFIX_HTTP2: RawEnum = 3;
const RawEnum_FULL_OVERRIDE: RawEnum = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(RawEnum, "RawEnum_PREFIX_", i32, u32)]
enum TestEnum {
    Default,
    #[prefix("RawEnum_")]
    Custom,
    #[suffix("HTTP2")]
    Http2,
    #[alias("RawEnum_FULL_OVERRIDE")]
    Alias,
    #[cfg(any())]
    Conditional,
}

#[test]
fn converts_enum_to_raw() {
    assert_eq!(RawEnum::from(TestEnum::Default), RawEnum_PREFIX_DEFAULT);
    assert_eq!(RawEnum::from(TestEnum::Custom), RawEnum_CUSTOM);
    assert_eq!(RawEnum::from(TestEnum::Http2), RawEnum_PREFIX_HTTP2);
    assert_eq!(RawEnum::from(TestEnum::Alias), RawEnum_FULL_OVERRIDE);
    assert_eq!(i32::from(TestEnum::Default), RawEnum_PREFIX_DEFAULT as i32);
    assert_eq!(u32::from(TestEnum::Custom), RawEnum_CUSTOM as u32);
}

#[test]
fn converts_raw_to_enum() {
    assert_eq!(TestEnum::from(RawEnum_PREFIX_DEFAULT), TestEnum::Default);
    assert_eq!(TestEnum::from(RawEnum_CUSTOM), TestEnum::Custom);
    assert_eq!(TestEnum::from(RawEnum_PREFIX_HTTP2), TestEnum::Http2);
    assert_eq!(TestEnum::from(RawEnum_FULL_OVERRIDE), TestEnum::Alias);
    assert_eq!(
        TestEnum::from(RawEnum_PREFIX_DEFAULT as i32),
        TestEnum::Default
    );
    assert_eq!(TestEnum::from(RawEnum_CUSTOM as u32), TestEnum::Custom);
    assert_eq!(TestEnum::try_from_raw(0), None);
}

const RawEnum_UNIT_UNKNOWN: RawEnum = 0;
const RawEnum_UNIT_FIRST: RawEnum = 1;
const RawEnum_UNIT_SECOND: RawEnum = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(RawEnum, "RawEnum_UNIT_", i32, u32)]
enum UnitFallbackEnum {
    #[fallback]
    Unknown,
    First,
    Second,
    #[cfg(any())]
    DisabledFutureVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(RawEnum, "RawEnum_UNIT_")]
enum UnitRawFallbackEnum {
    First,
    Second,
    #[fallback(raw = -1)]
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(RawEnum, "RawEnum_UNIT_", i32, u32)]
enum TupleFallbackEnum {
    Unknown,
    First,
    Second,
    #[cfg(any())]
    DisabledFutureVariant,
    #[fallback]
    Other(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(RawEnum, "RawEnum_UNIT_")]
#[fallback(|raw| if raw < 0 { ContainerFallbackEnum::Unknown } else { ContainerFallbackEnum::First })]
enum ContainerFallbackEnum {
    Unknown,
    First,
    Second,
}

#[test]
fn supports_unit_fallback_variant() {
    assert_eq!(
        UnitFallbackEnum::from(RawEnum_UNIT_FIRST),
        UnitFallbackEnum::First
    );
    assert_eq!(
        UnitFallbackEnum::from(RawEnum_UNIT_UNKNOWN),
        UnitFallbackEnum::Unknown
    );
    assert_eq!(UnitFallbackEnum::from(999isize), UnitFallbackEnum::Unknown);
    assert_eq!(UnitFallbackEnum::from(999u32), UnitFallbackEnum::Unknown);
    assert_eq!(
        RawEnum::from(UnitFallbackEnum::Unknown),
        RawEnum_UNIT_UNKNOWN
    );
    assert_eq!(
        UnitFallbackEnum::try_from_raw(RawEnum_UNIT_UNKNOWN),
        Some(UnitFallbackEnum::Unknown)
    );
    assert_eq!(UnitFallbackEnum::try_from_raw(999), None);
}

#[test]
fn supports_unit_fallback_with_explicit_raw_expr() {
    assert_eq!(
        UnitRawFallbackEnum::from(RawEnum_UNIT_SECOND),
        UnitRawFallbackEnum::Second
    );
    assert_eq!(UnitRawFallbackEnum::from(42), UnitRawFallbackEnum::Invalid);
    assert_eq!(RawEnum::from(UnitRawFallbackEnum::Invalid), -1);
    assert_eq!(UnitRawFallbackEnum::try_from_raw(-1), None);
}

#[test]
fn supports_tuple_fallback_variant_preserving_raw_value() {
    assert_eq!(
        TupleFallbackEnum::from(RawEnum_UNIT_UNKNOWN),
        TupleFallbackEnum::Unknown
    );
    assert_eq!(
        TupleFallbackEnum::from(77isize),
        TupleFallbackEnum::Other(77)
    );
    assert_eq!(TupleFallbackEnum::from(88i32), TupleFallbackEnum::Other(88));
    assert_eq!(RawEnum::from(TupleFallbackEnum::Other(77)), 77);
    assert_eq!(u32::from(TupleFallbackEnum::Other(77)), 77);
    assert_eq!(
        TupleFallbackEnum::try_from_raw(RawEnum_UNIT_UNKNOWN),
        Some(TupleFallbackEnum::Unknown)
    );
    assert_eq!(TupleFallbackEnum::try_from_raw(77), None);
}

#[test]
fn supports_container_fallback_closure() {
    assert_eq!(
        ContainerFallbackEnum::from(-5),
        ContainerFallbackEnum::Unknown
    );
    assert_eq!(
        ContainerFallbackEnum::from(99),
        ContainerFallbackEnum::First
    );
    assert_eq!(ContainerFallbackEnum::try_from_raw(99), None);
}
