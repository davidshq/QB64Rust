//! From the syntax tree to the typed [`Program`].
//!
//! Two passes (design D2 of `m2-procedures-and-errors`): pass 1 collects the procedure headers, so a call may come
//! before the definition; pass 2 checks every statement **in file order**, main-module statements and procedure
//! bodies as they come. File order is what the old compiler's scopes follow (measured, `verification\v14_*`): a
//! `DIM SHARED` after a procedure is not seen by it, and a `SHARED x AS T` in an earlier procedure acts like a
//! main-module `DIM x AS T` for the main-module code after it.

mod arraycopy;
mod auto;
mod blocks;
mod builtins;
mod constants;
mod decl;
mod embed;
mod events;
mod expr;
mod flow;
mod io;
mod library;
mod mem;
mod opengl;
mod ops;
mod places;
mod proc;
mod walk;
mod whole;

use blocks::line_if_statements;
use expr::Use;
use whole::{without_parens, written_whole};

use crate::{
    ConstId, LabelId, Place, ProcId, Program, Shape, Stmt, StmtKind, Storage, SymbolKind, Ty, TypeId, Var, VarId,
};
use qb64rust_base::{Diagnostics, FileId, SourceMap, Span, show_bytes, to_u32};
use qb64rust_syntax::ParsedProgram;
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::meta::{MemoryMode, comment_directives};
use qb64rust_syntax::pp::meta_line;
use qb64rust_syntax::tree::{Node, Tok, TreeId};
use std::collections::{HashMap, HashSet};

/// Checks a parsed program. Statements that already have a parse error are skipped (one error per statement). Text
/// is read through `map`, by each span's file.
pub fn check(map: &SourceMap, program: &ParsedProgram) -> (Program, Diagnostics) {
    check_with(map, program, true)
}

/// [`check`] with integer constant folding switched on or off. Off is for tests only: the slice programs must
/// print the same either way (design D5).
pub fn check_with(map: &SourceMap, program: &ParsedProgram, fold: bool) -> (Program, Diagnostics) {
    // A `REDIM` in a procedure that makes the name an array of the procedure in the whole procedure, above it too,
    // is found by a first check; a second one makes those arrays where each procedure starts (design D2 of
    // `m4-left-marked`; the old compiler compiles twice and keeps what its first pass made). Only a program with
    // such a `REDIM` is checked again, until a check finds none it did not have: after the first `REDIM` found, a
    // check makes no array used without `DIM` (design D5 of `m2-defxxx-common-arrays`), so it may miss the next.
    // So k such procedures may take k+1 checks (`SOMEDAY.md`, "Other ideas").
    let mut locals = Vec::new();
    loop {
        let (prog, diags, found) = check_once(map, program, fold, locals.clone());
        let known = |r: &RedimLocal| {
            locals
                .iter()
                .any(|l| (l.proc, &l.name, l.ty) == (r.proc, &r.name, r.ty))
        };
        let new: Vec<RedimLocal> = found.into_iter().filter(|r| !known(r)).collect();
        if new.is_empty() {
            return (prog, diags);
        }
        locals.extend(new);
    }
}

/// One check of the program, with the procedures' arrays of `redim_locals` made where their procedures start;
/// also gives the ones it found.
fn check_once(
    map: &SourceMap,
    program: &ParsedProgram,
    fold: bool,
    redim_locals: Vec<RedimLocal>,
) -> (Program, Diagnostics, Vec<RedimLocal>) {
    let root = program.main().root();
    // The lines that end with a line continuation, in every file (design D1 of `m3-errorline-chr-const`).
    let continued = program
        .trees
        .iter()
        .flat_map(|t| t.root().tokens())
        .filter(|t| t.kind == SyntaxKind::LineContinuation)
        .map(|t| (t.span.file, map.file(t.span.file).line_col(t.span.start).0))
        .collect();
    let mut c = Checker {
        map,
        program,
        continued,
        prog: Program::default(),
        main: Scope::default(),
        local: Scope::default(),
        cur: None,
        procs_by_name: HashMap::new(),
        proc_twins: HashMap::new(),
        proc_of_def: HashMap::new(),
        param_scopes: Vec::new(),
        broken: HashSet::new(),
        static_procs: HashSet::new(),
        dim_shared: HashSet::new(),
        dim_shared_plain: HashMap::new(),
        labels_by_name: HashMap::new(),
        label_of_def: HashMap::new(),
        jump_words: HashSet::new(),
        doubt_words: HashSet::new(),
        sub_call_lines: HashMap::new(),
        diags: Diagnostics::new(),
        at: FileState::new(),
        settings: Settings::new(fold),
        stmt: StmtState::default(),
        dims: DimSolver::default(),
        consts_main: HashMap::new(),
        consts_local: HashMap::new(),
        auto_consts: HashMap::new(),
        fell_back: HashSet::new(),
        gl_library: None,
        gl_consts: HashSet::new(),
        used_names: HashMap::new(),
        used_local: HashMap::new(),
        sinks: Vec::new(),
        bad_next: None,
        pp_drop: None,
        pp_ghosts: HashSet::new(),
        types_by_name: HashMap::new(),
        type_blocks: Vec::new(),
        type_defs: HashSet::new(),
        dim_shared_array_plain: HashMap::new(),
        types_marked: HashSet::new(),
        types_failed: HashSet::new(),
        const_names: Vec::new(),
        consts_above: HashMap::new(),
        consts_before: HashMap::new(),
        param_len: HashMap::new(),
        fixed_params: HashMap::new(),
        proc_defaults: HashMap::new(),
        implicit_names: HashSet::new(),
        type_bases: HashMap::new(),
        type_flags: HashSet::new(),
        types_above: HashMap::new(),
        type_order: HashMap::new(),
        type_const_bounds: HashMap::new(),
        layout_pending: HashSet::new(),
        auto_file: program.after_last_line.map(|t| program.tree(t).file),
        auto_procs: HashSet::new(),
        hidden_procs: Vec::new(),
        embeds: Vec::new(),
        embed_parents: HashMap::new(),
        color_file: program.colors.map(|t| program.tree(t).file),
        color_consts: HashSet::new(),
        color_set: None,
        first_color: None,
        redim_locals,
        redim_found: Vec::new(),
    };
    c.first_color = c.first_color_line(program);
    let skips = Skips::new(program);
    // Measured: an `OPTION _EXPLICIT` in an included file applies to the whole program, also before the include
    // (`verification\v19_explicit_*`).
    let has = |word: &[u8]| program.trees.iter().any(|t| has_option(map, t.root(), &skips, word));
    // An option is also in force by its precompiler flag, when its only line stands in the `$IF` branch the flag
    // switches off and so is not in the trees (measured, `verification\v26_x54`, `x55`; design D7 of
    // `m2-console-screenhide`).
    c.settings.explicit = has(b"_EXPLICIT") || program.flags.explicit();
    // Measured likewise for arrays (`verification\v24_x256`, `x257`): either option, above or below the use.
    c.settings.explicit_array = c.settings.explicit || has(b"_EXPLICITARRAY") || program.flags.explicit_array();
    // The main file's statements, and the same with every include statement followed by what it includes (pass 1
    // reads the user types and procedures of included files from these; pass 2 and the label pre-pass descend into
    // includes as they meet them).
    // QB64pe's file included after the last line follows the main file's statements, as if it stood there (design
    // D7 of `m3-network-embed`).
    let after_last_line = program.after_last_line.map(|t| program.tree(t).root());
    // QB64pe's colour file of `$COLOR` stands before the main file's first line (design D8 of
    // `m3-constant-evaluation`).
    let colors = program.colors.map(|t| program.tree(t).root());
    // The files of the `$USELIBRARY` libraries where the old compiler includes them (design D7 of
    // `m4-builtins-remaining`): the top files before the main file, then `aftermain.bas` (an `END`) and the
    // after-main files, then the bottom files, before `afterlastline.bm`.
    let roots = |ids: &[qb64rust_syntax::tree::TreeId]| ids.iter().map(|&t| program.tree(t).root()).collect::<Vec<_>>();
    let after_main = program.after_main.map(|t| program.tree(t).root());
    let top: Vec<Node> = colors
        .into_iter()
        .chain(roots(&program.library_top))
        .chain([root])
        .chain(after_main)
        .chain(roots(&program.library_after_main))
        .chain(roots(&program.library_bottom))
        .chain(after_last_line)
        .filter_map(ast::SourceFile::cast)
        .flat_map(|f| f.statements())
        .collect();
    let statements = expand(program, top.iter().copied());

    // Pass 1: the user types (a parameter may name one), then procedure names, then their parameters (a parameter may not have the name of any procedure).
    // A header with an error (a parse error, a reserved name...) still enters its name as a broken procedure, so
    // calls of it fail without a second error. A member array's bounds go by `OPTION BASE` as it stands at its
    // block, which follows the file (design D8 of `m2-defxxx-common-arrays`).
    let mut at_block = BlockSettings::default();
    let mut cur = walk::Cursor::new(program, &skips, top.iter().copied());
    c.collect_block_settings(&mut cur, &skips, &mut at_block);
    let type_blocks = std::mem::take(&mut c.type_blocks);
    c.declare_types(&type_blocks, &skips);
    // The default types follow the file (design D2): the table as it stands at each procedure's header.
    for &stmt in &top {
        c.collect_defaults(stmt, &skips);
    }
    let mut defs: Vec<ast::ProcDef> = statements
        .iter()
        .filter_map(|&s| ast::ProcDef::cast(s))
        .filter(|d| d.header().is_some() && !skips.past_cap(d.node()))
        .collect();
    // The functions of the file included after the last line are the language's: declared first, so that a
    // program's procedure of the same name is the one in error (measured, `verification\v44_x50`).
    defs.sort_by_key(|d| !c.in_after_last_line(d.node().span()));
    let mut declared = Vec::new();
    for &def in &defs {
        let header = def.header().unwrap();
        c.at.defaults = c.defaults_at(def);
        let id = if skips.usable(header.node()) {
            c.stmt.stmt_error = false;
            let id = c.declare_proc(header);
            c.flush_names();
            id
        } else {
            Err(Failed)
        };
        match id {
            Ok(id) => {
                let at = |n: Node| n.first_token().map_or(n.span(), |t| t.span);
                let line = c.line(at(header.node()));
                let file = header.node().span().file;
                let end = def.end().map(|e| e.node()).or_else(|| program.closer_of(def.node()));
                let (end_file, end_line) = end.map_or((file, line), |e| (at(e).file, c.line(at(e))));
                let proc = &mut c.prog.procs[id.0 as usize];
                (proc.file, proc.line) = (file, line);
                (proc.end_file, proc.end_line) = (end_file, end_line);
                declared.push((def, id))
            }
            Err(Failed) => c.declare_broken(header),
        }
    }
    // The declarations of `DECLARE LIBRARY` blocks, wherever they stand (design D1 of `m3-declare-library`): their
    // names are global and usable above the block (measured, `verification\v37_i`, `x49`).
    let mut externals = Vec::new();
    for block in c.library_blocks(&top) {
        c.declare_library(block, &skips, &mut externals);
    }
    for (def, id) in declared {
        c.stmt.stmt_error = false;
        c.at.defaults = c.defaults_at(def);
        let ok = c.declare_params(def.header().unwrap(), id).is_ok();
        c.flush_names();
        if ok {
            c.proc_of_def.insert(def.node().key(), id);
        } else {
            c.broken.insert(id);
        }
    }
    for d in externals {
        c.stmt.stmt_error = false;
        let id = d.id();
        let ok = c.external_params(d).is_ok();
        c.flush_names();
        if !ok {
            c.broken.insert(id);
        }
    }

    // The labels of each body, also those inside blocks, so that a jump or `ON ERROR GOTO` may name one further
    // down (design D5).
    c.collect_jump_words(&skips);
    c.declare_labels(&top, &defs, &skips);

    // Pass 2: everything in file order (procedures where they stand, included files at their include). The
    // default types start again and are set by each `DEFxxx` as it is met. The helpers of the file included after
    // the last line are known only in that file.
    c.at.defaults = DEFAULTS_AT_START;
    c.collect_embeds();
    for (name, _) in &c.hidden_procs {
        c.procs_by_name.remove(name);
    }
    let mut cur = walk::Cursor::new(program, &skips, top);
    while let Some(event) = cur.next() {
        if c.pp_dropped(event, &mut cur) {
            continue;
        }
        let (open, stmt) = match event {
            walk::Event::Stmt(n) => (false, n),
            walk::Event::Open(n) => (true, n),
            walk::Event::Branch(_) | walk::Event::Close(..) => continue,
        };
        if c.in_after_last_line(stmt.span()) {
            for (name, id) in std::mem::take(&mut c.hidden_procs) {
                c.procs_by_name.insert(name, id);
            }
        }
        if open {
            c.check_open(stmt, &mut cur, &skips);
        } else {
            c.check_statement(stmt, &skips);
        }
    }
    c.solve_dims();
    c.run_copy_checks();
    // The program has a console by its precompiler flag too: the old compiler reads the flag, which stays set
    // when the metacommand that set it stands in a `$IF` branch the flag itself switches off (design D1 of
    // `m2-console-screenhide`; measured, `verification\v26_x26`, `x38`). "No window" comes from the lines that are
    // compiled, so a `$CONSOLE:ONLY` in such a branch leaves the window.
    c.settings.console |= program.flags.console();
    // So are assertions: the old compiler writes the global from the feature's state, which a `$ASSERTS` met in any
    // round sets (`qb64pe.bas` 13088).
    c.settings.asserts |= program.flags.get(qb64rust_syntax::pp::Feature::Asserts) != 0;
    if !c.settings.console {
        // A program with neither metacommand is one with a window and no console (design D1 of
        // `m3-window-output`; measured, `verification\v27_*`: `console=0`, the same statement code).
        // Measured: "_ECHO requires $CONSOLE or $CONSOLE:ONLY to be set first" (`verification\v26_x17`).
        for at in std::mem::take(&mut c.settings.echoes) {
            c.diags
                .error(at, "`_ECHO` needs a program with `$CONSOLE` or `$CONSOLE:ONLY`");
        }
    }
    c.check_manifest_fields();
    c.prog.settings = crate::Settings {
        console: c.settings.console,
        window: !c.settings.console_only,
        window_hidden: c.settings.window_hidden,
        asserts: c.settings.asserts,
        resize: c.settings.resize,
        runtime_parts: std::mem::take(&mut c.settings.runtime_parts),
        compile_stamp: c.settings.compile_stamp,
        exe_icon: c.settings.exe_icon.take().map(|(icon, _)| icon),
        version_info: c.settings.version_info.take(),
        embedded: c.embedded_files(),
        error_location: c.settle_error_location(),
    };
    c.prog.shared = std::mem::take(&mut c.dim_shared);
    (c.prog, c.diags, c.redim_found)
}

/// In a procedure, a `REDIM` of a shared main-module array whose bound is the first place that names an array used
/// without `DIM`: the old compiler makes the name an array of the procedure in the whole procedure (measured,
/// `verification\v60_b1`–`b10`; design D2 of `m4-left-marked`). `plain`: the plain name names it.
#[derive(Clone, Debug)]
struct RedimLocal {
    proc: ProcId,
    name: String,
    ty: Ty,
    dims: usize,
    plain: bool,
}

/// One of the metacommands that are settings of the whole program: the four that choose its console and window,
/// and those of assertions and the debugger.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConsoleMeta {
    Console,
    ConsoleOnly,
    ScreenHide,
    ScreenShow,
    Asserts,
    AssertsConsole,
    Debug,
}

/// The setting metacommand a metacommand token is: `None` for any other metacommand, an error for a form of one of
/// them that the old compiler rejects. It compares the whole line, in upper case and without the blanks at its
/// ends, with the names, so anything else on the line is "Syntax error" (measured, `verification\v26_x09`–`x12`,
/// `x14`, `x29`–`x32`, `x35`, `x37`: `$CONSOLE:OFF`, a word, a comment or a colon and a statement after the name,
/// blanks around the colon of `$CONSOLE:ONLY`; `v33_x02`–`x13`: the same for `$ASSERTS`, `$ASSERTS:CONSOLE` and
/// `$DEBUG`).
fn console_meta(raw: &[u8]) -> Option<Result<ConsoleMeta, String>> {
    const NAMES: [(&str, ConsoleMeta); 7] = [
        ("$CONSOLE:ONLY", ConsoleMeta::ConsoleOnly),
        ("$CONSOLE", ConsoleMeta::Console),
        ("$SCREENHIDE", ConsoleMeta::ScreenHide),
        ("$SCREENSHOW", ConsoleMeta::ScreenShow),
        ("$ASSERTS:CONSOLE", ConsoleMeta::AssertsConsole),
        ("$ASSERTS", ConsoleMeta::Asserts),
        ("$DEBUG", ConsoleMeta::Debug),
    ];
    let text = meta_line(raw);
    for (name, meta) in NAMES {
        let Some(rest) = text.strip_prefix(name.as_bytes()) else {
            continue;
        };
        return match rest.first() {
            None => Some(Ok(meta)),
            // Another name that starts alike: `$CONSOLE:ONLYX` is a wrong form of `$CONSOLE` (measured,
            // `verification\v26_x60`), `$CONSOLEX` another metacommand.
            Some(b) if b.is_ascii_alphanumeric() || *b == b'_' => continue,
            Some(b':') if meta == ConsoleMeta::Console => {
                Some(Err("expected `$CONSOLE` or `$CONSOLE:ONLY`".to_string()))
            }
            Some(b':') if meta == ConsoleMeta::Asserts => {
                Some(Err("expected `$ASSERTS` or `$ASSERTS:CONSOLE`".to_string()))
            }
            Some(_) => Some(Err(format!("`{name}` must stand alone on its line"))),
        };
    }
    None
}

/// A `$RESIZE` line (design D3 of `m4-qb64pe-builtins`): the window's resizing and scaling ([`crate::Resize`]);
/// `None` for any other metacommand. The old compiler compares the whole line in upper case, so anything else after
/// `$RESIZE` is an error (measured, `verification\v53_x70`, `x71`, `x73`–`x75`, `x79`, `x81`–`x83`: "Syntax error",
/// and "Invalid expression" for `$RESIZE=ON`).
fn resize_meta(line: &[u8]) -> Option<Result<crate::Resize, &'static str>> {
    let rest = line.strip_prefix(b"$RESIZE".as_slice())?;
    if rest.first().is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_') {
        return None;
    }
    Some(match rest {
        b":OFF" => Ok(crate::Resize::Off),
        b":ON" => Ok(crate::Resize::On),
        b":STRETCH" => Ok(crate::Resize::Stretch),
        b":SMOOTH" => Ok(crate::Resize::Smooth),
        [b'=', ..] => Err("`$RESIZE` takes its setting after a colon, not `=`"),
        _ => Err("expected `$RESIZE:ON`, `:OFF`, `:STRETCH` or `:SMOOTH`, alone on its line"),
    })
}

/// The flag of a `$UNSTABLE` line (design D1 of `m3-typefields`; `qb64pe.bas` 2073–2089).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Unstable {
    /// `TYPEFIELDS`: the member markers.
    TypeFields,
    /// `MIDI` or `HTTP`: accepted, with a warning; nothing changes.
    Stable,
}

/// The flag a metacommand token sets when it is `$UNSTABLE`: `None` for any other metacommand, an error for a form
/// the old compiler rejects. It takes the line in upper case and without the blanks at its ends, and the flag
/// after the colon without its blanks, so any letter case and a blank after the colon are the same flag
/// (measured, `verification\v35_h`, `x13`); another flag or none is "Unrecognized unstable flag", and the name
/// without a colon "Syntax error" (`x10`–`x12`).
fn unstable_meta(raw: &[u8]) -> Option<Result<Unstable, String>> {
    let text = meta_line(raw);
    let rest = text.strip_prefix(b"$UNSTABLE".as_slice())?;
    match rest {
        [b':', flag @ ..] => Some(match meta_line(flag).as_slice() {
            b"TYPEFIELDS" => Ok(Unstable::TypeFields),
            b"MIDI" | b"HTTP" => Ok(Unstable::Stable),
            other => Err(format!("unrecognized `$UNSTABLE` flag \"{}\"", show_bytes(other))),
        }),
        // Another name that starts alike.
        [b, ..] if b.is_ascii_alphanumeric() || *b == b'_' => None,
        _ => Some(Err("expected `$UNSTABLE:` and a flag".to_string())),
    }
}

/// The bytes without the spaces at their ends (`LTRIM$`/`RTRIM$`, which take spaces only).
fn trim_spaces(s: &[u8]) -> &[u8] {
    let start = s.iter().position(|&b| b != b' ').unwrap_or(s.len());
    let end = s.iter().rposition(|&b| b != b' ').map_or(start, |e| e + 1);
    &s[start..end]
}

/// Checks the value of `FILEVERSION#` or `PRODUCTVERSION#` (`key`, in upper case). First the old compiler's check
/// (`qb64pe.bas` 3729–3739; measured, `verification\v40_x06`–`x13`, `x17`–`x19`): digits and commas only, three
/// commas, no comma at the end. Then each part as its resource compiler reads it (design D3 of `m3-versioninfo`;
/// `v40_h1`–`h7`): not empty, octal after a leading `0`, at most 65535.
fn version_number(key: &str, value: &[u8]) -> Result<(), String> {
    let shape = !value.is_empty()
        && value.iter().all(|&b| b.is_ascii_digit() || b == b',')
        && value.iter().filter(|&&b| b == b',').count() == 3
        && value.last() != Some(&b',');
    if !shape {
        return Err(format!(
            "expected `$VERSIONINFO:{key}=#,#,#,#` (four comma-separated numbers: major, minor, revision and build)"
        ));
    }
    for (n, part) in value.split(|&b| b == b',').enumerate() {
        let n = n + 1;
        if part.is_empty() {
            return Err(format!(
                "part {n} of `{key}` is empty; the resource compiler needs a number from 0 to 65535"
            ));
        }
        let octal = part.len() > 1 && part[0] == b'0';
        let radix = if octal { 8 } else { 10 };
        let number = part.iter().try_fold(0u32, |acc, &b| {
            let digit = u32::from(b - b'0');
            (digit < radix).then(|| acc * radix + digit).filter(|&v| v <= 65535)
        });
        if number.is_none() {
            let why = if octal { " (after a leading 0 it is octal)" } else { "" };
            return Err(format!(
                "part {n} of `{key}`, `{}`, is not a number from 0 to 65535 the resource compiler reads{why}",
                show_bytes(part)
            ));
        }
    }
    Ok(())
}

/// What in a value of the executable's manifest makes Windows refuse it (design D3 of `m3-versioninfo`): the
/// first `<` or `&`, or a byte that is not UTF-8; `None` when it has neither.
fn manifest_refusal(value: &[u8]) -> Option<String> {
    if let Some(&b) = value.iter().find(|&&b| b == b'<' || b == b'&') {
        return Some(format!("`{}`", b as char));
    }
    #[expect(clippy::disallowed_methods, reason = "whether the bytes are UTF-8 is the question")]
    let bad = std::str::from_utf8(value).err()?.valid_up_to();
    Some(format!("the byte {} (not UTF-8)", show_bytes(&value[bad..=bad])))
}

/// Which statements the checker skips: those with a parse error, and those past the parser's error cap. Kept per
/// tree, since one file included twice has two trees whose errors may differ.
struct Skips {
    /// Per tree: the start offsets of its parse errors.
    error_starts: HashMap<TreeId, Vec<u32>>,
    /// Per capped tree: where the parser stopped recording errors.
    cap_start: HashMap<TreeId, u32>,
}

impl Skips {
    fn new(program: &ParsedProgram) -> Skips {
        let mut skips = Skips {
            error_starts: HashMap::new(),
            cap_start: HashMap::new(),
        };
        for t in &program.trees {
            let list = t.diagnostics.list();
            skips
                .error_starts
                .insert(t.id, list.iter().map(|d| d.span.start).collect());
            // Past the error cap the parser records no more errors, so statements from the cap on may be malformed.
            if t.diagnostics.is_capped() {
                skips.cap_start.insert(t.id, list.last().map_or(0, |d| d.span.start));
            }
        }
        skips
    }

    fn past_cap(&self, node: Node) -> bool {
        self.cap_start.get(&node.tree).is_some_and(|&c| node.span().end >= c)
    }

    fn usable(&self, node: Node) -> bool {
        let s = node.span();
        let errors = self.error_starts.get(&node.tree).map_or(&[][..], Vec::as_slice);
        node.kind() != SyntaxKind::Error && !self.past_cap(node) && !errors.iter().any(|&o| o >= s.start && o <= s.end)
    }
}

/// The variables a name can mean in one scope: a name plus a type (design D5 of the last change). Arrays have a
/// name space of their own (design D4 of `m2-arrays-and-types`).
#[derive(Clone, Default)]
struct Scope {
    /// (name, type) -> variable.
    vars: HashMap<(String, Ty), VarId>,
    /// Type of the plain (suffix-less) name after a `DIM name AS type` (or a parameter or `SHARED` with `AS`).
    plain: HashMap<String, Ty>,
    /// (name, element type) -> array.
    arrays: HashMap<(String, Ty), VarId>,
    /// Element type of an array's plain name after `DIM name(…) AS type`.
    array_plain: HashMap<String, Ty>,
}

struct Checker<'a> {
    /// The procedures' arrays of a `REDIM` that a first check found ([`check_with`]), made where each procedure
    /// starts, and those this check finds.
    redim_locals: Vec<RedimLocal>,
    redim_found: Vec<RedimLocal>,
    map: &'a SourceMap,
    /// Every tree, for the included ones ([`Checker::expand`]).
    program: &'a ParsedProgram,
    /// The lines of each file that end with a line continuation (`_`), so that the next line continues them.
    continued: HashSet<(FileId, u32)>,
    prog: Program,
    main: Scope,
    /// The scope of the procedure being checked (only meaningful while `cur` is set).
    local: Scope,
    /// The procedure being checked; `None` in the main module.
    cur: Option<ProcId>,
    procs_by_name: HashMap<String, ProcId>,
    /// The library procedure of a name that a program procedure of the other kind has too, a SUB beside a
    /// FUNCTION ([`Self::proc_as`]).
    proc_twins: HashMap<String, ProcId>,
    /// Key of a `ProcDef` node whose header was declared without errors -> its procedure.
    proc_of_def: HashMap<(TreeId, u32), ProcId>,
    /// Per procedure: the scope holding its parameters, which each check of the body starts from.
    param_scopes: Vec<Scope>,
    /// Procedures whose parameter list has an error: calls of them fail without a second error.
    broken: HashSet<ProcId>,
    /// Procedures with `STATIC` after their header, and whether the body being checked still gives its own
    /// ([`FileState::static_scope`]) variables and arrays the life of the program: it does from its header to the end of its first `STATIC`
    /// statement, and below that statement it is a procedure like any other (measured,
    /// `verification\v24_x279`; the old compiler's flag for it is reset by that statement, `qb64pe.bas` 5827,
    /// 8805, 9558).
    static_procs: HashSet<ProcId>,
    /// Main-module variables declared with `DIM SHARED` so far (in file order), and the plain names they typed.
    dim_shared: HashSet<VarId>,
    dim_shared_plain: HashMap<String, Ty>,
    /// The labels by body (`None`: the main module) and name, and the label of each `LabelDef` node (by its key).
    labels_by_name: HashMap<(Option<ProcId>, String), LabelId>,
    label_of_def: HashMap<(TreeId, u32), LabelId>,
    /// The words the jumps of the whole program name as their targets, in upper case ([`Self::collect_jump_words`]).
    jump_words: HashSet<String>,
    /// The words on lines the parser could not read: one of them may be a jump's target.
    doubt_words: HashSet<String>,
    /// The lines that are a SUB's name and a colon and call the SUB.
    sub_call_lines: HashMap<(TreeId, u32), ProcId>,
    diags: Diagnostics,
    /// What follows the file as it is read.
    at: FileState,
    /// What the program's metacommands and options set wherever they stand.
    settings: Settings,
    /// The statement being checked.
    stmt: StmtState,
    /// The number of dimensions of the arrays whose number is found after the walk.
    dims: DimSolver,
    /// Constants by name (without suffix): the main module's so far, and those of the procedure being checked.
    consts_main: HashMap<String, ConstId>,
    consts_local: HashMap<String, ConstId>,
    /// The auto-included constants named so far, by their place in the file: the constant each became, or why
    /// its value is not computed yet (`auto.rs`).
    auto_consts: HashMap<usize, Result<ConstId, String>>,
    /// The constants whose value took a correctly rounded one where the old compiler's depends on the machine
    /// (`DIVERGENCES.md` D-026): a `CONST` that names one warns too, and so does a use of an auto-included one.
    fell_back: HashSet<ConstId>,
    /// The library entry the OpenGL routines named so far belong to, made with the first (`opengl.rs`).
    gl_library: Option<crate::LibraryId>,
    /// The OpenGL constants named so far, each a constant of the main module (`opengl.rs`).
    gl_consts: HashSet<ConstId>,
    /// The first use or declaration of each variable name so far, anywhere, and in the procedure being checked:
    /// a later `CONST` of the name is an error there (design D6).
    used_names: HashMap<String, Span>,
    used_local: HashMap<String, Span>,
    /// The statement lists of the blocks being checked, innermost last; [`Self::push`] adds to the last one.
    sinks: Vec<Vec<Stmt>>,
    /// The last `NEXT` statement (by key) whose variable did not match its `FOR` (`check\blocks.rs`).
    bad_next: Option<(TreeId, u32)>,
    /// Inside a `$IF` the old compiler's main pass compiles no branch of ([`Self::pp_dropped`]): how many `$IF`s
    /// are open inside it.
    pp_drop: Option<u32>,
    /// The names of the `CONST`s such a `$IF`'s first-pass branch defines: in use for the code below.
    pp_ghosts: HashSet<String>,
    /// The user types by name.
    types_by_name: HashMap<String, TypeId>,
    /// Every `TYPE` block of the program in file order, wherever it stands, as pass 1 lists them.
    type_blocks: Vec<ast::TypeBlock<'a>>,
    /// The `TYPE` blocks of pass 1 (by key) that define a type.
    type_defs: HashSet<(TreeId, u32)>,
    /// Element types of the plain names of main-module `DIM SHARED` arrays declared so far.
    dim_shared_array_plain: HashMap<String, Ty>,
    /// `TYPE` blocks (by key) that pass 1 marked "not supported yet".
    types_marked: HashSet<(TreeId, u32)>,
    /// The names of the `TYPE` blocks pass 1 did not enter (an error or a mark), in capitals: an `AS` clause that
    /// names one is "not supported yet", not "Unknown type".
    types_failed: HashSet<String>,
    /// The name of every `CONST` of the program in file order, in capitals without suffix, and whether it stands
    /// in a SUB or FUNCTION, as pass 1 walks to the `TYPE` blocks ([`Self::collect_block_settings`]); and for
    /// each block (by key) how many stand above it and whether the block stands in one.
    const_names: Vec<(String, bool)>,
    consts_above: HashMap<(TreeId, u32), (usize, bool)>,
    /// The same for each `CONST` statement (by key): how many of [`Self::const_names`] stand above it.
    consts_before: HashMap<(TreeId, u32), usize>,
    /// The parameters declared `STRING * n` or `t$n`, with n: `STRING` parameters whose `LEN` is n (design D6 of
    /// `m2-numeric-types`).
    param_len: HashMap<VarId, u32>,
    /// The `t$n` parameters by procedure, name and n: the name `t$n` in the body means the parameter (`t$` is
    /// another variable).
    fixed_params: HashMap<(ProcId, String, u32), VarId>,
    /// Per `ProcDef` and `DeclareLibraryBlock` node (by key): the table at its header.
    proc_defaults: HashMap<(TreeId, u32), [Ty; 27]>,
    /// The names met as `name(…)` or in `LBOUND`/`UBOUND` that name no array and for which none was made, with a
    /// "not supported yet" mark ([`Self::implicit_array`]). An `ERASE` of such a name is marked too, where any
    /// other name that is no array is an error.
    implicit_names: HashSet<String>,
    /// Per `TYPE` block (by key): `OPTION BASE` as it stands at the block, which the bounds of its member arrays
    /// go by ([`Self::collect_block_settings`]).
    type_bases: HashMap<(TreeId, u32), i64>,
    /// The `TYPE` blocks (by key) that stand below a `$UNSTABLE:TYPEFIELDS`, where the member markers are allowed
    /// ([`Self::collect_block_settings`]).
    type_flags: HashSet<(TreeId, u32)>,
    /// Per SUB or FUNCTION header (by key): the number of `TYPE` blocks above it in the file, the first of
    /// [`Self::type_blocks`] (design D5 of `m4-builtins-param-types`).
    types_above: HashMap<(TreeId, u32), usize>,
    /// Per user type: the index of its block in [`Self::type_blocks`].
    type_order: HashMap<TypeId, usize>,
    /// The `TYPE` blocks (by key, with their type) with a member array whose bounds name a `CONST`: they are read
    /// when pass 2 reaches the block, below the constants above it ([`Self::type_block`]).
    type_const_bounds: HashMap<(TreeId, u32), TypeId>,
    /// The user types whose layout waits for such bounds of a static member array, until pass 2 reaches their
    /// block ([`Self::waits_for_layout`]; design D7 of `m4-memory-records`).
    layout_pending: HashSet<TypeId>,
    /// The source file of QB64pe's file included after the last line, when the program has it (design D7 of
    /// `m3-network-embed`).
    auto_file: Option<FileId>,
    /// The procedures of that file, and those of them the program's own code cannot call (`_WhatIsMyIP$`): they
    /// are out of [`Self::procs_by_name`] while the program's own statements are checked.
    auto_procs: HashSet<ProcId>,
    hidden_procs: Vec<(String, ProcId)>,
    /// The program's compiled `$EMBED` lines in file order, and each included tree's `$INCLUDE` (design D1 of
    /// `m3-network-embed`, `check\embed.rs`).
    embeds: Vec<embed::EmbedLine>,
    embed_parents: HashMap<TreeId, (TreeId, u32)>,
    /// QB64pe's colour file of `$COLOR` (design D8 of `m3-constant-evaluation`), its constants, the set the first
    /// `$COLOR` checked chose (1 `:0`, 2 `:32`), and the place in the old compiler's first pass (an offset of the
    /// main file, [`Self::main_offset`]) of the first `$COLOR` line ([`StmtState::const_at`] is that place of the
    /// `CONST` being checked).
    color_file: Option<FileId>,
    color_consts: HashSet<ConstId>,
    color_set: Option<u8>,
    first_color: Option<u32>,
}

/// What follows the file as it is read ([`Checker::at`]): each holds from the statement that sets it on, in file
/// order.
struct FileState {
    /// The default type of a name without a suffix or `AS` clause, by its first letter: A to Z, then `_` (design
    /// D2). It is the table at the point of the file being read: pass 1 sets it from [`Checker::proc_defaults`] for
    /// each header, pass 2 changes it at each `DEFxxx` and `_DEFINE` statement, wherever that stands.
    defaults: [Ty; 27],
    /// `OPTION BASE` as it stands at this point of the file, 0 or 1 (design D3): the lower bound of a dimension
    /// written without `TO`. Like the default types it follows the file and nothing else.
    option_base: i64,
    /// `$DYNAMIC` is in effect at this point of the file (the last of `'$DYNAMIC` and `'$STATIC` above it): a `DIM`
    /// with constant bounds then makes a dynamic array. It follows the file too (measured,
    /// `verification\v24_b_metacommand*`).
    dynamic_mode: bool,
    /// No `$CHECKING:OFF` is in force at this point of the file (design D3 of `m4-handlers-logging-checking`): each
    /// statement takes it ([`Stmt::checked`]).
    checking: bool,
    /// Whether the body being checked still gives its own variables and arrays the life of the program
    /// ([`Checker::static_procs`]).
    static_scope: bool,
    /// The follow-on rule is on (design D10): a declaration was marked "not supported yet", so real errors are
    /// dropped from here on.
    follow_on: bool,
}

impl FileState {
    fn new() -> FileState {
        FileState {
            defaults: DEFAULTS_AT_START,
            option_base: 0,
            dynamic_mode: false,
            checking: true,
            static_scope: false,
            follow_on: false,
        }
    }
}

/// What the program's metacommands and options set wherever they stand, read after the walk ([`Checker::settings`]).
struct Settings {
    /// The program's console and window, from the metacommands of its compiled code, wherever they stand (design
    /// D1 of `m2-console-screenhide`): a `$CONSOLE` or `$CONSOLE:ONLY` was met; a `$CONSOLE:ONLY` was met (the
    /// program has no window); the last of `$SCREENHIDE` and `$SCREENSHOW` met was `$SCREENHIDE`.
    console: bool,
    console_only: bool,
    window_hidden: bool,
    /// A `$ASSERTS` or `$ASSERTS:CONSOLE` was met in the compiled code.
    asserts: bool,
    /// The window's resizing and scaling, from the last `$RESIZE` met ([`resize_meta`]).
    resize: crate::Resize,
    /// A call of `_COMPILEDATE$` or `_COMPILETIME$` was met.
    compile_stamp: bool,
    /// The first `$EXEICON` met, with the node it is (its tree and offset).
    exe_icon: Option<(crate::ExeIcon, (TreeId, u32))>,
    /// The fields the `$VERSIONINFO` lines met so far set, and the line that last set each string field (for the
    /// manifest check after the walk, design D3 of `m3-versioninfo`).
    version_info: Option<crate::VersionInfo>,
    version_lines: [Option<Span>; crate::VERSION_STRINGS],
    /// The `_ECHO` statements met: each is an error when the program turns out to have no console, which is known
    /// only after the walk (design D5 of `m2-console-screenhide`; measured, `verification\v26_x17`, `x62`).
    echoes: Vec<Span>,
    /// The `$ERRORLOCATION:ON` (true) and `:OFF` (false) lines met, in file order (design D6 of
    /// `m4-builtins-remaining`): the last one is the program's setting, the first that differs from it is an error
    /// after the walk ([`Checker::settle_error_location`]).
    error_location: Vec<(Span, bool)>,
    /// The optional parts of the runtime the built-ins met so far need.
    runtime_parts: std::collections::BTreeSet<crate::RuntimePart>,
    /// `OPTION _EXPLICIT` stands somewhere in the program (it applies to the whole program, design D7).
    explicit: bool,
    /// `OPTION _EXPLICIT` or `OPTION _EXPLICITARRAY` stands somewhere in the program: an array used without
    /// `DIM` is an error.
    explicit_array: bool,
    /// Integer constant folding is on (off for tests only, [`check_with`]).
    fold: bool,
}

impl Settings {
    fn new(fold: bool) -> Settings {
        Settings {
            console: false,
            console_only: false,
            window_hidden: false,
            asserts: false,
            resize: crate::Resize::Off,
            compile_stamp: false,
            exe_icon: None,
            version_info: None,
            version_lines: [None; crate::VERSION_STRINGS],
            echoes: Vec::new(),
            error_location: Vec::new(),
            runtime_parts: std::collections::BTreeSet::new(),
            explicit: false,
            explicit_array: false,
            fold,
        }
    }
}

/// The statement being checked ([`Checker::stmt`]).
#[derive(Default)]
struct StmtState {
    /// The statement had a real error or a "not supported yet" mark (at most one per statement).
    stmt_error: bool,
    /// Names resolved in the current statement, for the symbol table.
    names: Vec<(SymbolKind, Span)>,
    /// What is being checked is a place where an allocation that runs first in the statement would not be what
    /// the old compiler does ([`Checker::implicit_array`], [`Checker::unjoined`]).
    no_join: Option<NoJoin>,
    /// What the name being checked stands for ([`Checker::placing`]).
    placing: Placing,
    /// The place in the old compiler's first pass of the `CONST` being checked ([`Checker::color_file`]).
    const_at: Option<u32>,
    /// How many `CONST` names of the program stand above the `CONST` being checked ([`Checker::const_names`]).
    consts_before: Option<usize>,
}

/// The number of dimensions of the arrays whose number is not known where they are declared, found after every
/// body is checked ([`Checker::dims`], [`Checker::solve_dims`]).
#[derive(Default)]
struct DimSolver {
    /// What says how many dimensions an array has whose number is not known where it is declared, in file order
    /// ([`DimFact`]).
    dim_facts: Vec<DimFact>,
    /// The arrays listed by `COMMON a()` or `STATIC a()` that were passed to an array parameter before anything
    /// gave them a number of dimensions: theirs is found with the parameter's.
    dims_open: HashSet<VarId>,
    /// The checks of `_ARRAYCOPY` statements whose sides' numbers of dimensions were not known where they stand,
    /// run after [`Checker::solve_dims`] (design D3 of `m3-arraycopy`).
    copy_checks: Vec<arraycopy::CopyCheck>,
}

/// What pass 1 keeps as it walks the file in order to the `TYPE` blocks: `OPTION BASE` and whether
/// `$UNSTABLE:TYPEFIELDS` stood above.
#[derive(Clone, Copy, Debug, Default)]
struct BlockSettings {
    base: i64,
    typefields: bool,
    /// The walk is in a SUB or FUNCTION.
    in_proc: bool,
}

/// One thing that says how many dimensions an array has, for an array whose number is found from everything that
/// names it (design D6 of `m2-defxxx-common-arrays`): an array parameter, and a listed array passed to one.
enum DimFact {
    /// `array` (`shown`, as written) is named with `count` indexes or bounds at `span`; with `passed`, it is a
    /// parameter that is passed an array of `count` dimensions there.
    Count {
        array: VarId,
        shown: String,
        count: usize,
        span: Span,
        passed: bool,
    },
    /// `arg`, whose number is not known either, is passed to the parameter `param` at `span`: they have one
    /// number.
    Same {
        arg: VarId,
        param: VarId,
        shown: String,
        span: Span,
        /// `arg` is the source of a whole-array assignment and `param` its target.
        copied: bool,
    },
}

/// What a name with an index that names no array stands for, by which the old compiler words its error
/// ([`Checker::implicit_array`], [`Checker::call_place`]; measured, `verification\v55_x10`–`x20`, `x76`–`x79`).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Placing {
    /// A value: "Name already in use" for a SUB's or FUNCTION's name, "Expected (...)" for a built-in function's.
    #[default]
    Value,
    /// The place of an assignment: "Expected variable =".
    Store,
    /// A target of `READ`, `INPUT`, `LINE INPUT` or an operand of `SWAP`: "Expected variable".
    Target,
}

/// Where an array used without `DIM` is not made by its first use ([`Checker::implicit_array`]): the old
/// compiler allocates it where the name is evaluated, which is the start of the statement only when the
/// statement writes all its code after evaluating all its parts.
#[derive(Clone, Copy, PartialEq, Eq)]
enum NoJoin {
    /// The line of a block (`IF`, `ELSEIF`, `FOR`, `DO`, `LOOP`, `WHILE`, `SELECT CASE`, `CASE`), which is no
    /// statement of its own (measured, `verification\v24_x262`).
    Header,
    /// Part `n` of a statement that works item by item, after the part of it that runs first: a later item of
    /// `PRINT` or `READ`, an item of `WRITE` or of `PRINT #`, a target of `INPUT` or `LINE INPUT`, the format or a
    /// value of `PRINT USING` (measured, `verification\v24_x264`, `x265`: the earlier items are printed or read,
    /// the prompt is shown, then the allocation raises its error). The allocation runs there ([`crate::Join::Before`]).
    Item(usize),
    /// A bound of `REDIM`: the array is then dynamic, also where `$DYNAMIC` is not in effect, and is allocated in
    /// the `REDIM`'s own way (its mode), so a second run makes it anew or keeps it and raises nothing (measured,
    /// `verification\v24_x266`, `x268`).
    RedimBound(crate::AllocMode),
}

/// Every default type is SINGLE until a `DEFxxx` or `_DEFINE` says otherwise.
const DEFAULTS_AT_START: [Ty; 27] = [Ty::F32; 27];

/// An expression could not be typed; the error is already reported.
struct Failed;

type R<T> = Result<T, Failed>;

impl Checker<'_> {
    /// A real error, at most one per statement. After the follow-on rule started (design D10: a declaration was
    /// marked "not supported yet" earlier in file order) it is dropped: the statement fails without a diagnostic,
    /// and the program is rejected by that mark already.
    fn error(&mut self, span: Span, msg: impl Into<String>) -> Failed {
        if !self.stmt.stmt_error {
            self.stmt.stmt_error = true;
            if !self.at.follow_on {
                self.diags.error(span, msg);
            }
        }
        Failed
    }

    /// Like [`Self::error`], for a construct not handled yet: marked "not supported yet", `msg` names it.
    fn unsupported(&mut self, span: Span, msg: impl Into<String>) -> Failed {
        if !self.stmt.stmt_error {
            self.stmt.stmt_error = true;
            self.diags.unsupported(span, msg);
        }
        Failed
    }

    fn in_use(&mut self, t: Tok) -> Failed {
        let msg = format!("name already in use: `{}`", show_bytes(self.text(t.span)));
        self.error(t.span, msg)
    }

    fn text(&self, span: Span) -> &[u8] {
        self.map.text(span)
    }

    /// The 1-based line of a span in its own file, as the old compiler counts it: a line joined to the next by a
    /// line continuation is numbered as the last line of the join (measured, `verification\v43_d_continued`,
    /// `v43_e_continued_untrapped`: `ERROR 5: x = 1 + _` reports the line below it).
    fn line(&self, span: Span) -> u32 {
        let mut line = self.map.file(span.file).line_col(span.start).0;
        while self.continued.contains(&(span.file, line)) {
            line += 1;
        }
        line
    }

    /// The token in upper case, other than printable ASCII escaped (`\xC9`).
    fn word(&self, t: Tok) -> String {
        show_bytes(&self.name_text(t).to_ascii_uppercase())
    }

    /// The text of a name's token without blanks: a declared name written with blanks around its dots (`DIM a .
    /// b`) is one token over all of it ([`ast::DimItem::name`]), which the old compiler reads as `a.b` (design D7 of
    /// `m4-names-implicit-arrays`). Any other name's text has no blank.
    fn name_text(&self, t: Tok) -> std::borrow::Cow<'_, [u8]> {
        let bytes = self.text(t.span);
        if bytes.iter().any(|&b| b == b' ' || b == b'\t') {
            std::borrow::Cow::Owned(bytes.iter().copied().filter(|&b| b != b' ' && b != b'\t').collect())
        } else {
            std::borrow::Cow::Borrowed(bytes)
        }
    }

    /// The words of the type an `AS` clause names, each as [`Self::word`] gives it (`_UNSIGNED`, `INTEGER`).
    fn as_words(&self, a: ast::AsClause) -> Vec<String> {
        a.type_words().map(|t| self.word(t)).collect()
    }

    /// Whether `span` lies in QB64pe's file included after the last line (design D7 of `m3-network-embed`).
    fn in_after_last_line(&self, span: Span) -> bool {
        self.auto_file == Some(span.file)
    }

    /// The program's `$ERRORLOCATION` setting (design D6 of `m4-builtins-remaining`): the last line's. The old
    /// compiler's first pass keeps the last setting it meets and its main pass stops at the first line that
    /// disagrees, so one `ON` and one `OFF` in any order, also across an included file or in a `$IF` branch that
    /// is taken, are its error at the program's first line whose setting differs from the last (measured,
    /// `verification\v58_e07`–`e13`).
    fn settle_error_location(&mut self) -> bool {
        let lines = std::mem::take(&mut self.settings.error_location);
        let Some(&(_, setting)) = lines.last() else {
            return false;
        };
        if let Some(&(span, _)) = lines.iter().find(|&&(_, on)| on != setting) {
            self.diags.error(
                span,
                "`$ERRORLOCATION` is a whole-program option and cannot be changed within the program",
            );
        }
        setting
    }

    /// Records the names of the statement just checked, unless it had an error (one error per statement).
    fn flush_names(&mut self) {
        let names = std::mem::take(&mut self.stmt.names);
        if !self.stmt.stmt_error {
            self.prog.symbols.record(names);
        }
    }

    /// Pass 1 of the default types (design D2; measured, `verification\v24_a_procs_subs`, `v24_x30`): walks the
    /// file in order, through blocks, procedure bodies and included files, applies every `DEFxxx` and `_DEFINE`
    /// that pass 2 will apply, and keeps the table as it stands at each procedure's header. A statement with an
    /// error changes nothing here and is reported by pass 2.
    fn collect_defaults(&mut self, node: Node, skips: &Skips) {
        if let Some(s) = ast::DefTypeStmt::cast(node) {
            if skips.usable(node) {
                let (stmt_error, names) = (self.stmt.stmt_error, std::mem::take(&mut self.stmt.names));
                // With the statement's error "already reported", nothing is recorded.
                self.stmt.stmt_error = true;
                let _ = self.def_type(s);
                (self.stmt.stmt_error, self.stmt.names) = (stmt_error, names);
            }
            return;
        }
        if ast::InactiveCode::cast(node).is_some() {
            return;
        }
        if ast::ProcDef::cast(node).is_some() || ast::DeclareLibraryBlock::cast(node).is_some() {
            self.proc_defaults.insert(node.key(), self.at.defaults);
        }
        if ast::MetaCommentStmt::cast(node).is_some() {
            let program = self.program;
            if let Some(tree) = program.included(node) {
                self.collect_defaults(tree.root(), skips);
            }
            return;
        }
        for n in node.child_nodes() {
            self.collect_defaults(n, skips);
        }
    }
}

impl<'a> Checker<'a> {
    /// A statement of the walk ([`walk::Event::Stmt`]). An include statement is one too: the walk gives the
    /// included file's statements after it, in its place (design D10).
    fn check_statement(&mut self, stmt: Node<'a>, skips: &Skips) {
        if self.block_statement(stmt, skips) {
            return;
        }
        self.check_one(stmt, skips);
    }

    /// Whether the walk's `event` belongs to a `$IF` whose condition the old compiler's two passes read differently
    /// (design D9 of `m4-statements-metacommands`): its main pass compiles none of the branches, so the event is
    /// dropped, a block with all its events. A `CONST` of the branch the parser took, which the first pass takes,
    /// leaves its name in use for the code below ([`Self::pp_ghosts`]); nothing else of it has an effect
    /// (measured, `verification\v26_x34`, `v57_g01`–`g11`).
    pub(super) fn pp_dropped(&mut self, event: walk::Event<'a>, cur: &mut walk::Cursor<'a, '_>) -> bool {
        if self.pp_drop.is_some()
            && let walk::Event::Stmt(n) = event
            && let Some(c) = ast::ConstStmt::cast(n)
        {
            for t in c.items().filter_map(ast::ConstItem::name) {
                let word = self.text(t.span);
                let name = show_bytes(&word[..decl::name_end(word)].to_ascii_uppercase());
                self.pp_ghosts.insert(name);
            }
        }
        let mut drop = self.pp_drop;
        let dropped = self.pp_drop_event(&mut drop, event, cur);
        self.pp_drop = drop;
        dropped
    }

    /// The part of [`Self::pp_dropped`] the label pass shares ([`Self::declare_labels`]): `drop` is how many `$IF`s
    /// are open inside a dropped `$IF` (`None` outside one). A disagreeing `$IF` or `$ELSEIF` starts it, and is
    /// itself not dropped; its `$END IF` ends it. A block opened in the dropped part is dropped whole; the
    /// metacommands in it are still counted, so that an `$END IF` inside it (a block that crosses the `$END IF`, a
    /// parse error) ends the drop at the block's end, not at the end of the program.
    pub(super) fn pp_drop_event(
        &self,
        drop: &mut Option<u32>,
        event: walk::Event<'a>,
        cur: &mut walk::Cursor<'a, '_>,
    ) -> bool {
        let Some(depth) = *drop else {
            if let walk::Event::Stmt(n) = event
                && let Some(tok) = ast::MetaStmt::cast(n).and_then(|m| m.token())
                && self.program.passes_disagree.contains(&(n.key().0, tok.span.start))
            {
                *drop = Some(0);
            }
            return false;
        };
        match event {
            walk::Event::Open(_) => {
                let mut open = 1;
                while open > 0 {
                    match cur.next() {
                        None => break,
                        Some(walk::Event::Open(_)) => open += 1,
                        Some(walk::Event::Close(..)) => open -= 1,
                        Some(walk::Event::Stmt(n)) => {
                            if let Some(d) = *drop {
                                *drop = self.pp_drop_step(d, n);
                            }
                        }
                        Some(walk::Event::Branch(_)) => {}
                    }
                }
            }
            walk::Event::Stmt(n) => *drop = self.pp_drop_step(depth, n),
            walk::Event::Branch(_) | walk::Event::Close(..) => return false,
        }
        true
    }

    /// The `$IF`s open inside a dropped `$IF` after the statement `n` ([`Self::pp_drop_event`]), from `depth`
    /// before it; `None` after its own `$END IF`.
    fn pp_drop_step(&self, depth: u32, n: Node) -> Option<u32> {
        use qb64rust_syntax::pp::Directive;
        let Some(tok) = ast::MetaStmt::cast(n).and_then(|m| m.token()) else {
            return Some(depth);
        };
        match qb64rust_syntax::pp::directive(self.text(tok.span)) {
            Directive::If(_) | Directive::Malformed(_) => Some(depth + 1),
            Directive::EndIf => depth.checked_sub(1),
            Directive::ElseIf(_) | Directive::Else | Directive::Let(_) | Directive::Error(_) | Directive::Other => {
                Some(depth)
            }
        }
    }

    /// A block the walk opened ([`walk::Event::Open`]); its events are taken up to its end.
    fn check_open(&mut self, node: Node<'a>, cur: &mut walk::Cursor<'a, '_>, skips: &Skips) {
        match ast::ProcDef::cast(node) {
            Some(def) => self.proc_def(def, cur, skips),
            None => self.open_block(node, cur, skips),
        }
    }

    /// A procedure where it stands: in the main module (or a file included there), its body is checked in its own
    /// scope; anywhere else it came from a file included inside a SUB, FUNCTION or block (the parser keeps every
    /// other `SUB` at file level), which the old compiler rejects (measured M7: "Expected END SUB/FUNCTION before
    /// SUB"), and its body is not checked.
    fn proc_def(&mut self, def: ast::ProcDef<'a>, cur: &mut walk::Cursor<'a, '_>, skips: &Skips) {
        if self.cur.is_some() || !self.sinks.is_empty() {
            if let Some(h) = def.header().filter(|h| skips.usable(h.node())) {
                self.stmt.stmt_error = false;
                let _ = self.error(
                    first_token_span(h.node()),
                    "a SUB or FUNCTION from an included file cannot stand inside a SUB, FUNCTION or block",
                );
            }
            cur.skip_block();
            return;
        }
        let Some(&id) = self.proc_of_def.get(&def.node().key()) else {
            cur.skip_block();
            return;
        };
        self.cur = Some(id);
        self.local = self.param_scopes[id.0 as usize].clone();
        // The arrays a `REDIM` of the procedure makes its own, from its first statement (design D2 of
        // `m4-left-marked`): made on each call, also in a procedure with `STATIC` after its header (measured,
        // `verification\v60_b5`), and allocated by a `REDIM`.
        for r in self.redim_locals.iter().filter(|r| r.proc == id) {
            let array = VarId(to_u32(self.prog.vars.len()));
            self.prog.vars.push(Var {
                conventional: false,
                name: r.name.clone(),
                ty: r.ty,
                storage: Storage::Local(id),
                shape: Shape::Dynamic {
                    dims: Some(r.dims),
                    at_start: None,
                },
            });
            self.local.arrays.insert((r.name.clone(), r.ty), array);
            if r.plain {
                self.local.array_plain.insert(r.name.clone(), r.ty);
            }
        }
        self.at.static_scope = self.static_procs.contains(&id);
        self.consts_local.clear();
        self.used_local.clear();
        while let Some(walk::Event::Branch(_)) = self.walk_to_end(cur, skips) {}
        self.cur = None;
    }

    /// Pass 1 of `OPTION BASE` (design D8 of `m2-defxxx-common-arrays`; measured, `verification\v24_x182`,
    /// `x183`, `x217`): the `TYPE` blocks are read before the statements, and a member array declared without
    /// `TO` starts at the base in effect where its block stands. Walks the file in order as
    /// [`Self::collect_defaults`] does, applies every `OPTION BASE` that pass 2 will apply (wherever it stands,
    /// whether or not it runs) and keeps the base at each `TYPE` block. A statement with an error changes
    /// nothing here and is reported by pass 2.
    ///
    /// The walk lists every `TYPE` block in file order, in the main module, a procedure or a block alike, for
    /// [`Self::declare_types`] (design D4 of `m4-names-arrays`), and the names of the `CONST`s above each, for
    /// its member arrays' bounds (design D2 of `m4-upstream-rejections`).
    ///
    /// `$UNSTABLE:TYPEFIELDS` is read in the same walk (design D1 of `m3-typefields`; `qb64pe.bas` 2071–2090):
    /// the old compiler sets the flag as it reads the lines in order, before its first pass, so it allows the
    /// markers in the blocks below it, also after the first statement, and not in a `$IF` branch that is not taken
    /// (measured, `verification\v35_x14`, `x15`, `x45`, `x47`).
    ///
    /// The file is read by the walk of pass 2 ([`walk::Cursor`]), so a SUB opened in one file and closed in another
    /// holds the same statements here as there: a `CONST` in its part in the other file is the SUB's (measured,
    /// `verification\v57_b24`, `b25`).
    fn collect_block_settings(&mut self, cur: &mut walk::Cursor<'a, '_>, skips: &Skips, at: &mut BlockSettings) {
        let mut open = Vec::new();
        while let Some(event) = cur.next() {
            match event {
                walk::Event::Open(n) => {
                    open.push(at.in_proc);
                    if let Some(h) = ast::ProcDef::cast(n).and_then(|d| d.header()) {
                        // A parameter's type must be defined above the header, which the walk reaches before the
                        // body.
                        self.types_above.insert(h.node().key(), self.type_blocks.len());
                    }
                    at.in_proc |= ast::ProcDef::cast(n).is_some();
                }
                walk::Event::Close(..) => {
                    if let Some(outer) = open.pop() {
                        at.in_proc = outer;
                    }
                }
                walk::Event::Branch(_) => {}
                walk::Event::Stmt(n) => match line_if_statements(n) {
                    Some(inner) => {
                        let program = self.program;
                        let mut sub = walk::Cursor::new(program, skips, inner);
                        self.collect_block_settings(&mut sub, skips, at);
                    }
                    None => self.block_setting(n, skips, at),
                },
            }
        }
    }

    /// One statement of [`Self::collect_block_settings`].
    fn block_setting(&mut self, node: Node<'a>, skips: &Skips, at: &mut BlockSettings) {
        if let Some(s) = ast::OptionStmt::cast(node) {
            let digits = s.base().filter(|_| skips.usable(node)).map(|t| self.text(t.span));
            if let Some(n) = digits.and_then(decl::option_base_of) {
                at.base = n;
            }
            return;
        }
        if let Some(s) = ast::MetaStmt::cast(node) {
            let raw = s.token().filter(|_| skips.usable(node)).map(|t| self.text(t.span));
            if raw.and_then(unstable_meta) == Some(Ok(Unstable::TypeFields)) {
                at.typefields = true;
            }
            return;
        }
        if ast::InactiveCode::cast(node).is_some() {
            return;
        }
        if let Some(s) = ast::ConstStmt::cast(node) {
            self.consts_before.insert(node.key(), self.const_names.len());
            for t in s.items().filter_map(ast::ConstItem::name) {
                let word = self.text(t.span);
                let name = show_bytes(&word[..decl::name_end(word)].to_ascii_uppercase());
                self.const_names.push((name, at.in_proc));
            }
            return;
        }
        if let Some(b) = ast::TypeBlock::cast(node) {
            self.consts_above
                .insert(node.key(), (self.const_names.len(), at.in_proc));
            self.type_blocks.push(b);
            self.type_bases.insert(node.key(), at.base);
            if at.typefields {
                self.type_flags.insert(node.key());
            }
        }
    }
}

impl Checker<'_> {
    /// The default types at a procedure's header.
    fn defaults_at(&self, def: ast::ProcDef) -> [Ty; 27] {
        self.defaults_at_node(def.node())
    }

    /// The default types at a procedure's header or a `DECLARE LIBRARY` block.
    fn defaults_at_node(&self, node: Node) -> [Ty; 27] {
        self.proc_defaults
            .get(&node.key())
            .copied()
            .unwrap_or(DEFAULTS_AT_START)
    }

    /// One statement that is neither a block nor an include. A declaration marked "not supported yet" starts the
    /// follow-on rule (design D10).
    fn check_one(&mut self, stmt: Node, skips: &Skips) {
        if !skips.usable(stmt) {
            return;
        }
        self.stmt.stmt_error = false;
        let marks = self.diags.unsupported_count();
        self.statement(stmt);
        self.flush_names();
        if self.diags.unsupported_count() > marks && is_declaration(stmt) {
            self.at.follow_on = true;
        }
    }

    /// Gives every array parameter its number of dimensions, after every body is checked (design D6 of
    /// `m2-defxxx-common-arrays`; measured, `verification\v24_x148`-`x155`, `x286`-`x289`): the number belongs to
    /// the parameter and is one for the whole program. Every index count and every `DIM` or `REDIM` of it in its
    /// procedure, and every array passed to it, above or below the procedure, must agree; an array parameter
    /// passed on has the number of the parameter it goes to. The facts are read in file order, so the error
    /// stands at the later of two that disagree. A parameter that nothing says anything about keeps no number:
    /// no array is ever passed to it.
    fn solve_dims(&mut self) {
        let facts = std::mem::take(&mut self.dims.dim_facts);
        // Classes of arrays that have one number: each array's parent, and the number at each root.
        let mut parent: HashMap<VarId, VarId> = HashMap::new();
        let mut number: HashMap<VarId, usize> = HashMap::new();
        fn root(parent: &HashMap<VarId, VarId>, mut v: VarId) -> VarId {
            while let Some(&p) = parent.get(&v) {
                v = p;
            }
            v
        }
        let plural = |n: usize| if n == 1 { "" } else { "s" };
        for fact in facts {
            match fact {
                DimFact::Count {
                    array,
                    shown,
                    count,
                    span,
                    passed,
                } => {
                    let r = root(&parent, array);
                    match number.get(&r).copied() {
                        None => {
                            number.insert(r, count);
                        }
                        Some(n) if n == count => {}
                        Some(n) => {
                            let msg = if passed {
                                format!(
                                    "an array of {count} dimension{} is passed to `{shown}`, which has {n}",
                                    plural(count)
                                )
                            } else {
                                format!("`{shown}` has {n} dimension{}, not {count}", plural(n))
                            };
                            self.stmt.stmt_error = false;
                            let _ = self.error(span, msg);
                        }
                    }
                }
                DimFact::Same {
                    arg,
                    param,
                    shown,
                    span,
                    copied,
                } => {
                    let (a, b) = (root(&parent, arg), root(&parent, param));
                    if a == b {
                        continue;
                    }
                    match (number.get(&a).copied(), number.get(&b).copied()) {
                        (Some(n), Some(m)) if n != m => {
                            let to = if copied {
                                "is copied to an array"
                            } else {
                                "is passed to an array parameter"
                            };
                            let msg = format!("`{shown}` has {n} dimension{}, and {to} that has {m}", plural(n));
                            self.stmt.stmt_error = false;
                            let _ = self.error(span, msg);
                        }
                        (n, m) => {
                            parent.insert(a, b);
                            if let Some(n) = n.or(m) {
                                number.insert(b, n);
                            }
                        }
                    }
                }
            }
        }
        let members: Vec<VarId> = parent.keys().chain(number.keys()).copied().collect();
        for v in members {
            let var = &mut self.prog.vars[v.0 as usize];
            if let (Shape::Dynamic { dims: dims @ None, .. }, Some(&n)) =
                (&mut var.shape, number.get(&root(&parent, v)))
            {
                *dims = Some(n);
            }
        }
    }

    /// Runs `check` on a name that stands for `what` ([`Placing`]).
    fn placing<T>(&mut self, what: Placing, check: impl FnOnce(&mut Self) -> R<T>) -> R<T> {
        let before = std::mem::replace(&mut self.stmt.placing, what);
        let result = check(self);
        self.stmt.placing = before;
        result
    }

    /// Runs `check` as a part of the statement where a joined allocation has no place (`why`).
    fn unjoined<T>(&mut self, why: NoJoin, check: impl FnOnce(&mut Self) -> R<T>) -> R<T> {
        let before = self.stmt.no_join.replace(why);
        let result = check(self);
        self.stmt.no_join = before;
        result
    }

    /// Where a statement stands, as [`Self::push`] records it, for an `error_track_line` outside its code.
    fn track(&self, node: Node) -> crate::Track {
        let span = node.span();
        crate::Track {
            file: span.file,
            line: self.line(node.first_token().map_or(span, |t| t.span)),
            include_line: self.include_line(node),
        }
    }

    fn push(&mut self, node: Node, kind: StmtKind) {
        let span = node.span();
        let line = self.line(node.first_token().map_or(span, |t| t.span));
        let include_line = self.include_line(node);
        let stmt = Stmt {
            span,
            line,
            include_line,
            checked: self.at.checking,
            kind,
        };
        self.sink().push(stmt);
    }

    /// The statement list being added to: the innermost open block's, else the body's.
    fn sink(&mut self) -> &mut Vec<Stmt> {
        if let Some(sink) = self.sinks.last_mut() {
            return sink;
        }
        match self.cur {
            Some(p) => &mut self.prog.procs[p.0 as usize].stmts,
            None => &mut self.prog.stmts,
        }
    }

    /// A child the parser always builds for a statement without a parse error. Missing means a parser bug; it is
    /// reported rather than skipped, so no statement is silently dropped.
    fn need<T>(&mut self, x: Option<T>, span: Span) -> R<T> {
        x.ok_or_else(|| self.error(span, "internal error: incomplete syntax tree"))
    }

    fn statement(&mut self, node: Node) {
        let _ = if let Some(s) = ast::MetaStmt::cast(node) {
            self.meta(s)
        } else if let Some(s) = ast::MetaCommentStmt::cast(node) {
            self.meta_comment(s)
        } else if ast::InactiveCode::cast(node).is_some() {
            // A `$IF` branch not taken: not compiled.
            Ok(())
        } else if let Some(s) = ast::PrintStmt::cast(node) {
            self.print(s)
        } else if let Some(s) = ast::DimStmt::cast(node) {
            self.dim(s)
        } else if let Some(s) = ast::AssignStmt::cast(node) {
            self.assign(s)
        } else if ast::EndStmt::cast(node).is_some() {
            self.end_or_system(node, StmtKind::End)
        } else if ast::SystemStmt::cast(node).is_some() {
            self.end_or_system(node, StmtKind::System)
        } else if let Some(s) = ast::OnJumpStmt::cast(node) {
            self.on_jump(s)
        } else if let Some(s) = ast::OnEventStmt::cast(node) {
            self.on_event(s)
        } else if let Some(s) = ast::KeyStmt::cast(node) {
            self.key_stmt(s)
        } else if let Some(s) = ast::WriteStmt::cast(node) {
            self.write(s)
        } else if let Some(s) = ast::InputStmt::cast(node) {
            self.input(s)
        } else if let Some(s) = ast::LineInputStmt::cast(node) {
            self.line_input(s)
        } else if let Some(s) = ast::CloseStmt::cast(node) {
            self.close(s)
        } else if let Some(s) = ast::SwapStmt::cast(node) {
            self.swap(s)
        } else if let Some(s) = ast::LprintStmt::cast(node) {
            self.lprint(s)
        } else if let Some(s) = ast::FieldStmt::cast(node) {
            self.field_stmt(s)
        } else if let Some(s) = ast::LsetStmt::cast(node) {
            self.lset(s)
        } else if let Some(s) = ast::MemStmt::cast(node) {
            self.mem_stmt(s)
        } else if let Some(s) = ast::ArrayCopyStmt::cast(node) {
            self.array_copy(s)
        } else if let Some(s) = ast::BuiltinStmt::cast(node) {
            self.builtin_stmt(s)
        } else if ast::StopStmt::cast(node).is_some() {
            self.stop(node)
        } else if let Some(s) = ast::RunStmt::cast(node) {
            self.run(s)
        } else if let Some(s) = ast::PaletteUsingStmt::cast(node) {
            self.palette_using(s)
        } else if let Some(s) = ast::CallStmt::cast(node) {
            self.call_stmt(s)
        } else if ast::ExitStmt::cast(node).is_some() {
            self.exit(node)
        } else if ast::ContinueStmt::cast(node).is_some() {
            self.push(node, StmtKind::Continue);
            Ok(())
        } else if let Some(d) = ast::DeclareStmt::cast(node) {
            // Ignored, as by the old compiler: calls are checked against the definition (measured). Its name is a
            // use of the procedure of that name and kind, if there is one, for the language server's references and
            // rename (design D5 of `m5b-language-features`).
            if let Some(h) = d.header()
                && let (Some(kw), Some(name_tok)) = (h.keyword(), h.name())
            {
                let text = self.name_text(name_tok);
                let name = show_bytes(&text[..decl::name_end(&text)].to_ascii_uppercase());
                if let Some(p) = self.proc_as(&name, self.word(kw) == "SUB") {
                    self.stmt.names.push((SymbolKind::Proc(p), name_tok.span));
                }
            }
            Ok(())
        } else if let Some(s) = ast::SharedStmt::cast(node) {
            self.shared(s)
        } else if let Some(s) = ast::StaticStmt::cast(node) {
            self.static_stmt(s)
        } else if let Some(s) = ast::LabelDef::cast(node) {
            self.label_stmt(s)
        } else if let Some(s) = ast::OnErrorStmt::cast(node) {
            self.on_error(s)
        } else if let Some(s) = ast::ResumeStmt::cast(node) {
            self.resume(s)
        } else if let Some(s) = ast::ErrorStmt::cast(node) {
            self.error_stmt(s)
        } else if let Some(s) = ast::LineNumber::cast(node) {
            self.line_number_stmt(s)
        } else if let Some(s) = ast::ImplicitGoto::cast(node) {
            self.implicit_goto(s)
        } else if let Some(s) = ast::GotoStmt::cast(node) {
            self.goto(s)
        } else if let Some(s) = ast::GosubStmt::cast(node) {
            self.gosub(s)
        } else if let Some(s) = ast::ReturnStmt::cast(node) {
            self.return_stmt(s)
        } else if let Some(s) = ast::DataStmt::cast(node) {
            self.data(s)
        } else if let Some(s) = ast::ReadStmt::cast(node) {
            self.read(s)
        } else if let Some(s) = ast::RestoreStmt::cast(node) {
            self.restore(s)
        } else if let Some(s) = ast::ConstStmt::cast(node) {
            self.const_stmt(s)
        } else if let Some(s) = ast::OptionStmt::cast(node) {
            self.option_stmt(s)
        } else if let Some(s) = ast::RedimStmt::cast(node) {
            self.redim(s)
        } else if let Some(s) = ast::CommonStmt::cast(node) {
            self.common(s)
        } else if let Some(s) = ast::EraseStmt::cast(node) {
            self.erase(s)
        } else if let Some(s) = ast::DefTypeStmt::cast(node) {
            self.def_type(s)
        } else {
            Err(self.error(
                node.span(),
                format!("internal error: unexpected {:?} node", node.kind()),
            ))
        };
    }

    /// `END` or `SYSTEM` (`kind`); with an exit code not supported yet.
    /// `END [code]` or `SYSTEM [code]` (design D5 of `m3-asserts-debug-zlib`; `qb64pe.bas` 9850–9895). The old
    /// compiler converts the code with `evaluatetotyp(…, ISINTEGER64)`, which reads the target's width as 0: a float
    /// goes through `qbr_float_to_long` and an integer is stored as it is into libqb's `int64 exit_code`, whose low 32
    /// bits are the process's exit code (measured, `verification\v33_c*`: `SYSTEM 2.5` gives 2, `SYSTEM 2 ^ 40` 0).
    /// A string is "Illegal string-number conversion" there (`v33_x37`, `x38`).
    fn end_or_system(&mut self, node: Node, kind: fn(Option<crate::Expr>) -> StmtKind) -> R<()> {
        let code = match node.child_nodes().find_map(ast::Expr::cast) {
            None => None,
            Some(x) => {
                let e = self.expr(x)?;
                if e.ty == Ty::Str {
                    let word = show_bytes(&self.text(first_token_span(node)).to_ascii_uppercase());
                    return Err(self.error(e.span, format!("the exit code of `{word}` must be a number")));
                }
                Some(if e.ty.is_float() {
                    expr::conv(e, Ty::I32, crate::ConvKind::RoundEven)
                } else {
                    e
                })
            }
        };
        self.push(node, kind(code));
        Ok(())
    }

    /// `STOP [expression]`: the expression is checked as a number and never evaluated (measured,
    /// `verification\v52_x12`, `x13`, `x19`: a string is "Illegal string-number conversion").
    fn stop(&mut self, node: Node) -> R<()> {
        if let Some(x) = node.child_nodes().find_map(ast::Expr::cast) {
            let e = self.expr(x)?;
            if e.ty == Ty::Str {
                return Err(self.error(e.span, "the expression after `STOP` must be a number"));
            }
        }
        self.push(node, StmtKind::Stop);
        Ok(())
    }

    fn meta(&mut self, stmt: ast::MetaStmt) -> R<()> {
        let node = stmt.node();
        let tok = self.need(stmt.token(), node.span())?;
        let raw = self.text(tok.span);
        let line = meta_line(raw);
        let once = line.strip_prefix(b"$INCLUDEONCE".as_slice());
        if self.program.passes_disagree.contains(&(node.key().0, tok.span.start)) {
            // The parser read the condition with the flag's value on this line, as the old compiler's first pass
            // does; its main pass reads another value and compiles none of the branches (design D9 of
            // `m4-statements-metacommands`; measured, `verification\v26_x27`, `x34`, `v57_g01`–`g11`). The
            // statements up to the `$END IF` are dropped ([`Self::pp_dropped`], which starts the drop here).
            Ok(())
        } else if stmt.is_preprocessor(raw) || once == Some(b"") {
            // Evaluated by the parser (`$INCLUDEONCE` too); an error in it is a parse error, and the statement is
            // skipped then.
            Ok(())
        } else if once
            .and_then(<[u8]>::first)
            .is_some_and(|b| !b.is_ascii_alphanumeric() && *b != b'_')
        {
            // The old compiler compares the whole line here too: a tab or a word after the name is "Syntax
            // error" (measured, `verification\v26_x59`, `x61`).
            Err(self.error(tok.span, "`$INCLUDEONCE` must stand alone on its line"))
        } else if let Some(m) = console_meta(raw) {
            match m {
                Ok(ConsoleMeta::Console) => self.settings.console = true,
                Ok(ConsoleMeta::ConsoleOnly) => {
                    // Each one selects the console where it stands (measured, `verification\v26_x25`).
                    self.settings.console = true;
                    self.settings.console_only = true;
                    self.push(node, StmtKind::ConsoleOnly);
                }
                // The last of the two in the file counts (`qb64pe.bas` 3615–3624).
                Ok(ConsoleMeta::ScreenHide) => self.settings.window_hidden = true,
                Ok(ConsoleMeta::ScreenShow) => self.settings.window_hidden = false,
                // The global `asserts` (design D1 of `m3-asserts-debug-zlib`); `$ASSERTS:CONSOLE` gives a console as
                // `$CONSOLE` does (`qb64pe.bas` 1984; measured, `verification\v33_a06`).
                Ok(ConsoleMeta::Asserts) => self.settings.asserts = true,
                Ok(ConsoleMeta::AssertsConsole) => {
                    self.settings.asserts = true;
                    self.settings.console = true;
                }
                // No debugger code is written (user, design D2): only the flags, which the parser settles, and the
                // old compiler's warning from the command line (`qb64pe.bas` 3556).
                Ok(ConsoleMeta::Debug) => self.diags.warning(tok.span, "$DEBUG features only work from the IDE"),
                Err(msg) => return Err(self.error(tok.span, msg)),
            }
            Ok(())
        } else if let Some(r) = resize_meta(&line) {
            // The last one in the file counts.
            self.settings.resize = r.map_err(|msg| self.error(tok.span, msg))?;
            Ok(())
        } else if line.starts_with(b"$EXEICON") {
            self.exe_icon_meta(node, tok)
        } else if line.starts_with(b"$EMBED") {
            self.embed_meta(node, tok)
        } else if line.starts_with(b"$VERSIONINFO") {
            self.version_info_meta(tok)
        } else if let Some(u) = unstable_meta(raw) {
            match u {
                // Read by pass 1, where the blocks are (`Self::collect_block_settings`).
                Ok(Unstable::TypeFields) => {}
                // The old compiler's warning (`qb64pe.bas` 3891, 3895), given with `-w` as every warning is.
                Ok(Unstable::Stable) => self
                    .diags
                    .warning(tok.span, "No longer required, feature is stable now"),
                Err(msg) => return Err(self.error(tok.span, msg)),
            }
            Ok(())
        } else if line == b"$CHECKING:OFF" || line == b"$CHECKING:ON" {
            // For the lines after it in source order (design D3 of `m4-handlers-logging-checking`).
            self.at.checking = line == b"$CHECKING:ON";
            Ok(())
        } else if line
            .strip_prefix(b"$CHECKING".as_slice())
            .is_some_and(|rest| matches!(rest.first(), None | Some(b':' | b' ' | b'\t')))
        {
            // The old compiler's "Syntax error" for any other text (measured, `verification\v50_x07`–`x11`).
            Err(self.error(tok.span, "`$CHECKING` takes `:OFF` or `:ON`, alone on its line"))
        } else if line.starts_with(b"$COLOR") {
            // The parser included the colour file (design D8 of `m3-constant-evaluation`); measured,
            // `verification\v45_x45`, `x46`, `x51`–`x53`: the other set after one is an error, and so is any other
            // text.
            let set = match line.as_slice() {
                b"$COLOR:0" => 1,
                b"$COLOR:32" => 2,
                _ => return Err(self.error(tok.span, "`$COLOR` takes `:0` or `:32`, alone on its line")),
            };
            match self.color_set {
                Some(first) if first != set => {
                    Err(self.error(tok.span, "`$COLOR:0` and `$COLOR:32` cannot be used in one program"))
                }
                _ => {
                    self.color_set = Some(set);
                    Ok(())
                }
            }
        } else if line.starts_with(b"$MIDISOUNDFONT:") {
            // The old compiler's own error, whatever follows the colon (design D3 of `m3-sound-handles`;
            // `qb64pe.bas` 2092; measured, `verification\v39_x73`–`x81`). Without the colon it is an unknown
            // metacommand, as below.
            Err(self.error(
                tok.span,
                "$MIDISOUNDFONT is a deprecated keyword, use _MIDISOUNDBANK instead",
            ))
        } else if line == b"$ERRORLOCATION:ON" || line == b"$ERRORLOCATION:OFF" {
            // A setting of the whole program (design D6 of `m4-builtins-remaining`; `qb64pe.bas` 2010–2016,
            // 3576–3586): the lines are judged after the walk ([`Self::settle_error_location`]). The old compiler
            // compares the whole line, so `$ERRORLOCATION:ON ' comment` is an unknown metacommand below (measured,
            // `verification\v58_e14`–`e18`).
            self.settings
                .error_location
                .push((tok.span, line == b"$ERRORLOCATION:ON"));
            Ok(())
        } else if line.starts_with(b"$USELIBRARY") {
            // Read by the parser, which includes the library's files (design D7 of `m4-builtins-remaining`); an
            // error in the line is a parse error.
            Ok(())
        } else if line == b"$NOPREFIX" {
            // The old compiler's own error (`qb64pe.bas` 2020–2026; measured, `verification\v43_x32`).
            Err(self.error(
                tok.span,
                "`$NOPREFIX` is deprecated: QB64pe's own names must have their underscore (the IDE can convert the \
                 program)",
            ))
        } else {
            // Any other line reaches the old compiler's statement parser, which rejects it (design D3; measured,
            // `verification\v16_m6_unknown_meta`, `v24_x38`, `v39_x74`, `x79`–`x81`, `v57_m01`–`m08`).
            let shown = show_bytes(raw.split(|&b| b == b':' || b == b' ').next().unwrap_or(raw));
            Err(self.error(tok.span, format!("unknown metacommand `{shown}`")))
        }
    }

    /// `$EXEICON:'name'` (design D4 of `m3-sound`; `qb64pe.bas` 3829–3880; measured, `verification\v29_e01`–`e31`):
    /// the icon of the executable, and where it stands the statement `_ICON` with no argument. The old compiler
    /// takes any line that starts with `$EXEICON` in upper case; the name stands between the line's first two
    /// `'`, and the rest of the line is dropped (the lexer makes one token of a metacommand's whole line).
    fn exe_icon_meta(&mut self, node: Node, tok: Tok) -> R<()> {
        if let Some((_, first)) = &self.settings.exe_icon {
            // Before the line's own name is read (`e27`). The old compiler names a line of the main file: for a
            // first one inside an included file the line of its `$INCLUDE` (`e31`).
            let line = self.main_line(*first);
            return Err(self.error(tok.span, format!("`$EXEICON` is already defined in line {line}")));
        }
        let raw = self.text(tok.span);
        let mut quoted = raw.split(|&b| b == b'\'');
        let written = match (quoted.next(), quoted.next(), quoted.next()) {
            (Some(_), Some(name), Some(_)) => name.trim_ascii(),
            _ => b"",
        };
        if written.is_empty() {
            return Err(self.error(tok.span, "expected `$EXEICON:'filename'`"));
        }
        // The name is a file's, taken as UTF-8 as an included file's is (`qb64rust_syntax::include_name`).
        #[expect(clippy::disallowed_methods, reason = "a file name, not BASIC source")]
        let name = String::from_utf8_lossy(written).into_owned();
        let shown = show_bytes(written);
        if crate::drive_without_slash(written) {
            // The old compiler looks for `C:game.ico` in the folder it stands in on that drive, which has no
            // counterpart here. Any other relative name is looked for under the compiler root, as the old
            // compiler looks in its own folder (`e02`, `e03`, `e08`, `e18`; design D3 of
            // `m4-devices-continue-paths`): the driver's to find, as every icon file is.
            let msg = "a `$EXEICON` name with a drive and no slash after it";
            return Err(self.unsupported(tok.span, msg));
        }
        self.absent_stmt(node, "_ICON")?;
        let icon = crate::ExeIcon {
            name,
            shown,
            span: tok.span,
        };
        self.settings.exe_icon = Some((icon, node.key()));
        Ok(())
    }

    /// `$VERSIONINFO:key=value` (design D1 of `m3-versioninfo`; `qb64pe.bas` 3647–3741; measured,
    /// `verification\v40_*`): one field of the executable's version resource. The old compiler takes any line that
    /// starts with `$VERSIONINFO` in upper case (`$VERSIONINFOX:…` too, `v40_j`) and reads it with the spaces at
    /// its ends trimmed: the key between the first `:` and the first `=` after it, the value after that `=`, a
    /// trailing comment included (`v40_d`).
    fn version_info_meta(&mut self, tok: Tok) -> R<()> {
        let map = self.map;
        let line = trim_spaces(map.text(tok.span));
        let colon = line.iter().position(|&b| b == b':');
        let eq = colon.and_then(|c| line[c + 1..].iter().position(|&b| b == b'=').map(|e| c + 1 + e));
        let (Some(colon), Some(eq)) = (colon, eq) else {
            return Err(self.error(tok.span, "expected `$VERSIONINFO:key=value`"));
        };
        if eq == colon + 1 {
            return Err(self.error(tok.span, "expected `$VERSIONINFO:key=value`"));
        }
        let key = trim_spaces(&line[colon + 1..eq]).to_ascii_uppercase();
        let value: Vec<u8> = trim_spaces(&line[eq + 1..])
            .iter()
            .map(|&b| if b == b'"' { b'\'' } else { b })
            .collect();
        if key == b"FILEVERSION#" || key == b"PRODUCTVERSION#" {
            let key = show_bytes(&key);
            if let Err(msg) = version_number(&key, &value) {
                return Err(self.error(tok.span, msg));
            }
            let info = self.settings.version_info.get_or_insert_with(Default::default);
            // The string version of the same name takes the digits while it is empty (`v40_b`, `v40_c`).
            let (number, string) = if key == "FILEVERSION#" {
                (&mut info.file_version, crate::VersionInfo::FILE_VERSION)
            } else {
                (&mut info.product_version, crate::VersionInfo::PRODUCT_VERSION)
            };
            *number = value.clone();
            if info.strings[string].is_empty() {
                info.strings[string] = value;
            }
            return Ok(());
        }
        let Some(i) = crate::VersionInfo::STRING_KEYS
            .iter()
            .position(|k| k.as_bytes().eq_ignore_ascii_case(&key))
        else {
            let msg = "invalid `$VERSIONINFO` key; use FILEVERSION#, PRODUCTVERSION#, CompanyName, FileDescription, \
                       FileVersion, InternalName, LegalCopyright, LegalTrademarks, OriginalFilename, ProductName, \
                       ProductVersion, Comments or Web";
            return Err(self.error(tok.span, msg));
        };
        // `RemoveStringEnclosingPair`: one pair of `'` around a value longer than one byte.
        let quoted = value.len() > 1 && value.first() == Some(&b'\'') && value.last() == Some(&b'\'');
        let info = self.settings.version_info.get_or_insert_with(Default::default);
        info.strings[i] = if quoted {
            value[1..value.len() - 1].to_vec()
        } else {
            value.clone()
        };
        self.settings.version_lines[i] = Some(tok.span);
        if !quoted {
            // The old compiler's warning with the value on the next line (`qb64pe.bas` 3727; `v40_d`), given with
            // `-w` as every warning is.
            let msg = format!("Missing string bracket delimiters ('')\n    {}", show_bytes(&value));
            self.diags.warning(tok.span, msg);
        }
        Ok(())
    }

    /// The three string fields the old compiler writes unescaped into the executable's manifest (design D3 of
    /// `m3-versioninfo`; user, 2026-10-16): a `<`, an `&` or bytes that are not UTF-8 there give an executable
    /// Windows does not start (measured, `verification\v40_g2`–`g6`). Checked after the walk, at the line that set
    /// the value kept, so a bad value replaced by a later line compiles.
    fn check_manifest_fields(&mut self) {
        let Some(info) = &self.settings.version_info else {
            return;
        };
        for i in crate::VersionInfo::MANIFEST {
            let (Some(span), Some(why)) = (self.settings.version_lines[i], manifest_refusal(&info.strings[i])) else {
                continue;
            };
            let key = crate::VersionInfo::STRING_KEYS[i];
            self.diags.error(
                span,
                format!("`{key}` holds {why}, which the executable's manifest cannot hold; Windows would not start the program"),
            );
        }
    }

    /// The line of the main file a node of any tree belongs to: its own line, or the line of the `$INCLUDE` that
    /// brought its file in.
    fn main_line(&self, (mut tree, mut offset): (TreeId, u32)) -> u32 {
        while tree != self.program.main().id {
            let Some((&at, _)) = self.program.includes.iter().find(|&(_, &included)| included == tree) else {
                break;
            };
            (tree, offset) = at;
        }
        let file = self.program.tree(tree).file;
        self.line(Span::new(file, offset, offset))
    }

    /// The offset of the main file where a place of the program stands in the old compiler's first pass: its own
    /// in the main file, that of the `$INCLUDE` that brought the outermost included file in otherwise.
    fn main_offset(&self, (mut tree, mut offset): (TreeId, u32)) -> u32 {
        while tree != self.program.main().id {
            let Some((&at, _)) = self.program.includes.iter().find(|&(_, &included)| included == tree) else {
                break;
            };
            (tree, offset) = at;
        }
        offset
    }

    /// The place ([`Self::main_offset`]) of the first `$COLOR:0` or `$COLOR:32` line that is compiled, in any
    /// file of the program but QB64pe's own.
    fn first_color_line(&self, program: &ParsedProgram) -> Option<u32> {
        fn walk(node: Node, out: &mut Vec<Tok>) {
            for e in node.children() {
                match e {
                    qb64rust_syntax::tree::Element::Node(n) if n.kind() != SyntaxKind::InactiveCode => walk(n, out),
                    qb64rust_syntax::tree::Element::Token(t) if t.kind == SyntaxKind::Metacommand => out.push(t),
                    qb64rust_syntax::tree::Element::Node(_) | qb64rust_syntax::tree::Element::Token(_) => {}
                }
            }
        }
        let mut first: Option<u32> = None;
        for tree in &program.trees {
            if program.is_auto_tree(tree.id) {
                continue;
            }
            let mut toks = Vec::new();
            walk(tree.root(), &mut toks);
            for t in toks {
                let line = meta_line(self.text(t.span));
                if line == b"$COLOR:0" || line == b"$COLOR:32" {
                    let at = self.main_offset((tree.id, t.span.start));
                    first = Some(first.map_or(at, |f| f.min(at)));
                }
            }
        }
        first
    }

    /// For a node of an included file, the line of the main file that `_ERRORLINE` gives there: that of the
    /// `$INCLUDE` that brought the outermost included file in (measured, `verification\v43_b_include`); `None` in
    /// the main file.
    fn include_line(&self, node: Node) -> Option<u32> {
        let tree = node.key().0;
        if tree == self.program.main().id {
            return None;
        }
        // A library's file is counted in the main file where the old compiler includes it (design D7 of
        // `m4-builtins-remaining`; its C++ of `verification\v58_u01`, `u19`): a top file at line 0, one after the
        // main module at the main file's last line.
        match self.program.library_tree_is_top(tree) {
            Some(true) => Some(0),
            Some(false) => Some(self.main_line_count()),
            None => Some(self.main_line(node.key())),
        }
    }

    /// The number of lines of the main file (the old compiler's `linenumber` after its last line).
    fn main_line_count(&self) -> u32 {
        let bytes = &self.map.file(self.program.main().file).bytes;
        let newlines = bytes.iter().filter(|&&b| b == b'\n').count();
        qb64rust_base::to_u32(newlines + usize::from(bytes.last().is_some_and(|&b| b != b'\n')))
    }

    /// A metacommand comment. Its `$STATIC` and `$DYNAMIC` decide what a `DIM` with constant bounds makes from
    /// here on in the file (design D3 of `m2-defxxx-common-arrays`; the declarations `$DYNAMIC` turns dynamic are
    /// marked where they stand until dynamic arrays are compiled). A malformed `$INCLUDE` is an error, as in the
    /// old compiler. A well-formed `$INCLUDE` was followed by the parser (its statements come next,
    /// [`Self::expand`]; a missing file is a parse error).
    fn meta_comment(&mut self, stmt: ast::MetaCommentStmt) -> R<()> {
        let tok = self.need(stmt.token(), stmt.node().span())?;
        match comment_directives(self.text(tok.span)) {
            Err(msg) => Err(self.error(tok.span, msg)),
            Ok(d) => {
                match d.memory {
                    Some(MemoryMode::Static) => self.at.dynamic_mode = false,
                    Some(MemoryMode::Dynamic) => self.at.dynamic_mode = true,
                    None => {}
                }
                Ok(())
            }
        }
    }

    // ---- statements ----

    /// `place = value`. A plain variable is resolved after the value, as before places existed (an implicit
    /// variable in the value is created first); an element or a member before it.
    fn assign(&mut self, stmt: ast::AssignStmt) -> R<()> {
        let node = stmt.node();
        let target_node = self.need(stmt.target(), node.span())?;
        let value_node = self.need(stmt.value(), node.span())?;
        // Whole-array assignment (design D1 of `m3-whole-array-assign`): both sides written `name()` or as a member
        // chain ending in `()`, the source also in parentheses. One side a whole array `name()` and the other not
        // written so is the old compiler's guard (design D2), also with the value in parentheses (`n = (a())`,
        // `a() = (b(1))`); a whole member array beside a side of another form keeps the errors it gets below.
        let source = without_parens(value_node);
        if written_whole(target_node) && written_whole(source) {
            // Not a copy with a type suffix on both names: an ordinary assignment of the first elements.
            if self.copy_array(node, target_node, source)? {
                return Ok(());
            }
        } else if let Some((_, t)) = self.whole_array(target_node) {
            return Err(self.half_whole(node, (target_node, t), source, false));
        } else if let Some((_, t)) = self.whole_array(source) {
            return Err(self.half_whole(node, (source, t), target_node, true));
        }
        let place = match target_node {
            ast::Expr::NameRef(n) => {
                let t = self.need(n.name(), node.span())?;
                if let Some(var) = self.fixed_param_ref(t) {
                    let value = self.expr(value_node)?;
                    let value = self.store(value, Ty::Str)?;
                    let place = Place::Var(var);
                    self.push(node, StmtKind::Assign { place, value });
                    return Ok(());
                }
                let (name, suffix) = self.split_var_name(t)?;
                match self.dotted(t, &name, suffix)? {
                    Some(place) => place,
                    None => {
                        let whole = self.lookup_var(name, suffix).filter(|&v| self.is_whole_type(v));
                        match whole {
                            Some(v) => {
                                self.stmt.names.push((SymbolKind::Var(v), t.span));
                                Place::Var(v)
                            }
                            None => {
                                let value = self.expr(value_node)?;
                                let var = self.target(t)?;
                                let ty = self.prog.var(var).ty;
                                let value = self.store(value, ty)?;
                                let place = Place::Var(var);
                                self.push(node, StmtKind::Assign { place, value });
                                return Ok(());
                            }
                        }
                    }
                }
            }
            // `MID$(s$, start[, length]) = value$` is a statement of its own, not a store into an array `MID$`.
            ast::Expr::Call(c) if c.name().is_some_and(|t| self.word(t) == "MID$") => {
                return self.mid_assign(node, c, value_node);
            }
            // `ASC(s$[, position]) = code` likewise (design D4 of `m4-qb64pe-sources`).
            ast::Expr::Call(c) if c.name().is_some_and(|t| self.word(t) == "ASC") => {
                return self.asc_assign(node, c, value_node);
            }
            ast::Expr::Call(c) => self.call_place(c, Placing::Store)?,
            ast::Expr::Field(f) => self.field_place(f)?,
            e @ (ast::Expr::Literal(_) | ast::Expr::Paren(_) | ast::Expr::Prefix(_) | ast::Expr::Bin(_)) => {
                return Err(self.error(e.node().span(), "cannot assign to this expression"));
            }
        };
        let ty = self.prog.place_ty(&place);
        self.read_only_member(&place, target_node.node().span())?;
        let read = self.names_word(value_node, "_MEMGET");
        if (self.prog.is_mem(ty) && self.gives_block(value_node)) || (matches!(ty, Ty::User(_)) && read) {
            // A built-in that gives a block, stored into a `_MEM` place: a copy of its 56 bytes (design D2 of
            // `m3-mem`); a value of a user type `_MEMGET` reads, stored into a place of that type (design D4;
            // measured, `verification\v32_x01_accepted`).
            let value = self.expr_as(value_node, Use::Stored)?;
            if value.ty != ty {
                // Measured for two user types: "Expected = similar user defined type" (`v24_x208`).
                let msg = format!("a value of `TYPE {}` is required here", self.prog.type_name(ty));
                return Err(self.error(value.span, msg));
            }
            self.push(node, StmtKind::Assign { place, value });
            return Ok(());
        }
        if let Ty::User(_) = ty {
            // Measured (`verification\v24_h_whole`, `v24_x208`, `x209`): `q = p` copies the whole value between
            // two places of one `TYPE`; a place of another `TYPE`, also one with the same members, and a number
            // are "Expected = similar user defined type".
            let from = match self.whole_place(value_node)? {
                Some(from) if self.prog.place_ty(&from) == ty => from,
                Some(_) => {
                    let msg = format!("a value of `TYPE {}` is required here", self.prog.type_name(ty));
                    return Err(self.error(value_node.node().span(), msg));
                }
                None => {
                    self.expr(value_node)?;
                    return Err(self.error(
                        value_node.node().span(),
                        "a `TYPE` variable takes only a value of its `TYPE`",
                    ));
                }
            };
            self.push(node, StmtKind::CopyWhole { from, to: place });
            return Ok(());
        }
        let value = self.expr(value_node)?;
        let value = self.store(value, ty)?;
        self.push(node, StmtKind::Assign { place, value });
        Ok(())
    }

    /// Whether a variable holds a whole user-type value.
    fn is_whole_type(&self, v: VarId) -> bool {
        matches!(self.prog.var(v).ty, Ty::User(_))
    }

    /// A member of a `_MEM` block as the place of an assignment, `READ` or `INPUT #` (the old compiler's `setrefer`,
    /// `qb64pe.bas` 26850). Measured: "Cannot set read-only element of _MEM TYPE" (`verification\v32_x05`, `x06`,
    /// `x117`, `x118`).
    pub(super) fn read_only_member(&mut self, place: &Place, span: Span) -> R<()> {
        if let Place::Member { base, .. } = place
            && self.prog.is_mem(self.prog.place_ty(base))
        {
            return Err(self.error(span, "a member of a `_MEM` block cannot be stored to"));
        }
        Ok(())
    }

    /// Whether an expression is a call of a built-in that gives a block ([`crate::builtins::MEM_RESULTS`]),
    /// with or without its argument list.
    pub(super) fn gives_block(&self, node: ast::Expr) -> bool {
        crate::builtins::MEM_RESULTS.iter().any(|w| self.names_word(node, w))
    }

    /// Whether an expression is a call of the built-in `word`, with or without its argument list.
    fn names_word(&self, node: ast::Expr, word: &str) -> bool {
        let name = match node {
            ast::Expr::Call(c) => c.name(),
            ast::Expr::NameRef(n) => n.name(),
            ast::Expr::Literal(_)
            | ast::Expr::Paren(_)
            | ast::Expr::Prefix(_)
            | ast::Expr::Bin(_)
            | ast::Expr::Field(_) => None,
        };
        name.is_some_and(|t| self.word(t) == word)
    }
}

/// Whether an `OPTION word` (`_EXPLICIT`, `_EXPLICITARRAY`) without a parse error stands anywhere in the tree: in
/// the main module, a procedure or a block, before or after the variables it concerns (measured,
/// `verification\v17_f_*`).
fn has_option(map: &SourceMap, node: Node, skips: &Skips, word: &[u8]) -> bool {
    if let Some(s) = ast::OptionStmt::cast(node) {
        return skips.usable(node) && s.word().is_some_and(|w| map.text(w.span).eq_ignore_ascii_case(word));
    }
    node.child_nodes().any(|n| has_option(map, n, skips, word))
}

/// The declaration statements of the follow-on rule (design D10; `TYPE` and `DECLARE LIBRARY` blocks are handled
/// where they are checked).
fn is_declaration(stmt: Node) -> bool {
    ast::DimStmt::cast(stmt).is_some()
        || ast::RedimStmt::cast(stmt).is_some()
        || ast::CommonStmt::cast(stmt).is_some()
        || ast::SharedStmt::cast(stmt).is_some()
        || ast::StaticStmt::cast(stmt).is_some()
        || ast::ConstStmt::cast(stmt).is_some()
        || ast::DefTypeStmt::cast(stmt).is_some()
}

/// A statement list with every include statement followed by the statements of the tree it includes (design D10:
/// the walk descends into an included tree at its include statement), recursively.
fn expand<'t>(program: &'t ParsedProgram, nodes: impl IntoIterator<Item = Node<'t>>) -> Vec<Node<'t>> {
    let mut out = Vec::new();
    for n in nodes {
        out.push(n);
        if ast::MetaCommentStmt::cast(n).is_some()
            && let Some(tree) = program.included(n)
        {
            let inner: Vec<Node<'t>> = ast::SourceFile::cast(tree.root())
                .into_iter()
                .flat_map(|f| f.statements())
                .collect();
            out.extend(expand(program, inner));
        }
    }
    out
}

/// The span of a statement's first token (where a "not supported yet" mark goes, design D10).
fn first_token_span(node: Node) -> Span {
    node.first_token().map_or(node.span(), |t| t.span)
}

/// The words of an `AS` clause ([`Checker::as_words`]) split at a leading `_UNSIGNED`: whether there is one, and
/// the words after it.
fn split_unsigned(words: &[String]) -> (bool, &[String]) {
    match words.split_first() {
        Some((first, rest)) if first == "_UNSIGNED" => (true, rest),
        _ => (false, words),
    }
}
