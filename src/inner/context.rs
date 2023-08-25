use crate::{JSAnyWrapper, JSError, JSObject, JSResult, JSRuntime, qjs_c};
use crate::qjs_c::{JS_EVAL_TYPE_GLOBAL};
use crate::types::{JSAny};
use crate::utils::make_cstring;

#[derive(Debug)]
#[allow(dead_code)]
pub struct JSContext<'rt> {
    runtime: &'rt JSRuntime,
    pub(crate) c_ctx: *mut qjs_c::JSContext,
}


impl<'rt> Drop for JSContext<'rt> {
    fn drop(&mut self) {
        unsafe {
            qjs_c::JS_FreeContext(self.c_ctx)
        }
    }
}


impl<'rt> JSContext<'rt> {
    pub fn new(rt: &'rt JSRuntime) -> JSResult<Self> {
        let ctx = unsafe { qjs_c::JS_NewContext(rt.c_runtime) };
        if ctx.is_null() {
            return Err(JSError::CreateContext);
        }
        Ok(Self {
            runtime: rt,
            c_ctx: ctx,
        })
    }


    /// JS_DumpMemoryUsage
    pub fn dump_mem(&self) {
        self.runtime.dump_mem();
    }


    /// JS_Eval
    pub fn eval(&self, filename: &str, code: &str) -> JSResult<JSAny<'_, 'rt>> {
        let filename_c = make_cstring(filename)?;
        let code_c = make_cstring(code)?;
        let raw = unsafe {
            qjs_c::JS_Eval(
                self.c_ctx,
                code_c.as_ptr(),
                code.len(),
                filename_c.as_ptr(),
                JS_EVAL_TYPE_GLOBAL as _,
            )
        };
        Ok(JSAny::with(self, raw)?)
    }


    /// JS_GetGlobalObject
    pub fn global(&self) -> JSResult<JSObject> {
        let global_raw = unsafe { qjs_c::JS_GetGlobalObject(self.c_ctx) };
        match JSAny::with(self, global_raw)? {
            JSAny::Object(global_ref) => Ok(global_ref),
            _ => Err(JSError::UnexpectedType)
        }
    }


    /// set global JS_SetPropertyStr
    pub fn global_val<V>(&self, name: &str, value: V) -> JSResult<()>
        where V: Into<JSAnyWrapper> {
        let raw = JSAny::wrap(self, value.into())?;
        let global = self.global()?;
        global.set_property(name, raw)?;
        Ok(())
    }
}


