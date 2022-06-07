use std::os::raw::{c_char, c_int};

mod atom;

type BuiltinFnPtr = unsafe extern "C" fn(list: *mut scallop::bash::WordList) -> c_int;

// Manually define builtin struct since bindgen doesn't support non-null function pointers yet.
// Wrapping the function pointer field member in Option<fn> causes bash to segfault when loading
// a struct during an `enable -f /path/to/lib.so builtin` call.
//
// Related upstream issue: https://github.com/rust-lang/rust-bindgen/issues/1278
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(self) struct Builtin {
    name: *const c_char,
    function: BuiltinFnPtr,
    flags: c_int,
    long_doc: *const *mut c_char,
    short_doc: *const c_char,
    handle: *mut c_char,
}

/// Convert a Builtin to its C equivalent.
impl From<scallop::builtins::Builtin> for Builtin {
    fn from(b: scallop::builtins::Builtin) -> Self {
        // first convert to the Option wrapped variant
        let b: scallop::bash::Builtin = b.into();

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

#[export_name = "profile_struct"]
static mut PROFILE_STRUCT: Option<Builtin> = None;

#[no_mangle]
pub(super) extern "C" fn initialize() {
    use scallop::builtins::{profile, update_run_map};

    // update struct pointers
    unsafe {
        PROFILE_STRUCT = Some(profile::BUILTIN.into());
    }

    // add builtins to known run() mapping
    update_run_map([&profile::BUILTIN]);

    #[cfg(feature = "pkgcraft")]
    {
        // update struct pointers
        unsafe {
            atom::ATOM_STRUCT = Some(atom::BUILTIN.into());
        }

        // add builtins to known run() mapping
        update_run_map([&atom::BUILTIN]);
    }
}
