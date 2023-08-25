use crate::{JSArray, JSContext, JSError, JSResult, qjs_c};
pub(super) use crate::types::tag_int::JSInt;
pub(super) use crate::types::tag_big_float::JSBigFloat;
pub(super) use crate::types::tag_big_int::JSBigInt;
pub(super) use crate::types::tag_big_symbol::JSSymbol;
pub(super) use crate::types::tag_bool::JSBool;
pub(super) use crate::types::tag_catch_offset::JSCatchOffset;
pub(super) use crate::types::tag_exception::JSException;
pub(super) use crate::types::tag_first::JSFirst;
pub(super) use crate::types::tag_float64::JSFloat64;
pub(super) use crate::types::tag_func_bytecode::JSFuncBytecode;
pub(super) use crate::types::tag_module::JSModule;
pub(super) use crate::types::tag_null::JSNull;
pub(super) use crate::types::tag_object::JSObject;
pub(super) use crate::types::tag_string::JSStr;
pub(super) use crate::types::tag_undefined::JSUndefined;
pub(super) use crate::types::tag_uninitialized::JSUninitialized;


#[derive(Debug)]
pub enum JSAny<'ctx, 'rt> {
    First(JSFirst<'ctx, 'rt>),
    // BigDecimal(JSBigDecimal<'ctx, 'rt>),
    BigInt(JSBigInt<'ctx, 'rt>),
    BigFloat(JSBigFloat<'ctx, 'rt>),
    Symbol(JSSymbol<'ctx, 'rt>),
    String(JSStr<'ctx, 'rt>),
    Module(JSModule<'ctx, 'rt>),
    FuncBytecode(JSFuncBytecode<'ctx, 'rt>),
    Int(JSInt<'ctx, 'rt>),
    Bool(JSBool<'ctx, 'rt>),
    Null(JSNull<'ctx, 'rt>),
    Undefined(JSUndefined<'ctx, 'rt>),
    Uninitialized(JSUninitialized<'ctx, 'rt>),
    CatchOffset(JSCatchOffset<'ctx, 'rt>),
    Exception(JSException<'ctx, 'rt>),
    Float64(JSFloat64<'ctx, 'rt>),

    // -----------------------------
    Array(JSArray<'ctx, 'rt>),
    Object(JSObject<'ctx, 'rt>),
}


impl<'ctx, 'rt> JSAny<'ctx, 'rt> {
    pub(crate) fn with(ctx: &'ctx JSContext<'rt>, v: qjs_c::JSValue) -> JSResult<Self> {
        match v.tag as i32 {
            // first
            qjs_c::JS_TAG_FIRST => Ok(Self::First(JSFirst::with(
                ctx,
                v,
            )?)),

            // big decimal
            // qjs_c::JS_TAG_BIG_DECIMAL => Ok(Self::BigDecimal(JSBigDecimal::with(
            //     ctx,
            //     v,
            // )?)),

            // big int
            qjs_c::JS_TAG_BIG_INT => Ok(Self::BigInt(JSBigInt::with(
                ctx,
                v,
            )?)),

            // big float
            qjs_c::JS_TAG_BIG_FLOAT => Ok(Self::BigFloat(JSBigFloat::with(
                ctx,
                v,
            )?)),


            // symbol
            qjs_c::JS_TAG_SYMBOL => Ok(Self::Symbol(JSSymbol::with(
                ctx,
                v,
            )?)),

            // string
            qjs_c::JS_TAG_STRING => Ok(Self::String(JSStr::with(
                ctx,
                v,
            )?)),


            // module
            qjs_c::JS_TAG_MODULE => Ok(Self::Module(JSModule::with(
                ctx,
                v,
            )?)),


            // func_bytecode
            qjs_c::JS_TAG_FUNCTION_BYTECODE => Ok(Self::FuncBytecode(JSFuncBytecode::with(
                ctx,
                v,
            )?)),

            // int
            qjs_c::JS_TAG_INT => Ok(Self::Int(JSInt::with(
                ctx,
                v,
            )?)),

            // bool
            qjs_c::JS_TAG_BOOL => Ok(Self::Bool(JSBool::with(
                ctx,
                v,
            )?)),


            // null
            qjs_c::JS_TAG_NULL => Ok(Self::Null(JSNull::with(
                ctx,
                v,
            )?)),

            // undefined
            qjs_c::JS_TAG_UNDEFINED => Ok(Self::Undefined(JSUndefined::with(
                ctx,
                v,
            )?)),


            // uninitialized
            qjs_c::JS_TAG_UNINITIALIZED => Ok(Self::Uninitialized(JSUninitialized::with(
                ctx,
                v,
            )?)),


            // catch offset
            qjs_c::JS_TAG_CATCH_OFFSET => Ok(Self::CatchOffset(JSCatchOffset::with(
                ctx,
                v,
            )?)),


            // exception
            qjs_c::JS_TAG_EXCEPTION => Ok(Self::Exception(JSException::with(
                ctx,
                v,
            )?)),


            // float64
            qjs_c::JS_TAG_FLOAT64 => Ok(Self::Float64(JSFloat64::with(
                ctx,
                v,
            )?)),


            // ------------------------------------------------
            qjs_c::JS_TAG_OBJECT => {
                if unsafe{ qjs_c::JS_IsArray(ctx.c_ctx,v) == 1 } {
                    return Ok(Self::Array(JSArray::with(
                        ctx,
                        v
                    )?))
                }


                Ok(Self::Object(JSObject::with(
                    ctx,
                    v,
                )?))
            }

            _ => Err(JSError::UnexpectedType),
        }
    }


    pub(crate) fn wrap(ctx: &'ctx JSContext<'rt>, wrap: JSAnyWrapper) -> JSResult<qjs_c::JSValue> {
        match wrap {
            JSAnyWrapper::Undefined => Ok(qjs_c::JSValue {
                u: qjs_c::JSValueUnion { int32: 0 },
                tag: qjs_c::JS_TAG_UNDEFINED as _,
            }),
            JSAnyWrapper::Null => Ok(qjs_c::JSValue {
                u: qjs_c::JSValueUnion { int32: 0 },
                tag: qjs_c::JS_TAG_NULL as _,
            }),
            JSAnyWrapper::Bool(b) => Ok(qjs_c::JSValue {
                u: qjs_c::JSValueUnion { int32: if b { 1 } else { 0 } },
                tag: qjs_c::JS_TAG_BOOL as _,
            }),
            JSAnyWrapper::Int(v) => Ok(qjs_c::JSValue {
                u: qjs_c::JSValueUnion { int32: v },
                tag: qjs_c::JS_TAG_INT as _,
            }),
            JSAnyWrapper::Float(f) => Ok(qjs_c::JSValue {
                u: qjs_c::JSValueUnion { float64: f },
                tag: qjs_c::JS_TAG_FLOAT64 as _,
            }),
            JSAnyWrapper::String(s) => {
                let v = unsafe {
                    qjs_c::JS_NewStringLen(
                        ctx.c_ctx,
                        s.as_ptr() as *const libc::c_char,
                        s.len(),
                    )
                };
                if v.tag as i32 == qjs_c::JS_TAG_EXCEPTION {
                    return Err(JSError::Internal("could not create string in runtime".into()));
                };

                return Ok(v);
            },

            // ----------------------------------------
            JSAnyWrapper::Array(values) => {
                let lists = unsafe { qjs_c::JS_NewArray(ctx.c_ctx) };
                if lists.tag as i32 == qjs_c::JS_TAG_EXCEPTION {
                    return Err(JSError::Internal("could not create array in runtime".into()));
                }

                for (idx, value) in values.into_iter().enumerate() {
                    // fetch value
                    let v = match JSAny::wrap(ctx, value) {
                        Ok(hit) => hit,
                        Err(e) => {
                            unsafe { qjs_c::JS_FreeValue_Inline(ctx.c_ctx, lists) }
                            return Err(e);
                        }
                    };


                    // search key exists
                    let ret = unsafe {
                        qjs_c::JS_DefinePropertyValueUint32(
                            ctx.c_ctx,
                            lists,
                            idx as u32,
                            v,
                            qjs_c::JS_PROP_C_W_E as _,
                        )
                    };

                    if ret < 0 {
                        unsafe { qjs_c::JS_FreeValue_Inline(ctx.c_ctx, lists) }
                        return Err(JSError::Internal("could not append element to array".into()));
                    }
                }

                return Ok(lists);
            }
        }
    }
}


/// Rust -> JS native type
#[derive(Debug, PartialEq, Clone)]
pub enum JSAnyWrapper {
    Undefined,
    Null,
    Bool(bool),
    Int(i32),
    Float(f64),
    String(String),
    Array(Vec<JSAnyWrapper>),

    // Object(HashMap<String,JSAnyWrapper>)
    //BigInt(i128),
}


// bind native function  -----------------------------------------------------------------------


impl From<bool> for JSAnyWrapper {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl TryFrom<JSAnyWrapper> for bool {
    type Error = JSError;

    fn try_from(value: JSAnyWrapper) -> Result<Self, Self::Error> {
        return if let JSAnyWrapper::Bool(v) = value {
            Ok(v)
        } else {
            Err(JSError::UnexpectedType)
        };
    }
}

// bool end  -----------------------------------------------------------------------------------


impl From<i32> for JSAnyWrapper {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<i16> for JSAnyWrapper {
    fn from(value: i16) -> Self {
        Self::Int(i32::from(value))
    }
}

impl From<i8> for JSAnyWrapper {
    fn from(value: i8) -> Self {
        Self::Int(i32::from(value))
    }
}

impl From<u16> for JSAnyWrapper {
    fn from(value: u16) -> Self {
        Self::Int(i32::from(value))
    }
}

impl From<u8> for JSAnyWrapper {
    fn from(value: u8) -> Self {
        Self::Int(i32::from(value))
    }
}


impl TryFrom<JSAnyWrapper> for i32 {
    type Error = JSError;

    fn try_from(value: JSAnyWrapper) -> Result<Self, Self::Error> {
        return if let JSAnyWrapper::Int(v) = value {
            Ok(v)
        } else {
            Err(JSError::UnexpectedType)
        };
    }
}


// int  end  -----------------------------------------------------------------------------------


impl From<f64> for JSAnyWrapper {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<f32> for JSAnyWrapper {
    fn from(value: f32) -> Self {
        Self::Float(f64::from(value))
    }
}

impl TryFrom<JSAnyWrapper> for f64 {
    type Error = JSError;

    fn try_from(value: JSAnyWrapper) -> Result<Self, Self::Error> {
        return if let JSAnyWrapper::Float(v) = value {
            Ok(v)
        } else {
            Err(JSError::UnexpectedType)
        };
    }
}


// float end  -----------------------------------------------------------------------------------


impl From<String> for JSAnyWrapper {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}


impl From<&str> for JSAnyWrapper {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}


impl TryFrom<JSAnyWrapper> for String {
    type Error = JSError;

    fn try_from(value: JSAnyWrapper) -> Result<Self, Self::Error> {
        return if let JSAnyWrapper::String(v) = value {
            Ok(v)
        } else {
            Err(JSError::UnexpectedType)
        };
    }
}


// string end  -----------------------------------------------------------------------------------


impl<T> From<Option<T>> for JSAnyWrapper
    where T: Into<JSAnyWrapper> {
    fn from(value: Option<T>) -> Self {
        match value {
            None => Self::Null,
            Some(v) => v.into()
        }
    }
}


// null end  -----------------------------------------------------------------------------------


impl<T> From<Vec<T>> for JSAnyWrapper
    where T: Into<JSAnyWrapper> {
    fn from(value: Vec<T>) -> Self {
        let values = value
            .into_iter()
            .map(|v| v.into())
            .collect();
        Self::Array(values)
    }
}


impl<T> TryFrom<JSAnyWrapper> for Vec<T>
    where T: TryFrom<JSAnyWrapper> {
    type Error = JSError;

    fn try_from(value: JSAnyWrapper) -> Result<Self, Self::Error> {
        return if let JSAnyWrapper::Array(v) = value {
            v.into_iter()
                .map(|v| v.try_into().map_err(|_| JSError::UnexpectedType))
                .collect()
        } else {
            Err(JSError::UnexpectedType)
        };
    }
}


// array end  -----------------------------------------------------------------------------------
