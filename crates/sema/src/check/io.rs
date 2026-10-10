//! The I/O statements with nodes of their own (design D3 of `m2-builtin-statements`): `PRINT` to the console and to
//! a file, `WRITE`, `INPUT` and `LINE INPUT`, `CLOSE`. What each accepts and rejects is measured
//! (`verification\v22_b_*`, `v22_x21`–`x60`; `study\00` §5).

use super::{Checker, R};
use crate::builtins::stmt_lookup;
use crate::{DataItem, Expr, InputSource, Place, PrintItem, StmtArg, StmtKind, Storage, Ty};
use qb64rust_base::Span;
use qb64rust_syntax::ast::{self, PrintPart};
use qb64rust_syntax::tree::Node;

impl Checker<'_> {
    /// `#n,`: the file number, converted as a LONG slot (measured: `#1.5` is file 2). A string is an error
    /// ("Illegal string-number conversion").
    fn file_number(&mut self, f: ast::FileNumber, what: &str) -> R<Expr> {
        let node = self.need(f.number(), f.node().span())?;
        let e = self.expr(node)?;
        if e.ty == Ty::Str {
            return Err(self.error(e.span, format!("the file number of `{what}` must be a number")));
        }
        self.store(e, Ty::I32)
    }

    /// An item of `PRINT` or `WRITE`: a string, or a number converted to the type it is printed in.
    fn print_item(&mut self, n: ast::Expr) -> R<PrintItem> {
        let e = self.expr(n)?;
        Ok(if e.ty == Ty::Str {
            PrintItem::Str(e)
        } else {
            let qb = e.qb.printed();
            PrintItem::Num(self.convert_exact(e, qb))
        })
    }

    pub(super) fn print(&mut self, stmt: ast::PrintStmt) -> R<()> {
        if let Some(u) = stmt.using() {
            return Err(self.unsupported(u.node().span(), "`PRINT USING`"));
        }
        let to = match stmt.file() {
            Some(f) => Some(self.file_number(f, "PRINT #")?),
            None => None,
        };
        let mut items = Vec::new();
        let mut newline = true;
        // Whether the part before this one is an item.
        let mut after_item = false;
        for part in stmt.parts() {
            match part {
                PrintPart::Semicolon(_) => {
                    newline = false;
                    after_item = false;
                }
                PrintPart::Comma(_) => {
                    items.push(PrintItem::Zone);
                    newline = false;
                    after_item = false;
                }
                PrintPart::Expr(n) => {
                    if after_item && to.is_some() {
                        // Measured: "Expected operator in equation" (`v22_x54`, `x55`); only console `PRINT` puts a
                        // `;` between adjacent items.
                        let span = n.node().span();
                        return Err(self.error(span, "the items of `PRINT #` need `;` or `,` between them"));
                    }
                    items.push(self.print_item(n)?);
                    newline = true;
                    after_item = true;
                }
            }
        }
        self.push(stmt.node(), StmtKind::Print { to, items, newline });
        Ok(())
    }

    /// `WRITE [#n,] item, …`: the items between commas; a trailing comma leaves the line open (measured,
    /// `v22_x59`, `x60`); an item left out is "Syntax error" (`v22_x61`, `x62`: the parser rejects it).
    pub(super) fn write(&mut self, stmt: ast::WriteStmt) -> R<()> {
        let node = stmt.node();
        let to = match stmt.file() {
            Some(f) => Some(self.file_number(f, "WRITE #")?),
            None => None,
        };
        let mut items = Vec::new();
        let mut newline = true;
        for part in stmt.parts() {
            match part {
                PrintPart::Expr(n) => {
                    items.push(self.print_item(n)?);
                    newline = true;
                }
                PrintPart::Comma(t) | PrintPart::Semicolon(t) => {
                    if !newline || items.is_empty() {
                        return Err(self.error(t.span, "`WRITE` needs an item before this `,`"));
                    }
                    newline = false;
                }
            }
        }
        self.push(node, StmtKind::Write { to, items, newline });
        Ok(())
    }

    /// The source of a console `INPUT` or `LINE INPUT` (`qb64pe.bas` 11123–11152): the prompt's bytes when one is
    /// written, `? ` after it for an `INPUT` without a prompt or with `;` after it (never for `LINE INPUT`), and
    /// whether a `;` stands before the prompt (the cursor then stays on the line).
    fn console_source(&mut self, prompt: ast::Prompt, line: bool) -> InputSource {
        let text = prompt.text.map(|t| {
            let raw = self.text(t.span);
            let inner = raw.strip_prefix(b"\"").unwrap_or(raw);
            inner.strip_suffix(b"\"").unwrap_or(inner).to_vec()
        });
        let semicolon = prompt
            .separator
            .is_some_and(|t| t.kind == qb64rust_syntax::SyntaxKind::Semicolon);
        InputSource::Console {
            question: !line && (text.is_none() || semicolon),
            prompt: text,
            stay: prompt.stay.is_some(),
        }
    }

    /// `INPUT #n, target, …`, or `INPUT [;] ["prompt"{;|,}] target, …` from the console.
    pub(super) fn input(&mut self, stmt: ast::InputStmt) -> R<()> {
        let node = stmt.node();
        let Some(f) = stmt.file() else {
            let from = self.console_source(stmt.prompt(), false);
            // A `_BIT` variable is taken as the old compiler takes it: the answer is read and not stored
            // (`verification\v22_e_bit`; `SOMEDAY.md`).
            let targets = self.targets(stmt.parts(), node.span(), "INPUT", true)?;
            self.push(
                node,
                StmtKind::Input {
                    from,
                    line: false,
                    targets,
                },
            );
            return Ok(());
        };
        let from = InputSource::File(self.file_number(f, "INPUT #")?);
        let targets = self.targets(stmt.parts(), node.span(), "INPUT", false)?;
        self.push(
            node,
            StmtKind::Input {
                from,
                line: false,
                targets,
            },
        );
        Ok(())
    }

    /// `LINE INPUT #n, target$`: one string place (measured: a numeric target is "Expected string-variable", a
    /// second one "Too many variables"); or `LINE INPUT [;] ["prompt"{;|,}] target$` from the console.
    pub(super) fn line_input(&mut self, stmt: ast::LineInputStmt) -> R<()> {
        let node = stmt.node();
        let Some(f) = stmt.file() else {
            let from = self.console_source(stmt.prompt(), true);
            let targets = self.targets(stmt.parts(), node.span(), "LINE INPUT", true)?;
            self.line_target(&targets, node.span())?;
            self.push(
                node,
                StmtKind::Input {
                    from,
                    line: true,
                    targets,
                },
            );
            return Ok(());
        };
        let from = InputSource::File(self.file_number(f, "LINE INPUT #")?);
        let targets = self.targets(stmt.parts(), node.span(), "LINE INPUT", false)?;
        self.line_target(&targets, node.span())?;
        self.push(
            node,
            StmtKind::Input {
                from,
                line: true,
                targets,
            },
        );
        Ok(())
    }

    /// The one target of a `LINE INPUT` must be a string place.
    pub(super) fn line_target(&mut self, targets: &[Place], span: Span) -> R<()> {
        let [target] = targets else {
            return Err(self.error(span, "`LINE INPUT` takes one string variable"));
        };
        if !self.prog.place_ty(target).is_string() {
            return Err(self.error(span, "`LINE INPUT` needs a string variable"));
        }
        Ok(())
    }

    /// The targets of an `INPUT` or `LINE INPUT`: one place after the other, separated by commas. Measured: no
    /// target is "Expected , ...", a target that is no variable "Expected variable-name". The console forms take
    /// one `,` after the last target (`v22_x148`); the parser lets no second one through.
    pub(super) fn targets<'t>(
        &mut self,
        parts: impl Iterator<Item = PrintPart<'t>>,
        span: Span,
        what: &str,
        console: bool,
    ) -> R<Vec<Place>> {
        let mut targets = Vec::new();
        let mut want_target = true;
        for part in parts {
            match part {
                PrintPart::Expr(n) => {
                    targets.push(self.target_place(n, what)?);
                    want_target = false;
                }
                PrintPart::Comma(t) | PrintPart::Semicolon(t) => {
                    if want_target {
                        return Err(self.error(t.span, format!("`{what}` needs a variable before this `,`")));
                    }
                    want_target = true;
                }
            }
        }
        if want_target && !(console && !targets.is_empty()) {
            return Err(self.error(span, format!("`{what}` needs a variable to read into")));
        }
        Ok(targets)
    }

    /// A place a statement stores into (a target of `INPUT`, `LINE INPUT` or `READ`): a variable (created when it
    /// does not exist, as by an assignment), an element or a member. A whole `TYPE` variable is an error (measured,
    /// `v22_x48`), as is the name of the FUNCTION the statement stands in; a whole array and a member of an element
    /// are not supported yet.
    pub(super) fn target_place(&mut self, e: ast::Expr, what: &str) -> R<Place> {
        let span = e.node().span();
        let place = self.place_operand(e, what)?;
        if let Ty::User(_) = self.prog.place_ty(&place) {
            return Err(self.error(span, format!("a whole `TYPE` variable cannot be a target of `{what}`")));
        }
        Ok(place)
    }

    /// A place a statement names (a target, or an operand of `SWAP` or of the `MID$` statement), of any type: a
    /// variable (created when it does not exist), an element or a member. The name of the FUNCTION the statement
    /// stands in is an error; a member of an element is not supported yet.
    pub(super) fn place_operand(&mut self, e: ast::Expr, what: &str) -> R<Place> {
        let span = e.node().span();
        let place = match e {
            ast::Expr::NameRef(n) => {
                let t = self.need(n.name(), span)?;
                if let Some(var) = self.fixed_param_ref(t) {
                    Place::Var(var)
                } else {
                    let (name, suffix) = self.split_var_name(t)?;
                    match self.dotted(t, &name, suffix)? {
                        Some(place) => place,
                        None => Place::Var(self.target(t)?),
                    }
                }
            }
            ast::Expr::Call(c) => self.call_place(c)?,
            ast::Expr::Field(f) => self.field_place(f)?,
            ast::Expr::Literal(_) | ast::Expr::Paren(_) | ast::Expr::Prefix(_) | ast::Expr::Bin(_) => {
                return Err(self.error(span, format!("`{what}` needs a variable here")));
            }
        };
        // Measured (`v22_x87`–`x89`, `x107`, `x117`): only an assignment stores into a FUNCTION's own name; here the
        // old compiler reports "Expected variable".
        if let Place::Var(v) = place
            && let Storage::Result(_) = self.prog.var(v).storage
        {
            return Err(self.error(span, format!("the FUNCTION's own name cannot be a target of `{what}`")));
        }
        if matches!(place, Place::Member { .. }) && place.has_element() {
            // Not measured: where the old compiler evaluates the index of `a(i).m` as a target.
            return Err(self.unsupported(span, format!("a member of an array element as a target of `{what}`")));
        }
        Ok(place)
    }

    /// `DATA item, …`: the items join the program's data, where the statement stands in file order (measured,
    /// `verification\v22_c_order`: also in a procedure, in a block that never runs, after `SYSTEM`). The statement
    /// itself does nothing when it runs. Text after a closing quote is the parser's error.
    pub(super) fn data(&mut self, stmt: ast::DataStmt) -> R<()> {
        let file = stmt.node().span().file;
        let items = stmt.items(&self.map.file(file).bytes);
        for item in items {
            let text = self.text(item.span);
            // A quoted item's span holds its quotes; the closing one is missing when the line ended first (the old
            // compiler assumes it).
            let text = match text.strip_prefix(b"\"") {
                Some(inner) if item.quoted => inner.strip_suffix(b"\"").unwrap_or(inner),
                Some(_) | None => text,
            };
            self.prog.data.push(DataItem {
                text: text.to_vec(),
                quoted: item.quoted,
            });
        }
        Ok(())
    }

    /// `READ target, …`. Measured (`verification\v22_x66`, `x67`, `x74`, `x75`, `x79`): a target that is no variable
    /// is "Expected variable". The old compiler takes a variable in parentheses (`READ (a)` reads into `a`,
    /// `v22_x84`) and a whole array (`x70`); neither is supported yet.
    pub(super) fn read(&mut self, stmt: ast::ReadStmt) -> R<()> {
        let node = stmt.node();
        let mut targets = Vec::new();
        for e in stmt.targets() {
            if let ast::Expr::Paren(_) = e {
                return Err(self.unsupported(e.node().span(), "a target of `READ` in parentheses"));
            }
            targets.push(self.target_place(e, "READ")?);
        }
        if targets.is_empty() {
            return Err(self.error(node.span(), "`READ` needs a variable to read into"));
        }
        self.push(node, StmtKind::Read(targets));
        Ok(())
    }

    /// `SWAP a, b` ([`StmtRule::Swap`]). Measured (`verification\v22_d_swap`, `v22_x91`–`x107`, `x135`): an operand
    /// that is no variable is "Expected variable", two types that differ "Type mismatch" (signedness does not
    /// count; `_OFFSET` is not `_INTEGER64`), two `TYPE`s that differ "Expected SWAP with similar user defined
    /// type", a `_BIT` "Cannot SWAP bit-length variables". The old compiler also takes an operand in parentheses
    /// and two whole arrays; neither is supported yet.
    pub(super) fn swap(&mut self, stmt: ast::SwapStmt) -> R<()> {
        let node = stmt.node();
        let [form] = stmt_lookup("SWAP", false)[..] else {
            unreachable!("`SWAP` is one row with one stub entry");
        };
        let operands: Vec<ast::Expr> = stmt.operands().collect();
        let [a, b] = operands[..] else {
            return Err(self.error(node.span(), "`SWAP` takes two variables"));
        };
        let mut places = Vec::new();
        for e in [a, b] {
            if let ast::Expr::Paren(_) = e {
                return Err(self.unsupported(e.node().span(), "an operand of `SWAP` in parentheses"));
            }
            places.push(self.place_operand(e, "SWAP")?);
        }
        let (ta, tb) = (self.prog.place_ty(&places[0]), self.prog.place_ty(&places[1]));
        let same = match (ta, tb) {
            (Ty::User(x), Ty::User(y)) => x == y,
            (Ty::User(_), _) | (_, Ty::User(_)) => false,
            (x, y) if x.is_string() || y.is_string() => x.is_string() && y.is_string(),
            (x, y) => swap_class(x) == swap_class(y),
        };
        if !same {
            return Err(self.error(node.span(), "`SWAP` needs two variables of the same type"));
        }
        if let (Ty::Bit { .. }, _) | (_, Ty::Bit { .. }) = (ta, tb) {
            return Err(self.error(node.span(), "`SWAP` cannot exchange `_BIT` variables"));
        }
        let args = places.into_iter().map(StmtArg::Place).collect();
        self.push(node, StmtKind::Builtin { id: form.id, args });
        Ok(())
    }

    /// `MID$(target$, start[, length]) = value$` ([`StmtRule::MidAssign`]); `call` is the left side. Measured
    /// (`verification\v22_d_mid`, `v22_x108`–`x117`): a target that is no string variable, element or member is
    /// "MID$ expects a string variable/array-element as its first argument", a string for a number or a number for
    /// the value "Illegal string-number conversion", one argument or four an error.
    pub(super) fn mid_assign(&mut self, node: Node, call: ast::CallExpr, value: ast::Expr) -> R<()> {
        let [form] = stmt_lookup("MID", true)[..] else {
            unreachable!("`MID$` is one row with one stub entry");
        };
        let span = call.node().span();
        let nodes = self.present_args(call.arg_list())?;
        let (target, start, length) = match nodes[..] {
            [t, s] => (t, s, None),
            [t, s, l] => (t, s, Some(l)),
            _ => return Err(self.error(span, "the `MID$` statement takes a variable, a start and a length")),
        };
        if let ast::Expr::Paren(_) = target {
            // Not measured.
            return Err(self.unsupported(
                target.node().span(),
                "the target of the `MID$` statement in parentheses",
            ));
        }
        let place = self.place_operand(target, "MID$")?;
        if !self.prog.place_ty(&place).is_string() {
            let msg = "the `MID$` statement needs a string variable as its first argument";
            return Err(self.error(target.node().span(), msg));
        }
        let number = |c: &mut Self, n: ast::Expr, which: &str| -> R<StmtArg> {
            let e = c.expr(n)?;
            if e.ty == Ty::Str {
                return Err(c.error(e.span, format!("the {which} of the `MID$` statement must be a number")));
            }
            Ok(StmtArg::Value(c.store(e, Ty::I32)?))
        };
        let start = number(self, start, "start")?;
        let length = match length {
            Some(l) => number(self, l, "length")?,
            None => StmtArg::Absent,
        };
        let v = self.expr(value)?;
        if v.ty != Ty::Str {
            return Err(self.error(v.span, "the `MID$` statement stores a string"));
        }
        let args = vec![StmtArg::Place(place), start, length, StmtArg::Value(v)];
        self.push(node, StmtKind::Builtin { id: form.id, args });
        Ok(())
    }

    /// `CLOSE [[#]n, …]`: each number a LONG slot (measured: `CLOSE 1.5` closes file 2; a string is "Illegal
    /// string-number conversion").
    pub(super) fn close(&mut self, stmt: ast::CloseStmt) -> R<()> {
        let [form] = stmt_lookup("CLOSE", false)[..] else {
            unreachable!("`CLOSE` is one row with one stub entry");
        };
        let mut args = Vec::new();
        for n in stmt.numbers() {
            let e = self.expr(n)?;
            if e.ty == Ty::Str {
                return Err(self.error(e.span, "`CLOSE` needs file numbers"));
            }
            args.push(StmtArg::Value(self.store(e, Ty::I32)?));
        }
        self.push(stmt.node(), StmtKind::Builtin { id: form.id, args });
        Ok(())
    }
}

/// What `SWAP` compares of a numeric type: its kind and width, not its signedness (`qb64pe.bas` 11484–11497).
fn swap_class(t: Ty) -> (u8, u32) {
    match t {
        Ty::I8 | Ty::U8 => (0, 8),
        Ty::I16 | Ty::U16 => (0, 16),
        Ty::I32 | Ty::U32 => (0, 32),
        Ty::I64 | Ty::U64 => (0, 64),
        Ty::Off | Ty::UOff => (1, 64),
        Ty::F32 => (2, 32),
        Ty::F64 => (2, 64),
        Ty::F80 => (2, 256),
        Ty::Bit { width, .. } => (3, u32::from(width)),
        Ty::Str | Ty::FixedStr(_) | Ty::User(_) => unreachable!("a numeric type, not {t:?}"),
    }
}
