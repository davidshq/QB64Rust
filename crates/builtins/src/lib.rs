//! The built-in table, generated at build time from `tools/builtins/builtins.json` (`study\10` §3).
//!
//! Only data lives here. Which built-ins the compiler supports, and how each is typed and lowered, is decided by
//! `sema` and `ir`; in the first slice that is `INSTR` alone.

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
        .map(|i| BuiltinId(i as u16))
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
    fn table_is_complete() {
        assert_eq!(BUILTINS.len(), 455);
    }
}
