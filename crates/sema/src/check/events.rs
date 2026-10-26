//! Event trapping (design D2 of `m3-events`; measured, `verification\v42_*`, `study\00` §5): `ON TIMER`, `ON KEY`
//! and `ON STRIG` (design D4 of `m4-devices-continue-paths`, `v47_x04`–`x13`, `x21`, `x27`, `x30`, `x31`) with a
//! label of the main module or a SUB as the handler, and the forms of `KEY` outside its template. The switches
//! (`TIMER ON`, `KEY(1) OFF`, `STRIG(0) ON`) and `_FREETIMER` are plain built-ins of the table (`builtins.rs`).

use super::{Checker, R};
use crate::{Event, Handler, KeyForm, ProcKind, StmtKind, SymbolKind, Ty};
use qb64rust_base::show_bytes;
use qb64rust_syntax::SyntaxKind::Number;
use qb64rust_syntax::ast;

impl Checker<'_> {
    /// `ON TIMER([n,] seconds) GOSUB label`, `ON TIMER(…) name [argument]`, and the same of `ON KEY(key)`
    /// (`qb64pe.bas` 7726–8052). The operands first, then the handler, as the old compiler checks them.
    pub(super) fn on_event(&mut self, s: ast::OnEventStmt) -> R<()> {
        let node = s.node();
        let span = node.span();
        let ev = self.need(s.event(), span)?;
        let word = self.word(ev);
        match word.as_str() {
            "TIMER" | "KEY" => {}
            // With libqb's device-input part, set before the operands are read (`qb64pe.bas` 7535).
            "STRIG" => {
                self.settings.runtime_parts.insert(crate::RuntimePart::DeviceInput);
            }
            // Measured: "Name already in use (PEN)" (`v42_x17`–`x19`).
            _ => {
                return Err(self.error(
                    ev.span,
                    format!("`{word}` is not an event: `ON {word}` cannot be trapped"),
                ));
            }
        }
        let Some(list) = s.args() else {
            // Measured: "Expected (" (`v42_x30`).
            return Err(self.error(span, format!("`ON {word}` needs its operands in parentheses")));
        };
        let operands = list.args();
        let event = if word == "TIMER" {
            match operands[..] {
                [Some(seconds)] => Event::Timer {
                    number: None,
                    seconds: self.event_operand(seconds, Ty::F64, "the seconds of `ON TIMER`")?,
                },
                [Some(number), Some(seconds)] => Event::Timer {
                    number: Some(self.event_operand(number, Ty::I32, "the timer number of `ON TIMER`")?),
                    seconds: self.event_operand(seconds, Ty::F64, "the seconds of `ON TIMER`")?,
                },
                // Measured: "Expected ... )", "Expected ... ,", "Expected )" (`v42_x27`, `x28`).
                _ => {
                    let msg = "`ON TIMER` takes the seconds, or a timer number and the seconds";
                    return Err(self.error(list.node().span(), msg));
                }
            }
        } else if word == "STRIG" {
            // Design D4 of `m4-devices-continue-paths`: a button number, or a button and a controller, each LONG.
            match operands[..] {
                [Some(button)] => Event::Strig {
                    button: self.event_operand(button, Ty::I32, "the button number of `ON STRIG`")?,
                    controller: None,
                },
                [Some(button), Some(controller)] => Event::Strig {
                    button: self.event_operand(button, Ty::I32, "the button number of `ON STRIG`")?,
                    controller: Some(self.event_operand(controller, Ty::I32, "the controller number of `ON STRIG`")?),
                },
                // Measured: "Expected ... )", "Expected ... ,", "Expected )" (`verification\v47_x05`–`x08`).
                _ => {
                    let msg = "`ON STRIG` takes a button number, or a button and a controller number";
                    return Err(self.error(list.node().span(), msg));
                }
            }
        } else {
            match operands[..] {
                // Measured: the key is a DOUBLE that libqb's `int32` parameter cuts (`v42_d` 12.–14.).
                [Some(key)] => Event::Key {
                    key: self.event_operand(key, Ty::F64, "the key number of `ON KEY`")?,
                },
                // Measured: "Expected ... )", "Invalid expression" (`v42_x29`).
                _ => return Err(self.error(list.node().span(), "`ON KEY` takes one key number")),
            }
        };
        let words = s.handler_words();
        let handler = match words[..] {
            [g, target] if self.word(g) == "GOSUB" => {
                // The main module's label or line number, also from a SUB or FUNCTION (measured, `v42_x01`, `x02`, `x37`, `x50`).
                Handler::Label(self.label_ref(target, None)?)
            }
            [target] if self.word(target) != "GOSUB" => self.event_sub(s, &word, target)?,
            // `ON TIMER(1)`, `ON TIMER(1) GOSUB`: the parser reported it.
            _ => return Err(self.error(span, format!("`ON {word}` needs `GOSUB label` or a SUB's name"))),
        };
        self.push(node, StmtKind::OnEvent { event, handler });
        Ok(())
    }

    /// An operand of `ON TIMER` or `ON KEY`, converted to `to` as the old compiler's `evaluatetotyp` does.
    fn event_operand(&mut self, e: ast::Expr, to: Ty, what: &str) -> R<crate::Expr> {
        let e = self.expr(e)?;
        if e.ty == Ty::Str {
            // Measured: "Illegal string-number conversion" (`v42_x33`, `x34`).
            return Err(self.error(e.span, format!("{what} must be a number")));
        }
        self.store(e, to)
    }

    /// The SUB of a handler and its argument (measured, `v42_x03`–`x06`, `x10`, `x13`, `x14`, `x23`, `x24`, `x35`,
    /// `x36`, `x39`–`x42`, `x48`, `x52`, `x57`, `x58`).
    fn event_sub(&mut self, s: ast::OnEventStmt, word: &str, target: qb64rust_syntax::tree::Tok) -> R<Handler> {
        let shown = show_bytes(self.text(target.span));
        let found = if target.kind == Number {
            None
        } else {
            let (name, suffix) = self.split_name(target)?;
            suffix
                .is_none()
                .then(|| self.procs_by_name.get(&name).copied())
                .flatten()
                .filter(|p| self.prog.procs[p.0 as usize].kind == ProcKind::Sub)
        };
        let Some(proc) = found else {
            // Measured: "Expected GOSUB/sub" for no such SUB, a FUNCTION or a label (`v42_x10`, `x13`, `x52`).
            let msg = format!("`{shown}` is no SUB: `ON {word}` needs `GOSUB label` or a SUB's name");
            return Err(self.error(target.span, msg));
        };
        // A use of the SUB, for the language server's references and rename (`m5b-language-features`).
        self.stmt.names.push((SymbolKind::Proc(proc), target.span));
        let p = &self.prog.procs[proc.0 as usize];
        if p.external.is_some() {
            let msg = format!("a SUB of a `DECLARE LIBRARY` block as an event handler: `{shown}`");
            return Err(self.unsupported(target.span, msg));
        }
        let params = p.params.clone();
        if params.len() > 1 {
            // Measured: "SUB requires more than one argument" (`v42_x24`).
            let msg = format!("the SUB `{shown}` of an event handler may have one parameter at most");
            return Err(self.error(target.span, msg));
        }
        let arg = match (params.first(), s.argument()) {
            (None, None) => None,
            (None, Some(a)) => {
                // Measured: "SUB has no arguments" (`v42_x04`).
                return Err(self.error(a.node().span(), format!("the SUB `{shown}` has no parameter")));
            }
            (Some(_), None) => {
                // Measured: "Expected argument after SUB" (`v42_x03`, `x48`).
                let msg = format!("the SUB `{shown}` needs its argument after its name");
                return Err(self.error(target.span, msg));
            }
            (Some(&param), Some(a)) => {
                let var = &self.prog.vars[param.0 as usize];
                // An integer type, `_OFFSET` included; not `_BIT`, a float, a string, an array (measured: "Only SUB
                // arguments of integer-type allowed", `v42_x05`, `x14`, `x40`, `x41`, `x58`) nor a `TYPE`, which
                // the old compiler accepts and whose C++ then fails to build (`x42`).
                let integer = var.ty.is_int() && !matches!(var.ty, Ty::Bit { .. });
                if !integer || var.is_array() {
                    let msg = format!("the parameter of the SUB `{shown}` of an event handler must be an integer");
                    return Err(self.error(target.span, msg));
                }
                let e = self.expr(a)?;
                if e.ty == Ty::Str {
                    // Measured: "Illegal string-number conversion" (`v42_x35`).
                    return Err(self.error(e.span, format!("the argument of the SUB `{shown}` must be a number")));
                }
                Some(self.store(e, Ty::I64)?)
            }
        };
        Ok(Handler::Sub { proc, arg })
    }

    /// `KEY ON`, `KEY OFF`, `KEY LIST` and `KEY n, text$` (`qb64pe.bas` 7281–7330; measured, `v42_d`, `v42_x26`).
    pub(super) fn key_stmt(&mut self, s: ast::KeyStmt) -> R<()> {
        let node = s.node();
        let form = if let Some(w) = s.word() {
            match self.word(w).as_str() {
                "ON" => KeyForm::On,
                "OFF" => KeyForm::Off,
                "LIST" => KeyForm::List,
                _ => return Err(self.error(node.span(), "internal error: a `KEY` statement of no form")),
            }
        } else {
            let mut exprs = s.exprs();
            let (Some(number), Some(text)) = (exprs.next(), exprs.next()) else {
                // The parser reported it.
                return Err(self.error(node.span(), "`KEY n, text$` needs the key's number and its text"));
            };
            let number = self.event_operand(number, Ty::I32, "the key number of `KEY n, text$`")?;
            let text = self.expr(text)?;
            if text.ty != Ty::Str {
                // Measured: "Illegal string-number conversion" (`v42_x26`).
                return Err(self.error(text.span, "the text of `KEY n, text$` must be a string"));
            }
            KeyForm::Assign { number, text }
        };
        self.push(node, StmtKind::Key(form));
        Ok(())
    }
}
