use crate::{JSError, JSResult, JSRuntime, qjs};
use crate::inner::value::JSValue;
use crate::utils::make_cstring;

#[derive(Debug)]
#[allow(dead_code)]
pub struct JSContext<'a> {
    runtime: &'a JSRuntime,
    pub(crate) c_ctx: *mut qjs::JSContext,
}


impl<'a> Drop for JSContext<'a> {
    fn drop(&mut self) {
        unsafe {
            qjs::JS_FreeContext(self.c_ctx)
        }
    }
}


impl<'a> JSContext<'a> {
    pub(crate) const DEFAULT_SCRIPT_NAME: &'static str = "script.js";


    pub fn new(rt: &'a JSRuntime) -> JSResult<Self> {
        let ctx = unsafe { qjs::JS_NewContext(rt.c_runtime) };
        if ctx.is_null() {
            return Err(JSError::CreateContext);
        }
        Ok(Self {
            runtime: rt,
            c_ctx: ctx,
        })
    }


    pub(crate) fn qjs_eval(&self, script: &str, flag: i32, code: &str) -> JSResult<JSValue> {
        let filename_c = make_cstring(script)?;
        let code_c = make_cstring(code)?;
        let value = unsafe {
            qjs::JS_Eval(
                self.c_ctx,
                code_c.as_ptr(),
                code.len() as _,
                filename_c.as_ptr(),
                flag,
            )
        };

        JSValue::new(&self, value)
    }


    pub fn eval(&self, code: &str) -> JSResult<JSValue> {
        Ok(self.qjs_eval(Self::DEFAULT_SCRIPT_NAME, qjs::JS_EVAL_TYPE_GLOBAL as i32, code)?)
    }
}


