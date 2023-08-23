use crate::error::{JSError, JSResult};
use crate::qjs;

#[derive(Debug)]
pub struct JSRuntime {
    pub(crate) c_runtime: *mut qjs::JSRuntime,
}

impl Drop for JSRuntime {
    fn drop(&mut self) {
        unsafe {
            qjs::JS_FreeRuntime(self.c_runtime)
        }
    }
}

impl JSRuntime {
    pub fn new() -> JSResult<Self> {
        let runtime = unsafe { qjs::JS_NewRuntime() };
        if runtime.is_null() {
            return Err(JSError::CreateRuntime);
        }
        Ok(Self {
            c_runtime: runtime
        })
    }

    pub fn gc(&self) {
        unsafe { qjs::JS_RunGC(self.c_runtime) }
    }

    pub fn max_stack_size(&self, sz: usize) {
        unsafe { qjs::JS_SetMaxStackSize(self.c_runtime, sz) }
    }

    pub fn gc_threshold(&self, sz: usize) {
        unsafe { qjs::JS_SetGCThreshold(self.c_runtime, sz) }
    }

    pub fn update_stack_top(&self) {
        unsafe { qjs::JS_UpdateStackTop(self.c_runtime) }
    }
}

