use std::ffi::CString;
use crate::JSError;


#[allow(dead_code)]
pub(crate) fn make_cstring(data: &str) -> Result<CString, JSError> {
    return CString::new(data).map_err(JSError::CStringError);
}