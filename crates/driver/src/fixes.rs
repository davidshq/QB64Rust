//! The fixes of a build (spec `compiler/fixes`; OpenSpec change `fixes-switch`, design D1, D2): the `[fixes]` table
//! of the nearest `qb64rust.toml` above the main file, then the command line's `--fixes` and `--fix`.

use qb64rust_base::fixes::{Fix, Fixes};
use std::fmt;
use std::path::{Path, PathBuf};

/// The settings file's name.
pub const SETTINGS_FILE: &str = "qb64rust.toml";

/// A settings file that cannot be used: the program is not built (spec `compiler/fixes`, "A bad settings file").
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SettingsError {
    pub file: PathBuf,
    /// The 1-based line the error is on; `None` when the file could not be read.
    pub line: Option<u32>,
    pub message: String,
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.line {
            Some(line) => write!(f, "{}:{line}: {}", self.file.display(), self.message),
            None => write!(f, "{}: {}", self.file.display(), self.message),
        }
    }
}

/// The first `qb64rust.toml` in the folder of the main file `main` or a folder above it.
pub fn find_settings(main: &Path) -> Option<PathBuf> {
    let main = std::path::absolute(main).ok()?;
    main.ancestors()
        .skip(1)
        .map(|dir| dir.join(SETTINGS_FILE))
        .find(|f| f.is_file())
}

/// The fixes a settings file turns on: every fix off when it has no `[fixes]` table.
pub fn read_settings(path: &Path) -> Result<Fixes, SettingsError> {
    let bytes = std::fs::read(path).map_err(|e| SettingsError {
        file: path.to_path_buf(),
        line: None,
        message: format!("cannot be read: {e}"),
    })?;
    parse_settings(&bytes).map_err(|(line, message)| SettingsError {
        file: path.to_path_buf(),
        line: Some(line),
        message,
    })
}

/// [`read_settings`] of a file's bytes; an error is its line and message.
fn parse_settings(bytes: &[u8]) -> Result<Fixes, (u32, String)> {
    #[expect(
        clippy::disallowed_methods,
        reason = "a settings file is TOML, which is UTF-8 by its specification; not BASIC source"
    )]
    let text = std::str::from_utf8(bytes).map_err(|e| {
        let line = line_of(bytes, e.valid_up_to());
        (line, "the file is not UTF-8 text".to_string())
    })?;
    let line = |at: usize| line_of(text.as_bytes(), at);
    let doc = toml::de::DeTable::parse(text).map_err(|e| {
        let at = e.span().map_or(0, |s| s.start);
        let first = e.message().lines().next().unwrap_or("").trim_end();
        (line(at), format!("not valid TOML: {first}"))
    })?;
    let Some(fixes) = doc.get_ref().get("fixes") else {
        return Ok(Fixes::OFF);
    };
    let Some(table) = fixes.get_ref().as_table() else {
        return Err((line(fixes.span().start), "`fixes` must be a table, as `[fixes]`".into()));
    };
    // `all` first, whatever its place in the table; then each fix's own setting.
    let mut set = Fixes::OFF;
    let mut ones = Vec::new();
    for (key, value) in table {
        let name: &str = key.get_ref();
        let Some(on) = value.get_ref().as_bool() else {
            return Err((
                line(value.span().start),
                format!("the value of `{name}` must be true or false"),
            ));
        };
        if name == "all" {
            set = if on { Fixes::ALL } else { Fixes::OFF };
        } else if let Some(fix) = Fix::from_name(name) {
            ones.push((fix, on));
        } else {
            return Err((
                line(key.span().start),
                format!("unknown fix `{name}`; the fixes are `all`, {}", Fix::names()),
            ));
        }
    }
    Ok(ones.into_iter().fold(set, |s, (fix, on)| s.with(fix, on)))
}

/// The 1-based line of byte offset `at`.
fn line_of(bytes: &[u8], at: usize) -> u32 {
    let at = at.min(bytes.len());
    qb64rust_base::to_u32(bytes[..at].iter().filter(|&&b| b == b'\n').count() + 1)
}

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

fn on_off(s: &str) -> Option<bool> {
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

/// The fixes of the program whose main file is `main` on disk, from its settings file alone, and the file read if
/// one was found.
pub fn for_program(main: &Path) -> Result<(Fixes, Option<PathBuf>), SettingsError> {
    match find_settings(main) {
        Some(file) => Ok((read_settings(&file)?, Some(file))),
        None => Ok((Fixes::OFF, None)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &Path, rel: &str, text: &str) -> PathBuf {
        let path = dir.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, text).unwrap();
        path
    }

    fn parse(text: &str) -> Result<Fixes, (u32, String)> {
        parse_settings(text.as_bytes())
    }

    #[test]
    fn fixes_found_walking_up() {
        let dir = tempfile::tempdir().unwrap();
        let main = write(dir.path(), "proj/src/p.bas", "PRINT 1\n");
        assert_eq!(find_settings(&main), None, "no file");
        assert_eq!(for_program(&main).unwrap(), (Fixes::OFF, None));
        let above = write(dir.path(), "proj/qb64rust.toml", "[fixes]\nall = true\n");
        assert_eq!(find_settings(&main), Some(above.clone()), "found in a folder above");
        assert_eq!(for_program(&main).unwrap(), (Fixes::ALL, Some(above)));
        let near = write(dir.path(), "proj/src/qb64rust.toml", "[fixes]\nall = false\n");
        assert_eq!(find_settings(&main), Some(near.clone()), "the nearest file wins");
        assert_eq!(for_program(&main).unwrap(), (Fixes::OFF, Some(near)));
    }

    #[test]
    fn fixes_file_contents() {
        assert_eq!(parse(""), Ok(Fixes::OFF), "no [fixes]");
        assert_eq!(parse("[other]\nx = 1\n"), Ok(Fixes::OFF), "other tables are ignored");
        assert_eq!(parse("[fixes]\n"), Ok(Fixes::OFF));
        assert_eq!(
            parse("[fixes]\nresume-after-block = false # off\nall = true\n"),
            Ok(Fixes::ALL.with(Fix::ResumeAfterBlock, false)),
            "all plus an override, in any order"
        );
        assert_eq!(
            parse("fixes.whole-array-value = true\n"),
            Ok(Fixes::OFF.with(Fix::WholeArrayValue, true)),
            "a dotted key"
        );
        assert_eq!(
            parse("[fixes]\n\"dim-empty-error\" = true\n"),
            Ok(Fixes::OFF.with(Fix::DimEmptyError, true)),
            "a quoted key"
        );
    }

    #[test]
    fn fixes_file_errors() {
        let (line, msg) = parse("[fixes]\nresume-after-blok = true\n").unwrap_err();
        assert_eq!(line, 2);
        assert!(msg.contains("unknown fix `resume-after-blok`"), "{msg}");
        assert!(msg.contains("resume-after-block, dim-empty-error"), "{msg}");
        let (line, msg) = parse("[fixes]\n\nall = 1\n").unwrap_err();
        assert_eq!((line, msg.as_str()), (3, "the value of `all` must be true or false"));
        let (line, msg) = parse("[fixes]\nall = \"yes\"\n").unwrap_err();
        assert_eq!((line, msg.as_str()), (2, "the value of `all` must be true or false"));
        let (line, msg) = parse("[fixes]\nall = true\nall = false\n").unwrap_err();
        assert_eq!(line, 3, "{msg}");
        assert!(msg.starts_with("not valid TOML: "), "{msg}");
        let (line, msg) = parse("fixes = 1\n").unwrap_err();
        assert_eq!((line, msg.as_str()), (1, "`fixes` must be a table, as `[fixes]`"));
        let (line, msg) = parse_settings(b"[fixes]\nall = true # \xff\n").unwrap_err();
        assert_eq!((line, msg.as_str()), (2, "the file is not UTF-8 text"));

        let dir = tempfile::tempdir().unwrap();
        let file = write(dir.path(), "qb64rust.toml", "[fixes]\nnonsense = true\n");
        let e = read_settings(&file).unwrap_err();
        assert!(
            e.to_string().starts_with(&format!("{}:2: unknown fix", file.display())),
            "{e}"
        );
        let e = read_settings(&dir.path().join("missing.toml")).unwrap_err();
        assert_eq!(e.line, None);
    }

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
