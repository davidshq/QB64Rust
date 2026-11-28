//! `qb64rust init [<folder>]`: writes a folder's `qb64rust.toml` from a few answers (OpenSpec change `project-init`,
//! spec `compiler/project-init`). In a terminal it asks where the program goes and which data files it reads, and
//! which file is the program when there are several; `--yes`, or an input or output that is not a terminal, takes
//! the defaults. An option for each answer gives it without a question. In a folder with no `.bas` file it starts a
//! new project, with the files "QB64: New Project" writes (design D7).
//!
//! The parts (design D3): [`Plan::read`] looks at the folder, [`Given`] holds the options, [`ask`] asks over any
//! reader and writer so that the tests drive it with strings, [`Choice`] is the answers with their defaults,
//! [`render`] writes the file's text from a template (D4), and [`write`] writes the files and never overwrites one.
//! It edits no `.gitignore`: it prints the line to add, which runs no `git` and stages nothing.
//!
//! The terminal check is `std::io::IsTerminal`, in the standard library since Rust 1.70.0
//! (<https://doc.rust-lang.org/std/io/trait.IsTerminal.html>; the release notes,
//! <https://blog.rust-lang.org/2023/06/01/Rust-1.70.0/>); `rust-toolchain.toml` pins a later release.

use crate::settings::{self, SETTINGS_FILE};
use std::io::{BufRead, IsTerminal as _, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// The usage of `qb64rust init` (spec `compiler/project-init`, "Init subcommand").
pub const INIT_USAGE: &str = "usage: qb64rust init [<folder>] [--yes] [--main <file.bas>] [--output <folder>] \
[--copy <file or folder>]... [--no-copy] [--keyword-case camel|upper|lower] [--fixes on|off] [--plan]    (writes \
the folder's qb64rust.toml; in a terminal it asks where the program goes and which data files it needs, and --yes \
takes the defaults; --plan prints what it would ask, as JSON)";

/// The program a new project starts with: the bytes "QB64: New Project" writes (`vscode/src/newProject.ts`).
pub const MAIN_BAS: &str = "PRINT \"Hello, world!\"\r\n";

/// The workspace setting of a new project, which makes VS Code save its QB64 files as UTF-8: the bytes "QB64: New
/// Project" writes.
pub const WORKSPACE_SETTINGS: &str = "{\n    \"[qb64rust]\": {\n        \"files.encoding\": \"utf8\"\n    }\n}\n";

/// The output folder offered beside the source's own.
pub const BUILD_FOLDER: &str = "build";

/// What is in a folder, and so what `init` asks there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    /// The folder, absolute.
    pub folder: PathBuf,
    /// The folder's own `qb64rust.toml`, when it has one: `init` then writes nothing.
    pub settings: Option<PathBuf>,
    /// The nearest `qb64rust.toml` in a folder above, which the new file takes the place of for this folder.
    pub above: Option<PathBuf>,
    /// The folder's `.bas` files, sorted. None: a new project.
    pub programs: Vec<String>,
    /// The files and folders offered as data files (spec "The data files offered"), sorted.
    pub offered: Vec<String>,
    /// Whether an entry `.git` is in the folder or one above it.
    pub git: bool,
}

impl Plan {
    /// The plan of `folder` for the output folder `output` (`None`: the answer is not known yet, and `build`, the
    /// one other choice of the question, is left out of the data files).
    pub fn read(folder: &Path, output: Option<&str>) -> Plan {
        let folder = std::path::absolute(folder).unwrap_or_else(|_| folder.to_path_buf());
        let mut names: Vec<(String, bool)> = std::fs::read_dir(&folder)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| (e.file_name().to_string_lossy().into_owned(), e.path().is_dir()))
            .collect();
        names.sort();
        let has_ext = |name: &str, exts: &[&str]| {
            Path::new(name)
                .extension()
                .is_some_and(|e| exts.iter().any(|x| e.eq_ignore_ascii_case(x)))
        };
        let programs: Vec<String> = names
            .iter()
            .filter(|(n, dir)| !dir && has_ext(n, &["bas"]))
            .map(|(n, _)| n.clone())
            .collect();
        // What a build or a run of the folder's programs writes beside them: the program, its license file, its
        // debug symbols, `-z`'s C++ folder.
        let built: Vec<String> = programs
            .iter()
            .flat_map(|p| {
                let exe = program_name(p);
                ["", ".license.txt", ".debug.json", ".qb64rust"].map(|end| format!("{exe}{end}"))
            })
            .collect();
        let output_name = output
            .filter(|o| *o != ".")
            .unwrap_or(BUILD_FOLDER)
            .split('/')
            .next()
            .unwrap_or_default()
            .to_string();
        let same = |a: &str, b: &str| {
            if cfg!(windows) {
                a.eq_ignore_ascii_case(b)
            } else {
                a == b
            }
        };
        let offered = names
            .iter()
            .filter(|(n, _)| {
                // A name with `\` could not be a `copy` entry, which uses `/` between folders.
                !n.starts_with('.')
                    && !n.contains('\\')
                    && !same(n, SETTINGS_FILE)
                    && !same(n, "qb64pe-log.txt")
                    && !has_ext(n, &["bas", "bi", "bm"])
                    && !same(n, &output_name)
                    && !built.iter().any(|b| same(n, b))
            })
            .map(|(n, _)| n.clone())
            .collect();
        let settings = Some(folder.join(SETTINGS_FILE)).filter(|p| p.is_file());
        let above = folder.parent().and_then(|p| settings::find_settings(&p.join("x.bas")));
        let git = folder.ancestors().any(|p| p.join(".git").exists());
        Plan {
            folder,
            settings,
            above,
            programs,
            offered,
            git,
        }
    }

    /// A folder with no `.bas` file: `init` starts a new project there.
    pub fn new_project(&self) -> bool {
        self.programs.is_empty()
    }

    /// The plan as one line of JSON (`--plan`; spec "The plan of a folder"), for the extension.
    pub fn json(&self) -> String {
        let path = |p: &Option<PathBuf>| p.as_ref().map(|p| p.to_string_lossy().into_owned());
        serde_json::json!({
            "folder": self.folder.to_string_lossy(),
            "settings": path(&self.settings),
            "above": path(&self.above),
            "new_project": self.new_project(),
            "programs": self.programs,
            "data_files": self.offered,
            "git": self.git,
            "main": self.programs.first(),
            "fixes": self.new_project(),
        })
        .to_string()
    }
}

/// The program a build of the source `name` writes beside it: `game.exe` on Windows, `game` elsewhere.
fn program_name(name: &str) -> String {
    let stem = Path::new(name)
        .file_stem()
        .map_or_else(String::new, |s| s.to_string_lossy().into_owned());
    format!("{stem}{}", std::env::consts::EXE_SUFFIX)
}

/// The answers the options gave; `None` for one not given.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Given {
    pub main: Option<String>,
    /// `.` for beside the source.
    pub output: Option<String>,
    /// `--copy` entries, or an empty list for `--no-copy`.
    pub copy: Option<Vec<String>>,
    pub keyword_case: Option<String>,
    pub fixes: Option<bool>,
}

/// The keyword cases `--keyword-case` takes, the default first.
pub const KEYWORD_CASES: [&str; 3] = ["camel", "upper", "lower"];

/// Every answer, with the defaults filled in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Choice {
    /// `[project]` `main`, set only when the folder has several programs.
    pub main: Option<String>,
    /// `[build]` `output`; `None` for beside the source.
    pub output: Option<String>,
    pub copy: Vec<String>,
    pub keyword_case: &'static str,
    pub fixes: bool,
    /// A new project: `main.bas` and `.vscode/settings.json` are written, and `encoding = "utf-8"` is set.
    pub new_project: bool,
}

impl Given {
    /// The options after `init`, with the folder; an error is a usage error.
    pub fn parse(args: &[String]) -> Result<(Given, Option<PathBuf>, bool, bool), String> {
        let (mut g, mut folder, mut yes, mut plan) = (Given::default(), None, false, false);
        let mut copy: Vec<String> = Vec::new();
        let mut no_copy = false;
        let mut i = 0;
        while i < args.len() {
            let arg = args[i].as_str();
            let mut value = |what: &str| -> Result<String, String> {
                i += 1;
                args.get(i).cloned().ok_or_else(|| format!("{arg} needs {what}"))
            };
            match arg {
                "--yes" | "-y" => yes = true,
                "--plan" => plan = true,
                "--main" => g.main = Some(value("a .bas file")?),
                "--output" => g.output = Some(value("a folder, or . for beside the source")?),
                "--copy" => copy.push(value("a file or folder")?),
                "--no-copy" => no_copy = true,
                "--keyword-case" => {
                    let case = value("camel, upper or lower")?;
                    if !KEYWORD_CASES.contains(&case.as_str()) {
                        return Err(format!("--keyword-case is camel, upper or lower, not `{case}`"));
                    }
                    g.keyword_case = Some(case);
                }
                "--fixes" => {
                    g.fixes = Some(match value("on or off")?.as_str() {
                        "on" => true,
                        "off" => false,
                        other => return Err(format!("--fixes is on or off, not `{other}`")),
                    });
                }
                _ if arg.starts_with('-') => return Err(format!("init does not know the option `{arg}`")),
                _ if folder.is_some() => return Err("init takes one folder".to_string()),
                _ => folder = Some(PathBuf::from(arg)),
            }
            i += 1;
        }
        if no_copy && !copy.is_empty() {
            return Err("--copy and --no-copy cannot go together".to_string());
        }
        if no_copy || !copy.is_empty() {
            g.copy = Some(copy);
        }
        Ok((g, folder, yes, plan))
    }

    /// The answers of `self` in the folder of `plan`, with the defaults (spec "The answers and their options").
    /// An error names the bad answer; nothing is written then.
    pub fn choose(&self, plan: &Plan) -> Result<Choice, String> {
        let slash = |what: &str, path: &str| {
            if path.contains('\\') {
                Err(format!("{what} uses `/` between folders, not `\\`: \"{path}\""))
            } else {
                Ok(())
            }
        };
        let main = match &self.main {
            Some(m) => {
                slash("--main", m)?;
                let file = plan.folder.join(m);
                if !file.is_file() || !Path::new(m).extension().is_some_and(|e| e.eq_ignore_ascii_case("bas")) {
                    return Err(format!(
                        "--main names \"{m}\", and there is no such .bas file in the folder"
                    ));
                }
                Some(m.clone())
            }
            None if plan.programs.len() > 1 => plan.programs.first().cloned(),
            None => None,
        };
        let mut output = match self.output.as_deref() {
            None | Some(".") => None,
            Some("") => return Err("--output needs a folder, or . for beside the source".to_string()),
            Some(o) => {
                slash("--output", o)?;
                Some(o.trim_end_matches('/').to_string())
            }
        };
        if let Some(copy) = &self.copy
            && !copy.is_empty()
        {
            if self.output.as_deref() == Some(".") {
                return Err("--copy needs an output folder of its own, not --output .".to_string());
            }
            output.get_or_insert_with(|| BUILD_FOLDER.to_string());
        }
        let copy = match (&output, &self.copy) {
            (None, _) => Vec::new(),
            (Some(_), Some(given)) => {
                for entry in given {
                    slash("--copy", entry)?;
                    if !plan.folder.join(entry).exists() {
                        return Err(format!("--copy names \"{entry}\", and there is no such file or folder"));
                    }
                }
                given.clone()
            }
            (Some(_), None) => plan.offered.clone(),
        };
        let keyword_case = match self.keyword_case.as_deref() {
            None => KEYWORD_CASES[0],
            Some(c) => KEYWORD_CASES
                .into_iter()
                .find(|k| *k == c)
                .ok_or_else(|| format!("--keyword-case is camel, upper or lower, not `{c}`"))?,
        };
        Ok(Choice {
            main,
            output,
            copy,
            keyword_case,
            fixes: self.fixes.unwrap_or(plan.new_project()),
            new_project: plan.new_project(),
        })
    }
}

/// Asks, on `out` and from `input`, each question that no option in `given` answered and that applies to the
/// folder of `plan` (spec "Questions"). An empty line, or the end of the input, takes the default; any other answer
/// that is not one of the choices is asked again. The answers go into `given`.
pub fn ask(plan: &Plan, given: &mut Given, input: &mut dyn BufRead, out: &mut dyn Write) -> std::io::Result<()> {
    if given.main.is_none() && plan.programs.len() > 1 {
        writeln!(
            out,
            "Which file is the program? The others are its parts or programs of their own."
        )?;
        let n = choose_one(&plan.programs, input, out)?;
        given.main = Some(plan.programs[n].clone());
    }
    if given.output.is_none() && given.copy.is_none() {
        writeln!(out, "Where does the program go?")?;
        let choices = [
            "beside the source, as QB64pe puts it".to_string(),
            format!("into a folder `{BUILD_FOLDER}`, with the data files it reads copied there"),
        ];
        let n = choose_one(&choices, input, out)?;
        given.output = Some(if n == 0 { "." } else { BUILD_FOLDER }.to_string());
    }
    let output = given.output.as_deref().filter(|o| *o != ".");
    if let Some(output) = output
        && given.copy.is_none()
    {
        // The data files offered depend on the output folder chosen.
        let offered = Plan::read(&plan.folder, Some(output)).offered;
        if !offered.is_empty() {
            writeln!(
                out,
                "Which files does the program read? They are copied into `{output}` before each build."
            )?;
            given.copy = Some(choose_some(&offered, input, out)?);
        }
    }
    Ok(())
}

/// One line of the input without its line ending and surrounding spaces; `None` at the end of the input.
fn answer(input: &mut dyn BufRead) -> std::io::Result<Option<Vec<u8>>> {
    let mut line = Vec::new();
    if input.read_until(b'\n', &mut line)? == 0 {
        return Ok(None);
    }
    Ok(Some(line.trim_ascii().to_vec()))
}

/// The number of `n` (1-based) in `text`, when it is one of `1..=count`.
fn number(text: &[u8], count: usize) -> Option<usize> {
    let digits: String = text.iter().map(|&b| char::from(b)).collect();
    digits.parse::<usize>().ok().filter(|n| (1..=count).contains(n))
}

/// A numbered list with the first choice the default; the index chosen.
fn choose_one(choices: &[String], input: &mut dyn BufRead, out: &mut dyn Write) -> std::io::Result<usize> {
    for (i, c) in choices.iter().enumerate() {
        let default = if i == 0 { " (the default)" } else { "" };
        writeln!(out, "  {}. {c}{default}", i + 1)?;
    }
    loop {
        write!(out, "Type a number from 1 to {}, or press Enter for 1: ", choices.len())?;
        out.flush()?;
        let Some(line) = answer(input)? else {
            writeln!(out)?;
            return Ok(0);
        };
        if line.is_empty() {
            return Ok(0);
        }
        if let Some(n) = number(&line, choices.len()) {
            return Ok(n - 1);
        }
        writeln!(out, "That is not one of the numbers.")?;
    }
}

/// A numbered list, every entry the default; the entries chosen.
fn choose_some(choices: &[String], input: &mut dyn BufRead, out: &mut dyn Write) -> std::io::Result<Vec<String>> {
    for (i, c) in choices.iter().enumerate() {
        writeln!(out, "  {}. {c}", i + 1)?;
    }
    loop {
        write!(
            out,
            "Press Enter to copy them all, or type the numbers to copy (as `1 3`), or `none`: "
        )?;
        out.flush()?;
        let Some(line) = answer(input)? else {
            writeln!(out)?;
            return Ok(choices.to_vec());
        };
        if line.is_empty() {
            return Ok(choices.to_vec());
        }
        if line.eq_ignore_ascii_case(b"none") {
            return Ok(Vec::new());
        }
        let picked: Option<Vec<usize>> = line
            .split(|b| *b == b' ' || *b == b',')
            .filter(|w| !w.is_empty())
            .map(|w| number(w, choices.len()))
            .collect();
        if let Some(mut picked) = picked {
            picked.sort_unstable();
            picked.dedup();
            return Ok(picked.into_iter().map(|n| choices[n - 1].clone()).collect());
        }
        writeln!(out, "Use the numbers of the list, separated by spaces.")?;
    }
}

/// A TOML string of `text`, which holds no `\` or `"`: the options and the folder's names are checked for `\`, and
/// a name with `"` is written with `\"`.
fn quoted(text: &str) -> String {
    format!("\"{}\"", text.replace('"', "\\\""))
}

/// The text of the settings file (design D4): a plain line for each setting an answer changed from its default,
/// and every other setting of `[project]`, `[build]` and `[format]` as a line that starts with `# `. A table with
/// no plain line has its header commented too, so that the file of every default sets nothing.
pub fn render(plan: &Plan, c: &Choice) -> String {
    // Each line: the setting, whether it is set, and a few words.
    type Line = (String, bool, &'static str);
    let example_main = c
        .main
        .clone()
        .or_else(|| plan.programs.first().cloned())
        .unwrap_or_else(|| "main.bas".into());
    let list = |items: &[String]| format!("[{}]", items.iter().map(|s| quoted(s)).collect::<Vec<_>>().join(", "));
    let project: Vec<Line> = vec![
        (
            format!("main = {}", quoted(&example_main)),
            c.main.is_some(),
            "the program the editor checks an included file with",
        ),
        (
            "exclude = [\"web/**\"]".into(),
            false,
            "files the editor does not check",
        ),
        if c.new_project {
            (
                "encoding = \"utf-8\"".into(),
                true,
                "the source files are UTF-8; their strings hold code page 437 characters",
            )
        } else {
            (
                "encoding = \"utf-8\"".into(),
                false,
                "how the source files are saved: \"utf-8\" or \"cp437\"",
            )
        },
    ];
    let build: Vec<Line> = vec![
        match &c.output {
            Some(o) => (
                format!("output = {}", quoted(o)),
                true,
                "where a build puts the program",
            ),
            None => (
                format!("output = {}", quoted(BUILD_FOLDER)),
                false,
                "where a build puts the program; beside the source when not set",
            ),
        },
        if c.copy.is_empty() {
            (
                "copy = [\"GAME.CFG\", \"maps\"]".into(),
                false,
                "data files copied into the output folder before each build",
            )
        } else {
            (
                format!("copy = {}", list(&c.copy)),
                true,
                "data files copied into the output folder before each build",
            )
        },
        (
            "call_trace = true".into(),
            false,
            "the call stack in a runtime error's message",
        ),
    ];
    let keyword = c.keyword_case;
    let format: Vec<Line> = vec![
        (
            format!("keyword-case = {}", quoted(keyword)),
            keyword != KEYWORD_CASES[0],
            "how Format Document writes keywords: \"camel\", \"upper\" or \"lower\"",
        ),
        ("auto-indent = true".into(), false, "indent by block depth"),
        ("indent-size = 4".into(), false, "spaces per level, 1 to 64"),
        (
            "indent-subs = true".into(),
            false,
            "the body of a SUB or FUNCTION one level in",
        ),
        (
            "auto-layout = true".into(),
            false,
            "one space between the parts of a line",
        ),
        (
            "one-statement-per-line = false".into(),
            false,
            "each statement of a line with `:` on a line of its own",
        ),
        ("block-if = false".into(), false, "a single-line IF written as a block"),
    ];
    let lints: Vec<Line> = vec![(
        "all = false".into(),
        false,
        "turns every lint warning off; the manual's list of warnings names each",
    )];
    let fixes: Vec<Line> = vec![(
        "all = true".into(),
        c.fixes,
        "fixes QB64pe's odd behaviors; `qb64rust --list-fixes` names each",
    )];
    let mut text = String::from(
        "# The settings of this folder's program, written by `qb64rust init`. A line that starts with `# ` is a\n\
         # setting at its default: remove the `# ` to set it, and the one before its table's name too. The manual's\n\
         # \"The settings file\" says what each does.\n",
    );
    for (name, lines) in [
        ("project", project),
        ("build", build),
        ("format", format),
        ("lints", lints),
        ("fixes", fixes),
    ] {
        let set = lines.iter().any(|(_, on, _)| *on);
        text.push('\n');
        text.push_str(if set { "" } else { "# " });
        text.push_str(&format!("[{name}]\n"));
        for (line, on, words) in lines {
            let hash = if on { "" } else { "# " };
            text.push_str(&format!("{hash}{line}  # {words}\n"));
        }
    }
    text
}

/// Writes the files of `c` into the folder of `plan`: `qb64rust.toml`, and for a new project `main.bas` and
/// `.vscode/settings.json` first. Each file is created new, so that one made meanwhile is never overwritten; when
/// one fails, what this call wrote is removed. The paths written, or an error that names the file.
///
/// The settings file is first read with every check a build makes, so that `init` never writes one that a build
/// refuses: a `--copy ../shared`, or a `--copy maps` with `--output maps`, is the reader's message and nothing is
/// written.
pub fn write(plan: &Plan, c: &Choice) -> Result<Vec<PathBuf>, String> {
    let folder = &plan.folder;
    let settings_text = render(plan, c);
    if let Err((_, msg)) = settings::check_text(settings_text.as_bytes(), folder) {
        return Err(format!("{msg}; nothing was written"));
    }
    let mut files: Vec<(PathBuf, String)> = Vec::new();
    if c.new_project {
        files.push((folder.join("main.bas"), MAIN_BAS.to_string()));
        files.push((
            folder.join(".vscode").join("settings.json"),
            WORKSPACE_SETTINGS.to_string(),
        ));
    }
    files.push((folder.join(SETTINGS_FILE), settings_text));
    if let Some((there, _)) = files.iter().find(|(p, _)| p.exists()) {
        return Err(format!("{} is there already, so nothing was written", there.display()));
    }
    let (mut written, mut made) = (Vec::new(), Vec::new());
    let result = (|| {
        for (path, text) in &files {
            if let Some(dir) = path.parent()
                && !dir.exists()
            {
                std::fs::create_dir_all(dir).map_err(|e| format!("creating {}: {e}", dir.display()))?;
                made.push(dir.to_path_buf());
            }
            let mut file = std::fs::File::create_new(path).map_err(|e| format!("writing {}: {e}", path.display()))?;
            written.push(path.clone());
            file.write_all(text.as_bytes())
                .map_err(|e| format!("writing {}: {e}", path.display()))?;
        }
        Ok(())
    })();
    if let Err(e) = result {
        for path in &written {
            let _ = std::fs::remove_file(path);
        }
        // Only a folder this call made, and only when empty.
        for dir in made.iter().rev() {
            let _ = std::fs::remove_dir(dir);
        }
        return Err(e);
    }
    Ok(written)
}

/// The line to add to `.gitignore` for the programs of `c` (spec "The ignore file"): the output folder, or each
/// program built beside its source.
pub fn ignore_lines(plan: &Plan, c: &Choice) -> Vec<String> {
    match &c.output {
        Some(o) => vec![format!("/{o}/")],
        None if c.new_project => vec![format!("/{}", program_name("main.bas"))],
        None => plan.programs.iter().map(|p| format!("/{}", program_name(p))).collect(),
    }
}

/// `qb64rust init` with the arguments after `init`: prints what it wrote, or the error, to the standard output.
pub fn command(args: &[String]) -> ExitCode {
    let fail = |msg: &str| {
        println!("qb64rust: {msg}");
        ExitCode::FAILURE
    };
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("{INIT_USAGE}");
        return ExitCode::SUCCESS;
    }
    let (mut given, folder, yes, plan_only) = match Given::parse(args) {
        Ok(parsed) => parsed,
        Err(msg) => return fail(&format!("{msg}\n{INIT_USAGE}")),
    };
    let folder = folder.unwrap_or_else(|| PathBuf::from("."));
    if folder.exists() && !folder.is_dir() {
        return fail(&format!("{} is not a folder", folder.display()));
    }
    let output = given.output.as_deref();
    if plan_only {
        println!("{}", Plan::read(&folder, output).json());
        return ExitCode::SUCCESS;
    }
    let plan = Plan::read(&folder, output);
    if let Some(there) = &plan.settings {
        return fail(&format!(
            "{} is there already: init never changes a settings file, so edit it by hand",
            there.display()
        ));
    }
    // The options are checked before any question, so that a bad one is not found after the answers.
    if let Err(msg) = given.choose(&plan) {
        return fail(&msg);
    }
    if !yes && std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
        let mut out = std::io::stdout();
        if let Err(e) = ask(&plan, &mut given, &mut std::io::stdin().lock(), &mut out) {
            return fail(&format!("reading the answers: {e}"));
        }
    }
    let choice = match given.choose(&Plan::read(&folder, given.output.as_deref())) {
        Ok(c) => c,
        Err(msg) => return fail(&msg),
    };
    if let Err(e) = std::fs::create_dir_all(&plan.folder) {
        return fail(&format!("creating {}: {e}", plan.folder.display()));
    }
    match write(&plan, &choice) {
        Ok(files) => {
            for file in files {
                println!("wrote {}", file.display());
            }
        }
        Err(msg) => return fail(&msg),
    }
    if let Some(above) = &plan.above {
        println!(
            "{} is no longer read for this folder: the new file is the only one read here",
            above.display()
        );
    }
    if plan.git {
        let lines = ignore_lines(&plan, &choice);
        println!(
            "In git, add {} to .gitignore, so that what a build writes stays out of the repository: {}",
            if lines.len() == 1 { "this line" } else { "these lines" },
            lines.join(" ")
        );
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folder(files: &[&str]) -> tempfile::TempDir {
        let d = tempfile::tempdir().unwrap();
        for f in files {
            let path = d.path().join(f);
            if let Some(dir) = f.strip_suffix('/') {
                std::fs::create_dir_all(d.path().join(dir)).unwrap();
            } else {
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, "x\n").unwrap();
            }
        }
        d
    }

    fn exe(stem: &str) -> String {
        format!("{stem}{}", std::env::consts::EXE_SUFFIX)
    }

    /// Spec "The data files offered", scenario "What is offered", and what a build writes beside a program.
    #[test]
    fn the_data_files_offered() {
        let game = exe("game");
        let license = format!("{game}.license.txt");
        let d = folder(&[
            "game.bas",
            &game,
            &license,
            "turn.bm",
            "LIB.BI",
            "GAME.CFG",
            "maps/",
            ".git/",
            "build/",
            "qb64pe-log.txt",
            "qb64rust.toml.bak",
        ]);
        let plan = Plan::read(d.path(), None);
        assert_eq!(plan.programs, ["game.bas"]);
        assert_eq!(plan.offered, ["GAME.CFG", "maps", "qb64rust.toml.bak"]);
        assert!(plan.git && !plan.new_project());
        assert_eq!(
            Plan::read(d.path(), Some("maps")).offered,
            ["GAME.CFG", "build", "qb64rust.toml.bak"]
        );
    }

    /// Spec "The plan of a folder": a settings file here and one folder up, a `.git` file, no git, and the line of
    /// JSON.
    #[test]
    fn the_plan_of_a_folder() {
        let d = folder(&["proj/qb64rust.toml", "proj/demo/game.bas", "proj/demo/GAME.CFG"]);
        let demo = d.path().join("proj/demo");
        let plan = Plan::read(&demo, None);
        assert_eq!(plan.settings, None);
        assert_eq!(
            plan.above.as_deref(),
            Some(d.path().join("proj/qb64rust.toml").as_path())
        );
        let json: serde_json::Value = serde_json::from_str(&plan.json()).unwrap();
        assert_eq!(json["programs"], serde_json::json!(["game.bas"]));
        assert_eq!(json["data_files"], serde_json::json!(["GAME.CFG"]));
        assert_eq!(json["new_project"], false);
        assert_eq!(json["main"], "game.bas");
        let here = Plan::read(&d.path().join("proj"), None);
        assert!(here.settings.is_some());
        // A worktree's `.git` is a file.
        std::fs::write(d.path().join("proj/.git"), "gitdir: elsewhere\n").unwrap();
        assert!(Plan::read(&demo, None).git);
        let empty = folder(&[]);
        let plan = Plan::read(empty.path(), None);
        assert!(plan.new_project() && plan.offered.is_empty());
        // No git: unless the temporary folder itself is in a work tree on this machine.
        let in_git = empty.path().ancestors().any(|p| p.join(".git").exists());
        assert_eq!(plan.git, in_git);
    }

    fn choose(plan: &Plan, given: Given) -> Result<Choice, String> {
        given.choose(plan)
    }

    /// Spec "The answers and their options": each scenario, with the message of each bad value.
    #[test]
    fn the_answers_and_their_options() {
        let d = folder(&["game.bas", "editor.bas", "GAME.CFG", "maps/"]);
        let plan = Plan::read(d.path(), None);
        let c = choose(&plan, Given::default()).unwrap();
        assert_eq!(c.main.as_deref(), Some("editor.bas"), "several programs: the first");
        assert_eq!((c.output, c.copy, c.fixes), (None, vec![], false));
        let given = Given {
            main: Some("game.bas".into()),
            output: Some("build".into()),
            ..Given::default()
        };
        let c = choose(&plan, given).unwrap();
        assert_eq!(c.main.as_deref(), Some("game.bas"));
        assert_eq!(c.copy, ["GAME.CFG", "maps"], "every data file by default");
        let c = choose(
            &plan,
            Given {
                copy: Some(vec!["GAME.CFG".into()]),
                ..Given::default()
            },
        )
        .unwrap();
        assert_eq!(
            (c.output.as_deref(), c.copy.as_slice()),
            (Some("build"), &["GAME.CFG".to_string()][..])
        );
        let c = choose(
            &plan,
            Given {
                output: Some("build".into()),
                copy: Some(vec![]),
                ..Given::default()
            },
        )
        .unwrap();
        assert!(c.copy.is_empty(), "--no-copy");
        let bad = |g: Given| choose(&plan, g).unwrap_err();
        assert_eq!(
            bad(Given {
                copy: Some(vec!["NONE.CFG".into()]),
                ..Given::default()
            }),
            "--copy names \"NONE.CFG\", and there is no such file or folder"
        );
        assert_eq!(
            bad(Given {
                main: Some("none.bas".into()),
                ..Given::default()
            }),
            "--main names \"none.bas\", and there is no such .bas file in the folder"
        );
        assert_eq!(
            bad(Given {
                output: Some(".".into()),
                copy: Some(vec!["GAME.CFG".into()]),
                ..Given::default()
            }),
            "--copy needs an output folder of its own, not --output ."
        );
        assert_eq!(
            bad(Given {
                output: Some("out\\bin".into()),
                ..Given::default()
            }),
            "--output uses `/` between folders, not `\\`: \"out\\bin\""
        );
        let args = |a: &[&str]| Given::parse(&a.iter().map(|s| (*s).to_string()).collect::<Vec<_>>());
        assert_eq!(
            args(&["--keyword-case", "title"]).unwrap_err(),
            "--keyword-case is camel, upper or lower, not `title`"
        );
        assert_eq!(
            args(&["--fixes", "yes"]).unwrap_err(),
            "--fixes is on or off, not `yes`"
        );
        assert_eq!(
            args(&["--copy", "a", "--no-copy"]).unwrap_err(),
            "--copy and --no-copy cannot go together"
        );
        assert_eq!(
            args(&["--output"]).unwrap_err(),
            "--output needs a folder, or . for beside the source"
        );
        assert_eq!(args(&["--what"]).unwrap_err(), "init does not know the option `--what`");
        let (g, f, yes, plan_only) = args(&["there", "--yes", "--fixes", "on", "--copy", "a", "--copy", "b"]).unwrap();
        assert_eq!((f, yes, plan_only), (Some(PathBuf::from("there")), true, false));
        assert_eq!(
            (g.fixes, g.copy),
            (Some(true), Some(vec!["a".to_string(), "b".to_string()]))
        );
        // One program: no main; a new project: the fixes on.
        let one = folder(&["game.bas"]);
        assert_eq!(
            choose(&Plan::read(one.path(), None), Given::default()).unwrap().main,
            None
        );
        let empty = folder(&[]);
        let c = choose(&Plan::read(empty.path(), None), Given::default()).unwrap();
        assert!(c.fixes && c.new_project);
    }

    /// Every combination of answers, for a folder with a program and for a new one.
    fn every_choice() -> Vec<Choice> {
        let mut out = Vec::new();
        for main in [None, Some("game.bas".to_string())] {
            for (output, copy) in [
                (None, vec![]),
                (Some("build".to_string()), vec![]),
                (
                    Some("build".to_string()),
                    vec!["GAME.CFG".to_string(), "maps".to_string()],
                ),
                (Some("out/bin".to_string()), vec!["GAME.CFG".to_string()]),
            ] {
                for keyword_case in KEYWORD_CASES {
                    for fixes in [false, true] {
                        for new_project in [false, true] {
                            out.push(Choice {
                                main: main.clone(),
                                output: output.clone(),
                                copy: copy.clone(),
                                keyword_case,
                                fixes,
                                new_project,
                            });
                        }
                    }
                }
            }
        }
        out
    }

    /// Design D4, first test: the file of each combination of answers is read with every check a build makes.
    #[test]
    fn every_file_is_read() {
        let d = folder(&["game.bas", "GAME.CFG", "maps/"]);
        let plan = Plan::read(d.path(), None);
        for c in every_choice() {
            let text = render(&plan, &c);
            assert_eq!(settings::check_text(text.as_bytes(), d.path()), Ok(()), "{c:?}\n{text}");
        }
    }

    /// Design D4, second test: each commented setting, uncommented with its table's header, is read too (spec "The
    /// file written", scenario "A setting found later").
    #[test]
    fn each_commented_setting_is_read() {
        let d = folder(&["game.bas", "GAME.CFG", "maps/"]);
        let plan = Plan::read(d.path(), None);
        let mut seen = 0;
        for c in every_choice() {
            let text = render(&plan, &c);
            let lines: Vec<&str> = text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let Some(setting) = line.strip_prefix("# ").filter(|s| s.contains(" = ")) else {
                    continue;
                };
                let header = (0..i)
                    .rev()
                    .find(|&j| lines[j].trim_start_matches("# ").starts_with('['))
                    .unwrap();
                let mut changed: Vec<String> = lines.iter().map(|l| (*l).to_string()).collect();
                changed[i] = setting.to_string();
                changed[header] = lines[header].trim_start_matches("# ").to_string();
                let changed = changed.join("\n");
                assert_eq!(
                    settings::check_text(changed.as_bytes(), d.path()),
                    Ok(()),
                    "{c:?}\n{changed}"
                );
                seen += 1;
            }
        }
        assert!(seen > 100, "{seen}");
    }

    /// Design D4, third test: each key `settings.rs` reads in `[project]`, `[build]` and `[format]` is in the
    /// template, and `all` in `[lints]` and `[fixes]`.
    #[test]
    fn every_key_is_in_the_template() {
        let d = folder(&["game.bas"]);
        let plan = Plan::read(d.path(), None);
        let text = render(&plan, &choose(&plan, Given::default()).unwrap());
        let keys_of = |table: &str| -> Vec<String> {
            let start = text.find(&format!("[{table}]\n")).unwrap();
            text[start..]
                .lines()
                .skip(1)
                .take_while(|l| !l.trim_start_matches("# ").starts_with('['))
                .filter_map(|l| l.trim_start_matches("# ").split(" = ").next().map(str::to_string))
                .filter(|k| !k.is_empty())
                .collect()
        };
        for (table, keys) in [
            ("project", settings::PROJECT_KEYS),
            ("build", settings::BUILD_KEYS),
            ("format", settings::FORMAT_KEYS),
            ("lints", &["all"][..]),
            ("fixes", &["all"][..]),
        ] {
            let mut want: Vec<String> = keys.iter().map(|k| (*k).to_string()).collect();
            let mut got = keys_of(table);
            want.sort();
            got.sort();
            assert_eq!(got, want, "[{table}]\n{text}");
        }
    }

    /// Spec "The file written", scenario "Every default": a folder with one program gets a file that sets nothing.
    #[test]
    fn the_file_of_every_default_sets_nothing() {
        let d = folder(&["game.bas"]);
        let plan = Plan::read(d.path(), None);
        let text = render(&plan, &choose(&plan, Given::default()).unwrap());
        let plain: Vec<&str> = text.lines().filter(|l| !l.is_empty() && !l.starts_with('#')).collect();
        assert_eq!(plain, Vec::<&str>::new(), "{text}");
        // A new project: the two settings of "QB64: New Project".
        let d = folder(&[]);
        let plan = Plan::read(d.path(), None);
        let text = render(&plan, &choose(&plan, Given::default()).unwrap());
        let plain: Vec<&str> = text
            .lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(|l| l.split("  #").next().unwrap())
            .collect();
        assert_eq!(
            plain,
            ["[project]", "encoding = \"utf-8\"", "[fixes]", "all = true"],
            "{text}"
        );
    }

    fn asked(plan: &Plan, given: Given, input: &str) -> (Given, String) {
        let mut given = given;
        let mut out = Vec::new();
        ask(plan, &mut given, &mut input.as_bytes(), &mut out).unwrap();
        (given, out.iter().map(|&b| char::from(b)).collect())
    }

    /// The questions (spec "Questions"): Enter at each gives the answers of `--yes`, a wrong answer is asked again,
    /// an option's question is not asked, and which questions a folder gets.
    #[test]
    fn the_questions() {
        let d = folder(&["game.bas", "editor.bas", "GAME.CFG", "maps/"]);
        let plan = Plan::read(d.path(), None);
        // Enter at each: main, then where. Beside the source, so no data question.
        let (g, out) = asked(&plan, Given::default(), "\n\n");
        assert_eq!(choose(&plan, g).unwrap(), choose(&plan, Given::default()).unwrap());
        assert!(
            out.contains("Which file is the program?") && out.contains("Where does the program go?"),
            "{out}"
        );
        assert!(!out.contains("Which files does the program read?"), "{out}");
        // The end of the input takes the defaults too.
        let (g, _) = asked(&plan, Given::default(), "");
        assert_eq!(choose(&plan, g).unwrap(), choose(&plan, Given::default()).unwrap());
        // A wrong answer at each question, then `build`, then the data files 2 and 1.
        let (g, out) = asked(&plan, Given::default(), "7\n2\nx\n2\n9\n2 1\n");
        assert_eq!(out.matches("That is not one of the numbers.").count(), 2, "{out}");
        assert!(out.contains("Use the numbers of the list"), "{out}");
        let c = choose(&plan, g).unwrap();
        assert_eq!(c.main.as_deref(), Some("game.bas"));
        assert_eq!(
            (c.output.as_deref(), c.copy.as_slice()),
            (Some("build"), &["GAME.CFG".to_string(), "maps".to_string()][..])
        );
        // `none` copies nothing; Enter copies all.
        let given = Given {
            main: Some("game.bas".into()),
            ..Given::default()
        };
        let (g, out) = asked(&plan, given.clone(), "2\nnone\n");
        assert!(
            !out.contains("Which file is the program?"),
            "an option's question: {out}"
        );
        assert_eq!(g.copy, Some(vec![]));
        let (g, _) = asked(&plan, given, "2\n\n");
        assert_eq!(g.copy, Some(vec!["GAME.CFG".to_string(), "maps".to_string()]));
        // `--output build`: only the data files are asked.
        let (_, out) = asked(
            &plan,
            Given {
                main: Some("game.bas".into()),
                output: Some("build".into()),
                ..Given::default()
            },
            "\n",
        );
        assert!(
            !out.contains("Where does the program go?") && out.contains("copied into `build`"),
            "{out}"
        );
        // One program and no data files: one question.
        let one = folder(&["game.bas"]);
        let (_, out) = asked(&Plan::read(one.path(), None), Given::default(), "2\n");
        assert_eq!(out.matches('?').count(), 1, "{out}");
    }

    /// Spec "Init never overwrites", "A program that exists and a new project" and the ignore lines: the files of a
    /// new project, nothing written when one is there, and nothing left when one fails.
    #[test]
    fn the_files_written() {
        let d = folder(&[]);
        let plan = Plan::read(d.path(), None);
        let c = choose(&plan, Given::default()).unwrap();
        let files = write(&plan, &c).unwrap();
        assert_eq!(files.len(), 3);
        assert_eq!(std::fs::read(d.path().join("main.bas")).unwrap(), MAIN_BAS.as_bytes());
        assert_eq!(
            std::fs::read(d.path().join(".vscode/settings.json")).unwrap(),
            WORKSPACE_SETTINGS.as_bytes()
        );
        assert_eq!(ignore_lines(&plan, &c), [format!("/{}", exe("main"))]);
        // Run again: everything is there, and nothing changes.
        let before = std::fs::read(d.path().join(SETTINGS_FILE)).unwrap();
        assert!(
            write(&plan, &c)
                .unwrap_err()
                .ends_with("is there already, so nothing was written")
        );
        assert_eq!(std::fs::read(d.path().join(SETTINGS_FILE)).unwrap(), before);

        // `main.bas` there but no settings file: `.vscode` is not left behind.
        let d = folder(&[]);
        let plan = Plan::read(d.path(), None);
        std::fs::write(d.path().join("main.bas"), "PRINT 2\n").unwrap();
        assert!(write(&plan, &c).is_err());
        assert!(!d.path().join(".vscode").exists() && !d.path().join(SETTINGS_FILE).exists());

        // A failure at the second file: the first is removed. `.vscode` is a file, so its `settings.json` fails.
        let d = folder(&[]);
        let plan = Plan::read(d.path(), None);
        std::fs::write(d.path().join(".vscode"), "a file\n").unwrap();
        assert!(write(&plan, &c).is_err());
        assert!(!d.path().join("main.bas").exists() && !d.path().join(SETTINGS_FILE).exists());

        // A program that exists: only the settings file, and the lines of the output folder or of the program.
        let d = folder(&["game.bas", "GAME.CFG"]);
        let plan = Plan::read(d.path(), None);
        let c = choose(
            &plan,
            Given {
                output: Some("build".into()),
                ..Given::default()
            },
        )
        .unwrap();
        assert_eq!(write(&plan, &c).unwrap(), [d.path().join(SETTINGS_FILE)]);
        assert!(!d.path().join("main.bas").exists());
        assert_eq!(ignore_lines(&plan, &c), ["/build/"]);
        let c = choose(&plan, Given::default()).unwrap();
        assert_eq!(ignore_lines(&plan, &c), [format!("/{}", exe("game"))]);
    }

    /// Answers whose file a build would refuse are the reader's message, and nothing is written: a `copy` entry
    /// outside the folder, and one that is the output folder.
    #[test]
    fn a_file_a_build_refuses_is_not_written() {
        let d = folder(&["proj/game.bas", "proj/maps/", "shared/"]);
        let proj = d.path().join("proj");
        let plan = Plan::read(&proj, None);
        let refused = |output: &str, copy: &str| {
            let given = Given {
                output: Some(output.into()),
                copy: Some(vec![copy.into()]),
                ..Given::default()
            };
            let c = choose(&plan, given).unwrap();
            let e = write(&plan, &c).unwrap_err();
            assert!(!proj.join(SETTINGS_FILE).exists(), "{e}");
            e
        };
        assert_eq!(
            refused("build", "../shared"),
            "a `copy` entry must be a file or folder in the settings file's folder: \"../shared\"; nothing was \
             written"
        );
        assert_eq!(
            refused("maps", "maps"),
            "`maps` is the output folder, which a `copy` entry cannot be or hold; nothing was written"
        );
    }
}
