// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{AsyncResult, Cancellable, IOStream, Socket, SocketAddress, SocketFamily, SocketType};
use glib::{prelude::*, translate::*};
use std::{boxed::Box as Box_, fmt, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GSocketConnection")]
    pub struct SocketConnection(Object<ffi::GSocketConnection, ffi::GSocketConnectionClass>) @extends IOStream;

    match fn {
        type_ => || ffi::g_socket_connection_get_type(),
    }
}

impl SocketConnection {
    pub const NONE: Option<&'static SocketConnection> = None;

    #[doc(alias = "g_socket_connection_factory_lookup_type")]
    pub fn factory_lookup_type(
        family: SocketFamily,
        type_: SocketType,
        protocol_id: i32,
    ) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_socket_connection_factory_lookup_type(
                family.into_glib(),
                type_.into_glib(),
                protocol_id,
            ))
        }
    }

    #[doc(alias = "g_socket_connection_factory_register_type")]
    pub fn factory_register_type(
        g_type: glib::types::Type,
        family: SocketFamily,
        type_: SocketType,
        protocol: i32,
    ) {
        unsafe {
            ffi::g_socket_connection_factory_register_type(
                g_type.into_glib(),
                family.into_glib(),
                type_.into_glib(),
                protocol,
            );
        }
    }
}

pub trait SocketConnectionExt: IsA<SocketConnection> + 'static {
    #[doc(alias = "g_socket_connection_connect")]
    fn connect(
        &self,
        address: &impl IsA<SocketAddress>,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_socket_connection_connect(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_socket_connection_connect_async")]
    fn connect_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        address: &impl IsA<SocketAddress>,
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
        unsafe extern "C" fn connect_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ =
                ffi::g_socket_connection_connect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = connect_async_trampoline::<P>;
        unsafe {
            ffi::g_socket_connection_connect_async(
                self.as_ref().to_glib_none().0,
                address.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_future(
        &self,
        address: &(impl IsA<SocketAddress> + Clone + 'static),
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let address = address.clone();
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.connect_async(&address, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    #[doc(alias = "g_socket_connection_get_local_address")]
    #[doc(alias = "get_local_address")]
    fn local_address(&self) -> Result<SocketAddress, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_connection_get_local_address(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_socket_connection_get_remote_address")]
    #[doc(alias = "get_remote_address")]
    fn remote_address(&self) -> Result<SocketAddress, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_connection_get_remote_address(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_socket_connection_get_socket")]
    #[doc(alias = "get_socket")]
    fn socket(&self) -> Socket {
        unsafe {
            from_glib_none(ffi::g_socket_connection_get_socket(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_socket_connection_is_connected")]
    fn is_connected(&self) -> bool {
        unsafe {
            from_glib(ffi::g_socket_connection_is_connected(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<SocketConnection>> SocketConnectionExt for O {}

impl fmt::Display for SocketConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SocketConnection")
    }
}
