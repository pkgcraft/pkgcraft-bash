use ctor::ctor;
use scallop::builtins::DynBuiltin;

mod atom;

#[export_name = "profile_struct"]
static mut PROFILE_STRUCT: Option<DynBuiltin> = None;

#[ctor]
fn initialize() {
    use scallop::builtins::profile;

    // update struct pointers
    unsafe {
        PROFILE_STRUCT = Some(profile::BUILTIN.into());
    }

    #[cfg(feature = "pkgcraft")]
    {
        // update struct pointers
        unsafe {
            atom::ATOM_STRUCT = Some(atom::BUILTIN.into());
        }
    }
}
