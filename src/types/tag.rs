use crate::qjs_c;


/// trait js type
pub trait JSRawType {
    /// JS tag value
    fn tag(&self) -> i32;


    /// JS_TAG_FIRST
    #[inline]
    fn is_first(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_FIRST;
    }

    /// JS_TAG_BIG_DECIMAL
    #[inline]
    fn is_big_decimal(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_BIG_DECIMAL;
    }

    /// JS_TAG_BIG_INT
    #[inline]
    fn is_big_int(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_BIG_INT;
    }


    /// JS_TAG_BIG_FLOAT
    #[inline]
    fn is_big_float(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_BIG_FLOAT;
    }


    /// JS_TAG_SYMBOL
    #[inline]
    fn is_symbol(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_SYMBOL;
    }


    /// JS_TAG_STRING
    #[inline]
    fn is_string(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_STRING;
    }


    /// JS_TAG_MODULE
    #[inline]
    fn is_module(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_MODULE;
    }


    /// JS_TAG_FUNCTION_BYTECODE
    #[inline]
    fn is_func_bytecode(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_FUNCTION_BYTECODE;
    }

    /// JS_TAG_OBJECT
    #[inline]
    fn is_object(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_OBJECT;
    }


    /// JS_TAG_INT
    #[inline]
    fn is_int(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_INT;
    }


    /// JS_TAG_BOOL
    #[inline]
    fn is_bool(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_BOOL;
    }


    /// JS_TAG_NULL
    #[inline]
    fn is_null(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_NULL;
    }


    /// JS_TAG_UNDEFINED
    #[inline]
    fn is_undefined(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_UNDEFINED;
    }


    /// JS_TAG_UNINITIALIZED
    #[inline]
    fn is_uninitialized(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_UNINITIALIZED;
    }

    /// JS_TAG_CATCH_OFFSET
    #[inline]
    fn is_catch_offset(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_CATCH_OFFSET;
    }


    /// JS_TAG_EXCEPTION
    #[inline]
    fn is_exception(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_EXCEPTION;
    }


    /// JS_TAG_FLOAT64
    #[inline]
    fn is_float64(&self) -> bool {
        return self.tag() == qjs_c::JS_TAG_FLOAT64;
    }
}
