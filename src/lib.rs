use std::os::raw::{c_char, c_int};

use scallop::bash;
use scallop::builtins::{profile, update_run_map, Builtin as ScallopBuiltin};

#[cfg(feature = "pkgcraft")]
mod pkgcraft;

type BuiltinFnPtr = unsafe extern "C" fn(list: *mut bash::WordList) -> c_int;

// Manually define builtin struct since bindgen doesn't support non-null function pointers yet.
// Wrapping the function pointer field member in Option<fn> causes bash to segfault when loading
// a struct during an `enable -f /path/to/lib.so builtin` call.
//
// Related upstream issue: https://github.com/rust-lang/rust-bindgen/issues/1278
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Builtin {
    name: *const c_char,
    function: BuiltinFnPtr,
    flags: c_int,
    long_doc: *const *mut c_char,
    short_doc: *const c_char,
    handle: *mut c_char,
}

/// Convert a Builtin to its C equivalent.
impl From<ScallopBuiltin> for Builtin {
    fn from(b: ScallopBuiltin) -> Self {
        // first convert to the Option wrapped variant
        let b: bash::Builtin = b.into();

        // then convert to the non-Option wrapped variant
        Builtin {
            name: b.name,
            function: b.function.unwrap(),
            flags: b.flags,
            long_doc: b.long_doc,
            short_doc: b.short_doc,
            handle: b.handle,
        }
    }
}

#[cfg(feature = "pkgcraft")]
#[export_name = "atom_struct"]
static mut ATOM_STRUCT: Option<Builtin> = None;

#[export_name = "profile_struct"]
static mut PROFILE_STRUCT: Option<Builtin> = None;

#[used]
#[cfg_attr(
    any(
        target_os = "linux",
        target_os = "android",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ),
    link_section = ".init_array"
)]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INITIALIZE_BUILTINS: extern "C" fn() = initialize_builtins;

#[no_mangle]
extern "C" fn initialize_builtins() {
    // update struct pointers
    unsafe {
        PROFILE_STRUCT = Some(profile::BUILTIN.into());
    }

    // add builtins to known run() mapping
    update_run_map([&profile::BUILTIN]);

    #[cfg(feature = "pkgcraft")]
    initialize_pkgcraft_builtins();
}

#[cfg(feature = "pkgcraft")]
#[no_mangle]
extern "C" fn initialize_pkgcraft_builtins() {
    use crate::pkgcraft::*;

    // update struct pointers
    unsafe {
        ATOM_STRUCT = Some(atom::BUILTIN.into());
    }

    // add builtins to known run() mapping
    update_run_map([&atom::BUILTIN]);
}
