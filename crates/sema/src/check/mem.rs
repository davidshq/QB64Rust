//! Memory blocks (design D3, D4 and D6 of `m3-mem`): `_MEM(reference)`, `_MEMELEMENT(reference)`, `_MEM(offset,
//! size)`, `_OFFSET(reference)` and the `_MEMGET` function, each an expression of its own (`ExprKind::MemOf`,
//! `MemAt`, `AddressOf`, `MemRead`), and the statements `_MEMGET`, `_MEMPUT` and `_MEMFILL`, written inline
//! (`StmtKind::MemGet`, `MemPut`, `MemFill`): what they may name, and the old compiler's errors (measured,
//! `verification\v32_x01_accepted`, `v32_x02`–`x116`).

use super::expr::conv;
use super::places::{numeric_type, unsigned_of};
use super::{Checker, R};
use crate::builtins::{Rule, Supported};
use crate::{ArrayRef, Expr, ExprKind, MemValue, Place, Reference, StmtKind, SymbolKind, Ty};
use qb64rust_base::{Span, show_bytes};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::Node;

impl Checker<'_> {
    /// `_MEM(reference)` and `_MEM(offset, size)` (`element` false), or `_MEMELEMENT(reference)`. A block: only
    /// the value stored into a place of type `_MEM` (design D2; the checker's `mem_store`).
    pub(super) fn mem_fn(&mut self, call: ast::CallExpr, element: bool, span: Span) -> R<Expr> {
        let word = if element { "_MEMELEMENT" } else { "_MEM" };
        if !std::mem::take(&mut self.mem_store) {
            let msg = format!("`{word}` gives a `_MEM` block, which can only be stored into a `_MEM` variable");
            return Err(self.error(span, msg));
        }
        let list = call.arg_list();
        if list.is_some_and(|l| l.args().is_empty() && l.type_arg().is_none()) {
            // Measured: "Expected (...)" (`verification\v32_x18`).
            return Err(self.error(span, format!("`{word}` with empty parentheses")));
        }
        let args = self.present_args(list)?;
        let ty = self.mem_ty();
        let kind = match (&args[..], element) {
            (&[node], _) => {
                let of = self.reference(node, word)?;
                self.block_reference(&of, word, element, node.node().span())?;
                ExprKind::MemOf { of, element }
            }
            (&[offset, size], false) => {
                let offset = self.expr(offset)?;
                if !matches!(offset.ty, Ty::Off | Ty::UOff) {
                    // Measured: "Expected _MEM(_OFFSET-value,...)" (`v32_x09`, `x10`, `x115`).
                    let msg = "the first argument of `_MEM(offset, size)` must be an `_OFFSET` value";
                    return Err(self.error(offset.span, msg));
                }
                let size = self.expr(size)?;
                if size.ty == Ty::Str {
                    // Measured: "2nd function argument requires a number" (`v32_x116`).
                    return Err(self.error(size.span, "argument 2 of `_MEM` needs a number"));
                }
                let size = self.store(size, Ty::Off)?;
                ExprKind::MemAt {
                    offset: Box::new(offset),
                    size: Box::new(size),
                }
            }
            // Measured: "Incorrect number of arguments" (`v32_x19`).
            (_, false) => return Err(self.error(span, "`_MEM` takes 1 or 2 arguments")),
            (_, true) => return Err(self.error(span, "`_MEMELEMENT` takes 1 argument")),
        };
        Ok(Expr { span, ty, qb: ty, kind })
    }

    /// `_OFFSET(reference)`: the address of a place, of a string's characters or of an array's first element,
    /// an `_UNSIGNED _OFFSET` (design D6).
    pub(super) fn offset_fn(&mut self, call: ast::CallExpr, span: Span) -> R<Expr> {
        let list = call.arg_list();
        if list.is_some_and(|l| l.args().is_empty() && l.type_arg().is_none()) {
            // Measured: "Expected (...)" (`verification\v32_x24`).
            return Err(self.error(span, "`_OFFSET` with empty parentheses"));
        }
        let args = self.present_args(list)?;
        let &[node] = &args[..] else {
            return Err(self.error(span, "`_OFFSET` takes 1 argument"));
        };
        let of = self.reference(node, "_OFFSET")?;
        let at = node.node().span();
        let ty = self.reference_ty(&of);
        match (&of, ty) {
            // Measured: "_OFFSET cannot reference _BIT type arrays" (`v32_x22`).
            (Reference::Array(_) | Reference::Place(Place::Element { .. }), Ty::Bit { .. }) => {
                return Err(self.error(at, "`_OFFSET` cannot take a `_BIT` array"));
            }
            // Measured: "Variable/element cannot be _BIT aligned" (`v32_x23`).
            (Reference::Place(_), Ty::Bit { .. }) => {
                return Err(self.error(at, "`_OFFSET` cannot take a `_BIT` variable"));
            }
            // The characters of a variable-length string variable (`v32_x01_accepted`); of an element, a member or
            // a whole array of them, not measured.
            (Reference::Place(Place::Var(_)), Ty::Str) => {}
            (_, Ty::Str) => {
                return Err(self.unsupported(
                    at,
                    "`_OFFSET` of an element, member or array of variable-length strings",
                ));
            }
            _ => {}
        }
        if let Reference::Place(_) = of
            && crate::owns_desc(&self.prog.types, ty)
        {
            // Measured: "_MEM cannot reference a TYPE value that contains explicit _Dynamic member arrays"
            // (`verification\v35_x68`); its members' addresses are taken (`x52`).
            let msg = format!(
                "`_OFFSET` of a `TYPE {}` value: a `TYPE` with a dynamic member array has no fixed address",
                self.prog.type_name(ty)
            );
            return Err(self.error(at, msg));
        }
        if let Some(what) = self.unmeasured_path(&of) {
            return Err(self.unsupported(at, format!("`_OFFSET` of {what}")));
        }
        Ok(Expr {
            span,
            ty: Ty::UOff,
            qb: Ty::UOff,
            kind: ExprKind::AddressOf(of),
        })
    }

    /// What `_MEM`, `_MEMELEMENT` or `_OFFSET` names (`word`): a whole array (`a()`), a whole member array (`t.v()`,
    /// also `t.v`, as the old compiler takes for these functions, `qb64pe.bas` 20650), or a variable, element or
    /// member. Anything else is the old compiler's "Expected variable name/array element" (or "_OFFSET expects the
    /// name of a variable/array"; measured, `v32_x10`–`x12`, `x19`, `x20`).
    fn reference(&mut self, node: ast::Expr, word: &str) -> R<Reference> {
        let span = node.node().span();
        match self.reference_or_value(node, word)? {
            Named::Ref(of) => Ok(of),
            Named::Value(_) => Err(self.error(span, format!("`{word}` needs a variable, an element or an array"))),
        }
    }

    /// [`Self::reference`], or the value of an expression that names none (`_MEMPUT` takes a string so).
    fn reference_or_value(&mut self, node: ast::Expr, word: &str) -> R<Named> {
        let span = node.node().span();
        let empty = |l: Option<ast::ArgList>| l.is_some_and(|l| l.args().is_empty());
        match node {
            ast::Expr::Call(c) if empty(c.arg_list()) => {
                let t = self.need(c.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                if name.contains('.') {
                    if let Some((array, _, _)) = self.member_array(node)? {
                        return Ok(Named::Ref(Reference::Array(array)));
                    }
                } else if let Some(array) = self.find_array(&name, suffix) {
                    self.names.push((SymbolKind::Var(array), t.span));
                    return Ok(Named::Ref(Reference::Array(ArrayRef::Var(array))));
                }
                let msg = format!("`{word}` of a name that is no array, written with `()`");
                return Err(self.unsupported(span, msg));
            }
            ast::Expr::Field(f) if empty(f.arg_list()) => {
                if let Some((array, _, _)) = self.member_array(node)? {
                    return Ok(Named::Ref(Reference::Array(array)));
                }
            }
            ast::Expr::Field(f) if f.arg_list().is_none() => {
                let (place, _) = self.field_member(f)?;
                return Ok(Named::Ref(self.place_or_member_array(place)));
            }
            ast::Expr::NameRef(n) => {
                let t = self.need(n.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                if name.contains('.')
                    && let Some(place) = self.dotted_any(t, &name, suffix)?
                {
                    return Ok(Named::Ref(self.place_or_member_array(place)));
                }
            }
            ast::Expr::Call(_)
            | ast::Expr::Field(_)
            | ast::Expr::Literal(_)
            | ast::Expr::Paren(_)
            | ast::Expr::Prefix(_)
            | ast::Expr::Bin(_) => {}
        }
        // A place of a user type is a place here, not a value, as for `LEN` ([`Checker::load`]).
        let outer = self.len_place.replace(span);
        let e = self.expr(node);
        self.len_place = outer;
        let e = e?;
        match &e.kind {
            ExprKind::Load(place) => Ok(Named::Ref(Reference::Place(place.clone()))),
            ExprKind::Int(_)
            | ExprKind::Float(_)
            | ExprKind::Str(_)
            | ExprKind::Bound { .. }
            | ExprKind::VarPtr { .. }
            | ExprKind::VarSeg { .. }
            | ExprKind::Block { .. }
            | ExprKind::Convert { .. }
            | ExprKind::Binary { .. }
            | ExprKind::Unary { .. }
            | ExprKind::Concat(..)
            | ExprKind::StrCompare { .. }
            | ExprKind::Choice { .. }
            | ExprKind::Call { .. }
            | ExprKind::CallProc { .. }
            | ExprKind::MemOf { .. }
            | ExprKind::MemAt { .. }
            | ExprKind::AddressOf(_)
            | ExprKind::MemRead { .. } => Ok(Named::Value(e)),
        }
    }

    /// A member named without parentheses: a member array as a whole, or the member itself.
    fn place_or_member_array(&self, place: Place) -> Reference {
        if self.is_member_array(&place)
            && let Place::Member { base, member } = place
        {
            return Reference::Array(ArrayRef::Member { base, member });
        }
        Reference::Place(place)
    }

    /// The type of what a reference names: a place's, an array's elements'.
    fn reference_ty(&self, of: &Reference) -> Ty {
        match of {
            Reference::Place(place) => self.prog.place_ty(place),
            Reference::Array(array) => self.prog.array_ty(array),
        }
    }

    /// What the block of `_MEM` or `_MEMELEMENT` may name (design D3): not a variable-length string ("_MEM cannot
    /// reference variable-length strings", `v32_x07`, `x08`, `x16`, `x17`) nor a `_BIT` ("Variable/element cannot
    /// be _BIT aligned", `x14`, `x15`); `_MEMELEMENT` no whole member array ("Expected array index", `x04`). The
    /// kinds of place the old compiler takes and that were not measured are not supported yet: a value of a
    /// `TYPE` that holds a `STRING`, and a member array that is not the last step of the place or whose elements
    /// are of a user type.
    fn block_reference(&mut self, of: &Reference, word: &str, element: bool, at: Span) -> R<()> {
        let ty = self.reference_ty(of);
        let root = self.prog.var(of.root()).ty;
        if matches!(ty, Ty::Bit { .. }) || matches!(root, Ty::Bit { .. }) {
            return Err(self.error(at, format!("`{word}` cannot take a `_BIT` variable or element")));
        }
        if ty == Ty::Str {
            return Err(self.error(at, format!("`{word}` cannot take a variable-length string")));
        }
        if element && matches!(of, Reference::Array(ArrayRef::Member { .. })) {
            return Err(self.error(at, "`_MEMELEMENT` takes an element, not a whole member array"));
        }
        if crate::holds_strings(&self.prog.types, root) {
            let msg = format!("`{word}` of a value of a `TYPE` with a `STRING` member");
            return Err(self.unsupported(at, msg));
        }
        // A whole dynamic member array and an element of one, also of a user type's values: the block of its
        // descriptor (measured, `verification\v35_x23`, `x24`, `x66`).
        if let Reference::Place(Place::Element {
            array: array @ ArrayRef::Member { base, .. },
            ..
        })
        | Reference::Array(array @ ArrayRef::Member { base, .. }) = of
            && self.is_dynamic_member(array)
            && !through_member_array(base)
        {
            return Ok(());
        }
        if let Some(what) = self.unmeasured_path(of) {
            return Err(self.unsupported(at, format!("`{word}` of {what}")));
        }
        Ok(())
    }

    /// A reference through a member array the old compiler's rules were not measured for: a member array on the
    /// way to the place that is not its last step, or the last step's member array of user-type elements.
    fn unmeasured_path(&self, of: &Reference) -> Option<&'static str> {
        let (base, last) = match of {
            Reference::Place(Place::Element {
                array: ArrayRef::Member { base, member },
                ..
            })
            | Reference::Array(ArrayRef::Member { base, member }) => (base.as_ref(), Some((base, member))),
            Reference::Place(Place::Member { base, .. }) => (base.as_ref(), None),
            Reference::Place(Place::Var(_) | Place::Element { .. }) | Reference::Array(ArrayRef::Var(_)) => {
                return None;
            }
        };
        if through_member_array(base) {
            return Some("a place reached through an element of a member array");
        }
        if let Reference::Array(array) = of
            && self.is_dynamic_member(array)
        {
            return Some("a whole dynamic member array");
        }
        // Of a dynamic member array, measured (`verification\v35_x66`).
        if let Some((base, member)) = last
            && let m = self.prog.member(self.prog.place_ty(base), *member)
            && let Ty::User(_) = m.ty
            && !m.dynamic
        {
            return Some("a member array of a `TYPE`'s values");
        }
        None
    }
}

/// What an argument of `_MEMPUT` or `_MEMFILL` names: a reference, or (a string) a value.
enum Named {
    Ref(Reference),
    Value(Expr),
}

impl Checker<'_> {
    /// `_MEMGET(block, offset, type)` (design D4; measured, `verification\v32_x01_accepted`, `x47`–`x56`): the
    /// value of a numeric type, `STRING * n` or a user type at an offset of a block. A user type's value stands only
    /// as the value stored into a place of that type (the checker's `mem_store`; "Number required for function" as
    /// a value of `PRINT`, `x55`).
    pub(super) fn mem_get_fn(&mut self, call: ast::CallExpr, span: Span) -> R<Expr> {
        let whole = std::mem::take(&mut self.mem_store);
        let list = call.arg_list();
        let positions = list.map(|l| l.args()).unwrap_or_default();
        let (b, o, of) = match (&positions[..], list.and_then(|l| l.type_arg())) {
            (&[Some(b), Some(o), None], Some(t)) => (b, o, self.mem_type_arg(t)?),
            // A user type's name reads as an expression.
            (&[Some(b), Some(o), Some(ast::Expr::NameRef(n))], None) => {
                let t = self.need(n.name(), span)?;
                let shown = show_bytes(self.text(t.span));
                let name = self.word(t);
                match self.user_type_of(&name) {
                    Some(ty) => (b, o, ty),
                    // Measured: "Invalid TYPE name" (`v32_x50`).
                    None => return Err(self.error(t.span, format!("`{shown}` names no type"))),
                }
            }
            // Measured: "Invalid TYPE name" (`v32_x53`, a number).
            (&[Some(_), Some(_), Some(e)], None) => {
                return Err(self.error(e.node().span(), "the third argument of `_MEMGET` is a type name"));
            }
            (&[_, _, _], _) if positions.iter().any(Option::is_none) => {
                return Err(self.unsupported(span, "omitted arguments"));
            }
            // Measured: "Incorrect number of arguments" (`v32_x52`).
            _ => return Err(self.error(span, "`_MEMGET` takes 3 arguments")),
        };
        let block = self.mem_arg(b, "_MEMGET")?;
        let offset = self.mem_offset(o, "_MEMGET")?;
        if let Ty::User(_) = of
            && !whole
        {
            let msg = "`_MEMGET` of a `TYPE` gives a whole value, which can only be stored into a variable of it";
            return Err(self.error(span, msg));
        }
        let ty = if let Ty::FixedStr(_) = of { Ty::Str } else { of };
        Ok(Expr {
            span,
            ty,
            qb: ty,
            kind: ExprKind::MemRead {
                block,
                offset: Box::new(offset),
                of,
            },
        })
    }

    /// The type argument of `_MEMGET`: a numeric type, `STRING * n`, `_MEM`. Measured: `STRING` "Expected STRING *
    /// ..." (`v32_x47`), `_BIT` and `_BIT * n` "_BIT TYPE unsupported" (`x48`, `x49`), `_UNSIGNED STRING * n`
    /// "Invalid TYPE name" (`x120`).
    fn mem_type_arg(&mut self, t: Node) -> R<Ty> {
        let at = t.span();
        let tokens: Vec<_> = t.child_tokens().filter(|w| !w.kind.is_trivia()).collect();
        let words: Vec<String> = tokens
            .iter()
            .filter(|w| w.kind == SyntaxKind::Ident)
            .map(|&w| self.word(w))
            .collect();
        let size = tokens.iter().find(|w| w.kind == SyntaxKind::Number).copied();
        let (unsigned, text) = match words.split_first() {
            Some((first, rest)) if first == "_UNSIGNED" => (true, rest.join(" ")),
            _ => (false, words.join(" ")),
        };
        match (text.as_str(), size) {
            // Measured: "Invalid TYPE name" (`v32_x120`).
            ("STRING", _) if unsigned => Err(self.error(at, "`_UNSIGNED STRING` is no type")),
            ("STRING", Some(n)) => Ok(Ty::FixedStr(self.fixed_len_of(n, true)?)),
            ("STRING", None) => Err(self.error(at, "`_MEMGET` needs `STRING * n`, a length")),
            ("_BIT", _) => Err(self.error(at, "`_MEMGET` cannot read a `_BIT`")),
            ("_MEM", None) if !unsigned => Ok(self.mem_ty()),
            (_, None) => match numeric_type(&text) {
                Some(t) if !unsigned => Ok(t),
                Some(t) => match unsigned_of(t) {
                    Some(u) => Ok(u),
                    None => Err(self.error(at, format!("`{text}` cannot be `_UNSIGNED`"))),
                },
                None => Err(self.error(at, format!("`{text}` is no type `_MEMGET` reads"))),
            },
            (_, Some(_)) => Err(self.error(at, format!("`{text} * …` is no type `_MEMGET` reads"))),
        }
    }

    /// The offset (or count) of a statement or function of memory blocks: a number, stored as an `_OFFSET`.
    /// Measured: a string "Illegal string-number conversion" or "2nd function argument requires a number"
    /// (`v32_x37`, `x54`).
    fn mem_offset(&mut self, node: ast::Expr, word: &str) -> R<Expr> {
        let e = self.expr(node)?;
        if e.ty == Ty::Str {
            return Err(self.error(e.span, format!("the offset of `{word}` must be a number")));
        }
        self.store(e, Ty::Off)
    }

    /// The `_MEMGET` statement, `_MEMGET block, offset, variable` (design D4; `qb64pe.bas` 10170; measured,
    /// `verification\v32_x01_accepted`, `x40`–`x46`). `CALL _MEMGET(…)` is "Command not implemented" (`x90`).
    pub(super) fn mem_get_stmt(&mut self, stmt: ast::CallStmt) -> R<()> {
        let node = stmt.node();
        let span = node.span();
        if let Some(bad) = stmt.unparsed_args() {
            // Measured: "Name already in use (AS)" for `_MEMGET m, o, k AS LONG` (`x45`).
            return Err(self.error(bad.span(), "cannot read the arguments of `_MEMGET`"));
        }
        let args = self.present_args(stmt.arg_list())?;
        let &[b, o, v] = &args[..] else {
            // Measured: "Expected _MEMGET mem-reference, offset, variable" (`x39`, `x43`).
            return Err(self.error(span, "`_MEMGET` takes a block, an offset and a variable"));
        };
        let block = self.mem_arg(b, "_MEMGET")?;
        let offset = self.mem_offset(o, "_MEMGET")?;
        let target = self.reference(v, "_MEMGET")?;
        self.bytes_reference(&target, "_MEMGET", v.node().span())?;
        self.push(node, StmtKind::MemGet { block, offset, target });
        Ok(())
    }

    /// `_MEMPUT block, offset, value [AS type]` and `_MEMFILL block, offset, bytes, value [AS type]` (design D4;
    /// `qb64pe.bas` 10266–10500; measured, `verification\v32_x01_accepted`, `x25`–`x39`, `x57`–`x63`).
    pub(super) fn mem_stmt(&mut self, s: ast::MemStmt) -> R<()> {
        let node = s.node();
        let span = node.span();
        let first = self.need(s.word(), span)?;
        let word = self.word(first);
        let fill = word == "_MEMFILL";
        let args: Vec<ast::Expr> = s.args().collect();
        let count = if fill { 4 } else { 3 };
        if args.len() != count {
            // Measured: "Expected _MEMPUT mem-reference, offset, variable|value[AS type]" (`x32`–`x35`, `x60`).
            let what = if fill { "a count of bytes, " } else { "" };
            return Err(self.error(span, format!("`{word}` takes a block, an offset, {what}and a value")));
        }
        let block = self.mem_arg(args[0], &word)?;
        let offset = self.mem_offset(args[1], &word)?;
        let bytes = if fill {
            Some(self.mem_offset(args[2], &word)?)
        } else {
            None
        };
        let v = args[count - 1];
        let value = match s.as_clause() {
            Some(a) => {
                let ty = self.mem_value_type(a, &word)?;
                let e = self.expr(v)?;
                MemValue::Typed {
                    value: self.store(e, ty)?,
                    ty,
                }
            }
            None => match self.reference_or_value(v, &word)? {
                Named::Ref(of) => {
                    self.bytes_reference(&of, &word, v.node().span())?;
                    MemValue::Bytes(of)
                }
                Named::Value(e) if e.ty == Ty::Str => MemValue::Str(e),
                // Measured: "Expected variable name/array element" (`x24`, `x61`).
                Named::Value(e) => {
                    let msg = format!("`{word}` needs a variable, a string or a value with `AS type`");
                    return Err(self.error(e.span, msg));
                }
            },
        };
        let kind = match bytes {
            Some(bytes) => StmtKind::MemFill {
                block,
                offset,
                bytes,
                value,
            },
            None => StmtKind::MemPut { block, offset, value },
        };
        self.push(node, kind);
        Ok(())
    }

    /// The type after `AS` in `_MEMPUT` or `_MEMFILL`: a numeric type other than `_BIT`. Measured: `STRING`, `STRING
    /// * n`, a user type and `_BIT` "_MEMPUT requires numeric type" (`v32_x27`–`x30`, `x57`–`x59`), anything else
    /// "Invalid type" (`x31`).
    fn mem_value_type(&mut self, a: ast::AsClause, word: &str) -> R<Ty> {
        let words: Vec<String> = a.type_words().map(|t| self.word(t)).collect();
        let text = match words.split_first() {
            Some((first, rest)) if first == "_UNSIGNED" => rest.join(" "),
            _ => words.join(" "),
        };
        let at = a.node().span();
        let numeric = numeric_type(&text).is_some_and(|t| !matches!(t, Ty::Bit { .. }));
        if numeric && a.size().is_none() {
            return self.type_of(a);
        }
        if text == "STRING" || text == "_BIT" || self.user_type_of(&text).is_some() {
            return Err(self.error(at, format!("`{word}` needs a numeric type after `AS`")));
        }
        Err(self.error(at, format!("`{text}` is no type")))
    }

    /// The variable `_MEMGET` reads into, or `_MEMPUT` and `_MEMFILL` take the bytes of: not a `_BIT` ("Variable/
    /// element cannot be _BIT aligned", `v32_x38`, `x46`), not a whole array of variable-length strings ("Cannot
    /// pass array of variable-length strings", `x39`). An element or member of variable-length strings, a `TYPE`
    /// that holds a `STRING` and the paths [`Self::unmeasured_path`] names are not measured.
    fn bytes_reference(&mut self, of: &Reference, word: &str, at: Span) -> R<()> {
        let ty = self.reference_ty(of);
        let root = self.prog.var(of.root()).ty;
        if matches!(ty, Ty::Bit { .. }) || matches!(root, Ty::Bit { .. }) {
            return Err(self.error(at, format!("`{word}` cannot take a `_BIT` variable or element")));
        }
        match (of, ty) {
            (Reference::Array(_), Ty::Str) => {
                return Err(self.error(at, format!("`{word}` cannot take an array of variable-length strings")));
            }
            (Reference::Place(Place::Var(_)), Ty::Str) => {}
            (_, Ty::Str) => {
                let msg = format!("`{word}` of an element or member of variable-length strings");
                return Err(self.unsupported(at, msg));
            }
            _ => {}
        }
        if crate::holds_strings(&self.prog.types, root) {
            let msg = format!("`{word}` of a value of a `TYPE` with a `STRING` member");
            return Err(self.unsupported(at, msg));
        }
        if let Some(what) = self.unmeasured_path(of) {
            return Err(self.unsupported(at, format!("`{word}` of {what}")));
        }
        if let Reference::Place(Place::Element { array, .. }) | Reference::Array(array) = of
            && self.is_dynamic_member(array)
        {
            // Not measured; outside `m3-typefields` (its design, Non-Goals).
            return Err(self.unsupported(at, format!("`{word}` of a dynamic member array or its element")));
        }
        Ok(())
    }
}

impl Checker<'_> {
    /// `MKI$`, `MKL$`, `MKS$`, `MKD$`, `_MK$`, `CVI`, `CVL`, `CVS`, `CVD`, `_CV` (design D5; `qb64pe.bas`
    /// 20419–20441, 21217–21296; measured, `verification\v32_c_mk_cv`, `v32_x97`–`x113`). The number of an `MK`
    /// is converted to an integer type as a store converts it (a float to 16 bits or fewer through `qbr_float_to_long`
    /// of the value as it is, as the old compiler writes it) and passed to a float type as it is.
    pub(super) fn number_bytes(&mut self, s: Supported, list: Option<ast::ArgList>, span: Span) -> R<Expr> {
        let name = s.name;
        let (make, fixed) = match s.rule {
            Rule::Mk(t) => (true, t),
            Rule::Cv(t) => (false, t),
            Rule::Plain
            | Rule::ResultOfArg
            | Rule::IntFix
            | Rule::FloatByArg
            | Rule::Exp
            | Rule::Convert(_)
            | Rule::Fixed(_)
            | Rule::Len
            | Rule::Val
            | Rule::Radix(_)
            | Rule::Asc
            | Rule::StringFill
            | Rule::StrOrIndex
            | Rule::GivenArgs(_)
            | Rule::Widest
            | Rule::Rotate
            | Rule::Choice
            | Rule::Own => unreachable!("`{name}` is no conversion between a number and its bytes"),
        };
        let positions = list.map(|l| l.args()).unwrap_or_default();
        let type_arg = list.and_then(|l| l.type_arg());
        let (ty, node) = match (fixed, &positions[..], type_arg) {
            (Some(t), &[Some(node)], None) => (t, node),
            (None, &[None, Some(node)], Some(t)) => (self.bytes_type(t, name)?, node),
            // Measured: "Cannot convert type (K) to symbol" (`v32_x113`).
            (None, &[Some(first), Some(_)], None) => {
                let msg = format!("the first argument of `{name}` is a type name");
                return Err(self.error(first.node().span(), msg));
            }
            // Measured: "Incorrect number of arguments" (`v32_x107`–`x109`, `x112`).
            (Some(_), _, _) => return Err(self.error(span, format!("`{name}` takes 1 argument"))),
            (None, _, _) => return Err(self.error(span, format!("`{name}` takes a type and 1 argument"))),
        };
        let e = self.expr(node)?;
        // A `_BIT` is passed and read as an `_INTEGER64` (`qb64pe.bas` 21243, 21284).
        let held = match ty {
            Ty::Bit { signed: true, .. } => Ty::I64,
            Ty::Bit { signed: false, .. } => Ty::U64,
            Ty::I8
            | Ty::U8
            | Ty::I16
            | Ty::U16
            | Ty::I32
            | Ty::U32
            | Ty::I64
            | Ty::U64
            | Ty::Off
            | Ty::UOff
            | Ty::F32
            | Ty::F64
            | Ty::F80
            | Ty::Str
            | Ty::FixedStr(_)
            | Ty::User(_) => ty,
        };
        let value = if make {
            if e.ty == Ty::Str {
                // Measured: "Number required for function", "2nd function argument requires a number" (`v32_x102`,
                // `x106`).
                return Err(self.error(e.span, format!("`{name}` needs a number")));
            }
            if held.is_float() {
                e
            } else if e.ty.is_float() && held.int_bits().is_some_and(|b| b <= 16) {
                let rounded = conv(e, Ty::I32, crate::ConvKind::RoundEven);
                self.convert_exact(rounded, held)
            } else {
                self.store(e, held)?
            }
        } else {
            if e.ty != Ty::Str {
                // Measured: "CVL requires a STRING argument", "_CV requires a STRING argument" (`v32_x103`–`x105`).
                return Err(self.error(e.span, format!("`{name}` needs a string")));
            }
            e
        };
        // `_MK$` and `_CV` keep the type named in their first slot.
        let marker = Expr {
            span,
            ty,
            qb: ty,
            kind: ExprKind::Int(0),
        };
        let args = match fixed {
            Some(_) => vec![Some(value)],
            None => vec![Some(marker), Some(value)],
        };
        let result = if make { Ty::Str } else { held };
        Ok(Expr {
            span,
            ty: result,
            qb: result,
            kind: ExprKind::Call { builtin: s.id, args },
        })
    }

    /// The type `_MK$` and `_CV` take first: a numeric type, `_BIT` and `_UNSIGNED _BIT` one bit wide. Measured:
    /// `STRING` "_MK only accepts numeric types", "_CV cannot return STRING type!" (`v32_x99`, `x100`); a width
    /// after `_BIT`, `STRING * n` and any other word "Cannot convert type (…) to symbol" (`x96`–`x98`, `x101`,
    /// `x110`, `x111`).
    fn bytes_type(&mut self, t: Node, name: &str) -> R<Ty> {
        let at = t.span();
        let tokens: Vec<_> = t.child_tokens().filter(|w| !w.kind.is_trivia()).collect();
        let words: Vec<String> = tokens
            .iter()
            .filter(|w| w.kind == SyntaxKind::Ident)
            .map(|&w| self.word(w))
            .collect();
        let sized = tokens.iter().any(|w| w.kind == SyntaxKind::Number);
        let (unsigned, text) = match words.split_first() {
            Some((first, rest)) if first == "_UNSIGNED" => (true, rest.join(" ")),
            _ => (false, words.join(" ")),
        };
        if sized {
            return Err(self.error(at, format!("`{name}` takes no width or length in its type")));
        }
        match numeric_type(&text) {
            Some(Ty::Bit { .. }) => Ok(Ty::Bit {
                width: 1,
                signed: !unsigned,
            }),
            Some(t) if !unsigned => Ok(t),
            Some(t) => unsigned_of(t).ok_or_else(|| self.error(at, format!("`{text}` cannot be `_UNSIGNED`"))),
            None if text == "STRING" => Err(self.error(at, format!("`{name}` takes a numeric type"))),
            None => Err(self.error(at, format!("`{text}` is no type `{name}` takes"))),
        }
    }
}

/// Whether an element of a member array stands on the way to a place, the place included.
fn through_member_array(p: &Place) -> bool {
    match p {
        Place::Var(_) => false,
        Place::Element {
            array: ArrayRef::Var(_),
            ..
        } => false,
        Place::Element {
            array: ArrayRef::Member { .. },
            ..
        } => true,
        Place::Member { base, .. } => through_member_array(base),
    }
}
