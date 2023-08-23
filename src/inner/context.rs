use crate::{JSError, JSResult, JSRuntime, qjs_c};

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
    pub fn dump_mem(&self){
        self.runtime.dump_mem();
    }
}


