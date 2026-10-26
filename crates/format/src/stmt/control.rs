//! Control flow and jumps (`qb64pe.bas` 6155-7235: `NEXT`, `WHILE`/`WEND`, `DO`/`LOOP`, `FOR`, `ELSE`, `ELSEIF`, `IF`,
//! `END IF`, `SELECT CASE`, `END SELECT`, `CASE`; 9680-10130: `GOTO`, `_CONTINUE`, `RUN`, `END`, `SYSTEM`, `STOP`,
//! `GOSUB`, `RETURN`, `RESUME`, `ON ERROR`; 7440 `EXIT`; 27600 `ON … GOTO|GOSUB`).
//!
//! Keywords in the keyword style joined by `sp` to the expressions' layouts; `NEXT i, j` with `sp2` before each
//! comma; the items of a `CASE` with `sp2` before each comma, `Is` and its operator, `To`; a label as the first label
//! of that name in the source spells it.

use crate::fmt::Fmt;
use crate::lay::Lay;
use qb64rust_sema::SymbolKind;
use qb64rust_syntax::SyntaxKind::*;
use qb64rust_syntax::tree::{Element, Node, Tok};

impl Fmt<'_> {
    /// The layout of a control-flow statement, `None` for a kind this family does not lay out.
    pub(crate) fn control(&self, n: Node<'_>) -> Option<Lay> {
        let l = match n.kind() {
            IfHeader | ForHeader | DoHeader | LoopStmt | WhileHeader | NextStmt | SelectHeader => self.keyworded(n),
            CaseHeader => self.case(n),
            BlockEnd | ProcEnd => self.closer(n),
            EndStmt | SystemStmt | StopStmt | ExitStmt | ContinueStmt => self.keyworded(n),
            GotoStmt | GosubStmt | ReturnStmt | ResumeStmt | RestoreStmt | RunStmt => self.jump(n),
            OnErrorStmt | OnJumpStmt => self.jump(n),
            ErrorStmt => self.keyworded(n),
            ImplicitGoto => Lay(self.label_ref_of(n)),
            _ => return None,
        };
        Some(l)
    }

    /// A statement of keywords and expressions joined by `sp`, a comma with `sp2` before it.
    pub(crate) fn keyworded(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",");
                }
                Element::Token(t) if t.kind == Eq => {
                    l.then(&Lay::of(b"="));
                }
                Element::Token(t) if t.kind == Ident => {
                    l.then(&Lay(self.kw_tok(t)));
                }
                Element::Token(t) => {
                    l.then(&Lay::of(self.tok(t)));
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }

    /// `CASE` with its items (`qb64pe.bas` 6960-7210).
    fn case(&self, n: Node<'_>) -> Lay {
        let mut l = Lay(self.kw("CASE"));
        let mut first = true;
        for e in self.elems(n) {
            match e {
                Element::Token(t) if self.is_word(t, "CASE") => {}
                Element::Token(t) if self.is_word(t, "ELSE") => {
                    l.sp().text(&self.kw("ELSE"));
                }
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",");
                }
                Element::Token(_) => {}
                Element::Node(item) => {
                    let _ = first;
                    first = false;
                    self.case_item(item, &mut l);
                }
            }
        }
        l
    }

    /// `IS op expr`, `expr TO expr` or `expr`; an operator without `IS` gets it (`e$ = "IS" + sp + e$`).
    fn case_item(&self, n: Node<'_>, l: &mut Lay) {
        let elems = self.elems(n);
        let mut is_written = false;
        for e in &elems {
            match *e {
                Element::Token(t) if self.is_word(t, "IS") => {
                    is_written = true;
                    l.sp().text(&self.kw("IS"));
                }
                Element::Token(t) if self.is_word(t, "TO") => {
                    l.sp().text(&self.kw("TO"));
                }
                Element::Token(t) => {
                    if !is_written {
                        l.sp().text(&self.kw("IS"));
                        is_written = true;
                    }
                    l.sp().text(&self.operator(t));
                }
                Element::Node(c) => {
                    l.sp().append(&self.expr(c));
                }
            }
        }
    }

    /// `END IF`, `END SELECT`, `WEND`, `END TYPE`, `END DECLARE`, `END SUB`, `END FUNCTION`: the words in the keyword
    /// style (`ENDIF` as `End If`).
    fn closer(&self, n: Node<'_>) -> Lay {
        let words: Vec<Tok> = self.toks(n).into_iter().filter(|t| t.kind == Ident).collect();
        if words.len() == 1 && self.is_word(words[0], "ENDIF") {
            return Lay(self.kw("END IF"));
        }
        let mut l = Lay::new();
        for t in words {
            l.then(&Lay(self.kw_tok(t)));
        }
        l
    }

    /// A jump: its keywords, then the labels and expressions it names.
    fn jump(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        let mut after_jump = false;
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",");
                }
                Element::Token(t)
                    if after_jump
                        && (self.is_word(t, "NEXT")
                            || self.is_word(t, "_NEWHANDLER")
                            || self.is_word(t, "_LASTHANDLER")) =>
                {
                    l.then(&Lay(self.kw_tok(t)));
                }
                Element::Token(t) if t.kind == Ident && after_jump => {
                    l.then(&Lay(self.label_ref(t)));
                }
                Element::Token(t) if t.kind == Number && after_jump => {
                    l.then(&Lay(self.label_ref(t)));
                }
                Element::Token(t) if t.kind == Ident => {
                    after_jump = ["GOTO", "GOSUB", "RETURN", "RESUME", "RESTORE", "RUN"]
                        .iter()
                        .any(|k| self.is_word(t, k));
                    // `ON n GOTO|GOSUB`: the word from the line in capitals (`xongotogosub`: `l$ + sp + e2$`).
                    let w = if n.kind() == OnJumpStmt && (self.is_word(t, "GOTO") || self.is_word(t, "GOSUB")) {
                        self.tok(t).to_ascii_uppercase()
                    } else {
                        self.kw_tok(t)
                    };
                    l.then(&Lay(w));
                }
                Element::Token(t) => {
                    l.then(&Lay::of(self.tok(t)));
                }
                Element::Node(c) => {
                    l.then(&self.expr(c));
                }
            }
        }
        l
    }

    /// The label of an implicit `GOTO` (`IF x THEN 10`).
    fn label_ref_of(&self, n: Node<'_>) -> Vec<u8> {
        n.first_token().map(|t| self.label_ref(t)).unwrap_or_default()
    }

    /// A label as the label table spells it: the first label of that symbol in the source, as written there.
    pub(crate) fn label_ref(&self, t: Tok) -> Vec<u8> {
        let Some(id) = self.program.symbols.at(t.span.file, t.span.start) else {
            return self.tok(t).to_vec();
        };
        let sym = self.program.symbols.get(id);
        if !matches!(sym.kind, SymbolKind::Label(_)) {
            return self.name(t);
        }
        let first = std::iter::once(sym.def)
            .chain(sym.refs.iter().copied())
            .filter(|s| s.file == self.file)
            .min_by_key(|s| s.start)
            .unwrap_or(sym.def);
        self.text(first).to_vec()
    }
}
