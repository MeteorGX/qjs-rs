use crate::{JSContext, JSError, JSResult, qjs_c};
use crate::types::JSRawType;

#[derive(Debug)]
pub struct JSStr<'ctx, 'rt> {
    ctx: &'ctx JSContext<'rt>,
    pub(crate) c_value: qjs_c::JSValue,
}


impl<'ctx, 'rt> Drop for JSStr<'ctx, 'rt> {
    fn drop(&mut self) {
        unsafe { qjs_c::JS_FreeValue_Inline(self.ctx.c_ctx, self.c_value) }
    }
}


impl<'ctx, 'rt> JSRawType for JSStr<'ctx, 'rt> {
    fn tag(&self) -> i32 {
        return self.c_value.tag as _;
    }
}

#[allow(dead_code)]
impl<'ctx, 'rt> JSStr<'ctx, 'rt> {
    pub(crate) fn with(ctx: &'ctx JSContext<'rt>, c_value: qjs_c::JSValue) -> JSResult<Self> {
        return if c_value.tag as i32 != qjs_c::JS_TAG_STRING {
            Err(JSError::UnexpectedType)
        } else {
            Ok(Self {
                ctx,
                c_value,
            })
        };
    }


    pub fn to_string(&self) -> JSResult<String> {
        let ptr = unsafe { qjs_c::JS_ToCStringLen2(self.ctx.c_ctx, std::ptr::null_mut(), self.c_value, 0) };
        if ptr.is_null() {
            return Err(JSError::Internal("could not convert string: got a null point".into()));
        }

        let str_c = unsafe { std::ffi::CStr::from_ptr(ptr) };
        let str = str_c
            .to_str()
            .map_err(JSError::InvalidUtf8String)?
            .to_string();

        unsafe { qjs_c::JS_FreeCString(self.ctx.c_ctx, ptr) }
        Ok(str)
    }
}


impl<'ctx, 'rt> From<JSStr<'ctx, 'rt>> for String {
    fn from(_value: JSStr<'ctx, 'rt>) -> Self { todo!() }
}



