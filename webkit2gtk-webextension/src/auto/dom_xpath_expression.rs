// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMNode;
use crate::DOMObject;
use crate::DOMXPathResult;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitDOMXPathExpression")]
    pub struct DOMXPathExpression(Object<ffi::WebKitDOMXPathExpression, ffi::WebKitDOMXPathExpressionClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_xpath_expression_get_type(),
    }
}

pub const NONE_DOMX_PATH_EXPRESSION: Option<&DOMXPathExpression> = None;

pub trait DOMXPathExpressionExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_xpath_expression_evaluate")]
    fn evaluate(
        &self,
        contextNode: &impl IsA<DOMNode>,
        type_: libc::c_ushort,
        inResult: &impl IsA<DOMXPathResult>,
    ) -> Result<DOMXPathResult, glib::Error>;
}

impl<O: IsA<DOMXPathExpression>> DOMXPathExpressionExt for O {
    fn evaluate(
        &self,
        contextNode: &impl IsA<DOMNode>,
        type_: libc::c_ushort,
        inResult: &impl IsA<DOMXPathResult>,
    ) -> Result<DOMXPathResult, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_xpath_expression_evaluate(
                self.as_ref().to_glib_none().0,
                contextNode.as_ref().to_glib_none().0,
                type_,
                inResult.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl fmt::Display for DOMXPathExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMXPathExpression")
    }
}
