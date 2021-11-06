// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMEvent;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "WebKitDOMEventTarget")]
    pub struct DOMEventTarget(Interface<ffi::WebKitDOMEventTarget, ffi::WebKitDOMEventTargetIface>);

    match fn {
        type_ => || ffi::webkit_dom_event_target_get_type(),
    }
}

pub const NONE_DOM_EVENT_TARGET: Option<&DOMEventTarget> = None;

pub trait DOMEventTargetExt: 'static {
    //#[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    //#[doc(alias = "webkit_dom_event_target_add_event_listener")]
    //fn add_event_listener<P: FnOnce() + 'static>(&self, event_name: &str, handler: P, use_capture: bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_event_target_add_event_listener_with_closure")]
    fn add_event_listener_with_closure(
        &self,
        event_name: &str,
        handler: &glib::Closure,
        use_capture: bool,
    ) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_event_target_dispatch_event")]
    fn dispatch_event(&self, event: &impl IsA<DOMEvent>) -> Result<(), glib::Error>;

    //#[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    //#[doc(alias = "webkit_dom_event_target_remove_event_listener")]
    //fn remove_event_listener<P: FnMut()>(&self, event_name: &str, handler: P, use_capture: bool) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_event_target_remove_event_listener_with_closure")]
    fn remove_event_listener_with_closure(
        &self,
        event_name: &str,
        handler: &glib::Closure,
        use_capture: bool,
    ) -> bool;
}

impl<O: IsA<DOMEventTarget>> DOMEventTargetExt for O {
    //fn add_event_listener<P: FnOnce() + 'static>(&self, event_name: &str, handler: P, use_capture: bool, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:webkit_dom_event_target_add_event_listener() }
    //}

    fn add_event_listener_with_closure(
        &self,
        event_name: &str,
        handler: &glib::Closure,
        use_capture: bool,
    ) -> bool {
        unsafe {
            from_glib(
                ffi::webkit_dom_event_target_add_event_listener_with_closure(
                    self.as_ref().to_glib_none().0,
                    event_name.to_glib_none().0,
                    handler.to_glib_none().0,
                    use_capture.into_glib(),
                ),
            )
        }
    }

    fn dispatch_event(&self, event: &impl IsA<DOMEvent>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_event_target_dispatch_event(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //fn remove_event_listener<P: FnMut()>(&self, event_name: &str, handler: P, use_capture: bool) -> bool {
    //    unsafe { TODO: call ffi:webkit_dom_event_target_remove_event_listener() }
    //}

    fn remove_event_listener_with_closure(
        &self,
        event_name: &str,
        handler: &glib::Closure,
        use_capture: bool,
    ) -> bool {
        unsafe {
            from_glib(
                ffi::webkit_dom_event_target_remove_event_listener_with_closure(
                    self.as_ref().to_glib_none().0,
                    event_name.to_glib_none().0,
                    handler.to_glib_none().0,
                    use_capture.into_glib(),
                ),
            )
        }
    }
}

impl fmt::Display for DOMEventTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMEventTarget")
    }
}
