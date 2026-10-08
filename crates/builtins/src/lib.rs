//! The built-in table, generated at build time from `tools/builtins/builtins.json` (`study\10` §3).
//!
//! Only data lives here. Which built-ins the compiler supports, and how each is typed and lowered, is decided by
//! `sema` and `ir`; in the first slice that is `INSTR` alone.

pub mod template;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Function,
    Sub,
}

#[derive(Debug)]
pub struct Builtin {
    /// Name as registered by the old compiler (mixed case, e.g. `InStr`, `_Trim`).
    pub name: &'static str,
    pub kind: Kind,
    /// The C entry point in libqb (empty when the old compiler special-cases the call).
    pub callname: &'static str,
    /// Decoded argument type names, one per argument slot (`LONG`, `STRING`, `any-numeric`, ...).
    pub arg_types: &'static [&'static str],
    /// For each slot, whether it is optional; `None` when the `specialformat` is a statement syntax that is not
    /// a plain slot list.
    pub optional: Option<&'static [bool]>,
    pub specialformat: Option<&'static str>,
    /// Return type name for functions.
    pub ret: Option<&'static str>,
    /// The suffix the name must be written with (`$` for `LEFT$`, `CHR$`); `None` when it is written bare.
    pub musthave: Option<&'static str>,
}

include!(concat!(env!("OUT_DIR"), "/builtins_table.rs"));

/// Index of a built-in in [`BUILTINS`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BuiltinId(pub u16);

impl BuiltinId {
    pub fn get(self) -> &'static Builtin {
        &BUILTINS[self.0 as usize]
    }
}

/// The function-form entry for a name, compared without regard to ASCII case.
pub fn find_function(name: &[u8]) -> Option<BuiltinId> {
    BUILTINS
        .iter()
        .position(|b| b.kind == Kind::Function && b.name.as_bytes().eq_ignore_ascii_case(name))
        .map(|i| BuiltinId(u16::try_from(i).expect("fewer than 65536 built-ins")))
}

/// The built-in statements a statement starting with `name` (a word as written, suffix included) may be, with
/// their templates, in table order: SUB entries with a `specialformat` whose name matches, with the required
/// suffix if the entry has one (`TIME$ = …`) and without one otherwise (`LINE …`).
pub fn statement_templates(name: &[u8]) -> Vec<(&'static Builtin, Vec<template::Item>)> {
    let end = name
        .iter()
        .position(|&b| !(b.is_ascii_alphanumeric() || b == b'_'))
        .unwrap_or(name.len());
    let (bare, suffix) = name.split_at(end);
    BUILTINS
        .iter()
        .filter(|b| b.kind == Kind::Sub && b.name.as_bytes().eq_ignore_ascii_case(bare))
        .filter(|b| b.musthave.map_or(b"".as_slice(), str::as_bytes) == suffix)
        .filter_map(|b| {
            let t = b.specialformat?;
            Some((b, template::parse(t).expect("templates are checked by build.rs")))
        })
        .collect()
}

/// Whether `name` (without suffix, any case) is declared by QB64pe's always-included BASIC files (`_TRUE`,
/// `_CHR_CR`, `_LOG_TRACE`, ...; `tools\builtins\builtins.json` `auto_include`).
pub fn is_auto_include_name(name: &[u8]) -> bool {
    let upper = name.to_ascii_uppercase();
    AUTO_INCLUDE_NAMES
        .binary_search_by(|n| n.as_bytes().cmp(&upper[..]))
        .is_ok()
}

/// Every entry (function or SUB form) for a name without suffix, compared without regard to ASCII case.
pub fn find_any(name: &[u8]) -> impl Iterator<Item = &'static Builtin> + '_ {
    BUILTINS
        .iter()
        .filter(move |b| b.name.as_bytes().eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instr_has_three_slots_first_optional() {
        let b = find_function(b"instr").unwrap().get();
        assert_eq!(b.callname, "func_instr");
        assert_eq!(b.arg_types, &["LONG", "STRING", "STRING"]);
        assert_eq!(b.optional, Some(&[true, false, false][..]));
        assert_eq!(b.ret, Some("LONG"));
    }

    #[test]
    fn required_suffix() {
        assert!(find_any(b"left").all(|b| b.musthave == Some("$")));
        assert!(find_any(b"len").all(|b| b.musthave.is_none()));
        assert!(find_any(b"cls").next().is_some());
    }

    /// Design D6: every template parses and prints back to its string (`build.rs` checks the same).
    #[test]
    fn every_template_parses() {
        let mut n = 0;
        for b in BUILTINS {
            if let Some(t) = b.specialformat {
                let items = template::parse(t).unwrap_or_else(|e| panic!("{}: {e}", b.name));
                assert_eq!(template::print(&items), t, "{}", b.name);
                n += 1;
            }
        }
        assert_eq!(n, 172);
    }

    #[test]
    fn template_shapes() {
        use template::{Atom, Item};
        let line = statement_templates(b"line");
        assert_eq!(line.len(), 1);
        assert!(matches!(&line[0].1[..], [Item::Optional(_), Item::Punct(b'-'), ..]));
        let open = statement_templates(b"OPEN");
        assert_eq!(open.len(), 2, "the FOR … AS form and the legacy form");
        let Item::Choice(alts) = &template::parse("{Len =}").unwrap()[0] else {
            panic!("a choice");
        };
        assert_eq!(alts[0], vec![Atom::Word("Len".into()), Atom::Punct(b'=')]);
        assert_eq!(statement_templates(b"TIME$").len(), 1);
        assert!(statement_templates(b"TIME").is_empty(), "`TIME$ =` needs its `$`");
        assert!(statement_templates(b"PRINT").is_empty());
    }

    #[test]
    fn auto_include_names() {
        assert!(is_auto_include_name(b"_true"));
        assert!(is_auto_include_name(b"_CHR_CR"));
        assert!(is_auto_include_name(b"_LOG_TRACE"));
        assert!(is_auto_include_name(b"_WhatIsMyIP"));
        assert!(!is_auto_include_name(b"AQUA"), "only with $COLOR");
        assert!(!is_auto_include_name(b"_NOPE"));
    }

    #[test]
    fn table_is_complete() {
        assert_eq!(BUILTINS.len(), 455);
    }
}
