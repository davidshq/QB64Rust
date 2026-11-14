//! The fixes of a build (spec `compiler/fixes`; OpenSpec change `fixes-switch`, design D1, D2): the `[fixes]` table
//! of the nearest `qb64rust.toml` above the main file (read by `settings.rs`), then the command line's `--fixes` and
//! `--fix`.

use qb64rust_base::fixes::{Fix, Fixes};

/// One fix option of the command line, in the order given.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CliFix {
    /// `--fixes on|off`.
    All(bool),
    /// `--fix <name>=on|off`.
    One(Fix, bool),
}

/// The value of `--fixes`.
pub fn parse_fixes_option(value: &str) -> Result<CliFix, String> {
    on_off(value)
        .map(CliFix::All)
        .ok_or_else(|| format!("--fixes takes on or off, not `{value}`"))
}

/// The value of `--fix`.
pub fn parse_fix_option(value: &str) -> Result<CliFix, String> {
    let Some((name, state)) = value.split_once('=') else {
        return Err(format!("--fix takes <name>=on|off, not `{value}`"));
    };
    let Some(fix) = Fix::from_name(name) else {
        return Err(format!("unknown fix `{name}`; the fixes are {}", Fix::names()));
    };
    let on = on_off(state).ok_or_else(|| format!("--fix {name}= takes on or off, not `{state}`"))?;
    Ok(CliFix::One(fix, on))
}

/// `on` or `off`, the value of `--fixes`, `--lints` and their single options.
pub(crate) fn on_off(s: &str) -> Option<bool> {
    match s {
        "on" => Some(true),
        "off" => Some(false),
        _ => None,
    }
}

/// The command line over the file's set: the last `--fixes` replaces it, then each `--fix` in order sets its fix.
pub fn apply_cli(base: Fixes, cli: &[CliFix]) -> Fixes {
    let all = cli.iter().rev().find_map(|c| match c {
        CliFix::All(on) => Some(*on),
        CliFix::One(..) => None,
    });
    let base = match all {
        Some(true) => Fixes::ALL,
        Some(false) => Fixes::OFF,
        None => base,
    };
    cli.iter().fold(base, |s, c| match c {
        CliFix::One(fix, on) => s.with(*fix, *on),
        CliFix::All(_) => s,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_command_line_over_the_file() {
        let one = |s| parse_fix_option(s).unwrap();
        let all = |s| parse_fixes_option(s).unwrap();
        assert_eq!(apply_cli(Fixes::ALL, &[]), Fixes::ALL);
        assert_eq!(
            apply_cli(Fixes::ALL, &[all("off")]),
            Fixes::OFF,
            "off overrides the file"
        );
        let only = Fixes::OFF.with(Fix::WholeArrayValue, true);
        assert_eq!(apply_cli(Fixes::ALL, &[all("off"), one("whole-array-value=on")]), only);
        assert_eq!(
            apply_cli(Fixes::ALL, &[one("whole-array-value=on"), all("off")]),
            only,
            "--fix applies after --fixes, wherever it stands"
        );
        assert_eq!(
            apply_cli(Fixes::OFF, &[all("off"), all("on")]),
            Fixes::ALL,
            "the last --fixes"
        );
        assert_eq!(
            apply_cli(Fixes::OFF, &[one("dim-empty-error=on"), one("dim-empty-error=off")]),
            Fixes::OFF
        );
        assert!(parse_fixes_option("yes").is_err());
        assert!(parse_fix_option("whole-array-value").is_err());
        assert!(parse_fix_option("whole-array-value=true").is_err());
        let e = parse_fix_option("nonsense=on").unwrap_err();
        assert!(e.contains("unknown fix `nonsense`"), "{e}");
    }
}
