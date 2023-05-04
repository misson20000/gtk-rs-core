// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{AsyncResult, Cancellable, FileInfo, InputStream, Seekable};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, fmt, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GFileInputStream")]
    pub struct FileInputStream(Object<ffi::GFileInputStream, ffi::GFileInputStreamClass>) @extends InputStream, @implements Seekable;

    match fn {
        type_ => || ffi::g_file_input_stream_get_type(),
    }
}

impl FileInputStream {
    pub const NONE: Option<&'static FileInputStream> = None;
}

pub trait FileInputStreamExt: IsA<FileInputStream> + 'static {
    #[doc(alias = "g_file_input_stream_query_info")]
    fn query_info(
        &self,
        attributes: &str,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<FileInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_input_stream_query_info(
                self.as_ref().to_glib_none().0,
                attributes.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_file_input_stream_query_info_async")]
    fn query_info_async<P: FnOnce(Result<FileInfo, glib::Error>) + 'static>(
        &self,
        attributes: &str,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn query_info_async_trampoline<
            P: FnOnce(Result<FileInfo, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_file_input_stream_query_info_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = query_info_async_trampoline::<P>;
        unsafe {
            ffi::g_file_input_stream_query_info_async(
                self.as_ref().to_glib_none().0,
                attributes.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn query_info_future(
        &self,
        attributes: &str,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<FileInfo, glib::Error>> + 'static>> {
        let attributes = String::from(attributes);
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.query_info_async(&attributes, io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }
}

impl<O: IsA<FileInputStream>> FileInputStreamExt for O {}

impl fmt::Display for FileInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileInputStream")
    }
}
