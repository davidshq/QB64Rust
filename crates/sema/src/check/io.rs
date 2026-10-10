//! The I/O statements with nodes of their own (design D3 of `m2-builtin-statements`): `PRINT` to the console and to
//! a file, `WRITE`, `INPUT` and `LINE INPUT`, `CLOSE`. What each accepts and rejects is measured
//! (`verification\v22_b_*`, `v22_x21`–`x60`; `study\00` §5).

use super::{Checker, R};
use crate::builtins::stmt_lookup;
use crate::{Expr, InputSource, Place, PrintItem, StmtArg, StmtKind, Ty};
use qb64rust_base::Span;
use qb64rust_syntax::ast::{self, PrintPart};

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

    /// `INPUT #n, target, …`; from the console it is not supported yet.
    pub(super) fn input(&mut self, stmt: ast::InputStmt) -> R<()> {
        let node = stmt.node();
        let Some(f) = stmt.file() else {
            return Err(self.unsupported(super::first_token_span(node), "`INPUT`"));
        };
        let from = InputSource::File(self.file_number(f, "INPUT #")?);
        let targets = self.targets(stmt.parts(), node.span(), "INPUT")?;
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
    /// second one "Too many variables"); from the console it is not supported yet.
    pub(super) fn line_input(&mut self, stmt: ast::LineInputStmt) -> R<()> {
        let node = stmt.node();
        let Some(f) = stmt.file() else {
            return Err(self.unsupported(super::first_token_span(node), "`LINE INPUT`"));
        };
        let from = InputSource::File(self.file_number(f, "LINE INPUT #")?);
        let targets = self.targets(stmt.parts(), node.span(), "LINE INPUT")?;
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
    /// target is "Expected , ...", a target that is no variable "Expected variable-name".
    pub(super) fn targets<'t>(
        &mut self,
        parts: impl Iterator<Item = PrintPart<'t>>,
        span: Span,
        what: &str,
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
        if want_target {
            return Err(self.error(span, format!("`{what}` needs a variable to read into")));
        }
        Ok(targets)
    }

    /// A place a statement stores into (a target of `INPUT`, `LINE INPUT` or `READ`): a variable (created when it
    /// does not exist, as by an assignment), an element or a member. A whole `TYPE` variable is an error (measured,
    /// `v22_x48`); a whole array and a member of an element are not supported yet.
    pub(super) fn target_place(&mut self, e: ast::Expr, what: &str) -> R<Place> {
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
        if let Ty::User(_) = self.prog.place_ty(&place) {
            return Err(self.error(span, format!("a whole `TYPE` variable cannot be a target of `{what}`")));
        }
        if matches!(place, Place::Member { .. }) && place.has_element() {
            // Not measured: where the old compiler evaluates the index of `a(i).m` as a target.
            return Err(self.unsupported(span, format!("a member of an array element as a target of `{what}`")));
        }
        Ok(place)
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
