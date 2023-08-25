use crate::{JSContext, JSError, JSResult, qjs_c};
use crate::types::JSRawType;
use crate::utils::make_cstring;

#[derive(Debug)]
pub struct JSObject<'ctx, 'rt> {
    ctx: &'ctx JSContext<'rt>,
    pub(crate) c_value: qjs_c::JSValue,
}


impl<'ctx, 'rt> Drop for JSObject<'ctx, 'rt> {
    fn drop(&mut self) {
        unsafe { qjs_c::JS_FreeValue_Inline(self.ctx.c_ctx, self.c_value) }
    }
}


impl<'ctx, 'rt> JSRawType for JSObject<'ctx, 'rt> {
    fn tag(&self) -> i32 {
        return self.c_value.tag as _;
    }
}

#[allow(dead_code)]
impl<'ctx, 'rt> JSObject<'ctx, 'rt> {
    pub(crate) fn with(ctx: &'ctx JSContext<'rt>, c_value: qjs_c::JSValue) -> JSResult<Self> {
        return if c_value.tag as i32 != qjs_c::JS_TAG_OBJECT {
            Err(JSError::UnexpectedType)
        } else {
            Ok(Self {
                ctx,
                c_value,
            })
        };
    }


    /// JS_SetPropertyStr
    #[allow(forgetting_copy_types)]
    pub(crate) fn set_property(&self, name: &str, value: qjs_c::JSValue) -> JSResult<()> {
        let name_c = make_cstring(name)?;
        unsafe {
            let ret = qjs_c::JS_SetPropertyStr(
                self.ctx.c_ctx,
                self.c_value,
                name_c.as_ptr(),
                value,
            );
            if ret < 0 {
                return Err(JSError::JSException("could not set property".into()));
            } else {
                // double free
                let _ = std::mem::forget(value);
                Ok(())
            }
        }
    }


    pub fn is_array(&self) -> bool {
        return unsafe { qjs_c::JS_IsArray(self.ctx.c_ctx, self.c_value) == 1 };
    }
}



