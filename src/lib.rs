#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


/// private api regin
#[allow(unused)]
pub(crate) mod qjs_c {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


mod error;

mod utils;
mod inner;
mod types;


pub use error::{
    JSError,
    JSResult,
};

pub use types::*;
pub use inner::{
    JSEvalType,
    JSRuntime,
    JSContext,
};




