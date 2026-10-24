//! A whole program as parsed: one tree per file at each place it is included (design D9). The parser does no I/O;
//! included files come through a [`Loader`].

use crate::pp::{MAX_ROUNDS, PpFlags, PpState, Round, settle};
use crate::tree::{GreenNode, Node, TreeId};
use qb64rust_base::{Diagnostic, Diagnostics, FileId, SourceMap, Span};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// The lossless tree of one file at one inclusion, with the parse errors found in it.
pub struct Tree {
    pub id: TreeId,
    pub file: FileId,
    pub green: GreenNode,
    pub diagnostics: Diagnostics,
}

impl Tree {
    pub fn root(&self) -> Node<'_> {
        Node::root(&self.green, self.id, self.file)
    }
}

/// Every tree of a program; tree 0 is the main file's.
pub struct ParsedProgram {
    pub trees: Vec<Tree>,
    /// (tree, offset of an include statement) -> the tree of the file it includes.
    pub includes: HashMap<(TreeId, u32), TreeId>,
    /// The precompiler flags as settled over the whole program ([`parse_settled`]): the values every `$IF` of
    /// these trees was read with at the first line. A feature may be set though the line that sets it is not in
    /// the trees (it stands in a `$IF` branch its own flag switches off).
    pub flags: PpFlags,
    /// (tree, offset of the metacommand token) of each `$IF` and `$ELSEIF` that names a flag where the old
    /// compiler's two passes see different values of it; `sema` marks them "not supported yet".
    pub passes_disagree: HashSet<(TreeId, u32)>,
    /// The block nodes (tree, offset) whose closer stands in another tree (design D8 of
    /// `m4-statements-metacommands`), each with the `CrossCloser` there that ends it, not the end of its node.
    pub cross_open: HashMap<(TreeId, u32), (TreeId, u32)>,
    /// The tree of QB64pe's file included after the last line, when the program needs it (design D7 of
    /// `m3-network-embed`): parsed after the main file and its includes, with the preprocessor state the main file
    /// ended with, as if it stood after the main file's last line.
    pub after_last_line: Option<TreeId>,
    /// The tree of QB64pe's colour file (`color0.bi` or `color32.bi`) when the program has `$COLOR:0` or
    /// `$COLOR:32` (design D8 of `m3-constant-evaluation`): parsed before the main file, as the old compiler's
    /// recompile includes it before the program's first line.
    pub colors: Option<TreeId>,
    /// The libraries the program's `$USELIBRARY` lines registered, in the old compiler's order (design D7 of
    /// `m4-builtins-remaining`; measured, `verification\v58_u00`–`u26`).
    pub libraries: Vec<UsedLibrary>,
    /// The trees of the libraries' `IncAtTop` files in the order they are read (the last library's first), parsed
    /// after the colour file and before the main file.
    pub library_top: Vec<TreeId>,
    /// QB64pe's `aftermain.bas` (an `END`), parsed after the main file when a library has an `IncAfterMain` file,
    /// and the trees of those files after it, in order.
    pub after_main: Option<TreeId>,
    pub library_after_main: Vec<TreeId>,
    /// The trees of the libraries' `IncAtBottom` files, in order, parsed after those and before `afterlastline.bm`.
    pub library_bottom: Vec<TreeId>,
}

/// A library a `$USELIBRARY` line registered (design D7 of `m4-builtins-remaining`), kept across the rounds of a
/// compile as the old compiler keeps its list (`qb64pe.bas` 1955–1976): its name as written between the quotes,
/// the file and line that named it (a library named by two lines is registered twice and its files included twice,
/// as the old compiler has it: `verification\v58_u04`, `u05`), and the files its descriptor names, loaded under the
/// compiler root when it was registered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsedLibrary {
    pub name: Vec<u8>,
    pub needy: (FileId, u32),
    pub top: Option<FileId>,
    pub after_main: Option<FileId>,
    pub bottom: Option<FileId>,
}

impl ParsedProgram {
    pub fn main(&self) -> &Tree {
        &self.trees[0]
    }

    /// Whether tree `id` is one the compiler added itself: the colour file, `aftermain.bas`, `afterlastline.bm` or
    /// a library's file.
    pub fn is_auto_tree(&self, id: TreeId) -> bool {
        self.colors == Some(id)
            || self.after_last_line == Some(id)
            || self.after_main == Some(id)
            || self.library_top.contains(&id)
            || self.library_after_main.contains(&id)
            || self.library_bottom.contains(&id)
    }

    /// For a tree of a library's file or of `aftermain.bas`, where the old compiler counts it in the main file
    /// (design D7 of `m4-builtins-remaining`): `Some(true)` for a top file, before the first line (`_ERRORLINE` 0),
    /// `Some(false)` for one after the main module (the main file's last line); `None` for any other tree.
    pub fn library_tree_is_top(&self, id: TreeId) -> Option<bool> {
        if self.library_top.contains(&id) {
            Some(true)
        } else if self.after_main == Some(id)
            || self.library_after_main.contains(&id)
            || self.library_bottom.contains(&id)
        {
            Some(false)
        } else {
            None
        }
    }

    pub fn tree(&self, id: TreeId) -> &Tree {
        &self.trees[id.0 as usize]
    }

    /// The closer of a block whose closer stands in another tree ([`Self::cross_open`]): the node the `CrossCloser`
    /// there holds (`NextStmt`, `LoopStmt`, `BlockEnd` or `ProcEnd`).
    pub fn closer_of(&self, block: Node) -> Option<Node<'_>> {
        let &(tree, offset) = self.cross_open.get(&block.key())?;
        let mut node = self.tree(tree).root();
        loop {
            let span = |n: Node| n.offset..n.offset + n.green.len;
            node = node.child_nodes().find(|n| span(*n).contains(&offset))?;
            if node.offset == offset && node.kind() == crate::SyntaxKind::CrossCloser {
                return node.child_nodes().next();
            }
        }
    }

    /// The tree an include statement brings in.
    pub fn included(&self, include: Node) -> Option<&Tree> {
        self.includes.get(&include.key()).map(|&id| self.tree(id))
    }

    /// The parse errors of all trees, in tree order.
    pub fn diagnostics(&self) -> Diagnostics {
        let mut all = Diagnostics::new();
        for t in &self.trees {
            all.extend(t.diagnostics.clone());
        }
        all
    }
}

/// Why an included file could not be loaded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadError {
    NotFound,
    Unreadable(String),
}

/// Finds and reads an included file and adds it to the [`SourceMap`]. `from` is the including file; `path` is the
/// name as written in the include.
pub trait Loader {
    fn load(&mut self, map: &mut SourceMap, from: FileId, path: &[u8]) -> Result<FileId, LoadError>;

    /// Finds and reads a file by its path under the compiler root alone (the files of a `$USELIBRARY` library,
    /// design D7 of `m4-builtins-remaining`) and adds it to the map, named by that path. A loader that knows no
    /// root finds none.
    fn load_root(&mut self, _map: &mut SourceMap, _path: &std::path::Path) -> Result<FileId, LoadError> {
        Err(LoadError::NotFound)
    }

    /// The text of the descriptor of the library `name` under the compiler root
    /// ([`crate::library::descriptor_path`]); `NotFound` when there is none.
    fn library(&mut self, _name: &[u8]) -> Result<Vec<u8>, LoadError> {
        Err(LoadError::NotFound)
    }
}

/// The file name an include statement gives, as the old compiler reads it: the name as written (taken as UTF-8; the
/// inputs use ASCII names), a leading `.\` or `./` dropped. A loader looks a relative name up in the including
/// file's folder, then relative to the compiler root, never the working directory; an absolute name as written
/// (design D9, measured M7).
pub fn include_name(written: &[u8]) -> PathBuf {
    #[expect(clippy::disallowed_methods, reason = "a file name, not BASIC source")]
    let written = String::from_utf8_lossy(written).into_owned();
    let name = written
        .strip_prefix(".\\")
        .or_else(|| written.strip_prefix("./"))
        .unwrap_or(&written);
    PathBuf::from(name)
}

/// A loader that finds no file, for tests of single files.
pub struct NoLoader;

impl Loader for NoLoader {
    fn load(&mut self, _map: &mut SourceMap, _from: FileId, _path: &[u8]) -> Result<FileId, LoadError> {
        Err(LoadError::NotFound)
    }
}

/// What the parser needs to include files (design D9): the source map and loader, the trees made so far (by id;
/// a tree's slot is filled when its file is done, so an including tree's id comes before its includes'), the
/// include map, the files that hold `$INCLUDEONCE`, and how deep the inclusion is.
pub(crate) struct Includer<'h> {
    pub(crate) map: &'h mut SourceMap,
    pub(crate) loader: &'h mut dyn Loader,
    pub(crate) trees: Vec<Option<Tree>>,
    pub(crate) includes: HashMap<(TreeId, u32), TreeId>,
    pub(crate) once: HashSet<FileId>,
    pub(crate) depth: u32,
    /// [`ParsedProgram::passes_disagree`], as the parser fills it.
    pub(crate) passes_disagree: HashSet<(TreeId, u32)>,
    /// [`ParsedProgram::cross_open`], as the parser fills it.
    pub(crate) cross_open: HashMap<(TreeId, u32), (TreeId, u32)>,
    /// The libraries registered so far, across rounds ([`UsedLibrary`]), and whether this round added one (the
    /// program is then parsed again, [`Round::again`]).
    pub(crate) libraries: &'h mut Vec<UsedLibrary>,
    pub(crate) libraries_added: bool,
}

impl<'h> Includer<'h> {
    pub(crate) fn new(
        map: &'h mut SourceMap,
        loader: &'h mut dyn Loader,
        libraries: &'h mut Vec<UsedLibrary>,
    ) -> Includer<'h> {
        Includer {
            map,
            loader,
            trees: Vec::new(),
            includes: HashMap::new(),
            once: HashSet::new(),
            depth: 0,
            passes_disagree: HashSet::new(),
            cross_open: HashMap::new(),
            libraries,
            libraries_added: false,
        }
    }

    /// Reserves the next tree id.
    pub(crate) fn next_tree(&mut self) -> TreeId {
        self.trees.push(None);
        TreeId(qb64rust_base::to_u32(self.trees.len() - 1))
    }
}

/// Parses one of QB64pe's own files or a library's (in the map already) as a tree of its own, with the
/// preprocessor state as it stands; gives the tree and the state it leaves.
fn parse_auto(inc: &mut Includer<'_>, file: FileId, pp: PpState) -> (TreeId, PpState) {
    let id = inc.next_tree();
    let bytes = inc.map.file(file).bytes.clone();
    let (tree, end, _) = crate::parser::parse_tree_with(id, file, &bytes, pp, inc, false, Vec::new());
    inc.trees[id.0 as usize] = Some(tree);
    (id, end)
}

/// Parses the main file and the files it includes through `loader`, in file order, with one preprocessor state
/// for all of them, in a round where the precompiler flags are `flags`. Also gives the settings the round met
/// and whether a condition named a flag.
/// `auto` is the source file of QB64pe's file included after the last line once a round has added it to the map;
/// `after_main_file` that of its `aftermain.bas`; `libraries` the `$USELIBRARY` libraries registered so far, whose
/// files this round parses where the old compiler includes them (design D7 of `m4-builtins-remaining`).
#[expect(clippy::too_many_arguments, reason = "the state the rounds of one compile share")]
fn parse_round(
    map: &mut SourceMap,
    main: FileId,
    loader: &mut dyn Loader,
    flags: PpFlags,
    auto: &mut Option<FileId>,
    color_file: &mut Option<FileId>,
    after_main_file: &mut Option<FileId>,
    libraries: &mut Vec<UsedLibrary>,
) -> Round<ParsedProgram> {
    let bytes = map.file(main).bytes.clone();
    let mut inc = Includer::new(map, loader, libraries);
    let id = inc.next_tree();
    let mut pp = PpState::with_flags(flags);
    let mut colors = None;
    if let Some((name, text)) = match flags.get(crate::pp::Feature::Color) {
        1 => Some((qb64rust_builtins::COLOR0_NAME, qb64rust_builtins::COLOR0)),
        2 => Some((qb64rust_builtins::COLOR32_NAME, qb64rust_builtins::COLOR32)),
        _ => None,
    } {
        let file = *color_file.get_or_insert_with(|| inc.map.add(name, text.to_vec()));
        let cid = inc.next_tree();
        let (tree, end, _) = crate::parser::parse_tree_with(cid, file, text, pp, &mut inc, false, Vec::new());
        inc.trees[cid.0 as usize] = Some(tree);
        pp = end;
        colors = Some(cid);
    }
    // The libraries' top files, the last library's first (`qb64pe.bas` 1740–1744: dependencies, registered from
    // a library's own file, come before it).
    let tops: Vec<FileId> = inc.libraries.iter().rev().filter_map(|l| l.top).collect();
    let mut library_top = Vec::new();
    for file in tops {
        let (tid, end) = parse_auto(&mut inc, file, pp);
        pp = end;
        library_top.push(tid);
    }
    let (tree, mut pp, _) = crate::parser::parse_tree_with(id, main, &bytes, pp, &mut inc, false, Vec::new());
    inc.trees[0] = Some(tree);
    // After the main module: `aftermain.bas` (an `END`), then the libraries' after-main files in order (1755–1761);
    // then their bottom files in order (1776–1780), before `afterlastline.bm`.
    let after_mains: Vec<FileId> = inc.libraries.iter().filter_map(|l| l.after_main).collect();
    let mut after_main = None;
    let mut library_after_main = Vec::new();
    if !after_mains.is_empty() {
        let file = *after_main_file.get_or_insert_with(|| {
            inc.map.add(
                qb64rust_builtins::AFTER_MAIN_NAME,
                qb64rust_builtins::AFTER_MAIN.to_vec(),
            )
        });
        let (tid, end) = parse_auto(&mut inc, file, pp);
        pp = end;
        after_main = Some(tid);
        for file in after_mains {
            let (tid, end) = parse_auto(&mut inc, file, pp);
            pp = end;
            library_after_main.push(tid);
        }
    }
    let bottoms: Vec<FileId> = inc.libraries.iter().filter_map(|l| l.bottom).collect();
    let mut library_bottom = Vec::new();
    for file in bottoms {
        let (tid, end) = parse_auto(&mut inc, file, pp);
        pp = end;
        library_bottom.push(tid);
    }
    let mut after_last_line = None;
    if pp.needs_after_last_line() {
        let file = *auto.get_or_insert_with(|| {
            inc.map.add(
                qb64rust_builtins::AFTER_LAST_LINE_NAME,
                qb64rust_builtins::AFTER_LAST_LINE.to_vec(),
            )
        });
        let id = inc.next_tree();
        let (tree, end, _) = crate::parser::parse_tree_with(
            id,
            file,
            qb64rust_builtins::AFTER_LAST_LINE,
            pp,
            &mut inc,
            false,
            Vec::new(),
        );
        inc.trees[id.0 as usize] = Some(tree);
        pp = end;
        after_last_line = Some(id);
    }
    let (settings, flag_named) = pp.round_facts();
    let libraries = inc.libraries.clone();
    Round {
        again: inc.libraries_added,
        result: ParsedProgram {
            trees: inc
                .trees
                .into_iter()
                .map(|t| t.expect("every tree is filled when its file is done"))
                .collect(),
            includes: inc.includes,
            flags,
            passes_disagree: inc.passes_disagree,
            cross_open: inc.cross_open,
            after_last_line,
            colors,
            libraries,
            library_top,
            after_main,
            library_after_main,
            library_bottom,
        },
        settings,
        flag_named,
    }
}

/// Parses a program as the old compiler compiles it: again and again until the precompiler flags no longer
/// change ([`settle`]; design D7 of `m2-console-screenhide`). Almost every program is parsed once: only one with
/// a `$IF` that names a flag can read differently in another round. The trees and diagnostics are the last
/// round's. A loader must give the same file for the same name in every round.
pub fn parse_settled(map: &mut SourceMap, main: FileId, loader: &mut dyn Loader) -> ParsedProgram {
    let mut auto = None;
    let mut color_file = None;
    let mut after_main_file = None;
    let mut libraries = Vec::new();
    match settle(MAX_ROUNDS, |flags| {
        parse_round(
            map,
            main,
            loader,
            flags,
            &mut auto,
            &mut color_file,
            &mut after_main_file,
            &mut libraries,
        )
    }) {
        Ok((mut program, flags)) => {
            program.flags = flags;
            program
        }
        Err(mut program) => {
            // Every round but the last locks one of the seven features or adds a library, so only a program (with
            // its libraries' files) naming about 60 libraries reaches this: the last ones' files are then left out.
            let at = Span::new(main, 0, 0);
            let msg = format!(
                "the program was parsed {MAX_ROUNDS} times without settling: it names too many `$USELIBRARY` \
                 libraries (each adds a round), and those added last are left out"
            );
            program.trees[0].diagnostics.push(Diagnostic::error(at, msg));
            program
        }
    }
}
