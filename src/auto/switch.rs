// This file was generated by gir (a3f05e3) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Widget;
use ffi;
use ffi::GtkSwitch;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_14")]
use glib_ffi::gboolean;
use glib_ffi::gpointer;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Switch(Object<ffi::GtkSwitch>): Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_switch_get_type(),
    }
}

impl Switch {
    pub fn new() -> Switch {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_switch_new()).downcast_unchecked()
        }
    }

    pub fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_switch_get_active(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn get_state(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_switch_get_state(self.to_glib_none().0))
        }
    }

    pub fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_switch_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn set_state(&self, state: bool) {
        unsafe {
            ffi::gtk_switch_set_state(self.to_glib_none().0, state.to_glib());
        }
    }

    pub fn connect_activate<F: Fn(&Switch) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Switch) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn connect_state_set<F: Fn(&Switch, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Switch, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "state-set",
                transmute(state_set_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline(this: *mut GtkSwitch, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Switch) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_14")]
unsafe extern "C" fn state_set_trampoline(this: *mut GtkSwitch, state: gboolean, f: gpointer) -> gboolean {
    callback_guard!();
    let f: &Box_<Fn(&Switch, bool) -> bool + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(state)).to_glib()
}
