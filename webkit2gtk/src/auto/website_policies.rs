// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
use crate::AutoplayPolicy;
use glib::object::IsA;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
use glib::translate::*;
#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
use glib::StaticType;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitWebsitePolicies")]
    pub struct WebsitePolicies(Object<ffi::WebKitWebsitePolicies, ffi::WebKitWebsitePoliciesClass>);

    match fn {
        type_ => || ffi::webkit_website_policies_get_type(),
    }
}

impl WebsitePolicies {
    pub const NONE: Option<&'static WebsitePolicies> = None;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_policies_new")]
    pub fn new() -> WebsitePolicies {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::webkit_website_policies_new()) }
    }

    //#[cfg(any(feature = "v2_30", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    //#[doc(alias = "webkit_website_policies_new_with_policies")]
    //#[doc(alias = "new_with_policies")]
    //pub fn with_policies(first_policy_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> WebsitePolicies {
    //    unsafe { TODO: call ffi:webkit_website_policies_new_with_policies() }
    //}
}

#[cfg(any(feature = "v2_30", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
impl Default for WebsitePolicies {
    fn default() -> Self {
        Self::new()
    }
}

pub trait WebsitePoliciesExt: 'static {
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    #[doc(alias = "webkit_website_policies_get_autoplay_policy")]
    #[doc(alias = "get_autoplay_policy")]
    fn autoplay_policy(&self) -> AutoplayPolicy;

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn autoplay(&self) -> AutoplayPolicy;
}

impl<O: IsA<WebsitePolicies>> WebsitePoliciesExt for O {
    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn autoplay_policy(&self) -> AutoplayPolicy {
        unsafe {
            from_glib(ffi::webkit_website_policies_get_autoplay_policy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_30", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_30")))]
    fn autoplay(&self) -> AutoplayPolicy {
        glib::ObjectExt::property(self.as_ref(), "autoplay")
    }
}

impl fmt::Display for WebsitePolicies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WebsitePolicies")
    }
}
