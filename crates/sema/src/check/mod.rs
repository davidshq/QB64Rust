//! From the syntax tree to the typed [`Program`].
//!
//! Two passes (design D2 of `m2-procedures-and-errors`): pass 1 collects the procedure headers, so a call may come
//! before the definition; pass 2 checks every statement **in file order**, main-module statements and procedure
//! bodies as they come. File order is what the old compiler's scopes follow (measured, `verification\v14_*`): a
//! `DIM SHARED` after a procedure is not seen by it, and a `SHARED x AS T` in an earlier procedure acts like a
//! main-module `DIM x AS T` for the main-module code after it.

mod auto;
mod blocks;
mod builtins;
mod constants;
mod decl;
mod expr;
mod flow;
mod io;
mod library;
mod mem;
mod ops;
mod places;
mod proc;
mod whole;

use blocks::nested_statements;
use whole::{without_parens, written_whole};

use crate::{ConstId, LabelId, Place, ProcId, Program, Shape, Stmt, StmtKind, SymbolKind, Ty, TypeId, VarId};
use qb64rust_base::{Diagnostics, SourceMap, Span, show_bytes};
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
    let root = program.main().root();
    let mut c = Checker {
        fold,
        map,
        program,
        prog: Program::default(),
        main: Scope::default(),
        local: Scope::default(),
        cur: None,
        procs_by_name: HashMap::new(),
        proc_of_def: HashMap::new(),
        param_scopes: Vec::new(),
        broken: HashSet::new(),
        static_procs: HashSet::new(),
        static_scope: false,
        dim_shared: HashSet::new(),
        dim_shared_plain: HashMap::new(),
        labels_by_name: HashMap::new(),
        label_of_def: HashMap::new(),
        jump_words: HashSet::new(),
        doubt_words: HashSet::new(),
        sub_call_lines: HashMap::new(),
        diags: Diagnostics::new(),
        stmt_error: false,
        console: false,
        console_only: false,
        window_hidden: false,
        asserts: false,
        exe_icon: None,
        echoes: Vec::new(),
        runtime_parts: std::collections::BTreeSet::new(),
        names: Vec::new(),
        consts_main: HashMap::new(),
        consts_local: HashMap::new(),
        auto_consts: HashMap::new(),
        used_names: HashMap::new(),
        used_local: HashMap::new(),
        explicit: false,
        sinks: Vec::new(),
        bad_next: None,
        types_by_name: HashMap::new(),
        types_seen: HashSet::new(),
        type_defs: HashSet::new(),
        dim_shared_array_plain: HashMap::new(),
        len_place: None,
        mem_store: false,
        types_marked: HashSet::new(),
        follow_on: false,
        param_len: HashMap::new(),
        fixed_params: HashMap::new(),
        defaults: DEFAULTS_AT_START,
        proc_defaults: HashMap::new(),
        option_base: 0,
        dynamic_mode: false,
        implicit_names: HashSet::new(),
        explicit_array: false,
        no_join: None,
        dim_facts: Vec::new(),
        dims_open: HashSet::new(),
        type_bases: HashMap::new(),
        type_flags: HashSet::new(),
        type_const_bounds: HashMap::new(),
    };
    let skips = Skips::new(program);
    // Measured: an `OPTION _EXPLICIT` in an included file applies to the whole program, also before the include
    // (`verification\v19_explicit_*`).
    let has = |word: &[u8]| program.trees.iter().any(|t| has_option(map, t.root(), &skips, word));
    // An option is also in force by its precompiler flag, when its only line stands in the `$IF` branch the flag
    // switches off and so is not in the trees (measured, `verification\v26_x54`, `x55`; design D7 of
    // `m2-console-screenhide`).
    c.explicit = has(b"_EXPLICIT") || program.flags.explicit();
    // Measured likewise for arrays (`verification\v24_x256`, `x257`): either option, above or below the use.
    c.explicit_array = c.explicit || has(b"_EXPLICITARRAY") || program.flags.explicit_array();
    // The main file's statements, and the same with every include statement followed by what it includes (pass 1
    // reads the user types and procedures of included files from these; pass 2 and the label pre-pass descend into
    // includes as they meet them).
    let top: Vec<Node> = ast::SourceFile::cast(root)
        .into_iter()
        .flat_map(|f| f.statements())
        .collect();
    let statements = expand(program, top.iter().copied());

    // Pass 1: the user types (a parameter may name one), then procedure names, then their parameters (a parameter may not have the name of any procedure).
    // A header with an error (a parse error, a reserved name...) still enters its name as a broken procedure, so
    // calls of it fail without a second error. A member array's bounds go by `OPTION BASE` as it stands at its
    // block, which follows the file (design D8 of `m2-defxxx-common-arrays`).
    let mut at_block = BlockSettings::default();
    for &stmt in &top {
        c.collect_block_settings(stmt, &skips, &mut at_block);
    }
    c.declare_types(&statements, &skips);
    // The default types follow the file (design D2): the table as it stands at each procedure's header.
    for &stmt in &top {
        c.collect_defaults(stmt, &skips);
    }
    let defs: Vec<ast::ProcDef> = statements
        .iter()
        .filter_map(|&s| ast::ProcDef::cast(s))
        .filter(|d| d.header().is_some() && !skips.past_cap(d.node()))
        .collect();
    let mut declared = Vec::new();
    for &def in &defs {
        let header = def.header().unwrap();
        c.defaults = c.defaults_at(def);
        let id = if skips.usable(header.node()) {
            c.stmt_error = false;
            let id = c.declare_proc(header);
            c.flush_names();
            id
        } else {
            Err(Failed)
        };
        match id {
            Ok(id) => {
                let line_of = |n: Node| c.line(n.first_token().map_or(n.span(), |t| t.span));
                let line = line_of(header.node());
                let end_line = def.end().map_or(line, |e| line_of(e.node()));
                let proc = &mut c.prog.procs[id.0 as usize];
                (proc.line, proc.end_line) = (line, end_line);
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
        c.stmt_error = false;
        c.defaults = c.defaults_at(def);
        let ok = c.declare_params(def.header().unwrap(), id).is_ok();
        c.flush_names();
        if ok {
            c.proc_of_def.insert(def.node().key(), id);
        } else {
            c.broken.insert(id);
        }
    }
    for d in externals {
        c.stmt_error = false;
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
    c.declare_labels(&top, &skips, None);
    for &def in &defs {
        if let Some(&id) = c.proc_of_def.get(&def.node().key()) {
            let body: Vec<Node> = def.body().collect();
            c.declare_labels(&body, &skips, Some(id));
        }
    }

    // Pass 2: everything in file order (procedures where they stand, included files at their include). The
    // default types start again and are set by each `DEFxxx` as it is met.
    c.defaults = DEFAULTS_AT_START;
    for stmt in top {
        if skips.past_cap(stmt) {
            break;
        }
        c.check_statement(stmt, &skips);
    }
    c.solve_dims();
    // The program has a console by its precompiler flag too: the old compiler reads the flag, which stays set
    // when the metacommand that set it stands in a `$IF` branch the flag itself switches off (design D1 of
    // `m2-console-screenhide`; measured, `verification\v26_x26`, `x38`). "No window" comes from the lines that are
    // compiled, so a `$CONSOLE:ONLY` in such a branch leaves the window.
    c.console |= program.flags.console();
    // So are assertions: the old compiler writes the global from the feature's state, which a `$ASSERTS` met in any
    // round sets (`qb64pe.bas` 13088).
    c.asserts |= program.flags.get(qb64rust_syntax::pp::Feature::Asserts) != 0;
    if !c.console {
        // A program with neither metacommand is one with a window and no console (design D1 of
        // `m3-window-output`; measured, `verification\v27_*`: `console=0`, the same statement code).
        // Measured: "_ECHO requires $CONSOLE or $CONSOLE:ONLY to be set first" (`verification\v26_x17`).
        for at in std::mem::take(&mut c.echoes) {
            c.diags
                .error(at, "`_ECHO` needs a program with `$CONSOLE` or `$CONSOLE:ONLY`");
        }
    }
    c.prog.settings = crate::Settings {
        console: c.console,
        window: !c.console_only,
        window_hidden: c.window_hidden,
        asserts: c.asserts,
        runtime_parts: std::mem::take(&mut c.runtime_parts),
        exe_icon: c.exe_icon.take().map(|(icon, _)| icon),
    };
    (c.prog, c.diags)
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
    map: &'a SourceMap,
    /// Every tree, for the included ones ([`Checker::expand`]).
    program: &'a ParsedProgram,
    prog: Program,
    main: Scope,
    /// The scope of the procedure being checked (only meaningful while `cur` is set).
    local: Scope,
    /// The procedure being checked; `None` in the main module.
    cur: Option<ProcId>,
    procs_by_name: HashMap<String, ProcId>,
    /// Key of a `ProcDef` node whose header was declared without errors -> its procedure.
    proc_of_def: HashMap<(TreeId, u32), ProcId>,
    /// Per procedure: the scope holding its parameters, which each check of the body starts from.
    param_scopes: Vec<Scope>,
    /// Procedures whose parameter list has an error: calls of them fail without a second error.
    broken: HashSet<ProcId>,
    /// Procedures with `STATIC` after their header, and whether the body being checked still gives its own
    /// variables and arrays the life of the program: it does from its header to the end of its first `STATIC`
    /// statement, and below that statement it is a procedure like any other (measured,
    /// `verification\v24_x279`; the old compiler's flag for it is reset by that statement, `qb64pe.bas` 5827,
    /// 8805, 9558).
    static_procs: HashSet<ProcId>,
    static_scope: bool,
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
    stmt_error: bool,
    /// The program's console and window, from the metacommands of its compiled code, wherever they stand (design
    /// D1 of `m2-console-screenhide`): a `$CONSOLE` or `$CONSOLE:ONLY` was met; a `$CONSOLE:ONLY` was met (the
    /// program has no window); the last of `$SCREENHIDE` and `$SCREENSHOW` met was `$SCREENHIDE`.
    console: bool,
    console_only: bool,
    window_hidden: bool,
    /// A `$ASSERTS` or `$ASSERTS:CONSOLE` was met in the compiled code.
    asserts: bool,
    /// The first `$EXEICON` met, with the node it is (its tree and offset).
    exe_icon: Option<(crate::ExeIcon, (TreeId, u32))>,
    /// The `_ECHO` statements met: each is an error when the program turns out to have no console, which is known
    /// only after the walk (design D5 of `m2-console-screenhide`; measured, `verification\v26_x17`, `x62`).
    echoes: Vec<Span>,
    /// The optional parts of the runtime the built-ins met so far need.
    runtime_parts: std::collections::BTreeSet<crate::RuntimePart>,
    fold: bool,
    /// Names resolved in the current statement, for the symbol table.
    names: Vec<(SymbolKind, Span)>,
    /// Constants by name (without suffix): the main module's so far, and those of the procedure being checked.
    consts_main: HashMap<String, ConstId>,
    consts_local: HashMap<String, ConstId>,
    /// The auto-included constants named so far, by their place in the file: the constant each became, or why
    /// its value is not computed yet (`auto.rs`).
    auto_consts: HashMap<usize, Result<ConstId, String>>,
    /// The first use or declaration of each variable name so far, anywhere, and in the procedure being checked:
    /// a later `CONST` of the name is an error there (design D6).
    used_names: HashMap<String, Span>,
    used_local: HashMap<String, Span>,
    /// `OPTION _EXPLICIT` stands somewhere in the program (it applies to the whole program, design D7).
    explicit: bool,
    /// The statement lists of the blocks being checked, innermost last; [`Self::push`] adds to the last one.
    sinks: Vec<Vec<Stmt>>,
    /// The last `NEXT` statement (by key) whose variable did not match its `FOR` (`check\blocks.rs`).
    bad_next: Option<(TreeId, u32)>,
    /// The user types by name.
    types_by_name: HashMap<String, TypeId>,
    /// The main module's `TYPE` blocks met in pass 1 (by key), and those of them that define a type.
    types_seen: HashSet<(TreeId, u32)>,
    type_defs: HashSet<(TreeId, u32)>,
    /// Element types of the plain names of main-module `DIM SHARED` arrays declared so far.
    dim_shared_array_plain: HashMap<String, Ty>,
    /// The span of a `LEN` argument being typed: a whole user-type place loaded with exactly this span is taken as
    /// a place (its size), not rejected as a value.
    len_place: Option<Span>,
    /// The value being typed is stored into a place of type `_MEM`, so a built-in that gives a block may stand
    /// there (design D2 of `m3-mem`); the call takes it, so it holds for that call alone.
    mem_store: bool,
    /// Main-module `TYPE` blocks (by key) that pass 1 marked "not supported yet".
    types_marked: HashSet<(TreeId, u32)>,
    /// The follow-on rule is on (design D10): a declaration was marked "not supported yet", so real errors are
    /// dropped from here on.
    follow_on: bool,
    /// The parameters declared `STRING * n` or `t$n`, with n: `STRING` parameters whose `LEN` is n (design D6 of
    /// `m2-numeric-types`).
    param_len: HashMap<VarId, u32>,
    /// The `t$n` parameters by procedure, name and n: the name `t$n` in the body means the parameter (`t$` is
    /// another variable).
    fixed_params: HashMap<(ProcId, String, u32), VarId>,
    /// The default type of a name without a suffix or `AS` clause, by its first letter: A to Z, then `_` (design
    /// D2). It is the table at the point of the file being read: pass 1 sets it from [`Self::proc_defaults`] for
    /// each header, pass 2 changes it at each `DEFxxx` and `_DEFINE` statement, wherever that stands.
    defaults: [Ty; 27],
    /// Per `ProcDef` and `DeclareLibraryBlock` node (by key): the table at its header.
    proc_defaults: HashMap<(TreeId, u32), [Ty; 27]>,
    /// `OPTION BASE` as it stands at this point of the file, 0 or 1 (design D3): the lower bound of a dimension
    /// written without `TO`. Like the default types it follows the file and nothing else.
    option_base: i64,
    /// `$DYNAMIC` is in effect at this point of the file (the last of `'$DYNAMIC` and `'$STATIC` above it): a `DIM`
    /// with constant bounds then makes a dynamic array. It follows the file too (measured,
    /// `verification\v24_b_metacommand*`).
    dynamic_mode: bool,
    /// The names met as `name(…)` or in `LBOUND`/`UBOUND` that name no array and for which none was made, with a
    /// "not supported yet" mark ([`Self::implicit_array`]). An `ERASE` of such a name is marked too, where any
    /// other name that is no array is an error.
    implicit_names: HashSet<String>,
    /// `OPTION _EXPLICIT` or `OPTION _EXPLICITARRAY` stands somewhere in the program: an array used without
    /// `DIM` is an error.
    explicit_array: bool,
    /// What is being checked is a place where an allocation that runs first in the statement would not be what
    /// the old compiler does ([`Self::implicit_array`], [`Self::unjoined`]).
    no_join: Option<NoJoin>,
    /// What says how many dimensions an array has whose number is not known where it is declared, in file order
    /// ([`DimFact`]); read after every body is checked ([`Self::solve_dims`]).
    dim_facts: Vec<DimFact>,
    /// The arrays listed by `COMMON a()` or `STATIC a()` that were passed to an array parameter before anything
    /// gave them a number of dimensions: theirs is found with the parameter's.
    dims_open: HashSet<VarId>,
    /// Per `TYPE` block (by key): `OPTION BASE` as it stands at the block, which the bounds of its member arrays
    /// go by ([`Self::collect_block_settings`]).
    type_bases: HashMap<(TreeId, u32), i64>,
    /// The `TYPE` blocks (by key) that stand below a `$UNSTABLE:TYPEFIELDS`, where the member markers are allowed
    /// ([`Self::collect_block_settings`]).
    type_flags: HashSet<(TreeId, u32)>,
    /// The `TYPE` blocks (by key, with their type) with a dynamic member array whose bounds name a `CONST`: they
    /// are read when pass 2 reaches the block, below the constants above it ([`Self::type_block`]).
    type_const_bounds: HashMap<(TreeId, u32), TypeId>,
}

/// What pass 1 keeps as it walks the file in order to the `TYPE` blocks: `OPTION BASE` and whether
/// `$UNSTABLE:TYPEFIELDS` stood above.
#[derive(Clone, Copy, Debug, Default)]
struct BlockSettings {
    base: i64,
    typefields: bool,
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

/// Where an array used without `DIM` is not made by its first use ([`Checker::implicit_array`]): the old
/// compiler allocates it where the name is evaluated, which is the start of the statement only when the
/// statement writes all its code after evaluating all its parts.
#[derive(Clone, Copy, PartialEq, Eq)]
enum NoJoin {
    /// The line of a block (`IF`, `ELSEIF`, `FOR`, `DO`, `LOOP`, `WHILE`, `SELECT CASE`, `CASE`), which is no
    /// statement of its own (measured, `verification\v24_x262`).
    Header,
    /// An item of a statement that works item by item, after the part of it that runs first: a later item of
    /// `PRINT` or `READ`, an item of `WRITE` or of `PRINT #`, a target of `INPUT` or `LINE INPUT` (measured,
    /// `verification\v24_x264`, `x265`: the earlier items are printed or read, the prompt is shown, then the
    /// allocation raises its error).
    Item,
    /// A bound of `REDIM`: the array is then dynamic, also where `$DYNAMIC` is not in effect, and is allocated in
    /// the `REDIM`'s own way, so a second run makes it anew or keeps it and raises nothing (measured,
    /// `verification\v24_x266`, `x268`).
    RedimBound,
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
        if !self.stmt_error {
            self.stmt_error = true;
            if !self.follow_on {
                self.diags.error(span, msg);
            }
        }
        Failed
    }

    /// Like [`Self::error`], for a construct not handled yet: marked "not supported yet", `msg` names it.
    fn unsupported(&mut self, span: Span, msg: impl Into<String>) -> Failed {
        if !self.stmt_error {
            self.stmt_error = true;
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

    /// The 1-based line a span starts on, in its own file.
    fn line(&self, span: Span) -> u32 {
        self.map.file(span.file).line_col(span.start).0
    }

    /// The token in upper case, other than printable ASCII escaped (`\xC9`).
    fn word(&self, t: Tok) -> String {
        show_bytes(&self.text(t.span).to_ascii_uppercase())
    }

    /// Records the names of the statement just checked, unless it had an error (one error per statement).
    fn flush_names(&mut self) {
        let names = std::mem::take(&mut self.names);
        if !self.stmt_error {
            self.prog.symbols.record(names);
        }
    }

    fn check_statement(&mut self, stmt: Node, skips: &Skips) {
        if let Some(def) = ast::ProcDef::cast(stmt) {
            self.proc_def(def, skips);
            return;
        }
        if self.block_statement(stmt, skips) {
            return;
        }
        if ast::MetaCommentStmt::cast(stmt).is_some() {
            self.check_one(stmt, skips);
            // The included file's statements follow the include statement, in its place (design D10).
            let program = self.program;
            if let Some(tree) = program.included(stmt) {
                for s in ast::SourceFile::cast(tree.root())
                    .into_iter()
                    .flat_map(|f| f.statements())
                {
                    if skips.past_cap(s) {
                        break;
                    }
                    self.check_statement(s, skips);
                }
            }
            return;
        }
        self.check_one(stmt, skips);
    }

    /// A procedure where it stands: in the main module (or a file included there), its body is checked in its own
    /// scope; anywhere else it came from a file included inside a SUB, FUNCTION or block (the parser keeps every
    /// other `SUB` at file level), which the old compiler rejects (measured M7: "Expected END SUB/FUNCTION before
    /// SUB").
    fn proc_def(&mut self, def: ast::ProcDef, skips: &Skips) {
        if self.cur.is_some() || !self.sinks.is_empty() {
            if let Some(h) = def.header().filter(|h| skips.usable(h.node())) {
                self.stmt_error = false;
                let _ = self.error(
                    first_token_span(h.node()),
                    "a SUB or FUNCTION from an included file cannot stand inside a SUB, FUNCTION or block",
                );
            }
            return;
        }
        let Some(&id) = self.proc_of_def.get(&def.node().key()) else {
            return;
        };
        self.cur = Some(id);
        self.local = self.param_scopes[id.0 as usize].clone();
        self.static_scope = self.static_procs.contains(&id);
        self.consts_local.clear();
        self.used_local.clear();
        for s in def.body() {
            if skips.past_cap(s) {
                break;
            }
            self.check_statement(s, skips);
        }
        self.cur = None;
    }

    /// Pass 1 of the default types (design D2; measured, `verification\v24_a_procs_subs`, `v24_x30`): walks the
    /// file in order, through blocks, procedure bodies and included files, applies every `DEFxxx` and `_DEFINE`
    /// that pass 2 will apply, and keeps the table as it stands at each procedure's header. A statement with an
    /// error changes nothing here and is reported by pass 2.
    fn collect_defaults(&mut self, node: Node, skips: &Skips) {
        if let Some(s) = ast::DefTypeStmt::cast(node) {
            if skips.usable(node) {
                let (stmt_error, names) = (self.stmt_error, std::mem::take(&mut self.names));
                // With the statement's error "already reported", nothing is recorded.
                self.stmt_error = true;
                let _ = self.def_type(s);
                (self.stmt_error, self.names) = (stmt_error, names);
            }
            return;
        }
        if ast::InactiveCode::cast(node).is_some() {
            return;
        }
        if ast::ProcDef::cast(node).is_some() || ast::DeclareLibraryBlock::cast(node).is_some() {
            self.proc_defaults.insert(node.key(), self.defaults);
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

    /// Pass 1 of `OPTION BASE` (design D8 of `m2-defxxx-common-arrays`; measured, `verification\v24_x182`,
    /// `x183`, `x217`): the `TYPE` blocks are read before the statements, and a member array declared without
    /// `TO` starts at the base in effect where its block stands. Walks the file in order as
    /// [`Self::collect_defaults`] does, applies every `OPTION BASE` that pass 2 will apply (wherever it stands,
    /// whether or not it runs) and keeps the base at each `TYPE` block. A statement with an error changes
    /// nothing here and is reported by pass 2.
    ///
    /// `$UNSTABLE:TYPEFIELDS` is read in the same walk (design D1 of `m3-typefields`; `qb64pe.bas` 2071–2090):
    /// the old compiler sets the flag as it reads the lines in order, before its first pass, so it allows the
    /// markers in the blocks below it, also after the first statement, and not in a `$IF` branch that is not taken
    /// (measured, `verification\v35_x14`, `x15`, `x45`, `x47`).
    fn collect_block_settings(&mut self, node: Node, skips: &Skips, at: &mut BlockSettings) {
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
        if ast::TypeBlock::cast(node).is_some() {
            self.type_bases.insert(node.key(), at.base);
            if at.typefields {
                self.type_flags.insert(node.key());
            }
            return;
        }
        if ast::MetaCommentStmt::cast(node).is_some() {
            let program = self.program;
            if let Some(tree) = program.included(node) {
                self.collect_block_settings(tree.root(), skips, at);
            }
            return;
        }
        for n in node.child_nodes() {
            self.collect_block_settings(n, skips, at);
        }
    }

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
        self.stmt_error = false;
        let marks = self.diags.unsupported_count();
        self.statement(stmt);
        self.flush_names();
        if self.diags.unsupported_count() > marks && is_declaration(stmt) {
            self.follow_on = true;
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
        let facts = std::mem::take(&mut self.dim_facts);
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
                            self.stmt_error = false;
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
                            self.stmt_error = false;
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

    /// Runs `check` as a part of the statement where a joined allocation has no place (`why`).
    fn unjoined<T>(&mut self, why: NoJoin, check: impl FnOnce(&mut Self) -> R<T>) -> R<T> {
        let before = self.no_join.replace(why);
        let result = check(self);
        self.no_join = before;
        result
    }

    fn push(&mut self, node: Node, kind: StmtKind) {
        let span = node.span();
        let line = self.line(node.first_token().map_or(span, |t| t.span));
        let stmt = Stmt { span, line, kind };
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
        } else if ast::OnEventStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "event handlers (`ON TIMER`, `ON KEY`, …)"))
        } else if ast::EventSwitchStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "event switches (`TIMER ON`, `KEY(n) OFF`, …)"))
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
        } else if ast::ArrayCopyStmt::cast(node).is_some() {
            // Task 7.4: parsed, compiled later (file I/O and the other built-in statements, step 8).
            let t = first_token_span(node);
            let word = show_bytes(&self.text(t).to_ascii_uppercase());
            Err(self.unsupported(t, format!("`{word}`")))
        } else if let Some(s) = ast::BuiltinStmt::cast(node) {
            self.builtin_stmt(s)
        } else if ast::StopStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "`STOP`"))
        } else if ast::RunStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "`RUN`"))
        } else if let Some(s) = ast::CallStmt::cast(node) {
            self.call_stmt(s)
        } else if ast::ExitStmt::cast(node).is_some() {
            self.exit(node)
        } else if ast::DeclareStmt::cast(node).is_some() {
            // Ignored, as by the old compiler: calls are checked against the definition (measured).
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
        } else if ast::LineNumber::cast(node).is_some() || ast::ImplicitGoto::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "line numbers"))
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

    fn meta(&mut self, stmt: ast::MetaStmt) -> R<()> {
        let node = stmt.node();
        let tok = self.need(stmt.token(), node.span())?;
        let raw = self.text(tok.span);
        let line = meta_line(raw);
        let once = line.strip_prefix(b"$INCLUDEONCE".as_slice());
        if self.program.passes_disagree.contains(&(node.key().0, tok.span.start)) {
            // The parser read the condition with the flag's value on this line; the old compiler's main pass
            // reads it with the value of the first line (design D7 of `m2-console-screenhide`; measured,
            // `verification\v26_x27`, `x34`: its program prints nothing from the branches, and a `CONST` in them
            // is "Name already in use").
            let flag = stmt.precompiler_flag(raw).unwrap_or("a precompiler flag");
            let msg = format!("`{flag}` in a `$IF` between two lines that set it differently");
            Err(self.unsupported(tok.span, msg))
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
                Ok(ConsoleMeta::Console) => self.console = true,
                Ok(ConsoleMeta::ConsoleOnly) => {
                    // Each one selects the console where it stands (measured, `verification\v26_x25`).
                    self.console = true;
                    self.console_only = true;
                    self.push(node, StmtKind::ConsoleOnly);
                }
                // The last of the two in the file counts (`qb64pe.bas` 3615–3624).
                Ok(ConsoleMeta::ScreenHide) => self.window_hidden = true,
                Ok(ConsoleMeta::ScreenShow) => self.window_hidden = false,
                // The global `asserts` (design D1 of `m3-asserts-debug-zlib`); `$ASSERTS:CONSOLE` gives a console as
                // `$CONSOLE` does (`qb64pe.bas` 1984; measured, `verification\v33_a06`).
                Ok(ConsoleMeta::Asserts) => self.asserts = true,
                Ok(ConsoleMeta::AssertsConsole) => {
                    self.asserts = true;
                    self.console = true;
                }
                // No debugger code is written (user, design D2): only the flags, which the parser settles, and the
                // old compiler's warning from the command line (`qb64pe.bas` 3556).
                Ok(ConsoleMeta::Debug) => self.diags.warning(tok.span, "$DEBUG features only work from the IDE"),
                Err(msg) => return Err(self.error(tok.span, msg)),
            }
            Ok(())
        } else if line.starts_with(b"$EXEICON") {
            self.exe_icon_meta(node, tok)
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
        } else if line.starts_with(b"$MIDISOUNDFONT:") {
            // The old compiler's own error, whatever follows the colon (design D3 of `m3-sound-handles`;
            // `qb64pe.bas` 2092; measured, `verification\v39_x73`–`x81`). Without the colon it is an unknown
            // metacommand, as below.
            Err(self.error(
                tok.span,
                "$MIDISOUNDFONT is a deprecated keyword, use _MIDISOUNDBANK instead",
            ))
        } else {
            let shown = show_bytes(raw.split(|&b| b == b':' || b == b' ').next().unwrap_or(raw));
            Err(self.unsupported(tok.span, format!("metacommand `{shown}`")))
        }
    }

    /// `$EXEICON:'name'` (design D4 of `m3-sound`; `qb64pe.bas` 3829–3880; measured, `verification\v29_e01`–`e31`):
    /// the icon of the executable, and where it stands the statement `_ICON` with no argument. The old compiler
    /// takes any line that starts with `$EXEICON` in upper case; the name stands between the line's first two
    /// `'`, and the rest of the line is dropped (the lexer makes one token of a metacommand's whole line).
    fn exe_icon_meta(&mut self, node: Node, tok: Tok) -> R<()> {
        if let Some((_, first)) = &self.exe_icon {
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
        let absolute = matches!(written, [drive, b':', b'/' | b'\\', ..] if drive.is_ascii_alphabetic())
            || matches!(written, [b'/' | b'\\', b'/' | b'\\', ..]);
        if crate::ExeIcon::beside_source(&name).is_none() && !absolute {
            // The old compiler looks for such a name in its own folder, for one with a single leading slash on
            // its own drive (`e02`, `e03`, `e08`, `e18`) and for one with a drive and no slash after it
            // (`C:game.ico`) in the folder it stands in on that drive, none of which has a counterpart here.
            let msg = "a `$EXEICON` name that neither starts with `./` nor is absolute";
            return Err(self.unsupported(tok.span, msg));
        }
        self.absent_stmt(node, "_ICON")?;
        let icon = crate::ExeIcon {
            name,
            shown,
            span: tok.span,
        };
        self.exe_icon = Some((icon, node.key()));
        Ok(())
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
        self.map.file(self.program.tree(tree).file).line_col(offset).0
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
                    Some(MemoryMode::Static) => self.dynamic_mode = false,
                    Some(MemoryMode::Dynamic) => self.dynamic_mode = true,
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
            return self.copy_array(node, target_node, source);
        }
        if let Some((_, t)) = self.whole_array(target_node) {
            return Err(self.half_whole(node, (target_node, t), source, false));
        }
        if let Some((_, t)) = self.whole_array(source) {
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
                                self.names.push((SymbolKind::Var(v), t.span));
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
            ast::Expr::Call(c) => self.call_place(c)?,
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
            self.mem_store = true;
            let value = self.expr(value_node);
            self.mem_store = false;
            let value = value?;
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
