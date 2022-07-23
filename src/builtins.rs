use ctor::ctor;
use scallop::builtins::DynBuiltin;

mod atom;

#[export_name = "profile_struct"]
static mut PROFILE_STRUCT: Option<DynBuiltin> = None;

#[ctor]
fn initialize() {
    use scallop::builtins::{profile, update_run_map};

    // update struct pointers
    unsafe {
        PROFILE_STRUCT = Some(profile::BUILTIN.into());
    }

    // add builtins to known run() mapping
    update_run_map(&[profile::BUILTIN]);

    #[cfg(feature = "pkgcraft")]
    {
        // update struct pointers
        unsafe {
            atom::ATOM_STRUCT = Some(atom::BUILTIN.into());
        }

        // add builtins to known run() mapping
        update_run_map(&[atom::BUILTIN]);
    }
}
