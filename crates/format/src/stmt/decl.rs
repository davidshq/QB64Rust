//! Declarations (`qb64pe.bas` 8765-9565 `DIM`/`REDIM`/`STATIC`/`COMMON`/`SHARED`, `dim2` 17624; 6048 `CONST`; 5323-5775
//! `SUB`/`FUNCTION` headers and their parameters; 4378-4440 `TYPE` and its fields; 8531 `ERASE`; 4252 `DATA`; 27240
//! `READ`).
//!
//! - `Dim` [`Shared`], `ReDim` [`_Preserve`], each name as declared there (a `REDIM` of an existing array as its first
//!   declaration), its bounds glued to it, then `As` and the type: a built-in type through `SCase2$`, a user type as
//!   its `TYPE` line spells it. A comma with `sp2` before it.
//! - A procedure header: `Sub name (params)` with a space before the parenthesis, the first parameter glued to it,
//!   `ByVal`, `()` glued, `As` and the type; `Static` after.

use crate::fmt::Fmt;
use crate::lay::Lay;
use qb64rust_sema::SymbolKind;
use qb64rust_syntax::SyntaxKind::{self, *};
use qb64rust_syntax::tree::{Element, Node, Tok};

impl Fmt<'_> {
    /// The layout of a declaration, `None` for a kind this family does not lay out.
    pub(crate) fn decl(&self, n: Node<'_>) -> Option<Lay> {
        let l = match n.kind() {
            DimStmt | RedimStmt | SharedStmt | StaticStmt | CommonStmt => self.dim(n),
            DefTypeStmt => self.deftype(n),
            ConstStmt => self.constant(n),
            ProcHeader => self.proc_header(n),
            DeclareStmt => self.declare(n),
            TypeHeader => self.type_header(n),
            TypeField => self.type_field(n),
            DataStmt => self.data(n),
            ReadStmt | EraseStmt => self.listed(n),
            OptionStmt => self.option(n),
            _ => return None,
        };
        Some(l)
    }

    /// `OPTION BASE n` (the number element alone, `qb64pe.bas` 11514), `OPTION _EXPLICIT`, `OPTION _EXPLICITARRAY`.
    fn option(&self, n: Node<'_>) -> Lay {
        let toks: Vec<Tok> = self.toks(n);
        let mut l = Lay(self.kw("OPTION"));
        match toks.get(1) {
            Some(&t) if self.is_word(t, "BASE") => {
                l.sp().text(&self.kw("BASE"));
                let number = n
                    .tokens()
                    .into_iter()
                    .find(|t| t.kind == Number)
                    .map(|t| self.literal(t))
                    .unwrap_or_default();
                l.sp().text(&number);
            }
            Some(&t) => {
                l.sp().text(&self.kw_tok(t));
            }
            None => {}
        }
        l
    }

    /// `DEFINT a-z, b` and `_DEFINE a-z AS type` (`qb64pe.bas` 6110-6160): the letters in capitals, `-` and `,`
    /// glued to the letter before them; `_DEFINE`'s type in capitals with the `sp` after each word, the last too.
    fn deftype(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        let mut first = true;
        for e in self.elems(n) {
            match e {
                Element::Token(t) if first => {
                    first = false;
                    l.text(&self.kw_tok(t));
                }
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",");
                }
                Element::Token(_) => {}
                Element::Node(c) if c.kind() == LetterRange => {
                    let mut r = Lay::new();
                    for t in self.toks(c) {
                        match t.kind {
                            Minus => {
                                r.sp2().text(b"-").sp2();
                            }
                            _ => {
                                r.text(&self.tok(t).to_ascii_uppercase());
                            }
                        }
                    }
                    l.sp().append(&r);
                }
                Element::Node(c) if c.kind() == AsClause => {
                    l.sp().text(&self.kw("AS")).sp();
                    for t in self.toks(c).into_iter().skip(1) {
                        l.text(&self.tok(t).to_ascii_uppercase()).sp();
                    }
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }

    fn dim(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",");
                }
                Element::Token(t) if t.kind == Ident => {
                    l.then(&Lay(self.kw_tok(t)));
                }
                Element::Token(t) => {
                    l.then(&Lay::of(self.tok(t)));
                }
                Element::Node(c) if c.kind() == DimItem && n.kind() == RedimStmt && self.member_target(c) => {
                    l.then(&self.member_redim(c));
                }
                Element::Node(c) if c.kind() == DimItem => {
                    l.then(&self.dim_item(c, n.kind()));
                }
                Element::Node(c) if c.kind() == AsClause => {
                    l.then(&self.as_clause(c));
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }

    /// Whether a `REDIM` item is a member array (`a.v(…)`, `p(9).v(…)`), which the handler's path for members lays
    /// out (`qb64pe.bas` 8835-8960).
    fn member_target(&self, n: Node<'_>) -> bool {
        self.elems(n).iter().any(|e| match e {
            Element::Token(t) if t.kind == Dot => true,
            Element::Token(t) if t.kind == Ident => self
                .program
                .symbols
                .within(t.span)
                .any(|(_, id)| matches!(self.program.symbols.get(id).kind, SymbolKind::Member(..))),
            _ => false,
        })
    }

    /// `REDIM` of a member array: the target and its bounds as written, compacted, then the `AS` words in capitals.
    fn member_redim(&self, n: Node<'_>) -> Lay {
        let elems = self.elems(n);
        let last_bounds = elems
            .iter()
            .rposition(|e| matches!(e, Element::Node(c) if c.kind() == ArrayBounds));
        let mut target: Vec<Tok> = Vec::new();
        let mut bounds: Vec<Tok> = Vec::new();
        let mut words: Vec<u8> = Vec::new();
        for (i, e) in elems.iter().enumerate() {
            match *e {
                Element::Node(c) if Some(i) == last_bounds => {
                    let toks = c.tokens();
                    let inner: Vec<Tok> = toks.iter().copied().filter(|t| !t.kind.is_trivia()).collect();
                    bounds.extend_from_slice(&inner[1..inner.len().saturating_sub(1)]);
                }
                Element::Node(c) if c.kind() == AsClause => {
                    for (k, t) in self.toks(c).into_iter().enumerate() {
                        if k > 0 {
                            words.push(crate::apply::SP);
                        }
                        words.extend_from_slice(&self.tok(t).to_ascii_uppercase());
                    }
                }
                Element::Node(c) => target.extend(c.tokens()),
                Element::Token(t) => target.push(t),
            }
        }
        let mut l = Lay::new();
        l.text(&self.compact(&target, &|t| self.element(t)));
        l.text(b"(")
            .sp2()
            .text(&self.compact(&bounds, &|t| self.element(t)))
            .text(b")");
        if !words.is_empty() {
            l.sp().text(&words);
        }
        l
    }

    /// One name of a `DIM`: the name, its bounds, its `AS` clause. Bounds with nothing between the parentheses are
    /// left out, as `allocarray` lays out none for them.
    fn dim_item(&self, n: Node<'_>, stmt: SyntaxKind) -> Lay {
        let mut l = Lay::new();
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == Ident => {
                    l.text(&self.name(t));
                }
                Element::Token(t) => {
                    l.text(self.tok(t));
                }
                Element::Node(c) if c.kind() == ArrayBounds => {
                    // `DIM a()` and `REDIM a()` read no elements and lay out none; `STATIC` and `SHARED` keep `()`;
                    // `COMMON` reads any bounds as `?` and lays out `()` (`IF commonoption THEN elements$ = "?"`).
                    let empty = c.child_nodes().next().is_none();
                    match stmt {
                        DimStmt | RedimStmt if empty => {}
                        CommonStmt => {
                            l.sp2().text(b"(").sp2().text(b")");
                        }
                        _ => {
                            l.sp2().append(&self.bounds(c));
                        }
                    }
                }
                Element::Node(c) if c.kind() == AsClause => {
                    l.then(&self.as_clause(c));
                }
                Element::Node(c) if c.kind() == SpacedName => {
                    for t in c.child_tokens() {
                        l.text(&self.name(t));
                    }
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }

    /// `(lower TO upper, ...)` of an array declaration.
    pub(crate) fn bounds(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for e in self.elems(n) {
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
                Element::Token(t) if self.is_word(t, "TO") => {
                    l.sp().text(&self.kw("TO")).sp();
                }
                Element::Token(t) => {
                    l.text(self.tok(t));
                }
                Element::Node(c) => {
                    l.append(&self.expr(c));
                }
            }
        }
        l
    }

    /// `AS type`: a user type as its `TYPE` line spells it, a built-in type's words through `SCase2$`, `STRING * n`.
    pub(crate) fn as_clause(&self, n: Node<'_>) -> Lay {
        let mut l = Lay(self.kw("AS"));
        let toks = self.toks(n);
        let words: Vec<Tok> = toks
            .iter()
            .copied()
            .take_while(|t| t.kind != Star)
            .filter(|t| t.kind == Ident)
            .skip(1)
            .collect();
        let user = words
            .first()
            .and_then(|t| self.program.symbols.at(t.span.file, t.span.start))
            .is_some_and(|id| matches!(self.program.symbols.get(id).kind, SymbolKind::Type(_)));
        if user {
            l.sp().text(&self.name(words[0]));
            return l;
        }
        // The predefined `_MEM` is a user type of the old compiler, spelled as registered.
        if words.len() == 1 && self.tok(words[0]).eq_ignore_ascii_case(b"_MEM") {
            l.sp().text(&crate::lay::scase(b"_MEM", self.style()));
            return l;
        }
        let mut joined = Vec::new();
        for (i, w) in words.iter().enumerate() {
            if i > 0 {
                joined.push(crate::apply::SP);
            }
            joined.extend_from_slice(self.tok(*w));
        }
        l.sp().text(&self.kw2(&joined));
        if let Some(star) = toks.iter().position(|t| t.kind == Star) {
            l.sp().text(b"*");
            for t in &toks[star + 1..] {
                l.sp().text(&self.name(*t));
            }
        }
        l
    }

    /// `CONST name = value, ...`.
    fn constant(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",");
                }
                Element::Token(t) if t.kind == Ident => {
                    l.then(&Lay(self.kw_tok(t)));
                }
                Element::Token(_) => {}
                Element::Node(c) if c.kind() == ConstItem => {
                    let mut item = Lay::new();
                    for x in self.elems(c) {
                        match x {
                            Element::Token(t) if t.kind == Eq => {
                                item.sp().text(b"=").sp();
                            }
                            Element::Token(t) => {
                                item.text(&self.name(t));
                            }
                            Element::Node(v) => {
                                item.append(&self.expr(v));
                            }
                        }
                    }
                    l.then(&item);
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }

    /// `SUB name (params) [STATIC]` or `FUNCTION name[suffix] (params) [STATIC]`, the name as written there.
    fn proc_header(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        let mut named = false;
        let mut alias = false;
        for e in self.elems(n) {
            match e {
                Element::Token(t)
                    if t.kind == Ident && !named && (self.is_word(t, "SUB") || self.is_word(t, "FUNCTION")) =>
                {
                    l.then(&Lay(self.kw_tok(t)));
                }
                Element::Token(t) if t.kind == Ident && !named => {
                    named = true;
                    l.sp().text(self.tok(t));
                }
                Element::Token(t) if t.kind == Ident && alias => {
                    // The name in the library, as written (`getelement$(ca$, 4)`).
                    alias = false;
                    l.sp().text(self.tok(t));
                }
                Element::Token(t) if t.kind == Ident => {
                    alias = self.is_word(t, "ALIAS");
                    l.then(&Lay(self.kw_tok(t)));
                }
                Element::Token(t) if t.kind == StringLit => {
                    l.then(&Lay::of(self.tok(t)));
                }
                Element::Token(t) => {
                    l.then(&Lay::of(self.tok(t)));
                }
                Element::Node(c) if c.kind() == ParamList => {
                    l.sp().append(&self.params(c));
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }

    /// `(param, ...)` of a header.
    fn params(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        let mut first = true;
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == LParen => {
                    l.text(b"(");
                }
                Element::Token(t) if t.kind == RParen => {
                    l.sp2().text(b")");
                }
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",");
                }
                Element::Token(_) => {}
                Element::Node(p) => {
                    if first {
                        l.sp2();
                    } else {
                        l.sp();
                    }
                    first = false;
                    l.append(&self.param(p));
                }
            }
        }
        l
    }

    /// `[BYVAL] name[()] [AS type]`, the name as written.
    fn param(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        let mut named = false;
        for e in self.elems(n) {
            match e {
                Element::Token(t) if self.is_word(t, "BYVAL") && !named => {
                    l.then(&Lay(self.kw("BYVAL")));
                }
                Element::Token(t) if t.kind == Ident && !named => {
                    named = true;
                    l.then(&Lay::of(self.tok(t)));
                }
                Element::Token(t) if t.kind == LParen => {
                    l.sp2().text(b"(");
                }
                Element::Token(t) if t.kind == RParen => {
                    l.sp2().text(b")");
                }
                Element::Token(t) => {
                    l.sp().text(self.tok(t));
                }
                Element::Node(c) if c.kind() == AsClause => {
                    l.then(&self.as_clause(c));
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }

    /// `DECLARE SUB|FUNCTION header`.
    fn declare(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == Ident => {
                    l.then(&Lay(self.kw_tok(t)));
                }
                Element::Token(t) => {
                    l.then(&Lay::of(self.tok(t)));
                }
                Element::Node(c) if c.kind() == ProcHeader => {
                    l.then(&self.proc_header(c));
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }

    /// `TYPE name`, the name as written (`getelement(ca$, 2)`).
    fn type_header(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for (i, t) in self.toks(n).into_iter().enumerate() {
            if i == 0 {
                l.text(&self.kw("TYPE"));
            } else {
                l.sp().text(self.tok(t));
            }
        }
        l
    }

    /// A field (`qb64pe.bas` 4378-4440): `name[bounds] [markers] As type [markers]`, the name as written and the
    /// bounds compacted as written; or `As type names`, the names after the type compacted as written. A marker
    /// (`_DYNAMIC`, `_STATIC`) through `SCase2$`.
    fn type_field(&self, n: Node<'_>) -> Lay {
        let elems = self.elems(n);
        let as_first = matches!(elems.first(), Some(Element::Node(c)) if c.kind() == AsClause);
        let mut l = Lay::new();
        if as_first {
            let mut rest: Vec<Tok> = Vec::new();
            for e in &elems {
                match *e {
                    Element::Node(c) if c.kind() == AsClause => l.append(&self.as_clause(c)),
                    Element::Node(c) => {
                        rest.extend(c.tokens());
                        &mut l
                    }
                    Element::Token(t) => {
                        rest.push(t);
                        &mut l
                    }
                };
            }
            l.sp().text(&self.compact(&rest, &|t| self.element(t)));
            return l;
        }
        for e in elems {
            match e {
                Element::Node(c) if c.kind() == AsClause => {
                    l.then(&self.as_clause(c));
                }
                Element::Node(c) if c.kind() == FieldName => {
                    for x in self.elems(c) {
                        match x {
                            Element::Token(t) if t.kind == Ident && l.is_empty() => {
                                l.text(self.tok(t));
                            }
                            Element::Token(t) => {
                                l.then(&Lay::of(self.tok(t)));
                            }
                            Element::Node(b) if b.kind() == ArrayBounds => {
                                l.text(&self.compact(&b.tokens(), &|t| self.element(t)));
                            }
                            Element::Node(m) => {
                                l.then(&Lay(self.kw2(self.text(m.span()))));
                            }
                        }
                    }
                }
                Element::Node(c) => {
                    l.then(&Lay(self.kw2(self.text(c.span()))));
                }
                Element::Token(t) => {
                    l.then(&Lay::of(self.tok(t)));
                }
            }
        }
        l
    }

    /// `DATA items`: the items with the blanks around each left out, joined by commas (`lineformat$`'s data scan).
    fn data(&self, n: Node<'_>) -> Lay {
        let mut l = Lay(self.kw("DATA"));
        if let Some(t) = self.toks(n).into_iter().find(|t| t.kind == DataText) {
            let items = data_items(self.tok(t));
            if !items.is_empty() {
                l.sp().text(&items);
            }
        }
        l
    }

    /// `READ a, b` and `ERASE a, b`: the keyword, then the items with `sp2` before each comma. An `ERASE` of a member
    /// array names it as written, compacted (`qb64pe.bas` 8731).
    fn listed(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",");
                }
                Element::Token(t) if t.kind == Ident && l.is_empty() => {
                    l.text(&self.kw_tok(t));
                }
                Element::Token(t) if t.kind == Ident => {
                    l.then(&Lay(self.name(t)));
                }
                Element::Token(t) => {
                    l.then(&Lay::of(self.tok(t)));
                }
                Element::Node(c) if n.kind() == EraseStmt && self.text(c.span()).contains(&b'.') => {
                    l.then(&Lay(self.compact(&c.tokens(), &|t| self.element(t))));
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }
}

/// The items of a `DATA` line as `lineformat$` keeps them: each item from its first to its last character that is not
/// a blank (a quoted item to its quote, its closing quote added when missing), joined by commas.
fn data_items(text: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut i = 0;
    let n = text.len();
    let mut item: Option<(usize, usize)> = None;
    let mut quoted = false;
    let mut scan = false;
    let flush = |out: &mut Vec<u8>, item: &mut Option<(usize, usize)>, quoted: bool| {
        if let Some((a, b)) = *item {
            out.extend_from_slice(&text[a..=b]);
        }
        if quoted {
            out.push(b'"');
        }
        out.push(b',');
        *item = None;
    };
    while i < n {
        let c = text[i];
        if (c == b' ' || c == b'\t') && !scan {
            i += 1;
            continue;
        }
        if c == b',' && !quoted {
            flush(&mut out, &mut item, false);
            scan = false;
            i += 1;
            continue;
        }
        if c == b'"' {
            if quoted {
                quoted = false;
            } else if !scan {
                quoted = true;
            }
        }
        scan = true;
        match &mut item {
            None => item = Some((i, i)),
            Some((_, b)) if c != b' ' && c != b'\t' => *b = i,
            Some(_) => {}
        }
        i += 1;
    }
    flush(&mut out, &mut item, quoted);
    out.pop();
    out
}

#[cfg(test)]
mod tests {
    use super::data_items;

    #[test]
    fn data_items_lose_the_blanks_around_them() {
        // `lineformat$`'s data scan (`qb64pe.bas` 25210-25300); `format_tests\meta_format`: `DATA 0,   0,   0` is
        // `Data 0,0,0`.
        assert_eq!(data_items(b"0,   0,   0"), b"0,0,0");
        assert_eq!(data_items(b" a b , \"x, y\" ,3"), b"a b,\"x, y\",3");
        assert_eq!(data_items(b"\"open"), b"\"open\"");
        assert_eq!(data_items(b"1,,2"), b"1,,2");
    }
}
