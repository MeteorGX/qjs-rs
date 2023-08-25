use std::ffi::CString;
use crate::JSError;


#[allow(dead_code)]
pub(crate) fn make_cstring(data: impl Into<Vec<u8>>) -> Result<CString, JSError> {
    return CString::new(data).map_err(JSError::CStringError);
}