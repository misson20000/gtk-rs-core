// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    Object,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

crate::wrapper! {
    #[doc(alias = "GBindingGroup")]
    pub struct BindingGroup(Object<gobject_ffi::GBindingGroup>);

    match fn {
        type_ => || gobject_ffi::g_binding_group_get_type(),
    }
}

impl BindingGroup {
    #[doc(alias = "g_binding_group_new")]
    pub fn new() -> BindingGroup {
        unsafe { from_glib_full(gobject_ffi::g_binding_group_new()) }
    }

    #[doc(alias = "g_binding_group_dup_source")]
    #[doc(alias = "dup_source")]
    pub fn source(&self) -> Option<Object> {
        unsafe {
            from_glib_none(gobject_ffi::g_binding_group_dup_source(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_binding_group_set_source")]
    pub fn set_source(&self, source: Option<&impl IsA<Object>>) {
        unsafe {
            gobject_ffi::g_binding_group_set_source(
                self.to_glib_none().0,
                source.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v2_72")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
    #[doc(alias = "source")]
    pub fn connect_source_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_source_trampoline<
            F: Fn(&BindingGroup) + Send + Sync + 'static,
        >(
            this: *mut crate::gobject_ffi::GBindingGroup,
            _param_spec: ffi::gpointer,
            f: ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::source\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_source_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
impl Default for BindingGroup {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for BindingGroup {}
unsafe impl Sync for BindingGroup {}

impl fmt::Display for BindingGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BindingGroup")
    }
}
