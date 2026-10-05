//! A whole program as parsed: one tree per file at each place it is included (design D9). The parser does no I/O;
//! included files come through a [`Loader`].

use crate::tree::{GreenNode, Node, TreeId};
use qb64rust_base::{Diagnostics, FileId, SourceMap};
use std::collections::HashMap;

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

/// A loader that finds no file, for tests of single files.
pub struct NoLoader;

impl Loader for NoLoader {
    fn load(&mut self, _map: &mut SourceMap, _from: FileId, _path: &[u8]) -> Result<FileId, LoadError> {
        Err(LoadError::NotFound)
    }
}

/// Parses the main file and, from task 8.1 of `m2-parser-breadth` on, the files it includes through `_loader`.
pub fn parse(map: &mut SourceMap, main: FileId, _loader: &mut dyn Loader) -> ParsedProgram {
    let bytes = map.file(main).bytes.clone();
    ParsedProgram {
        trees: vec![crate::parser::parse_tree(TreeId(0), main, &bytes)],
        includes: HashMap::new(),
    }
}
