//! Procedures and bodies: signatures, prototypes, `main0.txt`, the per-procedure fragments, statements with their
//! labels, and `retK.txt`.

use crate::Emitter;
use crate::decl::{declare, is_qbs};
use crate::names::{c_type, proc_name, var_name};
use qb64rust_base::to_u32;
use qb64rust_ir::{Body, LabelId, Proc, ProcId, ProcKind, Storage, Var};
use std::fmt::Write as _;

/// The prologue of a procedure, after its signature line (`study\02` §8, verbatim from the old compiler).
const PROLOGUE: &[&str] = &[
    "qbs *tqbs;",
    "ptrszint tmp_long;",
    "int32 tmp_fileno;",
    "uint32 qbs_tmp_base=qbs_tmp_list_nexti;",
    "uint8 *tmp_mem_static_pointer=mem_static_pointer;",
    "uint32 tmp_cmem_sp=cmem_sp;",
];

const PROLOGUE_AFTER_DATA: &[&str] = &[
    "mem_lock *sf_mem_lock;",
    "new_mem_lock();",
    "sf_mem_lock=mem_lock_tmp;",
    "sf_mem_lock->type=3;",
    "libqb_check_stack();",
    "if (is_error_pending()) goto exit_subfunc;",
];

const EPILOGUE_AFTER_FREE: &[&str] = &[
    "if ((tmp_mem_static_pointer>=mem_static)&&(tmp_mem_static_pointer<=mem_static_limit)) mem_static_pointer=tmp_mem_static_pointer; else mem_static_pointer=mem_static;",
    "cmem_sp=tmp_cmem_sp;",
];

impl<'a> Emitter<'a> {
    /// `int32 FUNC_F(int32*_FUNC_F_LONG_A,qbs*_FUNC_F_STRING_S)`
    fn signature(&self, proc: &Proc) -> String {
        let ret = match proc.kind {
            ProcKind::Sub => "void",
            ProcKind::Function(t) => c_type(t),
        };
        let params: Vec<String> = proc
            .params
            .iter()
            .map(|&id| {
                let v = self.p.var(id);
                let t = if is_qbs(v.ty) {
                    "qbs*"
                } else {
                    &format!("{}*", c_type(v.ty))
                };
                format!("{t}{}", self.name(id))
            })
            .collect();
        format!("{ret} {}({})", proc_name(proc), params.join(","))
    }

    pub(crate) fn prototypes(&self) -> String {
        self.p
            .procs
            .iter()
            .map(|q| format!("{};\n", self.signature(q)))
            .collect()
    }

    pub(crate) fn main0(&mut self) -> String {
        // Tells the runtime that this program tracks error locations itself (it does not: no `$ERRORLOCATION`),
        // so a critical error says "Enable $ErrorLocation:ON" as with the old compiler, instead of a line number
        // from the runtime's fallback for old executables (measured with `s11_on_error`, error 11). Runs again
        // each time a handler re-enters `QBMAIN`, as in the old compiler.
        let mut out = String::from("error_track_line(0,0,NULL);\nS_0:;\n");
        let p = self.p;
        self.body(&p.main, 0, &mut out);
        out.push_str("sub_end();\nreturn;\n}\n");
        out
    }

    /// `retK.txt` of the body just emitted, if it has a `RETURN` without label: pop the last return id and go back
    /// to its `GOSUB`, if that was made in this body; otherwise, or with no `GOSUB` pending, error 3. Id 0 (pushed by
    /// event dispatch, which this compiler does not have yet) returns from `QBMAIN` in the main module and is error 3
    /// in a procedure, as in the old compiler (`qb64pe.bas` 3311, 5433, 17588).
    pub(crate) fn take_ret(&mut self, k: usize) -> Option<String> {
        let cases = std::mem::take(&mut self.ret_cases);
        if !std::mem::take(&mut self.returns) {
            return None;
        }
        let zero = if k == 0 { "return;" } else { "error(3);" };
        Some(format!(
            "if (next_return_point){{\nnext_return_point--;\nswitch(return_point[next_return_point]){{\n\
             case 0:\n{zero}\nbreak;\n{cases}}}\n}}\nerror(3);\n"
        ))
    }

    /// `mainK.txt`, `dataK.txt` and `freeK.txt` of procedure `i` (K = i + 1).
    pub(crate) fn procedure(&mut self, i: usize, proc: &'a Proc, k: usize) -> (String, String, String) {
        let (mut data, mut free) = (String::new(), String::new());
        let mine = |s: Storage| match s {
            Storage::Local(q) | Storage::Result(q) => q.0 as usize == i,
            Storage::Global | Storage::Static(_) | Storage::Param(_) | Storage::Temp(_) => false,
        };
        // The result first, then locals in order of creation.
        let mut per_call: Vec<&Var> = proc.result.iter().map(|&r| self.p.var(r)).collect();
        per_call.extend(
            self.p
                .vars
                .iter()
                .filter(|v| matches!(v.storage, Storage::Local(_)) && mine(v.storage)),
        );
        for v in per_call {
            let n = var_name(self.p, v);
            data.push_str(&declare(v, &n));
            data.push_str(&self.alloc(v, &n));
            // The result is returned with `qbs_maketmp`, so the caller's cleanup frees it.
            if is_qbs(v.ty) && !matches!(v.storage, Storage::Result(_)) {
                writeln!(free, "qbs_free({n});").unwrap();
            }
        }
        // A string argument that is a temporary, fixed-length or read-only is replaced by a copy for the call;
        // a fixed-length one gets the copy's final value back.
        for (j, &id) in proc.params.iter().enumerate() {
            if !is_qbs(self.p.var(id).ty) {
                continue;
            }
            let (n, old) = (self.name(id), format!("oldstr{j}"));
            write!(
                data,
                "qbs*{old}=NULL;\n\
                 if({n}->tmp||{n}->fixed||{n}->readonly){{\n\
                 {old}={n};\n\
                 if ({old}->cmem_descriptor){{\n\
                 {n}=qbs_new_cmem({old}->len,0);\n\
                 }}else{{\n\
                 {n}=qbs_new({old}->len,0);\n\
                 }}\n\
                 memcpy({n}->chr,{old}->chr,{old}->len);\n\
                 }}\n"
            )
            .unwrap();
            write!(
                free,
                "if({old}){{\nif({old}->fixed)qbs_set({old},{n});\nqbs_free({n});\n}}\n"
            )
            .unwrap();
        }
        data.push_str(&self.temps(Some(ProcId(to_u32(i)))));

        let mut main = String::new();
        let mut header = vec![format!("{}{{", self.signature(proc))];
        header.extend(PROLOGUE.iter().map(|s| s.to_string()));
        header.push(format!("#include \"data{k}.txt\""));
        header.extend(PROLOGUE_AFTER_DATA.iter().map(|s| s.to_string()));
        self.lines(proc.line, &header, &mut main);
        self.body(&proc.body, k, &mut main);
        let mut footer = vec![
            "exit_subfunc:;".to_string(),
            "free_mem_lock(sf_mem_lock);".to_string(),
            format!("#include \"free{k}.txt\""),
        ];
        footer.extend(EPILOGUE_AFTER_FREE.iter().map(|s| s.to_string()));
        if let Some(r) = proc.result {
            let n = self.name(r);
            footer.push(if is_qbs(self.p.var(r).ty) {
                format!("qbs_maketmp({n});return {n};")
            } else {
                format!("return *{n};")
            });
        }
        footer.push("}".to_string());
        self.lines(proc.end_line, &footer, &mut main);
        for d in std::mem::take(&mut self.pass_decls) {
            data.push_str(&d);
        }
        (main, data, free)
    }

    /// Body number `k` (0 for the main module, K for procedure K).
    fn body(&mut self, b: &'a Body, k: usize, out: &mut String) {
        self.body = b;
        self.k = k;
        for (i, s) in b.stmts.iter().enumerate() {
            self.labels(i, out);
            let mut body = Vec::new();
            // Whether an earlier operation of the statement may have left an error pending.
            let mut raised = false;
            for op in &s.ops {
                self.op(op, raised, &mut body);
                raised |= op.may_raise();
            }
            let mut lines = vec!["do{".to_string()];
            lines.extend(body);
            let file = Some(s.span.file);
            lines.push(format!(
                "if(!qbevent)break;evnt({});}}while(r);",
                self.evnt_args(file, s.line)
            ));
            self.lines_in(file, s.line, &lines, out);
        }
        self.labels(b.stmts.len(), out);
    }

    /// The labels that stand before statement `at`; a user label with the old compiler's event check.
    fn labels(&self, at: usize, out: &mut String) {
        for (i, l) in self.body.labels.iter().enumerate().filter(|(_, l)| l.at == at) {
            let mut lines = vec![format!("{}:;", self.local_label(LabelId(to_u32(i))))];
            if l.name.is_some() {
                lines.push(format!("if(qbevent){{evnt({});r=0;}}", self.evnt_args(l.file, l.line)));
            }
            self.lines_in(l.file, l.line, &lines, out);
        }
    }
}
