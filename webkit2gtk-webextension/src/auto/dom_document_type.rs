// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMEventTarget;
use crate::DOMNamedNodeMap;
use crate::DOMNode;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitDOMDocumentType")]
    pub struct DOMDocumentType(Object<ffi::WebKitDOMDocumentType, ffi::WebKitDOMDocumentTypeClass>) @extends DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        type_ => || ffi::webkit_dom_document_type_get_type(),
    }
}

pub const NONE_DOM_DOCUMENT_TYPE: Option<&DOMDocumentType> = None;

pub trait DOMDocumentTypeExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_document_type_get_entities")]
    #[doc(alias = "get_entities")]
    fn entities(&self) -> Option<DOMNamedNodeMap>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_document_type_get_internal_subset")]
    #[doc(alias = "get_internal_subset")]
    fn internal_subset(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_document_type_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_document_type_get_notations")]
    #[doc(alias = "get_notations")]
    fn notations(&self) -> Option<DOMNamedNodeMap>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_document_type_get_public_id")]
    #[doc(alias = "get_public_id")]
    fn public_id(&self) -> Option<glib::GString>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_document_type_get_system_id")]
    #[doc(alias = "get_system_id")]
    fn system_id(&self) -> Option<glib::GString>;

    #[doc(alias = "entities")]
    fn connect_entities_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "internal-subset")]
    fn connect_internal_subset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "notations")]
    fn connect_notations_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "public-id")]
    fn connect_public_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "system-id")]
    fn connect_system_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMDocumentType>> DOMDocumentTypeExt for O {
    fn entities(&self) -> Option<DOMNamedNodeMap> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_entities(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn internal_subset(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_internal_subset(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn notations(&self) -> Option<DOMNamedNodeMap> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_notations(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn public_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_public_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn system_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_system_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_entities_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_entities_trampoline<
            P: IsA<DOMDocumentType>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDocumentType,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDocumentType::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::entities\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_entities_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_internal_subset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_internal_subset_trampoline<
            P: IsA<DOMDocumentType>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDocumentType,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDocumentType::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::internal-subset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_internal_subset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<
            P: IsA<DOMDocumentType>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDocumentType,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDocumentType::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_notations_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_notations_trampoline<
            P: IsA<DOMDocumentType>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDocumentType,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDocumentType::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::notations\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_notations_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_public_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_public_id_trampoline<
            P: IsA<DOMDocumentType>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDocumentType,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDocumentType::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::public-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_public_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_system_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_system_id_trampoline<
            P: IsA<DOMDocumentType>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMDocumentType,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMDocumentType::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::system-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_system_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMDocumentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMDocumentType")
    }
}
