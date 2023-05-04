// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Emblem, Icon};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GEmblemedIcon")]
    pub struct EmblemedIcon(Object<ffi::GEmblemedIcon, ffi::GEmblemedIconClass>) @implements Icon;

    match fn {
        type_ => || ffi::g_emblemed_icon_get_type(),
    }
}

impl EmblemedIcon {
    pub const NONE: Option<&'static EmblemedIcon> = None;

    #[doc(alias = "g_emblemed_icon_new")]
    pub fn new(icon: &impl IsA<Icon>, emblem: Option<&Emblem>) -> EmblemedIcon {
        unsafe {
            from_glib_full(ffi::g_emblemed_icon_new(
                icon.as_ref().to_glib_none().0,
                emblem.to_glib_none().0,
            ))
        }
    }
}

pub trait EmblemedIconExt: IsA<EmblemedIcon> + 'static {
    #[doc(alias = "g_emblemed_icon_add_emblem")]
    fn add_emblem(&self, emblem: &Emblem) {
        unsafe {
            ffi::g_emblemed_icon_add_emblem(
                self.as_ref().to_glib_none().0,
                emblem.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_emblemed_icon_clear_emblems")]
    fn clear_emblems(&self) {
        unsafe {
            ffi::g_emblemed_icon_clear_emblems(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "g_emblemed_icon_get_emblems")]
    #[doc(alias = "get_emblems")]
    fn emblems(&self) -> Vec<Emblem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_emblemed_icon_get_emblems(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_emblemed_icon_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Icon {
        unsafe {
            from_glib_none(ffi::g_emblemed_icon_get_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn gicon(&self) -> Option<Icon> {
        glib::ObjectExt::property(self.as_ref(), "gicon")
    }
}

impl<O: IsA<EmblemedIcon>> EmblemedIconExt for O {}

impl fmt::Display for EmblemedIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EmblemedIcon")
    }
}
