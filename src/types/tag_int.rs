use crate::{JSContext, JSError, JSResult, qjs_c};
use crate::types::JSRawType;

#[derive(Debug)]
pub struct JSInt<'ctx, 'rt> {
    ctx: &'ctx JSContext<'rt>,
    pub(crate) c_value: qjs_c::JSValue,
}


impl<'ctx, 'rt> Drop for JSInt<'ctx, 'rt> {
    fn drop(&mut self) {
        unsafe { qjs_c::JS_FreeValue_Inline(self.ctx.c_ctx, self.c_value) }
    }
}


impl<'ctx, 'rt> JSRawType for JSInt<'ctx, 'rt> {
    fn tag(&self) -> i32 {
        return self.c_value.tag as _;
    }
}

#[allow(dead_code)]
impl<'ctx, 'rt> JSInt<'ctx, 'rt> {
    pub(crate) fn with(ctx: &'ctx JSContext<'rt>, c_value: qjs_c::JSValue) -> JSResult<Self> {
        return if c_value.tag as i32 != qjs_c::JS_TAG_INT {
            Err(JSError::UnexpectedType)
        } else {
            Ok(Self {
                ctx,
                c_value,
            })
        };
    }
}


impl<'ctx, 'rt> From<JSInt<'ctx, 'rt>> for i32 {
    fn from(value: JSInt<'ctx, 'rt>) -> Self {
        unsafe { value.c_value.u.int32 }
    }
}




