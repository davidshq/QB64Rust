//! A whole program as parsed: one tree per file at each place it is included (design D9). The parser does no I/O;
//! included files come through a [`Loader`].

use crate::tree::{GreenNode, Node, TreeId};
use qb64rust_base::{Diagnostics, FileId, SourceMap};
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
}

impl ParsedProgram {
    pub fn main(&self) -> &Tree {
        &self.trees[0]
    }

    pub fn tree(&self, id: TreeId) -> &Tree {
        &self.trees[id.0 as usize]
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
        }
    }

    /// Reserves the next tree id.
    pub(crate) fn next_tree(&mut self) -> TreeId {
        self.trees.push(None);
        TreeId(qb64rust_base::to_u32(self.trees.len() - 1))
    }
}

/// Parses the main file and the files it includes through `loader`, in file order, with one preprocessor state
/// for all of them.
pub fn parse(map: &mut SourceMap, main: FileId, loader: &mut dyn Loader) -> ParsedProgram {
    let bytes = map.file(main).bytes.clone();
    let mut inc = Includer::new(map, loader);
    let id = inc.next_tree();
    let (tree, _) = crate::parser::parse_tree_with(id, main, &bytes, crate::pp::PpState::default(), &mut inc, false);
    inc.trees[0] = Some(tree);
    ParsedProgram {
        trees: inc
            .trees
            .into_iter()
            .map(|t| t.expect("every tree is filled when its file is done"))
            .collect(),
        includes: inc.includes,
    }
}
