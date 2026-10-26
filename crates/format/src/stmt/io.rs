//! Assignment and the console and file statements (`qb64pe.bas` 12434-12450 `LET` and `assign` 17343; `xprint`
//! 27708 and the file `PRINT #` 27260).
//!
//! - An assignment: the target, ` = `, the value; `Let` before it when written.
//! - `PRINT` (also written `?`): `Print`, then each item after a space with its separator glued to it (`a; b`), a
//!   separator with no item after a space; a second `;` in a row is left out. `#n,` glued after `Print #`. `Using`,
//!   its format and `;` glued.

use crate::fmt::Fmt;
use crate::lay::Lay;
use qb64rust_syntax::SyntaxKind::*;
use qb64rust_syntax::tree::{Element, Node, Tok};

impl Fmt<'_> {
    /// `[LET] target = value`.
    pub(crate) fn assign(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for e in self.elems(n) {
            match e {
                Element::Token(t) if self.is_word(t, "LET") => {
                    l.text(&self.kw("LET"));
                }
                Element::Token(t) if t.kind == Eq => {
                    l.sp().text(b"=").sp();
                }
                Element::Token(t) => {
                    l.then(&Lay::of(self.tok(t)));
                }
                Element::Node(c) => {
                    if !l.is_empty() && l.last() != Some(crate::apply::SP) {
                        l.sp();
                    }
                    l.append(&self.expr(c));
                }
            }
        }
        l
    }

    /// `PRINT` and `LPRINT`.
    pub(crate) fn print(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        // An item waiting for the separator after it, and whether it ends with a string literal.
        let mut item: Option<Lay> = None;
        let mut last_string = false;
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == Question || self.is_word(t, "PRINT") => {
                    l.text(&self.kw("PRINT"));
                }
                Element::Token(t) if self.is_word(t, "LPRINT") => {
                    l.text(&self.kw("LPRINT"));
                }
                Element::Token(t) if t.kind == Semicolon || t.kind == Comma => {
                    let sep = self.tok(t);
                    match item.take() {
                        Some(x) => {
                            l.sp().append(&x).sp2().text(sep);
                        }
                        // `;;` is one `;`.
                        None if sep == b";" && l.last() == Some(b';') => {}
                        None => {
                            l.sp().text(sep);
                        }
                    }
                }
                Element::Token(_) => {}
                Element::Node(c) if c.kind() == FileNumber => {
                    l.sp().append(&self.file_number(c));
                }
                Element::Node(c) if c.kind() == UsingClause => {
                    // An item before `USING` with no separator: `;` after it when it ends with a string literal (the
                    // auto-added semicolons, `qb64pe.bas` 11270), else none (`gotopu`).
                    if let Some(x) = item.take() {
                        l.sp().append(&x);
                        if last_string {
                            l.sp2().text(b";");
                        }
                    }
                    l.sp().text(&self.kw("USING"));
                    for x in self.elems(c) {
                        match x {
                            Element::Token(t) if t.kind == Semicolon => {
                                l.sp2().text(b";");
                            }
                            Element::Token(_) => {}
                            Element::Node(f) => {
                                l.sp().append(&self.expr(f));
                            }
                        }
                    }
                }
                Element::Node(c) => {
                    // Two items with no separator between them are joined by `;` (`PRINT "a" "b"` is `Print "a"; "b"`).
                    if let Some(x) = item.take() {
                        l.sp().append(&x).sp2().text(b";");
                    }
                    last_string = c
                        .tokens()
                        .into_iter()
                        .rev()
                        .find(|t| !t.kind.is_trivia())
                        .is_some_and(|t| t.kind == StringLit);
                    item = Some(self.expr(c));
                }
            }
        }
        if let Some(x) = item {
            l.sp().append(&x);
        }
        l
    }

    /// The layout of an I/O statement, `None` for a kind this family does not lay out.
    pub(crate) fn io(&self, n: Node<'_>) -> Option<Lay> {
        match n.kind() {
            InputStmt | LineInputStmt => Some(self.input(n)),
            ArrayCopyStmt => Some(self.array_copy(n)),
            OnEventStmt => Some(self.on_event(n)),
            PaletteUsingStmt => Some(self.palette_using(n)),
            CloseStmt | WriteStmt | FieldStmt | LsetStmt | SwapStmt | MemStmt | KeyStmt | DeclareLibraryHeader => {
                Some(self.general(n))
            }
            _ => None,
        }
    }

    /// `ON TIMER|KEY|STRIG…(args) GOSUB label` or `… name [argument]` (`qb64pe.bas` 7600-8000): `On Timer(`
    /// glued, the arguments, `) `, then `GoSub` and the label, or the SUB as declared and its argument.
    fn on_event(&self, n: Node<'_>) -> Lay {
        let elems = self.elems(n);
        let mut l = Lay(self.kw("ON"));
        let mut gosub = false;
        for (i, e) in elems.into_iter().enumerate().skip(1) {
            match e {
                // The event word, the built-in's name (`Timer`, `Key`, `Strig`).
                Element::Token(t) if i == 1 => {
                    l.sp().text(&self.name(t));
                }
                Element::Token(t) if self.is_word(t, "GOSUB") => {
                    gosub = true;
                    l.then(&Lay(self.kw("GOSUB")));
                }
                Element::Token(t) if gosub && (t.kind == Ident || t.kind == Number) => {
                    l.then(&Lay(self.label_ref(t)));
                }
                // The SUB that handles the event, as declared.
                Element::Token(t) if t.kind == Ident => {
                    l.then(&Lay(self.name(t)));
                }
                Element::Token(_) => {}
                Element::Node(c) if c.kind() == ArgList => {
                    l.append(&self.expr_args(c));
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }

    /// `(args)` glued to what is before it, as an expression's argument list.
    fn expr_args(&self, c: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for e in self.elems(c) {
            match e {
                Element::Token(t) if t.kind == LParen => {
                    l.text(b"(").sp2();
                }
                Element::Token(t) if t.kind == RParen => {
                    l.sp2().text(b")");
                }
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",").sp();
                }
                Element::Token(_) => {}
                Element::Node(x) => {
                    l.append(&self.expr(x));
                }
            }
        }
        l
    }

    /// `PALETTE USING array[(index)]` (`qb64pe.bas` 7240): `()` after a bare array name.
    fn palette_using(&self, n: Node<'_>) -> Lay {
        let mut l = Lay(self.kw("PALETTE"));
        l.sp().text(&self.kw("USING")).sp();
        if let Some(c) = n.child_nodes().next() {
            let mut arg = self.expr(c);
            if c.kind() == NameRef {
                arg.text(b"(").sp2().text(b")");
            }
            l.append(&arg);
        }
        l
    }

    /// `[LINE] INPUT`: as the general shape, but a `;` right after the keywords takes a space before it (`l$ + sp +
    /// ";"`, `qb64pe.bas` 11128).
    fn input(&self, n: Node<'_>) -> Lay {
        let mut l = self.general(n);
        let words = if n.kind() == LineInputStmt { 2 } else { 1 };
        let elems = self.elems(n);
        if matches!(elems.get(words), Some(Element::Token(t)) if t.kind == Semicolon) {
            let mut head = self.kw("INPUT");
            if words == 2 {
                head = [self.kw("LINE"), vec![crate::apply::SP], head].concat();
            }
            if l.0.starts_with(&head) && l.0.get(head.len()) == Some(&crate::apply::SP2) {
                l.0[head.len()] = crate::apply::SP;
            }
        }
        l
    }

    /// `_ARRAYCOPY source TO target` (`ArrayCopyRefCaseLayout$`, `source\utilities\arrcpy.bm`): each reference
    /// compacted as written, its array and member names as declared, a `TO` of its last parentheses in the keyword
    /// style.
    fn array_copy(&self, n: Node<'_>) -> Lay {
        let mut l = Lay(self.kw("_ARRAYCOPY"));
        for e in self.elems(n).into_iter().skip(1) {
            match e {
                Element::Token(t) => {
                    l.sp().text(&self.kw_tok(t));
                }
                Element::Node(c) => {
                    let toks: Vec<Tok> = c.tokens().into_iter().filter(|t| !t.kind.is_trivia()).collect();
                    // The `TO`s at the top of the last parentheses.
                    let last_open = {
                        let mut depth = 0;
                        let mut open = None;
                        for (i, t) in toks.iter().enumerate().rev() {
                            match t.kind {
                                RParen => depth += 1,
                                LParen => {
                                    depth -= 1;
                                    if depth == 0 {
                                        open = Some(i);
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                        open
                    };
                    let mut top_to = Vec::new();
                    if let Some(open) = last_open {
                        let mut depth = 0;
                        for t in toks.iter().skip(open + 1) {
                            match t.kind {
                                LParen => depth += 1,
                                RParen => depth -= 1,
                                _ if depth == 0 && self.is_word(*t, "TO") => top_to.push(t.span.start),
                                _ => {}
                            }
                        }
                    }
                    // The names respelled: the array's (the first element) and each member after a `.` outside
                    // parentheses before the last ones; a subscript's names stay as written.
                    let prefix_end = last_open.unwrap_or(toks.len());
                    let mut named = Vec::new();
                    let mut depth = 0;
                    for (i, t) in toks[..prefix_end].iter().enumerate() {
                        match t.kind {
                            LParen => depth += 1,
                            RParen => depth -= 1,
                            Ident if depth == 0 && (i == 0 || toks[i - 1].kind == Dot) => named.push(t.span.start),
                            _ => {}
                        }
                    }
                    let spell = |t: Tok| -> Vec<u8> {
                        if top_to.contains(&t.span.start) {
                            self.kw("TO")
                        } else if named.contains(&t.span.start) {
                            self.name(t)
                        } else {
                            self.element(t)
                        }
                    };
                    l.sp().text(&self.compact(&toks, &spell));
                }
            }
        }
        l
    }

    /// The common shape of the I/O handlers: keywords and items joined by `sp`, `sp2` before `,` and `;`, `#` glued
    /// to the number after it, a file number's `#n,` glued, `=` with a space on each side.
    pub(crate) fn general(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        let mut glue = false;
        // After `GOSUB` or `GOTO` (`ON TIMER(1) GOSUB tick`): a label.
        let mut after_jump = false;
        let add = |l: &mut Lay, x: &Lay, glue: &mut bool| {
            if *glue {
                l.append(x);
            } else {
                l.then(x);
            }
            *glue = false;
        };
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == Comma || t.kind == Semicolon => {
                    l.sp2().text(self.tok(t));
                }
                Element::Token(t) if t.kind == Hash => {
                    add(&mut l, &Lay::of(b"#"), &mut glue);
                    glue = true;
                }
                Element::Token(t) if t.kind == Minus => {
                    l.sp2().text(b"-");
                    glue = true;
                }
                Element::Token(t) if (t.kind == Ident || t.kind == Number) && after_jump => {
                    add(&mut l, &Lay(self.label_ref(t)), &mut glue);
                }
                Element::Token(t) if t.kind == Ident => {
                    after_jump = self.is_word(t, "GOSUB") || self.is_word(t, "GOTO");
                    add(&mut l, &Lay(self.kw_tok(t)), &mut glue);
                }
                Element::Token(t) if t.kind == StringLit || t.kind == Number => {
                    add(&mut l, &Lay(self.literal(t)), &mut glue);
                }
                Element::Token(t) => {
                    add(&mut l, &Lay::of(self.tok(t)), &mut glue);
                }
                Element::Node(c) if c.kind() == FileNumber => {
                    add(&mut l, &self.file_number(c), &mut glue);
                }
                Element::Node(c) if c.kind() == AsClause => {
                    add(&mut l, &self.as_clause(c), &mut glue);
                }
                Element::Node(c) if c.kind() == LetterRange => {
                    add(&mut l, &self.general(c), &mut glue);
                }
                Element::Node(c) if c.kind() == DimItem => {
                    add(&mut l, &self.general(c), &mut glue);
                }
                Element::Node(c) if c.kind() == ArrayBounds => {
                    l.sp2().append(&self.bounds(c));
                }
                Element::Node(c) => {
                    add(&mut l, &self.expr(c), &mut glue);
                }
            }
        }
        l
    }
}
