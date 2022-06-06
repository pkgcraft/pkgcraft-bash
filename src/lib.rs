pub(crate) mod core;
mod pkgcraft;

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
static INITIALIZE_BUILTINS: extern "C" fn() = crate::core::initialize_builtins;
