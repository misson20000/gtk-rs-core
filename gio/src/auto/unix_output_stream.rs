// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{FileDescriptorBased, OutputStream, PollableOutputStream};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GUnixOutputStream")]
    pub struct UnixOutputStream(Object<ffi::GUnixOutputStream, ffi::GUnixOutputStreamClass>) @extends OutputStream, @implements FileDescriptorBased, PollableOutputStream;

    match fn {
        type_ => || ffi::g_unix_output_stream_get_type(),
    }
}

impl UnixOutputStream {
    pub const NONE: Option<&'static UnixOutputStream> = None;
}

pub trait UnixOutputStreamExt: IsA<UnixOutputStream> + 'static {
    #[doc(alias = "g_unix_output_stream_get_close_fd")]
    #[doc(alias = "get_close_fd")]
    fn closes_fd(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_output_stream_get_close_fd(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<UnixOutputStream>> UnixOutputStreamExt for O {}

impl fmt::Display for UnixOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UnixOutputStream")
    }
}
