// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::ContextMenuItem;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitContextMenu")]
    pub struct ContextMenu(Object<ffi::WebKitContextMenu, ffi::WebKitContextMenuClass>);

    match fn {
        type_ => || ffi::webkit_context_menu_get_type(),
    }
}

impl ContextMenu {
    pub const NONE: Option<&'static ContextMenu> = None;

    #[doc(alias = "webkit_context_menu_new")]
    pub fn new() -> ContextMenu {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::webkit_context_menu_new()) }
    }

    #[doc(alias = "webkit_context_menu_new_with_items")]
    #[doc(alias = "new_with_items")]
    pub fn with_items(items: &[ContextMenuItem]) -> ContextMenu {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_context_menu_new_with_items(
                items.to_glib_none().0,
            ))
        }
    }
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ContextMenuExt: 'static {
    #[doc(alias = "webkit_context_menu_append")]
    fn append(&self, item: &impl IsA<ContextMenuItem>);

    #[doc(alias = "webkit_context_menu_first")]
    fn first(&self) -> Option<ContextMenuItem>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
    #[doc(alias = "webkit_context_menu_get_event")]
    #[doc(alias = "get_event")]
    fn event(&self) -> Option<gdk::Event>;

    #[doc(alias = "webkit_context_menu_get_item_at_position")]
    #[doc(alias = "get_item_at_position")]
    fn item_at_position(&self, position: u32) -> Option<ContextMenuItem>;

    #[doc(alias = "webkit_context_menu_get_items")]
    #[doc(alias = "get_items")]
    fn items(&self) -> Vec<ContextMenuItem>;

    #[doc(alias = "webkit_context_menu_get_n_items")]
    #[doc(alias = "get_n_items")]
    fn n_items(&self) -> u32;

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_context_menu_get_user_data")]
    #[doc(alias = "get_user_data")]
    fn user_data(&self) -> Option<glib::Variant>;

    #[doc(alias = "webkit_context_menu_insert")]
    fn insert(&self, item: &impl IsA<ContextMenuItem>, position: i32);

    #[doc(alias = "webkit_context_menu_last")]
    fn last(&self) -> Option<ContextMenuItem>;

    #[doc(alias = "webkit_context_menu_move_item")]
    fn move_item(&self, item: &impl IsA<ContextMenuItem>, position: i32);

    #[doc(alias = "webkit_context_menu_prepend")]
    fn prepend(&self, item: &impl IsA<ContextMenuItem>);

    #[doc(alias = "webkit_context_menu_remove")]
    fn remove(&self, item: &impl IsA<ContextMenuItem>);

    #[doc(alias = "webkit_context_menu_remove_all")]
    fn remove_all(&self);

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    #[doc(alias = "webkit_context_menu_set_user_data")]
    fn set_user_data(&self, user_data: &glib::Variant);
}

impl<O: IsA<ContextMenu>> ContextMenuExt for O {
    fn append(&self, item: &impl IsA<ContextMenuItem>) {
        unsafe {
            ffi::webkit_context_menu_append(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
            );
        }
    }

    fn first(&self) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_first(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_40")))]
    fn event(&self) -> Option<gdk::Event> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_get_event(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn item_at_position(&self, position: u32) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_get_item_at_position(
                self.as_ref().to_glib_none().0,
                position,
            ))
        }
    }

    fn items(&self) -> Vec<ContextMenuItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_context_menu_get_items(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn n_items(&self) -> u32 {
        unsafe { ffi::webkit_context_menu_get_n_items(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn user_data(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_get_user_data(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert(&self, item: &impl IsA<ContextMenuItem>, position: i32) {
        unsafe {
            ffi::webkit_context_menu_insert(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    fn last(&self) -> Option<ContextMenuItem> {
        unsafe {
            from_glib_none(ffi::webkit_context_menu_last(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn move_item(&self, item: &impl IsA<ContextMenuItem>, position: i32) {
        unsafe {
            ffi::webkit_context_menu_move_item(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    fn prepend(&self, item: &impl IsA<ContextMenuItem>) {
        unsafe {
            ffi::webkit_context_menu_prepend(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
            );
        }
    }

    fn remove(&self, item: &impl IsA<ContextMenuItem>) {
        unsafe {
            ffi::webkit_context_menu_remove(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
            );
        }
    }

    fn remove_all(&self) {
        unsafe {
            ffi::webkit_context_menu_remove_all(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_8")))]
    fn set_user_data(&self, user_data: &glib::Variant) {
        unsafe {
            ffi::webkit_context_menu_set_user_data(
                self.as_ref().to_glib_none().0,
                user_data.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for ContextMenu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ContextMenu")
    }
}
