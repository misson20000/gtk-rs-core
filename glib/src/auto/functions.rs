// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
use crate::FileSetContentsFlags;
use crate::{
    translate::*, Bytes, ChecksumType, Error, FileTest, FormatSizeFlags, Pid, Source, SpawnFlags,
    UnicodeScript, UserDirectory,
};
use std::{boxed::Box as Box_, mem, ptr};

#[doc(alias = "g_access")]
pub fn access(filename: impl AsRef<std::path::Path>, mode: i32) -> i32 {
    unsafe { ffi::g_access(filename.as_ref().to_glib_none().0, mode) }
}

#[doc(alias = "g_base64_decode")]
pub fn base64_decode(text: &str) -> Vec<u8> {
    unsafe {
        let mut out_len = mem::MaybeUninit::uninit();
        let ret = FromGlibContainer::from_glib_full_num(
            ffi::g_base64_decode(text.to_glib_none().0, out_len.as_mut_ptr()),
            out_len.assume_init() as _,
        );
        ret
    }
}

//#[doc(alias = "g_base64_decode_inplace")]
//pub fn base64_decode_inplace(text: /*Unimplemented*/Vec<u8>) -> u8 {
//    unsafe { TODO: call ffi:g_base64_decode_inplace() }
//}

//#[doc(alias = "g_base64_decode_step")]
//pub fn base64_decode_step(in_: &[u8], out: Vec<u8>, state: &mut i32, save: &mut u32) -> usize {
//    unsafe { TODO: call ffi:g_base64_decode_step() }
//}

#[doc(alias = "g_base64_encode")]
pub fn base64_encode(data: &[u8]) -> crate::GString {
    let len = data.len() as _;
    unsafe { from_glib_full(ffi::g_base64_encode(data.to_glib_none().0, len)) }
}

//#[doc(alias = "g_base64_encode_close")]
//pub fn base64_encode_close(break_lines: bool, out: Vec<u8>, state: &mut i32, save: &mut i32) -> usize {
//    unsafe { TODO: call ffi:g_base64_encode_close() }
//}

//#[doc(alias = "g_base64_encode_step")]
//pub fn base64_encode_step(in_: &[u8], break_lines: bool, out: Vec<u8>, state: &mut i32, save: &mut i32) -> usize {
//    unsafe { TODO: call ffi:g_base64_encode_step() }
//}

#[doc(alias = "glib_check_version")]
pub fn check_version(
    required_major: u32,
    required_minor: u32,
    required_micro: u32,
) -> Option<crate::GString> {
    unsafe {
        from_glib_none(ffi::glib_check_version(
            required_major,
            required_minor,
            required_micro,
        ))
    }
}

#[doc(alias = "g_compute_checksum_for_bytes")]
pub fn compute_checksum_for_bytes(
    checksum_type: ChecksumType,
    data: &Bytes,
) -> Option<crate::GString> {
    unsafe {
        from_glib_full(ffi::g_compute_checksum_for_bytes(
            checksum_type.into_glib(),
            data.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_compute_checksum_for_data")]
pub fn compute_checksum_for_data(
    checksum_type: ChecksumType,
    data: &[u8],
) -> Option<crate::GString> {
    let length = data.len() as _;
    unsafe {
        from_glib_full(ffi::g_compute_checksum_for_data(
            checksum_type.into_glib(),
            data.to_glib_none().0,
            length,
        ))
    }
}

#[doc(alias = "g_compute_hmac_for_bytes")]
pub fn compute_hmac_for_bytes(
    digest_type: ChecksumType,
    key: &Bytes,
    data: &Bytes,
) -> crate::GString {
    unsafe {
        from_glib_full(ffi::g_compute_hmac_for_bytes(
            digest_type.into_glib(),
            key.to_glib_none().0,
            data.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_compute_hmac_for_data")]
pub fn compute_hmac_for_data(digest_type: ChecksumType, key: &[u8], data: &[u8]) -> crate::GString {
    let key_len = key.len() as _;
    let length = data.len() as _;
    unsafe {
        from_glib_full(ffi::g_compute_hmac_for_data(
            digest_type.into_glib(),
            key.to_glib_none().0,
            key_len,
            data.to_glib_none().0,
            length,
        ))
    }
}

#[doc(alias = "g_dcgettext")]
pub fn dcgettext(domain: Option<&str>, msgid: &str, category: i32) -> crate::GString {
    unsafe {
        from_glib_none(ffi::g_dcgettext(
            domain.to_glib_none().0,
            msgid.to_glib_none().0,
            category,
        ))
    }
}

#[doc(alias = "g_dgettext")]
pub fn dgettext(domain: Option<&str>, msgid: &str) -> crate::GString {
    unsafe {
        from_glib_none(ffi::g_dgettext(
            domain.to_glib_none().0,
            msgid.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_dngettext")]
pub fn dngettext(
    domain: Option<&str>,
    msgid: &str,
    msgid_plural: &str,
    n: libc::c_ulong,
) -> crate::GString {
    unsafe {
        from_glib_none(ffi::g_dngettext(
            domain.to_glib_none().0,
            msgid.to_glib_none().0,
            msgid_plural.to_glib_none().0,
            n,
        ))
    }
}

#[doc(alias = "g_dpgettext")]
pub fn dpgettext(domain: Option<&str>, msgctxtid: &str, msgidoffset: usize) -> crate::GString {
    unsafe {
        from_glib_none(ffi::g_dpgettext(
            domain.to_glib_none().0,
            msgctxtid.to_glib_none().0,
            msgidoffset,
        ))
    }
}

#[doc(alias = "g_dpgettext2")]
pub fn dpgettext2(domain: Option<&str>, context: &str, msgid: &str) -> crate::GString {
    unsafe {
        from_glib_none(ffi::g_dpgettext2(
            domain.to_glib_none().0,
            context.to_glib_none().0,
            msgid.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_file_set_contents")]
pub fn file_set_contents(
    filename: impl AsRef<std::path::Path>,
    contents: &[u8],
) -> Result<(), crate::Error> {
    let length = contents.len() as _;
    unsafe {
        let mut error = ptr::null_mut();
        let is_ok = ffi::g_file_set_contents(
            filename.as_ref().to_glib_none().0,
            contents.to_glib_none().0,
            length,
            &mut error,
        );
        debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
#[doc(alias = "g_file_set_contents_full")]
pub fn file_set_contents_full(
    filename: impl AsRef<std::path::Path>,
    contents: &[u8],
    flags: FileSetContentsFlags,
    mode: i32,
) -> Result<(), crate::Error> {
    let length = contents.len() as _;
    unsafe {
        let mut error = ptr::null_mut();
        let is_ok = ffi::g_file_set_contents_full(
            filename.as_ref().to_glib_none().0,
            contents.to_glib_none().0,
            length,
            flags.into_glib(),
            mode,
            &mut error,
        );
        debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_file_test")]
#[allow(dead_code)]
pub(crate) fn file_test(filename: impl AsRef<std::path::Path>, test: FileTest) -> bool {
    unsafe {
        from_glib(ffi::g_file_test(
            filename.as_ref().to_glib_none().0,
            test.into_glib(),
        ))
    }
}

#[doc(alias = "g_filename_display_basename")]
pub fn filename_display_basename(filename: impl AsRef<std::path::Path>) -> crate::GString {
    unsafe {
        from_glib_full(ffi::g_filename_display_basename(
            filename.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_filename_display_name")]
pub fn filename_display_name(filename: impl AsRef<std::path::Path>) -> crate::GString {
    unsafe {
        from_glib_full(ffi::g_filename_display_name(
            filename.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_filename_from_uri")]
pub fn filename_from_uri(
    uri: &str,
) -> Result<(std::path::PathBuf, Option<crate::GString>), crate::Error> {
    unsafe {
        let mut hostname = ptr::null_mut();
        let mut error = ptr::null_mut();
        let ret = ffi::g_filename_from_uri(uri.to_glib_none().0, &mut hostname, &mut error);
        if error.is_null() {
            Ok((from_glib_full(ret), from_glib_full(hostname)))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_filename_to_uri")]
pub fn filename_to_uri(
    filename: impl AsRef<std::path::Path>,
    hostname: Option<&str>,
) -> Result<crate::GString, crate::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_filename_to_uri(
            filename.as_ref().to_glib_none().0,
            hostname.to_glib_none().0,
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_find_program_in_path")]
pub fn find_program_in_path(program: impl AsRef<std::path::Path>) -> Option<std::path::PathBuf> {
    unsafe {
        from_glib_full(ffi::g_find_program_in_path(
            program.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_format_size")]
pub fn format_size(size: u64) -> crate::GString {
    unsafe { from_glib_full(ffi::g_format_size(size)) }
}

#[doc(alias = "g_format_size_full")]
pub fn format_size_full(size: u64, flags: FormatSizeFlags) -> crate::GString {
    unsafe { from_glib_full(ffi::g_format_size_full(size, flags.into_glib())) }
}

#[doc(alias = "g_get_application_name")]
#[doc(alias = "get_application_name")]
pub fn application_name() -> Option<crate::GString> {
    unsafe { from_glib_none(ffi::g_get_application_name()) }
}

#[doc(alias = "g_get_codeset")]
#[doc(alias = "get_codeset")]
pub fn codeset() -> crate::GString {
    unsafe { from_glib_full(ffi::g_get_codeset()) }
}

#[cfg(feature = "v2_62")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_62")))]
#[doc(alias = "g_get_console_charset")]
#[doc(alias = "get_console_charset")]
pub fn console_charset() -> Option<crate::GString> {
    unsafe {
        let mut charset = ptr::null();
        let ret = from_glib(ffi::g_get_console_charset(&mut charset));
        if ret {
            Some(from_glib_none(charset))
        } else {
            None
        }
    }
}

#[doc(alias = "g_get_current_dir")]
#[doc(alias = "get_current_dir")]
pub fn current_dir() -> std::path::PathBuf {
    unsafe { from_glib_full(ffi::g_get_current_dir()) }
}

#[doc(alias = "g_get_environ")]
#[doc(alias = "get_environ")]
pub fn environ() -> Vec<std::ffi::OsString> {
    unsafe { FromGlibPtrContainer::from_glib_full(ffi::g_get_environ()) }
}

#[doc(alias = "g_get_home_dir")]
#[doc(alias = "get_home_dir")]
pub fn home_dir() -> std::path::PathBuf {
    unsafe { from_glib_none(ffi::g_get_home_dir()) }
}

#[doc(alias = "g_get_host_name")]
#[doc(alias = "get_host_name")]
pub fn host_name() -> crate::GString {
    unsafe { from_glib_none(ffi::g_get_host_name()) }
}

#[doc(alias = "g_get_language_names")]
#[doc(alias = "get_language_names")]
pub fn language_names() -> Vec<crate::GString> {
    unsafe { FromGlibPtrContainer::from_glib_none(ffi::g_get_language_names()) }
}

#[cfg(feature = "v2_58")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_58")))]
#[doc(alias = "g_get_language_names_with_category")]
#[doc(alias = "get_language_names_with_category")]
pub fn language_names_with_category(category_name: &str) -> Vec<crate::GString> {
    unsafe {
        FromGlibPtrContainer::from_glib_none(ffi::g_get_language_names_with_category(
            category_name.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_get_locale_variants")]
#[doc(alias = "get_locale_variants")]
pub fn locale_variants(locale: &str) -> Vec<crate::GString> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::g_get_locale_variants(locale.to_glib_none().0))
    }
}

#[doc(alias = "g_get_monotonic_time")]
#[doc(alias = "get_monotonic_time")]
pub fn monotonic_time() -> i64 {
    unsafe { ffi::g_get_monotonic_time() }
}

#[doc(alias = "g_get_num_processors")]
#[doc(alias = "get_num_processors")]
pub fn num_processors() -> u32 {
    unsafe { ffi::g_get_num_processors() }
}

#[cfg(feature = "v2_64")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_64")))]
#[doc(alias = "g_get_os_info")]
#[doc(alias = "get_os_info")]
pub fn os_info(key_name: &str) -> Option<crate::GString> {
    unsafe { from_glib_full(ffi::g_get_os_info(key_name.to_glib_none().0)) }
}

#[doc(alias = "g_get_real_name")]
#[doc(alias = "get_real_name")]
pub fn real_name() -> std::ffi::OsString {
    unsafe { from_glib_none(ffi::g_get_real_name()) }
}

#[doc(alias = "g_get_real_time")]
#[doc(alias = "get_real_time")]
pub fn real_time() -> i64 {
    unsafe { ffi::g_get_real_time() }
}

#[doc(alias = "g_get_system_config_dirs")]
#[doc(alias = "get_system_config_dirs")]
pub fn system_config_dirs() -> Vec<std::path::PathBuf> {
    unsafe { FromGlibPtrContainer::from_glib_none(ffi::g_get_system_config_dirs()) }
}

#[doc(alias = "g_get_system_data_dirs")]
#[doc(alias = "get_system_data_dirs")]
pub fn system_data_dirs() -> Vec<std::path::PathBuf> {
    unsafe { FromGlibPtrContainer::from_glib_none(ffi::g_get_system_data_dirs()) }
}

#[doc(alias = "g_get_tmp_dir")]
#[doc(alias = "get_tmp_dir")]
pub fn tmp_dir() -> std::path::PathBuf {
    unsafe { from_glib_none(ffi::g_get_tmp_dir()) }
}

#[doc(alias = "g_get_user_cache_dir")]
#[doc(alias = "get_user_cache_dir")]
pub fn user_cache_dir() -> std::path::PathBuf {
    unsafe { from_glib_none(ffi::g_get_user_cache_dir()) }
}

#[doc(alias = "g_get_user_config_dir")]
#[doc(alias = "get_user_config_dir")]
pub fn user_config_dir() -> std::path::PathBuf {
    unsafe { from_glib_none(ffi::g_get_user_config_dir()) }
}

#[doc(alias = "g_get_user_data_dir")]
#[doc(alias = "get_user_data_dir")]
pub fn user_data_dir() -> std::path::PathBuf {
    unsafe { from_glib_none(ffi::g_get_user_data_dir()) }
}

#[doc(alias = "g_get_user_name")]
#[doc(alias = "get_user_name")]
pub fn user_name() -> std::ffi::OsString {
    unsafe { from_glib_none(ffi::g_get_user_name()) }
}

#[doc(alias = "g_get_user_runtime_dir")]
#[doc(alias = "get_user_runtime_dir")]
pub fn user_runtime_dir() -> std::path::PathBuf {
    unsafe { from_glib_none(ffi::g_get_user_runtime_dir()) }
}

#[doc(alias = "g_get_user_special_dir")]
#[doc(alias = "get_user_special_dir")]
pub fn user_special_dir(directory: UserDirectory) -> Option<std::path::PathBuf> {
    unsafe { from_glib_none(ffi::g_get_user_special_dir(directory.into_glib())) }
}

#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
#[doc(alias = "g_get_user_state_dir")]
#[doc(alias = "get_user_state_dir")]
pub fn user_state_dir() -> std::path::PathBuf {
    unsafe { from_glib_none(ffi::g_get_user_state_dir()) }
}

#[doc(alias = "g_getenv")]
pub fn getenv(variable: impl AsRef<std::ffi::OsStr>) -> Option<std::ffi::OsString> {
    unsafe { from_glib_none(ffi::g_getenv(variable.as_ref().to_glib_none().0)) }
}

#[doc(alias = "g_hostname_is_ascii_encoded")]
pub fn hostname_is_ascii_encoded(hostname: &str) -> bool {
    unsafe { from_glib(ffi::g_hostname_is_ascii_encoded(hostname.to_glib_none().0)) }
}

#[doc(alias = "g_hostname_is_ip_address")]
pub fn hostname_is_ip_address(hostname: &str) -> bool {
    unsafe { from_glib(ffi::g_hostname_is_ip_address(hostname.to_glib_none().0)) }
}

#[doc(alias = "g_hostname_is_non_ascii")]
pub fn hostname_is_non_ascii(hostname: &str) -> bool {
    unsafe { from_glib(ffi::g_hostname_is_non_ascii(hostname.to_glib_none().0)) }
}

#[doc(alias = "g_hostname_to_ascii")]
pub fn hostname_to_ascii(hostname: &str) -> Option<crate::GString> {
    unsafe { from_glib_full(ffi::g_hostname_to_ascii(hostname.to_glib_none().0)) }
}

#[doc(alias = "g_hostname_to_unicode")]
pub fn hostname_to_unicode(hostname: &str) -> Option<crate::GString> {
    unsafe { from_glib_full(ffi::g_hostname_to_unicode(hostname.to_glib_none().0)) }
}

#[doc(alias = "g_listenv")]
pub fn listenv() -> Vec<std::ffi::OsString> {
    unsafe { FromGlibPtrContainer::from_glib_full(ffi::g_listenv()) }
}

#[doc(alias = "g_main_current_source")]
pub fn main_current_source() -> Option<Source> {
    unsafe { from_glib_none(ffi::g_main_current_source()) }
}

#[doc(alias = "g_main_depth")]
pub fn main_depth() -> i32 {
    unsafe { ffi::g_main_depth() }
}

#[doc(alias = "g_markup_escape_text")]
pub fn markup_escape_text(text: &str) -> crate::GString {
    let length = text.len() as _;
    unsafe { from_glib_full(ffi::g_markup_escape_text(text.to_glib_none().0, length)) }
}

#[doc(alias = "g_mkdir_with_parents")]
pub fn mkdir_with_parents(pathname: impl AsRef<std::path::Path>, mode: i32) -> i32 {
    unsafe { ffi::g_mkdir_with_parents(pathname.as_ref().to_glib_none().0, mode) }
}

#[doc(alias = "g_on_error_query")]
pub fn on_error_query(prg_name: &str) {
    unsafe {
        ffi::g_on_error_query(prg_name.to_glib_none().0);
    }
}

#[doc(alias = "g_on_error_stack_trace")]
pub fn on_error_stack_trace(prg_name: &str) {
    unsafe {
        ffi::g_on_error_stack_trace(prg_name.to_glib_none().0);
    }
}

#[doc(alias = "g_path_get_basename")]
#[allow(dead_code)]
pub(crate) fn path_get_basename(file_name: impl AsRef<std::path::Path>) -> std::path::PathBuf {
    unsafe {
        from_glib_full(ffi::g_path_get_basename(
            file_name.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_path_get_dirname")]
#[allow(dead_code)]
pub(crate) fn path_get_dirname(file_name: impl AsRef<std::path::Path>) -> std::path::PathBuf {
    unsafe { from_glib_full(ffi::g_path_get_dirname(file_name.as_ref().to_glib_none().0)) }
}

//#[doc(alias = "g_poll")]
//pub fn poll(fds: /*Ignored*/&mut PollFD, nfds: u32, timeout: i32) -> i32 {
//    unsafe { TODO: call ffi:g_poll() }
//}

#[doc(alias = "g_random_double")]
pub fn random_double() -> f64 {
    unsafe { ffi::g_random_double() }
}

#[doc(alias = "g_random_double_range")]
pub fn random_double_range(begin: f64, end: f64) -> f64 {
    unsafe { ffi::g_random_double_range(begin, end) }
}

#[doc(alias = "g_random_int")]
pub fn random_int() -> u32 {
    unsafe { ffi::g_random_int() }
}

#[doc(alias = "g_random_int_range")]
pub fn random_int_range(begin: i32, end: i32) -> i32 {
    unsafe { ffi::g_random_int_range(begin, end) }
}

#[doc(alias = "g_random_set_seed")]
pub fn random_set_seed(seed: u32) {
    unsafe {
        ffi::g_random_set_seed(seed);
    }
}

#[doc(alias = "g_reload_user_special_dirs_cache")]
pub fn reload_user_special_dirs_cache() {
    unsafe {
        ffi::g_reload_user_special_dirs_cache();
    }
}

#[doc(alias = "g_set_application_name")]
pub fn set_application_name(application_name: &str) {
    unsafe {
        ffi::g_set_application_name(application_name.to_glib_none().0);
    }
}

#[doc(alias = "g_setenv")]
pub fn setenv(
    variable: impl AsRef<std::ffi::OsStr>,
    value: impl AsRef<std::ffi::OsStr>,
    overwrite: bool,
) -> Result<(), crate::error::BoolError> {
    unsafe {
        crate::result_from_gboolean!(
            ffi::g_setenv(
                variable.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
                overwrite.into_glib()
            ),
            "Failed to set environment variable"
        )
    }
}

#[doc(alias = "g_shell_parse_argv")]
pub fn shell_parse_argv(
    command_line: impl AsRef<std::ffi::OsStr>,
) -> Result<Vec<std::ffi::OsString>, crate::Error> {
    unsafe {
        let mut argcp = mem::MaybeUninit::uninit();
        let mut argvp = ptr::null_mut();
        let mut error = ptr::null_mut();
        let is_ok = ffi::g_shell_parse_argv(
            command_line.as_ref().to_glib_none().0,
            argcp.as_mut_ptr(),
            &mut argvp,
            &mut error,
        );
        debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(FromGlibContainer::from_glib_full_num(
                argvp,
                argcp.assume_init() as _,
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_shell_quote")]
pub fn shell_quote(unquoted_string: impl AsRef<std::ffi::OsStr>) -> std::ffi::OsString {
    unsafe {
        from_glib_full(ffi::g_shell_quote(
            unquoted_string.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_shell_unquote")]
pub fn shell_unquote(
    quoted_string: impl AsRef<std::ffi::OsStr>,
) -> Result<std::ffi::OsString, crate::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::g_shell_unquote(quoted_string.as_ref().to_glib_none().0, &mut error);
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "g_spaced_primes_closest")]
pub fn spaced_primes_closest(num: u32) -> u32 {
    unsafe { ffi::g_spaced_primes_closest(num) }
}

#[doc(alias = "g_spawn_async")]
pub fn spawn_async(
    working_directory: Option<impl AsRef<std::path::Path>>,
    argv: &[&std::path::Path],
    envp: &[&std::path::Path],
    flags: SpawnFlags,
    child_setup: Option<Box_<dyn FnOnce() + 'static>>,
) -> Result<Pid, crate::Error> {
    let child_setup_data: Box_<Option<Box_<dyn FnOnce() + 'static>>> = Box_::new(child_setup);
    unsafe extern "C" fn child_setup_func(data: ffi::gpointer) {
        let callback: Box_<Option<Box_<dyn FnOnce() + 'static>>> = Box_::from_raw(data as *mut _);
        let callback = (*callback).expect("cannot get closure...");
        callback()
    }
    let child_setup = if child_setup_data.is_some() {
        Some(child_setup_func as _)
    } else {
        None
    };
    let super_callback0: Box_<Option<Box_<dyn FnOnce() + 'static>>> = child_setup_data;
    unsafe {
        let mut child_pid = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let is_ok = ffi::g_spawn_async(
            working_directory
                .as_ref()
                .map(|p| p.as_ref())
                .to_glib_none()
                .0,
            argv.to_glib_none().0,
            envp.to_glib_none().0,
            flags.into_glib(),
            child_setup,
            Box_::into_raw(super_callback0) as *mut _,
            child_pid.as_mut_ptr(),
            &mut error,
        );
        debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib(child_pid.assume_init()))
        } else {
            Err(from_glib_full(error))
        }
    }
}

//#[cfg(feature = "v2_68")]
//#[cfg_attr(docsrs, doc(cfg(feature = "v2_68")))]
//#[doc(alias = "g_spawn_async_with_pipes_and_fds")]
//pub fn spawn_async_with_pipes_and_fds(working_directory: Option<impl AsRef<std::path::Path>>, argv: &[&std::path::Path], envp: &[&std::path::Path], flags: SpawnFlags, child_setup: Option<Box_<dyn FnOnce() + 'static>>, stdin_fd: i32, stdout_fd: i32, stderr_fd: i32, source_fds: &[i32], target_fds: &[i32], n_fds: usize) -> Result<(Pid, i32, i32, i32), crate::Error> {
//    unsafe { TODO: call ffi:g_spawn_async_with_pipes_and_fds() }
//}

#[cfg_attr(feature = "v2_70", deprecated = "Since 2.70")]
#[allow(deprecated)]
#[doc(alias = "g_spawn_check_exit_status")]
pub fn spawn_check_exit_status(wait_status: i32) -> Result<(), crate::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let is_ok = ffi::g_spawn_check_exit_status(wait_status, &mut error);
        debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(feature = "v2_70")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
#[doc(alias = "g_spawn_check_wait_status")]
pub fn spawn_check_wait_status(wait_status: i32) -> Result<(), crate::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let is_ok = ffi::g_spawn_check_wait_status(wait_status, &mut error);
        debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
#[doc(alias = "g_spawn_command_line_async")]
pub fn spawn_command_line_async(
    command_line: impl AsRef<std::ffi::OsStr>,
) -> Result<(), crate::Error> {
    unsafe {
        let mut error = ptr::null_mut();
        let is_ok =
            ffi::g_spawn_command_line_async(command_line.as_ref().to_glib_none().0, &mut error);
        debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

//#[doc(alias = "g_spawn_command_line_sync")]
//pub fn spawn_command_line_sync(command_line: impl AsRef<std::path::Path>, standard_output: Vec<u8>, standard_error: Vec<u8>) -> Result<i32, crate::Error> {
//    unsafe { TODO: call ffi:g_spawn_command_line_sync() }
//}

//#[doc(alias = "g_spawn_sync")]
//pub fn spawn_sync(working_directory: Option<impl AsRef<std::path::Path>>, argv: &[&std::path::Path], envp: &[&std::path::Path], flags: SpawnFlags, child_setup: Option<&mut dyn (FnMut())>, standard_output: Vec<u8>, standard_error: Vec<u8>) -> Result<i32, crate::Error> {
//    unsafe { TODO: call ffi:g_spawn_sync() }
//}

#[doc(alias = "g_unicode_script_from_iso15924")]
pub fn unicode_script_from_iso15924(iso15924: u32) -> UnicodeScript {
    unsafe { from_glib(ffi::g_unicode_script_from_iso15924(iso15924)) }
}

#[doc(alias = "g_unicode_script_to_iso15924")]
pub fn unicode_script_to_iso15924(script: UnicodeScript) -> u32 {
    unsafe { ffi::g_unicode_script_to_iso15924(script.into_glib()) }
}

//#[cfg(unix)]
//#[cfg_attr(docsrs, doc(cfg(unix)))]
//#[cfg(feature = "v2_64")]
//#[cfg_attr(docsrs, doc(cfg(feature = "v2_64")))]
//#[doc(alias = "g_unix_get_passwd_entry")]
//pub fn unix_get_passwd_entry(user_name: &str) -> Result</*Unimplemented*/Option<Basic: Pointer>, crate::Error> {
//    unsafe { TODO: call ffi:g_unix_get_passwd_entry() }
//}

#[doc(alias = "g_unlink")]
pub fn unlink(filename: impl AsRef<std::path::Path>) -> i32 {
    unsafe { ffi::g_unlink(filename.as_ref().to_glib_none().0) }
}

#[doc(alias = "g_unsetenv")]
pub fn unsetenv(variable: impl AsRef<std::ffi::OsStr>) {
    unsafe {
        ffi::g_unsetenv(variable.as_ref().to_glib_none().0);
    }
}

#[doc(alias = "g_usleep")]
pub fn usleep(microseconds: libc::c_ulong) {
    unsafe {
        ffi::g_usleep(microseconds);
    }
}

#[doc(alias = "g_uuid_string_is_valid")]
pub fn uuid_string_is_valid(str: &str) -> bool {
    unsafe { from_glib(ffi::g_uuid_string_is_valid(str.to_glib_none().0)) }
}

#[doc(alias = "g_uuid_string_random")]
pub fn uuid_string_random() -> crate::GString {
    unsafe { from_glib_full(ffi::g_uuid_string_random()) }
}
