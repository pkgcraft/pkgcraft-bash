use std::str::FromStr;

use itertools::Itertools;
use pkgcraft::atom::Atom;
use scallop::builtins::{declare, Builtin, ExecStatus};
use scallop::variables::unbind;
use scallop::{Error, Result};

static LONG_DOC: &str = "Parse an atom into an associative array.";

#[doc = stringify!(LONG_DOC)]
pub(crate) fn run(args: &[&str]) -> Result<ExecStatus> {
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
        ("repo", a.repo().unwrap_or("")),
    ]
    .iter()
    .map(|(k, v)| format!("[{k}]={v}"))
    .join(" ");

    unbind("ATOM")?;
    declare(&["-A", &format!("ATOM=( {attr_map} )")])?;

    Ok(ExecStatus::Success)
}

pub static BUILTIN: Builtin = Builtin {
    name: "atom",
    func: run,
    help: LONG_DOC,
    usage: "atom \">=cat/pkg-1:2/3[use]::repo\"",
};
