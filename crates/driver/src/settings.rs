//! The settings file `qb64rust.toml` of a program (OpenSpec change `project-settings`, design D1): the nearest one
//! above the main file, read once for its `[fixes]` (spec `compiler/fixes`), `[format]` (spec `compiler/format`)
//! and `[lints]` (spec `compiler/lints`) tables, and for `[project]` `main` (the language server's). Any other
//! table is ignored. The command line's options go over the file: `fixes.rs`, `lints.rs`, and `main.rs` for the
//! layout settings.

use crate::lints::Lints;
use qb64rust_base::fixes::{Fix, Fixes};
use qb64rust_base::layout::{KeywordStyle, Overrides};
use std::fmt;
use std::path::{Path, PathBuf};
use toml::de::DeTable;

/// The settings file's name.
pub const SETTINGS_FILE: &str = "qb64rust.toml";

/// A settings file that cannot be used: the program is not built, nor laid out (spec `compiler/fixes`, "A bad
/// settings file"; spec `compiler/format`, "Program with an error").
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

/// What a settings file gives a program. [`Default`] is the program with no settings file: every fix off, no layout
/// setting, every lint on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Project {
    pub fixes: Fixes,
    pub format: Overrides,
    pub lints: Lints,
}

/// The first `qb64rust.toml` in the folder of the main file `main` or a folder above it.
pub fn find_settings(main: &Path) -> Option<PathBuf> {
    let main = std::path::absolute(main).ok()?;
    main.ancestors()
        .skip(1)
        .map(|dir| dir.join(SETTINGS_FILE))
        .find(|f| f.is_file())
}

/// The settings of the program whose main file is `main` on disk, and the file read if one was. With
/// `no_settings` (`--no-settings`, spec `compiler/cli`), no file is looked for.
pub fn load(main: &Path, no_settings: bool) -> Result<(Project, Option<PathBuf>), SettingsError> {
    if no_settings {
        return Ok((Project::default(), None));
    }
    match find_settings(main) {
        Some(file) => Ok((read_settings(&file)?, Some(file))),
        None => Ok((Project::default(), None)),
    }
}

/// The main program the nearest settings file names for the document at `doc` (design D7 of
/// `include-file-program`): the `main` entry of its `[project]` table, joined to the settings file's folder. `None`
/// for no settings file, no such table or entry, a value that is not a string, a file that is not valid TOML, and
/// a main file that does not exist. The language server's hook `qb64rust_lsp::MainOf`.
pub fn main_of(doc: &Path) -> Option<PathBuf> {
    let settings = find_settings(doc)?;
    let main = project_main(&std::fs::read(&settings).ok()?)?;
    let path = settings.parent()?.join(main);
    path.is_file().then_some(path)
}

/// The string of `[project]` `main` in a settings file's bytes.
fn project_main(bytes: &[u8]) -> Option<String> {
    #[expect(
        clippy::disallowed_methods,
        reason = "a settings file is TOML, which is UTF-8 by its specification; not BASIC source"
    )]
    let text = std::str::from_utf8(bytes).ok()?;
    let doc = DeTable::parse(text).ok()?;
    let project = doc.get_ref().get("project")?;
    let main = project.get_ref().as_table()?.get("main")?;
    main.get_ref().as_str().map(str::to_string)
}

/// The settings a file gives.
pub fn read_settings(path: &Path) -> Result<Project, SettingsError> {
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
fn parse_settings(bytes: &[u8]) -> Result<Project, (u32, String)> {
    #[expect(
        clippy::disallowed_methods,
        reason = "a settings file is TOML, which is UTF-8 by its specification; not BASIC source"
    )]
    let text = std::str::from_utf8(bytes).map_err(|e| {
        let line = line_of(bytes, e.valid_up_to());
        (line, "the file is not UTF-8 text".to_string())
    })?;
    let line = |at: usize| line_of(text.as_bytes(), at);
    let doc = DeTable::parse(text).map_err(|e| {
        let at = e.span().map_or(0, |s| s.start);
        let first = e.message().lines().next().unwrap_or("").trim_end();
        (line(at), format!("not valid TOML: {first}"))
    })?;
    let table = |name: &str| -> Result<Option<&DeTable<'_>>, (u32, String)> {
        let Some(value) = doc.get_ref().get(name) else {
            return Ok(None);
        };
        match value.get_ref().as_table() {
            Some(t) => Ok(Some(t)),
            None => Err((
                line(value.span().start),
                format!("`{name}` must be a table, as `[{name}]`"),
            )),
        }
    };
    let mut project = Project::default();
    if let Some(t) = table("fixes")? {
        let (all, ones) = switches(t, &line, ("fix", "fixes"), Fix::from_name, &Fix::names())?;
        let base = match all {
            Some(true) => Fixes::ALL,
            _ => Fixes::OFF,
        };
        project.fixes = ones.into_iter().fold(base, |s, (fix, on)| s.with(fix, on));
    }
    if let Some(t) = table("lints")? {
        let (all, ones) = switches(t, &line, ("lint", "lints"), Lints::code, &Lints::names())?;
        let base = match all {
            Some(false) => Lints::OFF,
            _ => Lints::ALL,
        };
        project.lints = ones.into_iter().fold(base, |s, (code, on)| s.with(code, on));
    }
    if let Some(t) = table("format")? {
        project.format = format_table(t, &line)?;
    }
    Ok(project)
}

/// The switches of a table: `all`'s value, if given, and each other key's in the order written.
type Switches<T> = (Option<bool>, Vec<(T, bool)>);

/// A table of switches, as `[fixes]` and `[lints]` are: `all` and one key per name, each `true` or `false`, in any
/// order. The caller applies `all` first. `(kind, kinds)` names a key in an error, one and many (`fix`, `fixes`),
/// and `names` lists the known ones.
fn switches<T>(
    table: &DeTable<'_>,
    line: &dyn Fn(usize) -> u32,
    (kind, kinds): (&str, &str),
    from_name: impl Fn(&str) -> Option<T>,
    names: &str,
) -> Result<Switches<T>, (u32, String)> {
    let mut all = None;
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
            all = Some(on);
        } else if let Some(one) = from_name(name) {
            ones.push((one, on));
        } else {
            return Err((
                line(key.span().start),
                format!("unknown {kind} `{name}`; the {kinds} are `all`, {names}"),
            ));
        }
    }
    Ok((all, ones))
}

/// The keys of `[format]`, in the order an error lists them.
const FORMAT_KEYS: &str =
    "auto-indent, indent-size, indent-subs, auto-layout, keyword-case, one-statement-per-line, block-if";

/// The `[format]` table (spec `compiler/format`, "Format table of the settings file" and "A bad format table").
fn format_table(table: &DeTable<'_>, line: &dyn Fn(usize) -> u32) -> Result<Overrides, (u32, String)> {
    let mut o = Overrides::default();
    for (key, value) in table {
        let name: &str = key.get_ref();
        let at = line(value.span().start);
        let v = value.get_ref();
        let boolean = || {
            v.as_bool()
                .ok_or_else(|| (at, format!("the value of `{name}` must be true or false")))
        };
        match name {
            "auto-indent" => o.auto_indent = Some(boolean()?),
            "indent-subs" => o.indent_subs = Some(boolean()?),
            "auto-layout" => o.auto_layout = Some(boolean()?),
            "one-statement-per-line" => o.one_statement_per_line = Some(boolean()?),
            "block-if" => o.block_if = Some(boolean()?),
            "indent-size" => {
                let size = v
                    .as_integer()
                    .and_then(|i| i64::from_str_radix(i.as_str(), i.radix()).ok())
                    .ok_or_else(|| (at, format!("the value of `{name}` must be a whole number")))?;
                o.indent_size = Some(
                    u32::try_from(size)
                        .ok()
                        .filter(|s| (1..=64).contains(s))
                        .ok_or_else(|| (at, format!("`{name}` must be in the range 1 to 64, not {size}")))?,
                );
            }
            "keyword-case" => {
                let words = "\"camel\", \"upper\" or \"lower\"";
                let Some(case) = v.as_str() else {
                    return Err((at, format!("the value of `{name}` must be {words}")));
                };
                o.keywords = Some(match case {
                    "camel" => KeywordStyle::Camel,
                    "upper" => KeywordStyle::Upper,
                    "lower" => KeywordStyle::Lower,
                    _ => return Err((at, format!("`{name}` must be {words}, not \"{case}\""))),
                });
            }
            _ => {
                return Err((
                    line(key.span().start),
                    format!("unknown format setting `{name}`; the settings are {FORMAT_KEYS}"),
                ));
            }
        }
    }
    Ok(o)
}

/// The 1-based line of byte offset `at`.
fn line_of(bytes: &[u8], at: usize) -> u32 {
    let at = at.min(bytes.len());
    qb64rust_base::to_u32(bytes[..at].iter().filter(|&&b| b == b'\n').count() + 1)
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

    fn parse(text: &str) -> Result<Project, (u32, String)> {
        parse_settings(text.as_bytes())
    }

    fn fixes(text: &str) -> Result<Fixes, (u32, String)> {
        parse(text).map(|p| p.fixes)
    }

    fn with_fixes(fixes: Fixes) -> Project {
        Project {
            fixes,
            ..Project::default()
        }
    }

    #[test]
    fn fixes_found_walking_up() {
        let dir = tempfile::tempdir().unwrap();
        let main = write(dir.path(), "proj/src/p.bas", "PRINT 1\n");
        assert_eq!(find_settings(&main), None, "no file");
        assert_eq!(load(&main, false).unwrap(), (Project::default(), None));
        let above = write(dir.path(), "proj/qb64rust.toml", "[fixes]\nall = true\n");
        assert_eq!(find_settings(&main), Some(above.clone()), "found in a folder above");
        assert_eq!(load(&main, false).unwrap(), (with_fixes(Fixes::ALL), Some(above)));
        let near = write(dir.path(), "proj/src/qb64rust.toml", "[fixes]\nall = false\n");
        assert_eq!(find_settings(&main), Some(near.clone()), "the nearest file wins");
        assert_eq!(load(&main, false).unwrap(), (Project::default(), Some(near)));
    }

    /// `--no-settings` (spec `compiler/cli`, "No settings file"): no file is looked for, a bad one included.
    #[test]
    fn no_settings_reads_no_file() {
        let dir = tempfile::tempdir().unwrap();
        let main = write(dir.path(), "p.bas", "PRINT 1\n");
        write(dir.path(), "qb64rust.toml", "[fixes\n");
        assert!(load(&main, false).is_err());
        assert_eq!(load(&main, true).unwrap(), (Project::default(), None));
    }

    /// `[project]` `main` (design D7 of `include-file-program`): a `main`, no table, a value that is not a string,
    /// a missing file, and a settings file one folder up, whose `main` is relative to its own folder.
    #[test]
    fn main_of_the_settings_file() {
        let dir = tempfile::tempdir().unwrap();
        let doc = write(dir.path(), "proj/src/lib.bm", "SUB a\nEND SUB\n");
        assert_eq!(main_of(&doc), None, "no settings file");
        let game = write(dir.path(), "proj/src/game.bas", "PRINT 1\n");
        write(dir.path(), "proj/src/qb64rust.toml", "[project]\nmain = \"game.bas\"\n");
        assert_eq!(main_of(&doc), Some(dir.path().join("proj/src").join("game.bas")));
        assert!(main_of(&doc).unwrap().is_file());
        write(dir.path(), "proj/src/qb64rust.toml", "[fixes]\nall = true\n");
        assert_eq!(main_of(&doc), None, "no [project] table");
        write(dir.path(), "proj/src/qb64rust.toml", "[project]\nmain = 1\n");
        assert_eq!(main_of(&doc), None, "not a string");
        write(dir.path(), "proj/src/qb64rust.toml", "[project]\nmain = \"none.bas\"\n");
        assert_eq!(main_of(&doc), None, "a missing file");
        std::fs::remove_file(dir.path().join("proj/src/qb64rust.toml")).unwrap();
        write(dir.path(), "proj/qb64rust.toml", "[project]\nmain = \"src/game.bas\"\n");
        assert_eq!(
            main_of(&doc),
            Some(dir.path().join("proj").join("src/game.bas")),
            "one folder up"
        );
        assert_eq!(
            std::fs::read(main_of(&doc).unwrap()).unwrap(),
            std::fs::read(game).unwrap()
        );
    }

    #[test]
    fn fixes_file_contents() {
        assert_eq!(fixes(""), Ok(Fixes::OFF), "no [fixes]");
        assert_eq!(fixes("[other]\nx = 1\n"), Ok(Fixes::OFF), "other tables are ignored");
        assert_eq!(fixes("[fixes]\n"), Ok(Fixes::OFF));
        assert_eq!(
            fixes("[fixes]\nresume-after-block = false # off\nall = true\n"),
            Ok(Fixes::ALL.with(Fix::ResumeAfterBlock, false)),
            "all plus an override, in any order"
        );
        assert_eq!(
            fixes("fixes.whole-array-value = true\n"),
            Ok(Fixes::OFF.with(Fix::WholeArrayValue, true)),
            "a dotted key"
        );
        assert_eq!(
            fixes("[fixes]\n\"dim-empty-error\" = true\n"),
            Ok(Fixes::OFF.with(Fix::DimEmptyError, true)),
            "a quoted key"
        );
    }

    #[test]
    fn fixes_file_errors() {
        let (line, msg) = parse("[fixes]\nresume-after-blok = true\n").unwrap_err();
        assert_eq!(line, 2);
        assert!(
            msg.starts_with(
                "unknown fix `resume-after-blok`; the fixes are `all`, resume-after-block, dim-empty-error"
            ),
            "{msg}"
        );
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

    fn format(text: &str) -> Result<Overrides, (u32, String)> {
        parse(text).map(|p| p.format)
    }

    #[test]
    fn format_file_contents() {
        assert_eq!(format("[fixes]\nall = true\n"), Ok(Overrides::default()), "no [format]");
        assert_eq!(
            format(
                "[format]\nauto-indent = false\nindent-size = 2\nindent-subs = false\nauto-layout = false\n\
                 keyword-case = \"upper\"\none-statement-per-line = true\nblock-if = false\n"
            ),
            Ok(Overrides {
                auto_indent: Some(false),
                indent_size: Some(2),
                indent_subs: Some(false),
                auto_layout: Some(false),
                keywords: Some(KeywordStyle::Upper),
                one_statement_per_line: Some(true),
                block_if: Some(false),
            }),
            "each key"
        );
        assert_eq!(
            format("[format]\nblock-if = true\n"),
            Ok(Overrides {
                block_if: Some(true),
                ..Overrides::default()
            }),
            "one rewrite key"
        );
        assert_eq!(
            format("format.keyword-case = \"lower\"\n"),
            Ok(Overrides {
                keywords: Some(KeywordStyle::Lower),
                ..Overrides::default()
            }),
            "a dotted key"
        );
        assert_eq!(
            format("[format]\nindent-size = 0x40\n").map(|o| o.indent_size),
            Ok(Some(64)),
            "a TOML integer in hex"
        );
        assert_eq!(
            format("[format]\nkeyword-case = \"camel\"\n").map(|o| o.keywords),
            Ok(Some(KeywordStyle::Camel))
        );
    }

    #[test]
    fn format_file_errors() {
        let (line, msg) = format("[format]\nindent = 2\n").unwrap_err();
        assert_eq!(line, 2);
        assert_eq!(
            msg,
            "unknown format setting `indent`; the settings are auto-indent, indent-size, indent-subs, \
             auto-layout, keyword-case, one-statement-per-line, block-if"
        );
        let (line, msg) = format("[format]\n\nindent-size = 65\n").unwrap_err();
        assert_eq!(
            (line, msg.as_str()),
            (3, "`indent-size` must be in the range 1 to 64, not 65")
        );
        let (_, msg) = format("[format]\nindent-size = 0\n").unwrap_err();
        assert!(msg.contains("range 1 to 64"), "{msg}");
        let (_, msg) = format("[format]\nindent-size = -1\n").unwrap_err();
        assert!(msg.contains("range 1 to 64"), "{msg}");
        let (_, msg) = format("[format]\nindent-size = 2.5\n").unwrap_err();
        assert_eq!(msg, "the value of `indent-size` must be a whole number");
        let (line, msg) = format("[format]\nkeyword-case = \"uper\"\n").unwrap_err();
        assert_eq!(
            (line, msg.as_str()),
            (
                2,
                "`keyword-case` must be \"camel\", \"upper\" or \"lower\", not \"uper\""
            )
        );
        let (_, msg) = format("[format]\nkeyword-case = 1\n").unwrap_err();
        assert_eq!(
            msg,
            "the value of `keyword-case` must be \"camel\", \"upper\" or \"lower\""
        );
        let (_, msg) = format("[format]\nauto-layout = \"yes\"\n").unwrap_err();
        assert_eq!(msg, "the value of `auto-layout` must be true or false");
        let (line, msg) = format("[format]\nblock-if = \"yes\"\n").unwrap_err();
        assert_eq!(
            (line, msg.as_str()),
            (2, "the value of `block-if` must be true or false")
        );
        let (_, msg) = format("[format]\none-statement-per-line = 1\n").unwrap_err();
        assert_eq!(msg, "the value of `one-statement-per-line` must be true or false");
        let (line, msg) = format("format = \"upper\"\n").unwrap_err();
        assert_eq!((line, msg.as_str()), (1, "`format` must be a table, as `[format]`"));
    }

    fn lints(text: &str) -> Result<Lints, (u32, String)> {
        parse(text).map(|p| p.lints)
    }

    #[test]
    fn lints_file_contents() {
        let code = |c| Lints::code(c).unwrap();
        assert_eq!(lints(""), Ok(Lints::ALL), "no [lints]: every lint on");
        assert_eq!(lints("[lints]\n"), Ok(Lints::ALL));
        assert_eq!(
            lints("[lints]\nunused-label = false\n"),
            Ok(Lints::ALL.with(code("unused-label"), false))
        );
        assert_eq!(
            lints("[lints]\nused-once = true\nall = false\n"),
            Ok(Lints::OFF.with(code("used-once"), true)),
            "all off but one, in any order"
        );
        assert_eq!(lints("lints.all = false\n"), Ok(Lints::OFF), "a dotted key");
    }

    #[test]
    fn lints_file_errors() {
        let (line, msg) = parse("[lints]\nunused-labl = false\n").unwrap_err();
        assert_eq!(line, 2);
        assert!(
            msg.starts_with("unknown lint `unused-labl`; the lints are `all`, used-once,"),
            "{msg}"
        );
        assert!(msg.ends_with("debug-incompatible"), "{msg}");
        let (line, msg) = parse("[lints]\nunreachable = 0\n").unwrap_err();
        assert_eq!(
            (line, msg.as_str()),
            (2, "the value of `unreachable` must be true or false")
        );
    }
}
