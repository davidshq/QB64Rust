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
}

impl ParsedProgram {
    pub fn main(&self) -> &Tree {
        &self.trees[0]
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
}

impl<'h> Includer<'h> {
    pub(crate) fn new(map: &'h mut SourceMap, loader: &'h mut dyn Loader) -> Includer<'h> {
        Includer {
            map,
            loader,
            trees: Vec::new(),
            includes: HashMap::new(),
            once: HashSet::new(),
            depth: 0,
            passes_disagree: HashSet::new(),
            cross_open: HashMap::new(),
        }
    }

    /// Reserves the next tree id.
    pub(crate) fn next_tree(&mut self) -> TreeId {
        self.trees.push(None);
        TreeId(qb64rust_base::to_u32(self.trees.len() - 1))
    }
}

/// Parses the main file and the files it includes through `loader`, in file order, with one preprocessor state
/// for all of them, in a round where the precompiler flags are `flags`. Also gives the settings the round met
/// and whether a condition named a flag.
/// `auto` is the source file of QB64pe's file included after the last line once a round has added it to the map.
fn parse_round(
    map: &mut SourceMap,
    main: FileId,
    loader: &mut dyn Loader,
    flags: PpFlags,
    auto: &mut Option<FileId>,
    color_file: &mut Option<FileId>,
) -> Round<ParsedProgram> {
    let bytes = map.file(main).bytes.clone();
    let mut inc = Includer::new(map, loader);
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
    let (tree, mut pp, _) = crate::parser::parse_tree_with(id, main, &bytes, pp, &mut inc, false, Vec::new());
    inc.trees[0] = Some(tree);
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
    Round {
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
    match settle(MAX_ROUNDS, |flags| {
        parse_round(map, main, loader, flags, &mut auto, &mut color_file)
    }) {
        Ok((mut program, flags)) => {
            program.flags = flags;
            program
        }
        Err(mut program) => {
            // Every round but the last locks a feature, so this is not reached; an error, not a panic, if it is.
            let at = Span::new(main, 0, 0);
            let msg = "internal error: the precompiler flags did not settle";
            program.trees[0].diagnostics.push(Diagnostic::error(at, msg));
            program
        }
    }
}
