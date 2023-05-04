// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Converter, FilterInputStream, InputStream, PollableInputStream};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GConverterInputStream")]
    pub struct ConverterInputStream(Object<ffi::GConverterInputStream, ffi::GConverterInputStreamClass>) @extends FilterInputStream, InputStream, @implements PollableInputStream;

    match fn {
        type_ => || ffi::g_converter_input_stream_get_type(),
    }
}

impl ConverterInputStream {
    pub const NONE: Option<&'static ConverterInputStream> = None;

    #[doc(alias = "g_converter_input_stream_new")]
    pub fn new(
        base_stream: &impl IsA<InputStream>,
        converter: &impl IsA<Converter>,
    ) -> ConverterInputStream {
        unsafe {
            InputStream::from_glib_full(ffi::g_converter_input_stream_new(
                base_stream.as_ref().to_glib_none().0,
                converter.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ConverterInputStream`] objects.
    ///
    /// This method returns an instance of [`ConverterInputStreamBuilder`](crate::builders::ConverterInputStreamBuilder) which can be used to create [`ConverterInputStream`] objects.
    pub fn builder() -> ConverterInputStreamBuilder {
        ConverterInputStreamBuilder::new()
    }
}

impl Default for ConverterInputStream {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ConverterInputStream`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ConverterInputStreamBuilder {
    builder: glib::object::ObjectBuilder<'static, ConverterInputStream>,
}

impl ConverterInputStreamBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn converter(self, converter: &impl IsA<Converter>) -> Self {
        Self {
            builder: self
                .builder
                .property("converter", converter.clone().upcast()),
        }
    }

    pub fn base_stream(self, base_stream: &impl IsA<InputStream>) -> Self {
        Self {
            builder: self
                .builder
                .property("base-stream", base_stream.clone().upcast()),
        }
    }

    pub fn close_base_stream(self, close_base_stream: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("close-base-stream", close_base_stream),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ConverterInputStream`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ConverterInputStream {
        self.builder.build()
    }
}

pub trait ConverterInputStreamExt: IsA<ConverterInputStream> + 'static {
    #[doc(alias = "g_converter_input_stream_get_converter")]
    #[doc(alias = "get_converter")]
    fn converter(&self) -> Converter {
        unsafe {
            from_glib_none(ffi::g_converter_input_stream_get_converter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<ConverterInputStream>> ConverterInputStreamExt for O {}

impl fmt::Display for ConverterInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ConverterInputStream")
    }
}
