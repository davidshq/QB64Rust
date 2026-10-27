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
//! The expressions of the file have three shapes: one term; two terms with `+`, `*` or `/` between them. A term is
//! an operand or a function call of one operand (`CHR$(13)`, `EXP(1.0##)`); an operand is a number with or without
//! a minus, a string, or a constant that stands earlier in the file. Literals and calls take the paths of a program's
//! `CONST` (`consteval::literal`, `consteval::function`; design D7 of `m3-constant-evaluation`); a value that does
//! not certify takes the correctly rounded one (`_E`; `QB64PE-DIVERGENCES.md` D-026), and each use of it warns: a `CONST`
//! that names it, or a use outside one ([`Checker::auto_const_use`]). A value that cannot be computed is "not
//! supported yet" where it is used, with that reason. A shape that is none of these is marked too, so a new version
//! of the file cannot be misread.

use super::{Checker, R};
use crate::consteval::{self, Problem, Value};
use crate::literal;
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

    /// [`Self::auto_const`] for a use outside a `CONST`: an auto-included constant whose value took the correctly
    /// rounded one (`_E`; `QB64PE-DIVERGENCES.md` D-026) warns at the use, as it has no `CONST` line of the program's.
    pub(super) fn auto_const_use(&mut self, name: &str, at: Span) -> R<()> {
        self.auto_const(name, at)?;
        if let Some((index, _)) = auto_constant(name.as_bytes())
            && let Some(Ok(id)) = self.auto_consts.get(&index)
            && self.fell_back.contains(id)
        {
            let shown = qb64rust_base::show_bytes(self.text(at));
            self.diags.warning(
                at,
                format!(
                    "`{shown}` is computed with correctly rounded math functions; QB64pe's value depends on the CPU \
                     and the system's library and may differ"
                ),
            );
        }
        Ok(())
    }

    /// The constant at `index` of the file's order, defined on the first call.
    fn auto_const_at(&mut self, index: usize) -> Result<ConstId, String> {
        if let Some(known) = self.auto_consts.get(&index) {
            return known.clone();
        }
        let c = &AUTO_CONSTANTS[index];
        // The first use may stand inside a program's `CONST`, whose own note of a fallback is kept apart.
        let outer = consteval::take_fallback();
        let value = self.auto_value(index);
        let fell_back = consteval::take_fallback();
        if outer {
            consteval::fell_back();
        }
        let result = value.map(|(ty, value)| {
            let id = ConstId(to_u32(self.prog.consts.len()));
            self.prog.consts.push(Const {
                name: c.name.to_string(),
                ty,
                value,
                proc: None,
            });
            self.consts_main.insert(c.name.to_string(), id);
            if fell_back {
                self.fell_back.insert(id);
            }
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
        let value = consteval::reread(value, true).map_err(reason)?;
        consteval::settle(value, suffix).map_err(reason)
    }

    /// The value of an expression of the file (the shapes: this module's comment).
    fn auto_expr(&mut self, index: usize, text: &str) -> Result<Value, String> {
        for (sign, op) in [(" + ", BinOp::Add), (" * ", BinOp::Mul), (" / ", BinOp::Div)] {
            if let Some((left, right)) = text.split_once(sign) {
                if cuts_parentheses(left) || cuts_parentheses(right) {
                    // A sign inside parentheses: none of the file's shapes, named whole.
                    return Err(format!("its expression `{text}` is not read yet"));
                }
                let a = self.auto_term(index, left)?;
                let b = self.auto_term(index, right)?;
                return consteval::binary(op, &a, &b).map_err(reason);
            }
        }
        self.auto_term(index, text)
    }

    /// An operand, or a call of a function of the evaluator whose argument is one (`CHR$(13)`, `EXP(1.0##)`),
    /// computed as in a `CONST` (`consteval::function`).
    fn auto_term(&mut self, index: usize, text: &str) -> Result<Value, String> {
        let text = text.trim();
        let Some(open) = text.find('(') else {
            return self.auto_operand(index, text);
        };
        let function = text[..open].trim();
        let arg = text[open + 1..].strip_suffix(')');
        if function.is_empty() || function.contains(' ') || arg.is_none_or(|a| a.contains(['(', ')', ','])) {
            // Parentheses that are no call of one argument: none of the file's shapes.
            return Err(format!("its expression `{text}` is not read yet"));
        }
        let arg = self.auto_operand(index, arg.unwrap_or_default())?;
        // The argument is read back as the evaluator reads a function's arguments.
        let arg = consteval::reread(arg, false).map_err(reason)?;
        let v = consteval::function(function, &[arg]).map_err(reason)?;
        consteval::reread(v, false).map_err(reason)
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
                    if self.fell_back.contains(&id) {
                        consteval::fell_back();
                    }
                    let c = self.prog.constant(id);
                    Ok(consteval::lookup(c.ty, &c.value))
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
        let value = consteval::literal(digits.as_bytes()).map_err(reason)?;
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

/// Whether one side of a split at a sign holds a parenthesis without its partner: the sign stood inside
/// parentheses (`CHR$(1 + 2)`, `(1 + 2)`).
fn cuts_parentheses(part: &str) -> bool {
    part.matches('(').count() != part.matches(')').count()
}

#[cfg(test)]
mod tests {
    use super::cuts_parentheses;

    #[test]
    fn a_sign_inside_parentheses_cuts_them() {
        for (left, right) in [("CHR$(13)", "CHR$(10)"), ("_CHR_CR", "_CHR_LF"), ("1", "2")] {
            assert!(!cuts_parentheses(left) && !cuts_parentheses(right), "{left} + {right}");
        }
        for (left, right) in [("CHR$(1", "2)"), ("(1", "2)")] {
            assert!(cuts_parentheses(left) && cuts_parentheses(right), "{left} + {right}");
        }
    }
}
