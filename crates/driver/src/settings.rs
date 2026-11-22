//! The settings file `qb64rust.toml` of a program (OpenSpec change `project-settings`, design D1): the nearest one
//! above the main file, read once for its `[fixes]` (spec `compiler/fixes`), `[format]` (spec `compiler/format`)
//! and `[lints]` (spec `compiler/lints`) tables, for `[project]` `main` and `exclude` (the language server's), and
//! for `[build]` `output` and `copy` (spec `compiler/cli`, "The output folder"). Any other table is ignored. The command line's options go over the file: `fixes.rs`, `lints.rs`, and `main.rs` for the
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

/// Where a build without `-o` puts the program, and what it copies there: the `[build]` table, its paths joined
/// to the project's folder (design D1 and D2 of `build-output-folder`). Read beside [`Project`] so that `Project`
/// stays `Copy`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Build {
    /// The project's folder: the settings file's, or the main file's when there is no settings file.
    pub root: PathBuf,
    /// The output folder: `output` joined to `root`, `build` when the table does not set it.
    pub output: PathBuf,
    /// The entries of `copy`, in the order written.
    pub copy: Vec<CopyEntry>,
    /// `call_trace`, when the table sets it (OpenSpec change `call-trace`, design D7).
    pub call_trace: Option<bool>,
}

/// One entry of `[build]` `copy`: a file or a folder that a build copies into the output folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CopyEntry {
    /// The entry as written, such as `graphics`; its copy is at this path in the output folder.
    pub name: String,
    /// The entry joined to the project's folder.
    pub path: PathBuf,
    /// The 1-based line of the entry in the settings file, for an entry that is missing.
    pub line: u32,
}

/// The output folder's name when `[build]` does not set `output`.
pub const DEFAULT_OUTPUT: &str = "build";

impl Build {
    /// The table `table` of the project whose folder is `root`.
    fn new(root: &Path, table: &BuildTable) -> Build {
        // `absolute` takes out a `.`, so that `output = "."` is the settings file's folder itself.
        let output = root.join(table.output.as_deref().unwrap_or(DEFAULT_OUTPUT));
        Build {
            root: root.to_path_buf(),
            output: std::path::absolute(&output).unwrap_or(output),
            copy: table
                .copy
                .iter()
                .map(|(name, line)| CopyEntry {
                    name: name.clone(),
                    path: root.join(name),
                    line: *line,
                })
                .collect(),
            call_trace: table.call_trace,
        }
    }

    /// Whether a build has the call trace (spec `compiler/cli`, "When a build has the call trace"): `cli`, the
    /// command line's `--call-trace` or `--no-call-trace`, over the table's `call_trace`, over `debug`.
    pub fn call_trace(&self, cli: Option<bool>, debug: bool) -> bool {
        cli.or(self.call_trace).unwrap_or(debug)
    }

    /// A program with no settings file, whose main file is `main`: `build` in the main file's folder, nothing to
    /// copy.
    pub fn without_settings(main: &Path) -> Build {
        let main = std::path::absolute(main).unwrap_or_else(|_| main.to_path_buf());
        let root = main.parent().map_or_else(PathBuf::new, Path::to_path_buf);
        Build::new(&root, &BuildTable::default())
    }
}

/// The `[build]` table as written: `output`, and each `copy` entry with its line.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct BuildTable {
    output: Option<String>,
    copy: Vec<(String, u32)>,
    /// The line of the `copy` key, for an error about the list as a whole.
    copy_line: u32,
    call_trace: Option<bool>,
}

/// What [`parse_all`] reads from a settings file.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Parsed {
    project: Project,
    /// The patterns of `[project]` `exclude` ([`excluded`]).
    exclude: Vec<String>,
    build: BuildTable,
}

/// The first `qb64rust.toml` in the folder of the main file `main` or a folder above it.
pub fn find_settings(main: &Path) -> Option<PathBuf> {
    let main = std::path::absolute(main).ok()?;
    main.ancestors()
        .skip(1)
        .map(|dir| dir.join(SETTINGS_FILE))
        .find(|f| f.is_file())
}

/// The settings of the program whose main file is `main` on disk, its `[build]` table, and the file read if one
/// was. With `no_settings` (`--no-settings`, spec `compiler/cli`), no file is looked for.
pub fn load(main: &Path, no_settings: bool) -> Result<(Project, Build, Option<PathBuf>), SettingsError> {
    let none = || (Project::default(), Build::without_settings(main), None);
    if no_settings {
        return Ok(none());
    }
    match find_settings(main) {
        Some(file) => {
            let parsed = read_all(&file)?;
            let root = file.parent().map_or_else(PathBuf::new, Path::to_path_buf);
            let build = resolve_build(&root, &parsed.build).map_err(|(line, message)| SettingsError {
                file: file.clone(),
                line: Some(line),
                message,
            })?;
            Ok((parsed.project, build, Some(file)))
        }
        None => Ok(none()),
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

/// Whether the editor leaves the document at `doc` on disk alone: a pattern of the `[project]` table's `exclude` in
/// the nearest settings file matches its path from that file's folder ([`matches`]). For files in another
/// dialect, such as QBJS. False for no settings file, no such entry, and a file that cannot be used (the check
/// reports that file's error). The language server's hook `qb64rust_lsp::Excluded`; the compiler's command line
/// checks the entry's form but does not use it.
pub fn excluded(doc: &Path) -> bool {
    let Some(settings) = find_settings(doc) else {
        return false;
    };
    let Ok(Parsed { exclude: patterns, .. }) = std::fs::read(&settings)
        .map_err(|_| ())
        .and_then(|b| parse_all(&b).map_err(|_| ()))
    else {
        return false;
    };
    let (Some(folder), Ok(doc)) = (settings.parent(), std::path::absolute(doc)) else {
        return false;
    };
    let Ok(rel) = doc.strip_prefix(folder) else {
        return false;
    };
    let segments: Vec<String> = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    patterns.iter().any(|p| matches(p, &segments))
}

/// Whether the `exclude` pattern `pattern` matches the file whose path from the settings file's folder is
/// `segments`, or a folder that holds it. Patterns use `/` between folders on every system and start at the settings
/// file's folder. `*` matches any characters within one name, `?` one character, and `**` any number of folders,
/// none included. On Windows letter case is ignored, as the file system ignores it.
fn matches(pattern: &str, segments: &[String]) -> bool {
    let fold = |s: &str| if cfg!(windows) { s.to_lowercase() } else { s.to_string() };
    let pattern = fold(pattern);
    let parts: Vec<&str> = pattern
        .trim_start_matches("./")
        .split('/')
        .filter(|p| !p.is_empty())
        .collect();
    let segments: Vec<String> = segments.iter().map(|s| fold(s)).collect();
    // The file itself, or one of the folders it is in.
    (1..=segments.len()).any(|n| match_parts(&parts, &segments[..n]))
}

/// Whether the pattern's names `parts` match the path's names `path`, `**` taking any number of them.
fn match_parts(parts: &[&str], path: &[String]) -> bool {
    match parts.split_first() {
        None => path.is_empty(),
        Some((&"**", rest)) => (0..=path.len()).any(|skip| match_parts(rest, &path[skip..])),
        Some((part, rest)) => path
            .split_first()
            .is_some_and(|(name, more)| match_name(part.as_bytes(), name.as_bytes()) && match_parts(rest, more)),
    }
}

/// Whether one name of a pattern, with `*` and `?`, matches one name of a path.
fn match_name(pattern: &[u8], name: &[u8]) -> bool {
    match pattern.split_first() {
        None => name.is_empty(),
        Some((b'*', rest)) => (0..=name.len()).any(|skip| match_name(rest, &name[skip..])),
        Some((b'?', rest)) => {
            // One character, which may be several bytes of UTF-8.
            let len = name.first().map_or(0, |&b| utf8_len(b));
            len > 0 && match_name(rest, &name[len.min(name.len())..])
        }
        Some((c, rest)) => name.first() == Some(c) && match_name(rest, &name[1..]),
    }
}

/// The length in bytes of the UTF-8 character that starts with `first`.
fn utf8_len(first: u8) -> usize {
    match first {
        0xF0.. => 4,
        0xE0.. => 3,
        0xC0.. => 2,
        _ => 1,
    }
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
    read_all(path).map(|p| p.project)
}

/// All that a file gives ([`parse_all`]).
fn read_all(path: &Path) -> Result<Parsed, SettingsError> {
    let bytes = std::fs::read(path).map_err(|e| SettingsError {
        file: path.to_path_buf(),
        line: None,
        message: format!("cannot be read: {e}"),
    })?;
    parse_all(&bytes).map_err(|(line, message)| SettingsError {
        file: path.to_path_buf(),
        line: Some(line),
        message,
    })
}

/// [`read_settings`] of a file's bytes; an error is its line and message.
#[cfg(test)]
fn parse_settings(bytes: &[u8]) -> Result<Project, (u32, String)> {
    parse_all(bytes).map(|p| p.project)
}

/// The settings of a file's bytes, with the patterns of `[project]` `exclude` ([`excluded`]) and the `[build]`
/// table; an error is its line and message.
fn parse_all(bytes: &[u8]) -> Result<Parsed, (u32, String)> {
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
    // `[project]`'s `main` is read by `main_of`; its other keys are ignored.
    let mut exclude = Vec::new();
    if let Some(value) = table("project")?.and_then(|t| t.get("exclude")) {
        let list = "the value of `exclude` must be a list of patterns, as `exclude = [\"web/**\"]`";
        let items = value
            .get_ref()
            .as_array()
            .ok_or_else(|| (line(value.span().start), list.to_string()))?;
        for item in items {
            let at = line(item.span().start);
            let pattern = item.get_ref().as_str().ok_or_else(|| (at, list.to_string()))?;
            if pattern.contains('\\') {
                return Err((
                    at,
                    format!("`exclude` patterns use `/` between folders, not `\\`: \"{pattern}\""),
                ));
            }
            exclude.push(pattern.to_string());
        }
    }
    let build = match table("build")? {
        Some(t) => build_table(t, &line)?,
        None => BuildTable::default(),
    };
    Ok(Parsed {
        project,
        exclude,
        build,
    })
}

/// The `[build]` table (spec `compiler/cli`, "A bad build table"). The paths are checked as written, before they
/// are joined to the settings file's folder, so that the check is of the file alone.
fn build_table(table: &DeTable<'_>, line: &dyn Fn(usize) -> u32) -> Result<BuildTable, (u32, String)> {
    let mut out = BuildTable::default();
    let mut copy_line = 0;
    let slash = |at: u32, key: &str, path: &str| {
        if path.contains('\\') {
            Err((at, format!("`{key}` uses `/` between folders, not `\\`: \"{path}\"")))
        } else {
            Ok(())
        }
    };
    for (key, value) in table {
        let name: &str = key.get_ref();
        let at = line(value.span().start);
        match name {
            "output" => {
                let folder = "the value of `output` must be a folder, as `output = \"build\"`";
                let path = value.get_ref().as_str().ok_or_else(|| (at, folder.to_string()))?;
                if path.is_empty() {
                    return Err((
                        at,
                        "`output` must not be empty; `output = \".\"` builds into the settings file's folder".into(),
                    ));
                }
                slash(at, "output", path)?;
                out.output = Some(path.to_string());
            }
            "copy" => {
                let list = "the value of `copy` must be a list of files and folders, as `copy = [\"GAME.CFG\"]`";
                let items = value.get_ref().as_array().ok_or_else(|| (at, list.to_string()))?;
                copy_line = at;
                for item in items {
                    let at = line(item.span().start);
                    let path = item.get_ref().as_str().ok_or_else(|| (at, list.to_string()))?;
                    slash(at, "copy", path)?;
                    let outside = Path::new(path).has_root() || (cfg!(windows) && path.contains(':'));
                    if path.is_empty() || outside || parts(path).is_none() {
                        return Err((
                            at,
                            format!(
                                "a `copy` entry must be a file or folder in the settings file's folder: \"{path}\""
                            ),
                        ));
                    }
                    out.copy.push((path.to_string(), at));
                }
            }
            "call_trace" => {
                let switch = "the value of `call_trace` must be `true` or `false`";
                out.call_trace = Some(value.get_ref().as_bool().ok_or_else(|| (at, switch.to_string()))?);
            }
            _ => {
                return Err((
                    line(key.span().start),
                    format!("unknown build setting `{name}`; the settings are output, copy, call_trace"),
                ));
            }
        }
    }
    out.copy_line = copy_line;
    Ok(out)
}

/// The `[build]` table `table` of the settings file in the folder `root`, its paths joined to that folder, after
/// the check that needs the folder (spec `compiler/cli`, "A bad build table"): no `copy` entry is or holds the
/// output folder, which would copy the output into itself without end. The paths are compared once joined and
/// made plain, so that an `output` with `..` or an absolute one is held to the same rule.
fn resolve_build(root: &Path, table: &BuildTable) -> Result<Build, (u32, String)> {
    let build = Build::new(root, table);
    if build.copy.is_empty() {
        return Ok(build);
    }
    let output = plain(&build.output);
    if output == plain(root) {
        return Err((
            table.copy_line,
            "`copy` needs an output folder of its own: with the settings file's folder as the output folder \
             (`output = \".\"`), the files are already in the output folder"
                .into(),
        ));
    }
    for entry in &build.copy {
        let path = plain(&entry.path);
        if output.starts_with(&path) {
            let what = if path.len() == output.len() { "is" } else { "holds" };
            return Err((
                entry.line,
                format!(
                    "`{}` {what} the output folder, which a `copy` entry cannot be or hold",
                    entry.name
                ),
            ));
        }
    }
    Ok(build)
}

/// The names of `path` made plain without reading the disk: `.` left out, `..` taking off the name before it (never
/// the root), and letter case folded on Windows, as the file system ignores it.
fn plain(path: &Path) -> Vec<String> {
    use std::path::Component;
    let fold = |s: &std::ffi::OsStr| {
        let s = s.to_string_lossy();
        if cfg!(windows) {
            s.to_lowercase()
        } else {
            s.into_owned()
        }
    };
    let mut out: Vec<String> = Vec::new();
    let mut fixed = 0;
    for c in path.components() {
        match c {
            Component::Prefix(p) => {
                out.push(fold(p.as_os_str()));
                fixed = out.len();
            }
            Component::RootDir => {
                out.push("/".into());
                fixed = out.len();
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if out.len() > fixed {
                    out.pop();
                }
            }
            Component::Normal(name) => out.push(fold(name)),
        }
    }
    out
}

/// The names of a relative path written with `/`, without `.` and empty names, `..` taking off the name before it.
/// `None` for a path that goes above its start.
fn parts(path: &str) -> Option<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    for name in path.split('/') {
        match name {
            "" | "." => {}
            ".." => {
                out.pop()?;
            }
            _ => out.push(name.to_string()),
        }
    }
    Some(out)
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

    fn exclude(text: &str) -> Result<Vec<String>, (u32, String)> {
        parse_all(text.as_bytes()).map(|p| p.exclude)
    }

    /// `[project]` `exclude`: a list of strings, each without `\`; anything else is an error.
    #[test]
    fn project_exclude_contents() {
        assert_eq!(exclude(""), Ok(vec![]));
        assert_eq!(exclude("[project]\nmain = \"g.bas\"\n"), Ok(vec![]));
        assert_eq!(
            exclude("[project]\nexclude = [\"**\", \"web/*.bas\"]\n"),
            Ok(vec!["**".to_string(), "web/*.bas".to_string()])
        );
        let list = "the value of `exclude` must be a list of patterns, as `exclude = [\"web/**\"]`";
        assert_eq!(exclude("[project]\n\nexclude = \"web\"\n"), Err((3, list.to_string())));
        assert_eq!(
            exclude("[project]\nexclude = [\n  \"a\",\n  1,\n]\n"),
            Err((4, list.to_string()))
        );
        let (line, msg) = exclude("[project]\nexclude = ['web\\x.bas']\n").unwrap_err();
        assert_eq!(line, 2);
        assert!(msg.starts_with("`exclude` patterns use `/` between folders"), "{msg}");
    }

    fn path(p: &str) -> Vec<String> {
        p.split('/').map(str::to_string).collect()
    }

    /// The patterns: a file, a folder and what it holds, `*`, `?`, `**`, and letter case on Windows only.
    #[test]
    fn exclude_patterns() {
        assert!(matches("tools/qb2js.bas", &path("tools/qb2js.bas")));
        assert!(!matches("tools/qb2js.bas", &path("tools/other.bas")));
        assert!(matches("./tools/qb2js.bas", &path("tools/qb2js.bas")), "a leading ./");
        assert!(matches("test", &path("test/a/b.bas")), "a folder holds the file");
        assert!(matches("test/", &path("test/b.bas")), "a trailing /");
        assert!(
            !matches("test", &path("src/test/b.bas")),
            "from the settings file's folder only"
        );
        assert!(matches("**/test", &path("src/test/b.bas")));
        assert!(matches("**", &path("p.bas")), "the whole folder");
        assert!(matches("*.bas", &path("p.bas")));
        assert!(!matches("*.bas", &path("web/p.bas")), "`*` stays within a name");
        assert!(matches("**/*.bas", &path("web/p.bas")));
        assert!(matches("web/**/x.bas", &path("web/x.bas")), "`**` may be no folder");
        assert!(matches("web/**/x.bas", &path("web/a/b/x.bas")));
        assert!(matches("p?.bas", &path("pé.bas")), "`?` is one character");
        assert!(!matches("p?.bas", &path("p.bas")));
        assert_eq!(matches("Web/P.BAS", &path("web/p.bas")), cfg!(windows));
    }

    /// [`excluded`] from the nearest settings file; no file, no entry and a bad file exclude nothing.
    #[test]
    fn excluded_from_the_nearest_file() {
        let dir = tempfile::tempdir().unwrap();
        let doc = write(dir.path(), "qbjs/test/t.bas", "PRINT 1\n");
        let other = write(dir.path(), "qbjs/src/s.bas", "PRINT 1\n");
        assert!(!excluded(&doc), "no settings file");
        write(dir.path(), "qbjs/qb64rust.toml", "[project]\nexclude = [\"test\"]\n");
        assert!(excluded(&doc), "a folder above");
        assert!(!excluded(&other));
        write(dir.path(), "qbjs/test/qb64rust.toml", "[project]\nexclude = 1\n");
        assert!(!excluded(&doc), "the nearest file is bad");
        write(dir.path(), "qbjs/test/qb64rust.toml", "[lints]\nall = false\n");
        assert!(!excluded(&doc), "the nearest file has no entry");
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
        let project = |main: &Path| load(main, false).map(|(p, _, f)| (p, f));
        assert_eq!(find_settings(&main), None, "no file");
        assert_eq!(project(&main).unwrap(), (Project::default(), None));
        let above = write(dir.path(), "proj/qb64rust.toml", "[fixes]\nall = true\n");
        assert_eq!(find_settings(&main), Some(above.clone()), "found in a folder above");
        assert_eq!(project(&main).unwrap(), (with_fixes(Fixes::ALL), Some(above)));
        let near = write(dir.path(), "proj/src/qb64rust.toml", "[fixes]\nall = false\n");
        assert_eq!(find_settings(&main), Some(near.clone()), "the nearest file wins");
        assert_eq!(project(&main).unwrap(), (Project::default(), Some(near)));
    }

    fn build(text: &str) -> Result<BuildTable, (u32, String)> {
        parse_all(text.as_bytes()).map(|p| p.build)
    }

    /// The `[build]` table of `load` (design D1 and D2 of `build-output-folder`): no settings file, no table,
    /// `output` alone, `output = "."`, `copy`, a settings file one folder up, a dotted key, and `--no-settings`.
    #[test]
    fn build_table_of_a_program() {
        let dir = tempfile::tempdir().unwrap();
        let root = std::path::absolute(dir.path()).unwrap();
        let main = write(dir.path(), "g/game.bas", "PRINT 1\n");
        let g = root.join("g");
        let of = |main: &Path| load(main, false).unwrap().1;
        assert_eq!(
            of(&main),
            Build {
                root: g.clone(),
                output: g.join("build"),
                copy: vec![],
                call_trace: None,
            },
            "no settings file"
        );
        write(dir.path(), "g/qb64rust.toml", "[fixes]\nall = true\n");
        assert_eq!(of(&main).output, g.join("build"), "no table");
        write(dir.path(), "g/qb64rust.toml", "[build]\noutput = \"out/bin\"\n");
        assert_eq!(of(&main).output, g.join("out/bin"), "output alone");
        write(dir.path(), "g/qb64rust.toml", "[build]\noutput = \".\"\n");
        assert_eq!(of(&main).output, g, "output = \".\"");
        write(
            dir.path(),
            "g/qb64rust.toml",
            "[build]\n\ncopy = [\"GAME.CFG\", \"maps\"]\n",
        );
        let b = of(&main);
        assert_eq!(b.output, g.join("build"));
        assert_eq!(
            b.copy,
            vec![
                CopyEntry {
                    name: "GAME.CFG".into(),
                    path: g.join("GAME.CFG"),
                    line: 3
                },
                CopyEntry {
                    name: "maps".into(),
                    path: g.join("maps"),
                    line: 3
                },
            ],
            "copy"
        );
        assert_eq!(load(&main, true).unwrap().1.copy, vec![], "--no-settings");
        assert_eq!(load(&main, true).unwrap().1.output, g.join("build"));
        std::fs::remove_file(g.join("qb64rust.toml")).unwrap();
        let src = write(dir.path(), "proj/src/game.bas", "PRINT 1\n");
        write(dir.path(), "proj/qb64rust.toml", "build.output = \"b\"\n");
        let b = of(&src);
        assert_eq!(
            (b.root, b.output),
            (root.join("proj"), root.join("proj").join("b")),
            "a settings file one folder up, and a dotted key"
        );
    }

    /// Spec `compiler/cli`, "A bad build table": each error with its line.
    #[test]
    fn build_table_errors() {
        let err = |text: &str| build(text).unwrap_err();
        assert_eq!(
            err("[build]\nout = \"build\"\n"),
            (
                2,
                "unknown build setting `out`; the settings are output, copy, call_trace".to_string()
            )
        );
        let switch = "the value of `call_trace` must be `true` or `false`".to_string();
        assert_eq!(err("[build]\ncall_trace = \"yes\"\n"), (2, switch.clone()));
        assert_eq!(err("[build]\n\ncall_trace = 1\n"), (3, switch));
        let folder = "the value of `output` must be a folder, as `output = \"build\"`".to_string();
        assert_eq!(err("[build]\noutput = 1\n"), (2, folder));
        let (line, msg) = err("[build]\n\noutput = \"\"\n");
        assert_eq!(line, 3);
        assert!(msg.starts_with("`output` must not be empty"), "{msg}");
        let list = "the value of `copy` must be a list of files and folders, as `copy = [\"GAME.CFG\"]`";
        assert_eq!(err("[build]\ncopy = \"GAME.CFG\"\n"), (2, list.to_string()));
        assert_eq!(err("[build]\ncopy = [\n  \"a\",\n  2,\n]\n"), (4, list.to_string()));
        assert_eq!(
            err("[build]\noutput = 'out\\bin'\n"),
            (
                2,
                "`output` uses `/` between folders, not `\\`: \"out\\bin\"".to_string()
            )
        );
        assert_eq!(
            err("[build]\ncopy = ['data\\a.cfg']\n"),
            (
                2,
                "`copy` uses `/` between folders, not `\\`: \"data\\a.cfg\"".to_string()
            )
        );
        for outside in ["../shared", "/etc", "", "a/../.."] {
            let (line, msg) = err(&format!("[build]\ncopy = [\"{outside}\"]\n"));
            assert_eq!(line, 2);
            assert!(
                msg.starts_with("a `copy` entry must be a file or folder in the settings file's folder"),
                "{msg}"
            );
        }
        assert_eq!(
            err("build = 1\n"),
            (1, "`build` must be a table, as `[build]`".to_string())
        );
    }

    /// `call_trace` as written, and [`Build::call_trace`]'s order (spec `compiler/cli`, "When a build has the call
    /// trace"): the command line, then the settings file, then `--debug`.
    #[test]
    fn call_trace_setting() {
        assert_eq!(build("[build]\ncall_trace = true\n").unwrap().call_trace, Some(true));
        assert_eq!(build("[build]\ncall_trace = false\n").unwrap().call_trace, Some(false));
        assert_eq!(build("[build]\noutput = \"b\"\n").unwrap().call_trace, None);
        let with = |file| Build {
            call_trace: file,
            ..Build::without_settings(Path::new("g.bas"))
        };
        // No option and no setting: the call trace only in a debug build.
        assert!(!with(None).call_trace(None, false));
        assert!(with(None).call_trace(None, true));
        // The settings file over `--debug`.
        assert!(with(Some(true)).call_trace(None, false));
        assert!(!with(Some(false)).call_trace(None, true));
        // The command line over the settings file.
        assert!(!with(Some(true)).call_trace(Some(false), false));
        assert!(!with(None).call_trace(Some(false), true));
        assert!(with(Some(false)).call_trace(Some(true), false));
    }

    /// [`resolve_build`] of a settings file in the folder `proj`: no `copy` entry is or holds the output folder,
    /// with the paths joined and made plain, so that `..` and an absolute `output` cannot get past the check.
    #[test]
    fn copy_entries_never_hold_the_output_folder() {
        let root = std::path::absolute(std::env::temp_dir().join("qb64rust-settings").join("proj")).unwrap();
        let resolved = |text: &str| resolve_build(&root, &build(text).unwrap());
        let err = |text: &str| resolved(text).unwrap_err();
        assert_eq!(
            err("[build]\noutput = \"data/build\"\ncopy = [\"x\", \"data\"]\n"),
            (
                3,
                "`data` holds the output folder, which a `copy` entry cannot be or hold".to_string()
            )
        );
        let (line, msg) = err("[build]\ncopy = [\"build\"]\n");
        assert_eq!(line, 2);
        assert!(msg.starts_with("`build` is the output folder"), "{msg}");
        let (_, msg) = err("[build]\ncopy = [\".\"]\n");
        assert!(msg.starts_with("`.` holds the output folder"), "{msg}");
        let (_, msg) = err("[build]\ncopy = [\"data/..\"]\n");
        assert!(msg.starts_with("`data/..` holds the output folder"), "{msg}");
        // Up and back down again, and absolute: the same folder, so the same error.
        let (line, msg) = err("[build]\noutput = \"../proj/data/out\"\ncopy = [\"data\"]\n");
        assert_eq!(line, 3);
        assert!(msg.starts_with("`data` holds the output folder"), "{msg}");
        let absolute = root.join("data").join("out").to_string_lossy().replace('\\', "/");
        let (_, msg) = err(&format!("[build]\noutput = \"{absolute}\"\ncopy = [\"data\"]\n"));
        assert!(msg.starts_with("`data` holds the output folder"), "{msg}");
        // The settings file's folder as the output folder, written as "." or another way.
        let (line, msg) = err("[build]\noutput = \".\"\n\ncopy = [\"GAME.CFG\"]\n");
        assert_eq!(line, 4);
        assert!(msg.contains("the files are already in the output folder"), "{msg}");
        let (_, msg) = err("[build]\noutput = \"../proj\"\ncopy = [\"GAME.CFG\"]\n");
        assert!(msg.contains("the files are already in the output folder"), "{msg}");
        // An output folder outside the project holds no entry, even one with a name of its path.
        let outside = root
            .parent()
            .unwrap()
            .join("srv")
            .join("out")
            .to_string_lossy()
            .replace('\\', "/");
        assert!(resolved(&format!("[build]\noutput = \"{outside}\"\ncopy = [\"srv\"]\n")).is_ok());
        assert!(resolved("[build]\noutput = \"../out\"\ncopy = [\"data\"]\n").is_ok());
        assert_eq!(
            resolved("[build]\noutput = \".\"\ncopy = []\n").map(|b| b.output),
            Ok(root.clone()),
            "an empty list with output = \".\""
        );
    }

    /// `--no-settings` (spec `compiler/cli`, "No settings file"): no file is looked for, a bad one included.
    #[test]
    fn no_settings_reads_no_file() {
        let dir = tempfile::tempdir().unwrap();
        let main = write(dir.path(), "p.bas", "PRINT 1\n");
        write(dir.path(), "qb64rust.toml", "[fixes\n");
        assert!(load(&main, false).is_err());
        let (project, _, file) = load(&main, true).unwrap();
        assert_eq!((project, file), (Project::default(), None));
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
