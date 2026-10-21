//! The I/O statements with nodes of their own (design D3 of `m2-builtin-statements`): `PRINT` to the console and to
//! a file, `WRITE`, `INPUT` and `LINE INPUT`, `CLOSE`. What each accepts and rejects is measured
//! (`verification\v22_b_*`, `v22_x21`–`x60`; `study\00` §5).

use super::expr::Use;
use super::{Checker, NoJoin, R};
use crate::builtins::{qb_bits, stmt_lookup};
use crate::{
    DataItem, Expr, ExprKind, InputSource, Place, PrintItem, PrintTo, StmtArg, StmtKind, Storage, Ty, UsingValue,
};
use qb64rust_base::Span;
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast::{self, PrintPart};
use qb64rust_syntax::tree::Node;

/// A value of `PRINT USING` by its type (`qb64pe.bas` 27340–27378): a string; `TAB(n)` or `SPC(n)` written as the
/// whole value (measured: `TAB(3) + "x"` is a string); a number for the runtime's entry of what it is believed.
fn using_value(e: Expr, node: ast::Expr) -> UsingValue {
    if e.ty == Ty::Str {
        let blanks = match (&e.kind, node) {
            (ExprKind::Call { builtin, .. }, ast::Expr::Call(_)) => {
                matches!(builtin.get().callname, "func_tab" | "func_spc")
            }
            _ => false,
        };
        return if blanks {
            UsingValue::Blanks(e)
        } else {
            UsingValue::Str(e)
        };
    }
    let as_ = match e.qb.float_rank() {
        Some(_) => e.qb,
        None if e.qb.is_unsigned() && qb_bits(e.qb) == Some(64) => Ty::U64,
        None => Ty::I64,
    };
    UsingValue::Num { value: e, as_ }
}

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
        let to = match stmt.file() {
            Some(f) => PrintTo::File(self.file_number(f, "PRINT #")?),
            None => PrintTo::Screen,
        };
        let usings: Vec<_> = stmt.usings().collect();
        self.print_like(stmt.node(), to, &usings, stmt.parts().collect())
    }

    /// `LPRINT`: `PRINT` to the printer, in every form console `PRINT` has (design D8 of `m3-console-files`;
    /// measured, `verification\v30_x13`). A file number is no part of it: the parser reads `#` as an item's error.
    pub(super) fn lprint(&mut self, stmt: ast::LprintStmt) -> R<()> {
        // The old compiler links the printer part of the runtime for any `LPRINT` (`qb64pe.bas` 27717).
        self.settings.runtime_parts.insert(crate::RuntimePart::Printer);
        let usings: Vec<_> = stmt.usings().collect();
        self.print_like(stmt.node(), PrintTo::Printer, &usings, stmt.parts().collect())
    }

    /// A `PRINT` or `LPRINT` statement: its items, and with a `USING` clause the items before it, the format and
    /// the values after it.
    fn print_like(&mut self, node: Node, to: PrintTo, usings: &[ast::UsingClause], parts: Vec<PrintPart>) -> R<()> {
        let split = usings.first().map_or(u32::MAX, |u| u.node().span().start);
        let file = matches!(to, PrintTo::File(_));
        let mut items = Vec::new();
        let mut newline = true;
        // Whether the part before this one is an item.
        let mut after_item = false;
        for part in parts.iter().filter(|p| p.start() < split) {
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
                    if after_item && file {
                        // Measured: "Expected operator in equation" (`v22_x54`, `x55`); only console `PRINT` puts a
                        // `;` between adjacent items.
                        let span = n.node().span();
                        return Err(self.error(span, "the items of `PRINT #` need `;` or `,` between them"));
                    }
                    // The first item of a console `PRINT` is evaluated before anything is printed.
                    let item = if items.is_empty() && !file {
                        self.print_item(*n)?
                    } else {
                        self.unjoined(NoJoin::Item, |c| c.print_item(*n))?
                    };
                    items.push(item);
                    newline = true;
                    after_item = true;
                }
            }
        }
        let Some(&using) = usings.first() else {
            self.push(node, StmtKind::Print { to, items, newline });
            return Ok(());
        };
        let first = items.is_empty() && !file;
        let (format, values, newline) = self.using(using, usings.get(1), &parts, first)?;
        let kind = StmtKind::PrintUsing {
            to,
            before: items,
            format,
            values,
            newline,
        };
        self.push(node, kind);
        Ok(())
    }

    /// The `USING` clause of a `PRINT` or `LPRINT` and the values after it: the format, the values, and whether
    /// the line ends (design D1 of `m3-console-files`). Measured (`verification\v30_x01`–`x19`; `study\00` §5):
    /// the format is a string and is followed by `;` (a `,` is "Expected PRINT USING formatstring ; ..."); at
    /// least one value follows; the values stand between `;` or `,`, which mean the same, and none may be left
    /// out; a second `USING` is an error. `first`: nothing of the statement is evaluated before the format.
    fn using(
        &mut self,
        using: ast::UsingClause,
        second: Option<&ast::UsingClause>,
        parts: &[PrintPart],
        first: bool,
    ) -> R<(Expr, Vec<UsingValue>, bool)> {
        let span = using.node().span();
        let node = self.need(using.format(), span)?;
        let format = if first {
            self.expr(node)?
        } else {
            self.unjoined(NoJoin::Item, |c| c.expr(node))?
        };
        if format.ty != Ty::Str {
            return Err(self.error(format.span, "the format of `USING` must be a string"));
        }
        if let Some(t) = using.separator()
            && t.kind == SyntaxKind::Comma
        {
            return Err(self.error(t.span, "`USING` needs a `;` after its format, not a `,`"));
        }
        if let Some(u) = second {
            // Measured: "Name already in use (USING)" (`v30_x11`, `x12`).
            return Err(self.error(u.node().span(), "a second `USING` in one statement"));
        }
        let after = span.end;
        let mut values = Vec::new();
        // Whether the part before this one is a value.
        let mut after_value = false;
        for part in parts.iter().filter(|p| p.start() >= after) {
            match part {
                PrintPart::Semicolon(t) | PrintPart::Comma(t) => {
                    if !after_value {
                        // Measured: "Syntax error" (`v30_x09`, `x10`).
                        return Err(self.error(t.span, "`USING` needs a value before this separator"));
                    }
                    after_value = false;
                }
                PrintPart::Expr(n) => {
                    let at = n.node().span();
                    if after_value {
                        // Measured: "Expected operator in equation", also next to a string (`v30_x08`).
                        return Err(self.error(at, "the values of `USING` need `;` or `,` between them"));
                    }
                    let e = self.unjoined(NoJoin::Item, |c| c.expr(*n))?;
                    values.push(using_value(e, *n));
                    after_value = true;
                }
            }
        }
        if values.is_empty() {
            // Measured: "Expected PRINT USING formatstring ; ..." (`v30_x02`, `x03`, `x16`).
            return Err(self.error(span, "`USING` needs a value after its format"));
        }
        Ok((format, values, after_value))
    }

    /// A string place as the target of `FIELD`, `LSET` or `RSET` (`what`): a variable, an element or a member.
    /// `fixed`: a `STRING * n` place is one too. Measured (`verification\v30_x100`–`x110`, `x140`–`x168`): a
    /// number, a `TYPE` value, a literal, an expression, a function's result and a `CONST` are errors. A whole
    /// array `e()`, which the old compiler takes as the array's first element without a bounds check (`x109`,
    /// `x153`), is not supported yet.
    fn string_target(&mut self, node: ast::Expr, what: &str, fixed: bool) -> R<Place> {
        let span = node.node().span();
        if let ast::Expr::Call(c) = node
            && c.arg_list().is_some_and(|l| l.args().is_empty())
        {
            if let Some(t) = c.name()
                && let (word, _) = self.split_name(t)?
                && self.procs_by_name.contains_key(&word)
            {
                // Measured: "Expected (...)" for a FUNCTION's name (`v30_x173`–`x175`).
                return Err(self.error(
                    span,
                    format!("`{what}` needs a string variable, element or member here"),
                ));
            }
            return Err(self.unsupported(span, format!("a whole array as the string of `{what}`")));
        }
        // A place of a user type is a place here, so that it gets this statement's error and not an expression's.
        let e = self.expr_as(node, Use::Place);
        let ExprKind::Load(place) = e?.kind else {
            return Err(self.error(
                span,
                format!("`{what}` needs a string variable, element or member here"),
            ));
        };
        match self.prog.place_ty(&place) {
            Ty::Str => Ok(place),
            Ty::FixedStr(_) if fixed => Ok(place),
            // Measured: "Fixed length strings cannot be used in a FIELD statement" (`v30_x100`, `x107`, `x108`).
            Ty::FixedStr(_) => Err(self.error(span, format!("`{what}` cannot take a fixed-length string"))),
            crate::numeric_types!() | Ty::User(_) => Err(self.error(
                span,
                format!("`{what}` needs a string variable, not a number or a `TYPE` value"),
            )),
        }
    }

    /// `FIELD [#]n, width AS string, …` (design D3 of `m3-console-files`; `qb64pe.bas` 7340–7432): the file
    /// number and each width a number, taken as a LONG; each target a variable-length string place. The parser
    /// read the statement's shape (the commas and each `AS`).
    pub(super) fn field_stmt(&mut self, stmt: ast::FieldStmt) -> R<()> {
        let node = stmt.node();
        let mut exprs = stmt.exprs();
        let file = match stmt.file() {
            Some(f) => self.file_number(f, "FIELD")?,
            None => {
                let n = self.need(exprs.next(), node.span())?;
                let e = self.expr(n)?;
                if e.ty == Ty::Str {
                    return Err(self.error(e.span, "the file number of `FIELD` must be a number"));
                }
                self.store(e, Ty::I32)?
            }
        };
        let mut parts = Vec::new();
        while let Some(width) = exprs.next() {
            let target = self.need(exprs.next(), node.span())?;
            // The target stands second and is evaluated first (measured, `v30_k_field_errors`: `fn ix wd`).
            let place = self.string_target(target, "FIELD", false)?;
            let w = self.expr(width)?;
            if w.ty == Ty::Str {
                return Err(self.error(w.span, "a width of `FIELD` must be a number"));
            }
            parts.push((place, self.store(w, Ty::I32)?));
        }
        if parts.is_empty() {
            return Err(self.error(node.span(), "`FIELD` needs a width and a string after the file number"));
        }
        self.push(node, StmtKind::Field { file, parts });
        Ok(())
    }

    /// `LSET target = value` and `RSET target = value` (`qb64pe.bas` 11327–11374): the target a string place of
    /// either kind, the value a string.
    pub(super) fn lset(&mut self, stmt: ast::LsetStmt) -> R<()> {
        let node = stmt.node();
        let word = self.need(stmt.keyword(), node.span())?;
        let word = self.word(word);
        let mut operands = stmt.operands();
        let target = self.need(operands.next(), node.span())?;
        let value = self.need(operands.next(), node.span())?;
        if let ast::Expr::Paren(_) = target {
            // Measured: `LSET (a) = "x"` is an assignment to an array named `LSET` to the old compiler, "Name
            // already in use (LSET)" (`verification\v30_x151`).
            return Err(self.error(target.node().span(), format!("name already in use: `{word}`")));
        }
        let target = self.string_target(target, &word, true)?;
        let value = self.expr(value)?;
        if value.ty != Ty::Str {
            // Measured: "Illegal string-number conversion" (`v30_x149`, `x154`, `x168`).
            return Err(self.error(value.span, format!("`{word}` needs a string as its value")));
        }
        let kind = StmtKind::Lset {
            right: word == "RSET",
            target,
            value,
        };
        self.push(node, kind);
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
                    items.push(self.unjoined(NoJoin::Item, |c| c.print_item(n))?);
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
                    let place = self.unjoined(NoJoin::Item, |c| c.target_place(n, what))?;
                    // Measured: "INPUT cannot handle _BIT array elements" for the console form (the runtime
                    // stores through an address, which a packed element has none of; `verification\v24_x253`).
                    // `INPUT #` and `READ` store into one (`v24_x251`, `x252`).
                    if console && place.has_element() && matches!(self.prog.place_ty(&place), Ty::Bit { .. }) {
                        let msg = format!("`{what}` from the console cannot read into a `_BIT` array element");
                        return Err(self.error(n.node().span(), msg));
                    }
                    // `INPUT #` stores a number as an assignment does, `READ` too (`Self::read`): not into a member
                    // of a `_MEM` block (measured, `verification\v32_x118`).
                    if !console && what == "INPUT" {
                        self.read_only_member(&place, n.node().span())?;
                    }
                    targets.push(place);
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
    /// `v22_x48`), as is the name of the FUNCTION the statement stands in; a whole array is not supported yet.
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
    /// stands in is an error.
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
        // A place reached through a member with an index on the way (`a(i).m`, `t.v(i)`): `SWAP` takes the two
        // places by reference, as an argument does (measured, `verification\v24_x202`, `x314`: an index outside
        // its bounds raises error 9 and the first element stands in); the others store by the rule of the place,
        // and `MID$` evaluates the place before its value (measured, `v35_x62`; the IR's crate documentation).
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
            // The first target is evaluated before anything is read.
            let target = if targets.is_empty() {
                self.target_place(e, "READ")?
            } else {
                self.unjoined(NoJoin::Item, |c| c.target_place(e, "READ"))?
            };
            self.read_only_member(&target, e.node().span())?;
            targets.push(target);
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
