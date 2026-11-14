//! The lints that are on for a program (spec `compiler/lints`, "Lints table of the settings file" and
//! "Command-line lints"; OpenSpec change `project-settings`, design D3): the `[lints]` table of `qb64rust.toml`
//! (`settings.rs`), then `--lints` and `--lint`. A lint that is off still runs in `sema`; its warnings are dropped
//! from the program's diagnostics ([`Lints::drop_from`]).

use crate::fixes::on_off;
use qb64rust_base::Diagnostics;
use qb64rust_sema::lints::CODES;

/// One lint, by its place in [`CODES`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Code(usize);

/// A set of lints, one bit per code of [`CODES`]. [`Default`] is every lint on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Lints(u32);

impl Default for Lints {
    fn default() -> Self {
        Lints::ALL
    }
}

impl Lints {
    /// Every lint on.
    pub const ALL: Lints = Lints((1 << CODES.len()) - 1);
    /// Every lint off.
    pub const OFF: Lints = Lints(0);

    /// The lint of a code, if it is one.
    pub fn code(name: &str) -> Option<Code> {
        CODES.iter().position(|c| *c == name).map(Code)
    }

    /// The known codes, comma-separated, for an error message.
    pub fn names() -> String {
        CODES.join(", ")
    }

    #[must_use]
    pub fn with(self, code: Code, on: bool) -> Lints {
        let bit = 1 << code.0;
        Lints(if on { self.0 | bit } else { self.0 & !bit })
    }

    /// Whether the lint of the code `name` is on; a name that is no lint's is.
    pub fn is_on(self, name: &str) -> bool {
        Lints::code(name).is_none_or(|c| self.0 & (1 << c.0) != 0)
    }

    /// Drops the warnings of the lints that are off from `diagnostics`.
    pub fn drop_from(self, diagnostics: &mut Diagnostics) {
        if self != Lints::ALL {
            diagnostics.drop_lints(|code| !self.is_on(code));
        }
    }
}

/// One lint option of the command line, in the order given.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliLint {
    /// `--lints on|off`.
    All(bool),
    /// `--lint <code>=on|off`.
    One(Code, bool),
}

/// The value of `--lints`.
pub fn parse_lints_option(value: &str) -> Result<CliLint, String> {
    on_off(value)
        .map(CliLint::All)
        .ok_or_else(|| format!("--lints takes on or off, not `{value}`"))
}

/// The value of `--lint`.
pub fn parse_lint_option(value: &str) -> Result<CliLint, String> {
    let Some((name, state)) = value.split_once('=') else {
        return Err(format!("--lint takes <code>=on|off, not `{value}`"));
    };
    let Some(code) = Lints::code(name) else {
        return Err(format!("unknown lint `{name}`; the lints are {}", Lints::names()));
    };
    let on = on_off(state).ok_or_else(|| format!("--lint {name}= takes on or off, not `{state}`"))?;
    Ok(CliLint::One(code, on))
}

/// The command line over the file's set: the last `--lints` replaces it, then each `--lint` in order sets its lint.
pub fn apply_cli(base: Lints, cli: &[CliLint]) -> Lints {
    let all = cli.iter().rev().find_map(|c| match c {
        CliLint::All(on) => Some(*on),
        CliLint::One(..) => None,
    });
    let base = match all {
        Some(true) => Lints::ALL,
        Some(false) => Lints::OFF,
        None => base,
    };
    cli.iter().fold(base, |s, c| match c {
        CliLint::One(code, on) => s.with(*code, *on),
        CliLint::All(_) => s,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_code_is_a_bit() {
        assert_eq!(CODES.len(), 9);
        for c in CODES {
            assert!(Lints::ALL.is_on(c) && !Lints::OFF.is_on(c), "{c}");
            let code = Lints::code(c).unwrap();
            assert!(!Lints::ALL.with(code, false).is_on(c));
            assert!(Lints::OFF.with(code, true).is_on(c));
        }
        assert!(
            Lints::OFF.is_on("not-a-lint"),
            "a name that is no lint's is never dropped"
        );
        assert_eq!(Lints::default(), Lints::ALL);
    }

    #[test]
    fn lints_command_line_over_the_file() {
        let one = |s| parse_lint_option(s).unwrap();
        let all = |s| parse_lints_option(s).unwrap();
        let label = Lints::code("unused-label").unwrap();
        assert_eq!(apply_cli(Lints::OFF, &[]), Lints::OFF);
        assert_eq!(apply_cli(Lints::OFF, &[all("on")]), Lints::ALL, "on again for one run");
        assert_eq!(
            apply_cli(Lints::ALL, &[one("unused-label=off")]),
            Lints::ALL.with(label, false)
        );
        assert_eq!(
            apply_cli(Lints::ALL, &[one("unused-label=on"), all("off")]),
            Lints::OFF.with(label, true),
            "--lint applies after --lints, wherever it stands"
        );
        assert!(parse_lints_option("yes").is_err());
        assert!(parse_lint_option("unused-label").is_err());
        assert!(parse_lint_option("unused-label=false").is_err());
        let e = parse_lint_option("unused-labl=off").unwrap_err();
        assert!(
            e.starts_with("unknown lint `unused-labl`; the lints are used-once,"),
            "{e}"
        );
    }
}
