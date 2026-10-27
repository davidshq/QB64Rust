//! The fixes switch (spec `compiler/fixes`; OpenSpec change `fixes-switch`, design D1): named fixes of QB64pe
//! behaviours the compiler keeps by default. The driver works out one set per build from `qb64rust.toml` and the
//! command line; `sema`, the IR and `codegen-cpp` read it from the program. A fix that is off changes nothing.

/// One fix. Its name never changes once released (`DECISIONS.md` 2026-10-27).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Fix {
    /// `study\39` row 1.
    ResumeAfterBlock,
    /// Row 3.
    DimEmptyError,
    /// Row 4.
    StaticLocalDynamic,
    /// Row 6.
    WholeArrayValue,
    /// Row 13.
    ResumeInProcedure,
    /// Row 14.
    RunInProcedure,
    /// Row 19.
    CheckingOnService,
}

/// Every fix, in the order `qb64rust --list-fixes` prints them, with its name and a line on what it changes.
pub const FIXES: [(Fix, &str, &str); 7] = [
    (
        Fix::ResumeAfterBlock,
        "resume-after-block",
        "RESUME NEXT after an error in a block's condition goes on after the block",
    ),
    (
        Fix::DimEmptyError,
        "dim-empty-error",
        "DIM or REDIM with no element, or with a bound that raises, is error 5 and allocates nothing",
    ),
    (
        Fix::StaticLocalDynamic,
        "static-local-dynamic",
        "STATIC a(n) in a SUB beside a shared dynamic array declares the SUB's own array",
    ),
    (
        Fix::WholeArrayValue,
        "whole-array-value",
        "a whole array a() used as a value is a compile error",
    ),
    (
        Fix::ResumeInProcedure,
        "resume-in-procedure",
        "RESUME in a SUB or FUNCTION is a compile error",
    ),
    (
        Fix::RunInProcedure,
        "run-in-procedure",
        "RUN in a SUB or FUNCTION is a compile error",
    ),
    (
        Fix::CheckingOnService,
        "checking-on-service",
        "an error raised under $CHECKING:OFF is handled before the first checked statement",
    ),
];

impl Fix {
    /// The fix's name, as written in `qb64rust.toml` and after `--fix`.
    pub fn name(self) -> &'static str {
        FIXES[self.index()].1
    }

    /// The line `--list-fixes` prints after the name.
    pub fn description(self) -> &'static str {
        FIXES[self.index()].2
    }

    /// The fix of a name, if it is one.
    pub fn from_name(name: &str) -> Option<Fix> {
        FIXES.iter().find(|f| f.1 == name).map(|f| f.0)
    }

    /// The known names, comma-separated, for an error message.
    pub fn names() -> String {
        FIXES.iter().map(|f| f.1).collect::<Vec<_>>().join(", ")
    }

    fn index(self) -> usize {
        match self {
            Fix::ResumeAfterBlock => 0,
            Fix::DimEmptyError => 1,
            Fix::StaticLocalDynamic => 2,
            Fix::WholeArrayValue => 3,
            Fix::ResumeInProcedure => 4,
            Fix::RunInProcedure => 5,
            Fix::CheckingOnService => 6,
        }
    }

    fn bit(self) -> u32 {
        1 << self.index()
    }
}

/// A set of fixes, one bit per fix.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Fixes(u32);

impl Fixes {
    /// Every fix off: QB64pe's behaviour.
    pub const OFF: Fixes = Fixes(0);
    /// Every fix on.
    pub const ALL: Fixes = Fixes((1 << FIXES.len()) - 1);

    pub fn is_on(self, fix: Fix) -> bool {
        self.0 & fix.bit() != 0
    }

    /// The set with one fix turned on or off.
    #[must_use]
    pub fn with(self, fix: Fix, on: bool) -> Fixes {
        if on {
            Fixes(self.0 | fix.bit())
        } else {
            Fixes(self.0 & !fix.bit())
        }
    }

    /// The set as bits, for the build cache's key.
    pub fn bits(self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_names_round_trip() {
        for (i, &(fix, name, description)) in FIXES.iter().enumerate() {
            assert_eq!(fix.index(), i);
            assert_eq!(fix.name(), name);
            assert_eq!(fix.description(), description);
            assert_eq!(Fix::from_name(name), Some(fix));
            assert!(
                name.bytes().all(|b| b.is_ascii_lowercase() || b == b'-'),
                "{name} is kebab-case"
            );
        }
        assert_eq!(Fix::from_name("all"), None);
        assert_eq!(Fix::from_name("resume-after-blok"), None);
        assert_eq!(FIXES[0].1, "resume-after-block");
        assert!(Fix::names().starts_with("resume-after-block, dim-empty-error"));
    }

    #[test]
    fn fixes_set() {
        assert!(FIXES.iter().all(|f| !Fixes::OFF.is_on(f.0) && Fixes::ALL.is_on(f.0)));
        let one = Fixes::OFF.with(Fix::WholeArrayValue, true);
        assert!(one.is_on(Fix::WholeArrayValue));
        assert!(!one.is_on(Fix::ResumeAfterBlock));
        assert_eq!(one.with(Fix::WholeArrayValue, false), Fixes::OFF);
        let most = Fixes::ALL.with(Fix::ResumeAfterBlock, false);
        assert!(!most.is_on(Fix::ResumeAfterBlock));
        assert!(most.is_on(Fix::CheckingOnService));
        assert_eq!(Fixes::default(), Fixes::OFF);
        assert_ne!(Fixes::ALL.bits(), most.bits());
    }
}
