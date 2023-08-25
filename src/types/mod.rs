mod tag;
mod tag_int;
mod tag_big_int;
mod tag_big_decimal;
mod tag_big_float;
mod tag_big_symbol;
mod tag_string;
mod tag_module;
mod tag_func_bytecode;
mod tag_null;
mod tag_undefined;
mod tag_float64;
mod tag_bool;
mod tag_catch_offset;
mod tag_exception;
mod tag_object;
mod tag_uninitialized;
mod tag_first;
mod tag_any;
mod tag_array;


use std::fmt::{Debug, Formatter};
use crate::{qjs_c};
pub use tag::JSRawType;
pub use tag_any::{JSAnyWrapper, JSAny};
pub use tag_big_float::*;
pub use tag_big_int::*;
pub use tag_big_symbol::*;
pub use tag_bool::*;
pub use tag_catch_offset::*;
pub use tag_exception::*;
pub use tag_first::*;
pub use tag_float64::*;
pub use tag_func_bytecode::*;
pub use tag_int::*;
pub use tag_module::*;
pub use tag_null::*;
pub use tag_object::*;
pub use tag_string::*;
pub use tag_undefined::*;
pub use tag_uninitialized::*;
pub use tag_array::*;



#[allow(unused_imports)]
pub(super) use tag_big_decimal::JSBigDecimal;


/// print JSValue Debug information
impl Debug for qjs_c::JSValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ JSValue: {{ tag:{} }} }}", self.tag)
    }
}


