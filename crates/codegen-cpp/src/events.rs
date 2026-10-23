//! Event trapping (design D4 of `m3-events`): the setup call where `ON TIMER`, `ON KEY` or `ON STRIG` (design D4
//! of `m4-devices-continue-paths`) stands, and what
//! `qbx.cpp`'s `events()` runs when libqb services the event, as the old compiler writes them (`qb64pe.bas`
//! 7726–8052). The handlers of each event are numbered from 1 in the order the bodies are emitted (the main module,
//! then the procedures in definition order), which is the old compiler's source order. Per handler `events()` gets
//! a `case` (`ontimer.txt`, `onkey.txt`, `onstrig.txt`): for a label, it re-enters `QBMAIN` with return point 0,
//! and `QBMAIN`'s prologue jumps to the label (`ontimerj.txt`, `onkeyj.txt`, `onstrigj.txt`); for a SUB, it calls the SUB with a pointer to the
//! `_INTEGER64` argument (`i64`, which libqb copied from the setup's) cast to the parameter's type.

use crate::Emitter;
use crate::names::{c_type, proc_name};
use qb64rust_ir::{Event, EventHandler, KeyForm};
use std::fmt::Write as _;

/// The generated parts of one event: its handlers so far, the `case`s of `events()` and the jumps at the start of
/// `QBMAIN`.
#[derive(Default)]
pub(crate) struct EventCode {
    count: u32,
    pub(crate) cases: String,
    pub(crate) jumpers: String,
}

impl Emitter<'_> {
    /// `ON TIMER(…)`, `ON KEY(…)` or `ON STRIG(…)` and its handler.
    pub(crate) fn on_event(&mut self, event: &Event, handler: &EventHandler, out: &mut Vec<String>) {
        let (word, setup) = match event {
            Event::Timer { number, seconds } => {
                let n = number.as_ref().map_or_else(|| "0".to_string(), |n| self.value(n));
                ("timer", format!("ontimer_setup({n},{}", self.value(seconds)))
            }
            Event::Key { key } => ("key", format!("onkey_setup({}", self.value(key))),
            // The controller and whether it was given; left out, 0 and 0 (`qb64pe.bas` 7566–7593).
            Event::Strig { button, controller } => {
                let (c, passed) = controller.as_ref().map_or(("0".to_string(), 0), |c| (self.value(c), 1));
                ("strig", format!("onstrig_setup({},{c},{passed}", self.value(button)))
            }
        };
        let (case, pass) = match handler {
            EventHandler::Label(l) => (Case::Label(self.label_name(*l)), "0".to_string()),
            EventHandler::Sub { proc, arg } => {
                let q = self.p.proc(*proc);
                let call = match q.params.first() {
                    Some(&v) => format!("{}(({}*)&i64);", proc_name(q), c_type(self.p.var(v).ty)),
                    None => format!("{}();", proc_name(q)),
                };
                let pass = arg.as_ref().map_or_else(|| "0".to_string(), |a| self.value(a));
                (Case::Call(call), pass)
            }
        };
        let code = match word {
            "timer" => &mut self.on_timer,
            "key" => &mut self.on_key,
            _ => &mut self.on_strig,
        };
        code.count += 1;
        let id = code.count;
        match case {
            Case::Label(label) => {
                writeln!(code.jumpers, "if({word}_event_id=={id})goto {label};").unwrap();
                write!(
                    code.cases,
                    "case {id}:\n{word}_event_occurred++;\n{word}_event_id={id};\n{word}_event_occurred++;\n\
                     return_point[next_return_point++]=0;\n\
                     if (next_return_point>=return_points) more_return_points();\nQBMAIN(NULL);\nbreak;\n"
                )
                .unwrap();
            }
            Case::Call(call) => write!(code.cases, "case {id}:\n{call}\nbreak;\n").unwrap(),
        }
        out.push(format!("{setup},{id},{pass});"));
    }

    /// `KEY ON`, `KEY OFF`, `KEY LIST`, `KEY n, text$`.
    pub(crate) fn key_stmt(&mut self, form: &KeyForm, out: &mut Vec<String>) {
        match form {
            KeyForm::On => out.push("key_on();".into()),
            KeyForm::Off => out.push("key_off();".into()),
            KeyForm::List => out.push("key_list();".into()),
            KeyForm::Assign { number, text } => {
                out.push(format!("key_assign({},{});", self.value(number), self.value(text)));
                out.push("qbs_cleanup(qbs_tmp_base,0);".into());
            }
        }
    }
}

/// What the `case` of a handler does.
enum Case {
    /// Re-enter `QBMAIN` and jump to this label there.
    Label(String),
    /// This call of a SUB.
    Call(String),
}
