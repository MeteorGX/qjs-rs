mod runtime;
mod context;

pub use runtime::JSRuntime;
pub use context::JSContext;
use crate::qjs_c;


#[derive(Debug)]
pub enum JSEvalType {
    Global,
    Module,
    Direct,
    Indirect,
    Mask,
}


impl From<JSEvalType> for u32 {
    fn from(value: JSEvalType) -> Self {
        match value {
            JSEvalType::Global => qjs_c::JS_EVAL_TYPE_GLOBAL,
            JSEvalType::Module => qjs_c::JS_EVAL_TYPE_MODULE,
            JSEvalType::Direct => qjs_c::JS_EVAL_TYPE_DIRECT,
            JSEvalType::Indirect => qjs_c::JS_EVAL_TYPE_INDIRECT,
            JSEvalType::Mask => qjs_c::JS_EVAL_TYPE_MASK,
        }
    }
}


impl From<JSEvalType> for i32 {
    fn from(value: JSEvalType) -> Self {
        let flag: u32 = value.into();
        return flag as _;
    }
}

#[derive(Debug)]
pub enum JSEvalFlag {
    Strict,
    Strip,
    CompileOnly,
    BacktraceBarrier,
}


impl From<JSEvalFlag> for u32 {
    fn from(value: JSEvalFlag) -> Self {
        match value {
            JSEvalFlag::Strict => qjs_c::JS_EVAL_FLAG_STRICT,
            JSEvalFlag::Strip => qjs_c::JS_EVAL_FLAG_STRIP,
            JSEvalFlag::CompileOnly => qjs_c::JS_EVAL_FLAG_COMPILE_ONLY,
            JSEvalFlag::BacktraceBarrier => qjs_c::JS_EVAL_FLAG_BACKTRACE_BARRIER,
        }
    }
}

impl From<JSEvalFlag> for i32 {
    fn from(value: JSEvalFlag) -> Self {
        let flag: u32 = value.into();
        return flag as _;
    }
}