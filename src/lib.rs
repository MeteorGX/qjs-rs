#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


/// private api regin
#[allow(unused)]
pub(crate) mod qjs{
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


mod error;
mod inner;
mod utils;


pub use error::{
    JSError,
    JSResult,
};


pub use inner::{
    JSRuntime,
    JSContext,
    JSValue,
};