use std::fmt::{Debug, Formatter};
use crate::{JSContext, JSError, JSResult, qjs};


pub struct JSValueRaw<'a, 'b> {
    ctx: &'a JSContext<'b>,
    pub(crate) c_value: qjs::JSValue,
}


#[derive(Debug)]
pub enum JSValue<'a, 'b> {
    BigDecimal(JSValueRaw<'a, 'b>),
    BigInt(JSValueRaw<'a, 'b>),
    BigFloat(JSValueRaw<'a, 'b>),
    Symbol(JSValueRaw<'a, 'b>),
    String(JSValueRaw<'a, 'b>),
    Module(JSValueRaw<'a, 'b>),
    FuncBytecode(JSValueRaw<'a, 'b>),
    Object(JSValueRaw<'a, 'b>),
    Int(JSValueRaw<'a, 'b>),
    Bool(JSValueRaw<'a, 'b>),
    Null(JSValueRaw<'a, 'b>),
    Undefined(JSValueRaw<'a, 'b>),
    Uninitialized(JSValueRaw<'a, 'b>),
    CatchOffset(JSValueRaw<'a, 'b>),
    Exception(JSValueRaw<'a, 'b>),
    Float64(JSValueRaw<'a, 'b>),
}


impl<'a, 'b> Drop for JSValueRaw<'a, 'b> {
    fn drop(&mut self) {
        unsafe {
            qjs::JS_FreeValue_Inline(self.ctx.c_ctx, self.c_value);
        }
    }
}

impl<'a, 'b> Debug for JSValueRaw<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "JSValue[ Context:[{:?}], Value:[ Tag:{} ] ]",
               self.ctx,
               self.c_value.tag
        )
    }
}


impl<'a, 'b> JSValue<'a, 'b> {
    #[inline]
    pub(crate) fn new(ctx: &'a JSContext<'b>, value: qjs::JSValue) -> JSResult<Self> {
        let tag = value.tag as i32;
        match tag {
            qjs::JS_TAG_BIG_DECIMAL => Ok(Self::BigDecimal(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_BIG_INT => Ok(Self::BigInt(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_BIG_FLOAT => Ok(Self::BigFloat(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_SYMBOL => Ok(Self::Symbol(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_STRING => Ok(Self::String(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_MODULE => Ok(Self::Module(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_FUNCTION_BYTECODE => Ok(Self::FuncBytecode(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_OBJECT => Ok(Self::Object(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_INT => Ok(Self::Int(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_BOOL => Ok(Self::Bool(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_NULL => Ok(Self::Null(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_UNDEFINED => Ok(Self::Undefined(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_UNINITIALIZED => Ok(Self::Uninitialized(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_CATCH_OFFSET => Ok(Self::CatchOffset(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_EXCEPTION => Ok(Self::Exception(JSValueRaw { ctx, c_value: value })),
            qjs::JS_TAG_FLOAT64 => Ok(Self::Float64(JSValueRaw { ctx, c_value: value })),
            _ => Err(JSError::UnexpectedType)
        }
    }
}


impl<'a, 'b> TryFrom<JSValue<'a, 'b>> for i32 {
    type Error = JSError;

    fn try_from(value: JSValue<'a, 'b>) -> Result<Self, Self::Error> {
        match value {
            JSValue::Int(raw) => Ok(unsafe { raw.c_value.u.int32 }),
            _ => Err(JSError::Internal(format!("{:?}", value)))
        }
    }
}

impl<'a, 'b> TryFrom<JSValue<'a, 'b>> for bool {
    type Error = JSError;

    fn try_from(value: JSValue<'a, 'b>) -> Result<Self, Self::Error> {
        match value {
            JSValue::Bool(raw) => Ok(unsafe { raw.c_value.u.int32 > 0 }),
            _ => Err(JSError::Internal(format!("{:?}", value)))
        }
    }
}


impl<'a, 'b> TryFrom<JSValue<'a, 'b>> for f64 {
    type Error = JSError;

    fn try_from(value: JSValue<'a, 'b>) -> Result<Self, Self::Error> {
        match value {
            JSValue::Float64(raw) => Ok(unsafe { raw.c_value.u.float64 }),
            _ => Err(JSError::Internal(format!("{:?}", value)))
        }
    }
}


impl<'a, 'b> TryFrom<JSValue<'a, 'b>> for String {
    type Error = JSError;

    fn try_from(value: JSValue<'a, 'b>) -> Result<Self, Self::Error> {
        match value {
            JSValue::String(raw) => {
                let ptr = unsafe { qjs::JS_ToCStringLen2(raw.ctx.c_ctx, std::ptr::null_mut(), raw.c_value, 0) };
                if ptr.is_null() {
                    return Err(JSError::Internal(
                        "could not convert string: got a null pointer".into()
                    ));
                }
                let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };

                let s = cstr
                    .to_str()
                    .map_err(JSError::InvalidUtf8String)?
                    .to_string();

                unsafe { qjs::JS_FreeCString(raw.ctx.c_ctx, ptr) }
                Ok(s)
            }
            _ => Err(JSError::Internal(format!("{:?}", value)))
        }
    }
}


impl<'a, 'b> TryFrom<JSValue<'a, 'b>> for Option<()> {
    type Error = JSError;

    fn try_from(value: JSValue<'a, 'b>) -> Result<Self, Self::Error> {
        match value {
            JSValue::Null(_raw) => Ok(None),
            _ => Err(JSError::Internal(format!("{:?}", value)))
        }
    }
}
