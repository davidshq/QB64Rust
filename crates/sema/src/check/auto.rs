//! The constants of the file QB64pe includes before a program's first line (`beforefirstline.bi`; design D8 of
//! `m2-console-screenhide`): `_TRUE`, `_FALSE`, `_ONE_MB`, the limits of the numeric types and the others.
//!
//! The file is not included. The built-in table carries each constant's suffix and the text of its expression
//! (`qb64rust_builtins::AUTO_CONSTANTS`, in the file's order), and a constant becomes a constant of the main
//! module the first time a program names it: its text is read here and computed by the `CONST` evaluator's
//! arithmetic ([`crate::consteval`]), as its `CONST` line would be. So a program that names none is checked as
//! before, and a constant is visible in the main module and in every procedure (a user's name cannot start with an
//! underscore, so nothing hides it).
//!
//! The expressions of the file have three shapes: one operand; two operands with `+`, `*` or `/` between them; a
//! function call. An operand is a number with or without a minus, a string, or a constant that stands earlier in
//! the file. A function (`CHR$`, `EXP`) is not computed in a `CONST` yet, and neither is a value beyond `DOUBLE`
//! range; such a constant is "not supported yet" where it is used, with that reason. A shape that is none of these
//! is marked too, so a new version of the file cannot be misread.

use super::{Checker, R};
use crate::consteval::{self, Problem, Value};
use crate::literal::{self, LitError, NumLit};
use crate::{BinOp, Const, ConstId, Ty, UnOp};
use qb64rust_base::{Span, to_u32};
use qb64rust_builtins::{AUTO_CONSTANTS, auto_constant};

/// A constant's type and value, or why it is not computed yet.
type Computed = Result<(Ty, Value), String>;

impl Checker<'_> {
    /// Makes the auto-included constant `name` (upper case, without suffix) a constant of the main module, if it
    /// is one and is not defined yet. `Ok` for any other name. An auto-included constant whose value is not
    /// computed yet is "not supported yet" at `at`.
    pub(super) fn auto_const(&mut self, name: &str, at: Span) -> R<()> {
        let Some((index, _)) = auto_constant(name.as_bytes()) else {
            return Ok(());
        };
        match self.auto_const_at(index) {
            Ok(_) => Ok(()),
            Err(why) => {
                let msg = format!("`{name}`, a constant of QB64pe's auto-included file: {why}");
                Err(self.unsupported(at, msg))
            }
        }
    }

    /// The constant at `index` of the file's order, defined on the first call.
    fn auto_const_at(&mut self, index: usize) -> Result<ConstId, String> {
        if let Some(known) = self.auto_consts.get(&index) {
            return known.clone();
        }
        let c = &AUTO_CONSTANTS[index];
        let result = self.auto_value(index).map(|(ty, value)| {
            let id = ConstId(to_u32(self.prog.consts.len()));
            self.prog.consts.push(Const {
                name: c.name.to_string(),
                ty,
                value,
                proc: None,
            });
            self.consts_main.insert(c.name.to_string(), id);
            id
        });
        self.auto_consts.insert(index, result.clone());
        result
    }

    /// The type and value of the constant at `index`, as its `CONST` line gives them: the expression computed,
    /// read back as the old evaluator reads a result, and settled by the suffix of the name
    /// (`check\constants.rs`, `const_stmt`).
    fn auto_value(&mut self, index: usize) -> Computed {
        let c = &AUTO_CONSTANTS[index];
        let suffix = match c.suffix {
            "" => None,
            s => match literal::suffix_type(s.as_bytes()) {
                Some(Ok(ty)) => Some(ty),
                Some(Err(_)) | None => return Err(format!("the type suffix `{s}`")),
            },
        };
        let value = self.auto_expr(index, c.value)?;
        let value = consteval::reread(value).map_err(reason)?;
        consteval::settle(value, suffix).map_err(reason)
    }

    /// The value of an expression of the file (the shapes: this module's comment).
    fn auto_expr(&mut self, index: usize, text: &str) -> Result<Value, String> {
        if let Some(open) = text.find('(') {
            let function = text[..open].trim();
            if function.is_empty() || function.contains(' ') {
                // Parentheses that are no function call: none of the file's shapes.
                return Err(format!("its expression `{text}` is not read yet"));
            }
            return Err(format!("the function `{function}` in a `CONST`"));
        }
        for (sign, op) in [(" + ", BinOp::Add), (" * ", BinOp::Mul), (" / ", BinOp::Div)] {
            if let Some((left, right)) = text.split_once(sign) {
                let a = self.auto_operand(index, left)?;
                let b = self.auto_operand(index, right)?;
                return consteval::binary(op, &a, &b).map_err(reason);
            }
        }
        self.auto_operand(index, text)
    }

    /// One operand: a string, a constant that stands earlier in the file, or a number with or without a minus.
    fn auto_operand(&mut self, index: usize, text: &str) -> Result<Value, String> {
        let text = text.trim();
        let unread = || format!("its expression `{text}` is not read yet");
        if let Some(inner) = text.strip_prefix('"') {
            return match inner.strip_suffix('"') {
                Some(s) if !s.contains('"') => Ok(Value::Str(s.as_bytes().to_vec())),
                _ => Err(unread()),
            };
        }
        if text.starts_with('_') {
            return match auto_constant(text.as_bytes()) {
                Some((earlier, _)) if earlier < index => {
                    let id = self.auto_const_at(earlier)?;
                    Ok(self.prog.constant(id).value.clone())
                }
                _ => Err(unread()),
            };
        }
        let (negative, digits) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        if !digits.starts_with(|c: char| c.is_ascii_digit()) || digits.contains(' ') {
            return Err(unread());
        }
        if digits.bytes().all(|b| b.is_ascii_digit()) && digits.parse::<i64>().is_err() {
            // The evaluator reads no literal of this size yet (it would be an unsigned one).
            return Err("a number beyond `_INTEGER64` in a `CONST`".to_string());
        }
        let value = match literal::number(digits.as_bytes(), false) {
            Ok(NumLit::Int { value, .. }) => Value::Int(value),
            Ok(NumLit::Float { text, .. }) => Value::Float(consteval::float_literal(&text).map_err(reason)?),
            Err(LitError::Overflow) => return Err("a number beyond 64 bits".to_string()),
            Err(LitError::Error(msg)) => return Err(msg.to_string()),
            Err(LitError::Unsupported(what)) => return Err(what.to_string()),
        };
        if negative {
            consteval::unary(UnOp::Neg, &value).map_err(reason)
        } else {
            Ok(value)
        }
    }
}

/// Why the evaluator did not compute a value.
fn reason(p: Problem) -> String {
    match p {
        Problem::Error(msg) => msg,
        Problem::Unsupported(what) => what,
    }
}
