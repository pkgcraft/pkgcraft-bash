use ctor::ctor;
use scallop::builtins::DynBuiltin;

mod atom;
mod atom_version;

#[export_name = "profile_struct"]
static mut PROFILE_STRUCT: Option<DynBuiltin> = None;

#[ctor]
fn initialize() {
    use scallop::builtins::profile;

    unsafe {
        PROFILE_STRUCT = Some(profile::BUILTIN.into());
    }

    #[cfg(feature = "pkgcraft")]
    {
        unsafe {
            atom::ATOM_STRUCT = Some(atom::BUILTIN.into());
            atom_version::ATOM_VERSION_STRUCT = Some(atom_version::BUILTIN.into());
        }
    }
}
