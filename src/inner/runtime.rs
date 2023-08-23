use crate::error::{JSError, JSResult};
use crate::qjs_c;

#[derive(Debug)]
pub struct JSRuntime {
    pub(crate) c_runtime: *mut qjs_c::JSRuntime,
}

impl Drop for JSRuntime {
    fn drop(&mut self) {
        unsafe {
            qjs_c::JS_FreeRuntime(self.c_runtime)
        }
    }
}

impl JSRuntime {
    /// JS_NewRuntime
    pub fn new() -> JSResult<Self> {
        let runtime = unsafe { qjs_c::JS_NewRuntime() };
        if runtime.is_null() {
            return Err(JSError::CreateRuntime);
        }
        Ok(Self {
            c_runtime: runtime
        })
    }

    pub fn memory_limit(&self, mem: usize) -> &Self {
        unsafe { qjs_c::JS_SetMemoryLimit(self.c_runtime, mem) };
        self
    }

    /// JS_RunGC
    pub fn gc(&self) -> &Self {
        unsafe { qjs_c::JS_RunGC(self.c_runtime) };
        self
    }

    /// JS_SetMaxStackSize
    pub fn max_stack_size(&self, sz: usize) -> &Self {
        unsafe { qjs_c::JS_SetMaxStackSize(self.c_runtime, sz) };
        self
    }

    /// JS_SetGCThreshold
    pub fn gc_threshold(&self, sz: usize) -> &Self {
        unsafe { qjs_c::JS_SetGCThreshold(self.c_runtime, sz) }
        self
    }

    /// JS_UpdateStackTop
    pub fn update_stack_top(&self) -> &Self {
        unsafe { qjs_c::JS_UpdateStackTop(self.c_runtime) };
        self
    }


    /// JS_DumpMemoryUsage
    pub fn dump_mem(&self) {
        unsafe {
            qjs_c::JS_DupRTMemoryInfo(self.c_runtime)
        }
    }
}

