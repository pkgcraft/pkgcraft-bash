#![cfg(feature = "pkgcraft")]
use std::str::FromStr;

use itertools::Itertools;
use pkgcraft::atom::Atom;
use scallop::builtins::{declare, make_builtin, DynBuiltin, ExecStatus};
use scallop::variables::unbind;
use scallop::{Error, Result};

static LONG_DOC: &str = "Parse an atom into the $ATOM associative array.";

#[doc = stringify!(LONG_DOC)]
fn run(args: &[&str]) -> Result<ExecStatus> {
    let s = match args.len() {
        1 => args[0],
        n => return Err(Error::Builtin(format!("requires 1 arg, got {n}"))),
    };

    let a = Atom::from_str(s)?;
    let attr_map: String = [
        ("category", a.category()),
        ("package", a.package()),
        ("version", a.version().map(|v| v.as_str()).unwrap_or_default()),
        ("revision", a.revision().map(|v| v.as_str()).unwrap_or_default()),
        ("slot", a.slot().unwrap_or_default()),
        ("subslot", a.subslot().unwrap_or_default()),
        ("use", &a.use_deps().map(|v| v.join(" ")).unwrap_or_default()),
        ("repo", a.repo().unwrap_or_default()),
    ]
    .iter()
    .map(|(k, v)| format!("[{k}]=\"{v}\""))
    .join(" ");

    unbind("ATOM")?;
    // TODO: natively assign to associative array instead of using `declare`
    declare(&["-A", &format!("ATOM=( {attr_map} )")])?;

    Ok(ExecStatus::Success)
}

#[export_name = "atom_struct"]
pub(super) static mut ATOM_STRUCT: Option<DynBuiltin> = None;

make_builtin!("atom", atom_builtin, run, LONG_DOC, "atom \">=cat/pkg-1:2/3[use]::repo\"");
