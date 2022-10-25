#![cfg(feature = "pkgcraft")]
use std::str::FromStr;

use itertools::Itertools;
use pkgcraft::atom::Version;
use scallop::builtins::{make_builtin, DynBuiltin, ExecStatus};
use scallop::{Error, Result};

static LONG_DOC: &str = "Compare two package versions.";

#[doc = stringify!(LONG_DOC)]
fn run(args: &[&str]) -> Result<ExecStatus> {
    let s = match args.len() {
        1 => args[0],
        n => return Err(Error::Base(format!("requires 1 arg, got {n}"))),
    };

    let (v1, op, v2) = match s.split(' ').collect_tuple() {
        Some((v1, op, v2)) => (Version::from_str(v1)?, op, Version::from_str(v2)?),
        _ => return Err(Error::Base(format!("invalid argument format: {s}"))),
    };

    let cmp = match op {
        "<" => PartialOrd::lt(&v1, &v2),
        "<=" => PartialOrd::le(&v1, &v2),
        "==" => PartialEq::eq(&v1, &v2),
        "!=" => PartialEq::ne(&v1, &v2),
        ">=" => PartialOrd::ge(&v1, &v2),
        ">" => PartialOrd::gt(&v1, &v2),
        _ => return Err(Error::Base(format!("invalid operator: {op}"))),
    };

    Ok(ExecStatus::from(cmp))
}

#[export_name = "atom_version_struct"]
pub(super) static mut ATOM_VERSION_STRUCT: Option<DynBuiltin> = None;

make_builtin!(
    "atom_version",
    atom_version_builtin,
    run,
    LONG_DOC,
    "atom_version \"1.2.3 >= 1.2.4\""
);
