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
