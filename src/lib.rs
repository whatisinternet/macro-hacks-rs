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

trait_with_impl!(
    Inflector;
    str;
    (
        to_camel_case => String,
        is_camel_case => bool
    );
    (
        fn to_camel_case(test: &str) -> String {
            let camel_options = CamelOptions {
                new_word: false,
                last_char: ' ',
                first_word: false,
                injectable_char: ' ',
                has_seperator: false,
                inverted: false,
                convertable_string: test,
            };
            to_case_camel_like(camel_options)
        },
        fn is_camel_case(test: &str) -> bool {
            test == to_camel_case(test)
        }
    )
);

#[test]
fn to_camel_case_returns_the_string() {
    assert_eq!("foo_bar".to_camel_case(), "fooBar".to_owned())
}

#[test]
fn is_camel_case_returns_true() {
    assert_eq!("fooBar".is_camel_case(), true)
}

#[test]
fn is_camel_case_returns_false() {
    assert_eq!("foo_bar".is_camel_case(), false)
}
