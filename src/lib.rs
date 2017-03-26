#![deny(warnings, unused_variables, unsafe_code, unused_extern_crates)]
#![allow(dead_code)]

mod case;
use ::case::*;

macro_rules! define_trait_for {
    ( $trait:ident; $($imp_trait:ident => $typ:ident), *) => {
        pub trait $trait {
            $(
                fn $imp_trait(&self) -> $typ;
            )*
        }
    }
}
macro_rules! define_implementation {
    ( $slf:ident; $imp_trait:ident => $typ:ident) => {
        #[inline]
        fn $imp_trait(&$slf) -> $typ {
            $imp_trait($slf)
        }
    }
}

macro_rules! implement_for {
    ( $trt:ident; $typ:ident; $($imp_trait:ident  => $type:ident), *) => {
        impl $trt for $typ {
            $(
                define_implementation![self; $imp_trait => $type];
            )*
        }
    }
}

macro_rules! trait_with_impl {
    ( $trt:ident; $typ:ident; ($($imp_trait:ident  => $type:ident), *); ($($impl:item), *)) => {
        define_trait_for!(
            $trt;
            $($imp_trait => $type),*
        );

        implement_for!(
            $trt;
            $typ;
            $($imp_trait => $type),*
        );

        $(
            $impl
        )*
    }
}

macro_rules! _impl{
    ( $trt:ident; $typ:ident; ($($imp_trait:ident => $struct:ident => $type:ident), *); ($($impl:item), *)) => {
        define_trait_for!(
            $trt;
            $($imp_trait => $type),*
        );

        implement_for!(
            $trt;
            $typ;
            $($imp_trait => $type),*
        );

        $(
            #[inline]
            $impl
        )*

        $(
            fn $imp_trait(test: &$typ) -> $type {
                to_case_camel_like(test, $struct())
            }
        )*
    }
}

trait_with_impl!(
    InflectorOther;
    str;
    (
        is_camel_case => bool,
        is_pascal_case => bool
    );
    (
        fn is_camel_case(test: &str) -> bool {
            test == test.to_camel_case()
        },

        fn is_pascal_case(test: &str) -> bool {
            test == test.to_pascal_case()
        }
    )
);

_impl!(
    Inflector;
    str;
    (
        to_camel_case => camel => String,
        to_pascal_case => pascal => String
    );
    (
        fn camel() -> CamelOptions {
            CamelOptions {
                new_word: false,
                last_char: ' ',
                first_word: false,
                injectable_char: ' ',
                has_seperator: false,
                inverted: false,
            }
        },
        fn pascal() -> CamelOptions {
            CamelOptions {
                new_word: true,
                last_char: ' ',
                first_word: false,
                injectable_char: ' ',
                has_seperator: false,
                inverted: false,
            }
        }
    )
);


#[test]
fn to_camel_case_returns_the_string_snake() {
    assert_eq!("foo_bar".to_camel_case(), "fooBar".to_owned())
}

#[test]
fn to_camel_case_returns_the_string_kebab() {
    assert_eq!("foo-bar".to_camel_case(), "fooBar".to_owned())
}

#[test]
fn to_camel_case_returns_the_string_screaming_snake() {
    assert_eq!("FOO_BAR".to_camel_case(), "fooBar".to_owned())
}

#[test]
fn to_camel_case_returns_the_string_camel() {
    assert_eq!("fooBar".to_camel_case(), "fooBar".to_owned())
}

#[test]
fn to_camel_case_returns_the_string_pascal() {
    assert_eq!("FooBar".to_camel_case(), "fooBar".to_owned())
}

#[test]
fn to_camel_case_returns_the_string_sentence() {
    assert_eq!("Foo bar".to_camel_case(), "fooBar".to_owned())
}

#[test]
fn to_camel_case_returns_the_string_title() {
    assert_eq!("Foo Bar".to_camel_case(), "fooBar".to_owned())
}

#[test]
fn to_camel_case_returns_the_string_train() {
    assert_eq!("Foo-Bar".to_camel_case(), "fooBar".to_owned())
}

#[test]
fn to_pascal_case_returns_the_string() {
    assert_eq!("foo_bar".to_pascal_case(), "FooBar".to_owned())
}

#[test]
fn is_camel_case_returns_true() {
    assert_eq!("fooBar".is_camel_case(), true)
}

#[test]
fn is_camel_case_returns_false() {
    assert_eq!("foo_bar".is_camel_case(), false)
}

#[test]
fn is_pascal_case_returns_true() {
    assert_eq!("FooBar".is_pascal_case(), true)
}

#[test]
fn is_pascal_case_returns_false() {
    assert_eq!("foo_bar".is_pascal_case(), false)
}
