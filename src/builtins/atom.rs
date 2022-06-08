#![cfg(feature = "pkgcraft")]
use std::str::FromStr;

use itertools::Itertools;
use pkgcraft::atom::Atom;
use scallop::builtins::{declare, Builtin, ExecStatus, Plugin};
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
        ("version", a.version().map(|v| v.as_str()).unwrap_or("")),
        ("revision", a.revision().map(|v| v.as_str()).unwrap_or("")),
        ("slot", a.slot().unwrap_or("")),
        ("subslot", a.subslot().unwrap_or("")),
        ("use", &a.use_deps().map(|v| v.join(" ")).unwrap_or_else(|| "".into())),
        ("repo", a.repo().unwrap_or("")),
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
pub(super) static mut ATOM_STRUCT: Option<Plugin> = None;

pub(super) static BUILTIN: Builtin = Builtin {
    name: "atom",
    func: run,
    help: LONG_DOC,
    usage: "atom \">=cat/pkg-1:2/3[use]::repo\"",
};
