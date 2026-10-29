//! The errors of a failed C++ build that are the program's (design D3, D4 of `cpp-failure-errors`): when every error
//! clang and the linker print names the C function of a `DECLARE LIBRARY` declaration, each becomes an error of the
//! program, at the call clang names through the `#line` directives or at the declaration of a symbol the linker
//! does not find. Anything this module cannot read or match gives `None`, and the build's failure stays the
//! toolchain's output: it may be this compiler's own fault.

use crate::Frontend;
use qb64rust_base::{Diagnostic, Span};
use qb64rust_sema::{LibraryKind, ProcId, SymbolKind};

/// The headers of the system packages a Linux build includes (spec `compiler/toolchain`, "The system packages").
/// `toolchain install` looks for them too (`toolchain::check_system`).
pub const SYSTEM_HEADERS: [&str; 5] = [
    "GL/gl.h",
    "GL/glu.h",
    "X11/Xlib.h",
    "X11/extensions/Xrandr.h",
    "alsa/asoundlib.h",
];

/// The libraries of the system packages a Linux build links (the `Makefile`'s `OS=lnx` lines).
const SYSTEM_LIBRARIES: [&str; 7] = ["GL", "GLU", "X11", "Xrandr", "xcb", "asound", "png"];

/// The header or library of the system packages that the output of a failed Linux build says is missing:
/// `fatal error: GL/glu.h: No such file or directory`, or the linker's `cannot find -lGLU`.
pub fn missing_system_file(output: &str) -> Option<String> {
    for line in output.lines() {
        if let Some((_, rest)) = line.split_once("fatal error: ")
            && let Some((file, _)) = rest.split_once(": No such file or directory")
            && SYSTEM_HEADERS.contains(&file)
        {
            return Some(file.to_string());
        }
        if let Some((_, rest)) = line.split_once("cannot find -l") {
            let lib = rest.split([':', ' ']).next().unwrap_or("");
            if SYSTEM_LIBRARIES.contains(&lib) {
                return Some(format!("-l{lib}"));
            }
        }
    }
    None
}

/// One error of the toolchain, as read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Failure {
    /// clang's error at line `line` of the program's file `file` (its name in the `#line` directives): its message,
    /// with the notes that explain it, and the first name in single quotes in it.
    Compile {
        file: String,
        line: u32,
        name: String,
        message: String,
    },
    /// A symbol the linker does not find, without its parameters (`f(int)` is `f`).
    Link { name: String },
}

/// The failures of `output`, the toolchain's output, given the names of the program's files; `None` at the first
/// line with an error this module cannot read.
pub fn read(output: &str, files: &[&str]) -> Option<Vec<Failure>> {
    let mut files = files.to_vec();
    // The longest name first, so that `a.bas` is not taken for the start of `a.bas.bi`.
    files.sort_by_key(|f| std::cmp::Reverse(f.len()));
    let mut failures = Vec::new();
    // The compile error the notes that follow belong to.
    let mut open: Option<usize> = None;
    for line in output.lines() {
        let line = line.trim_end();
        // Code and caret lines, the linker's `>>> referenced by`, the includes that led to an error, `make`'s lines,
        // and the commands `make` echoes, which start with the tool (`…\bin\c++.exe`, on Linux `/usr/bin/g++`; a link
        // command names `error_handle.o`).
        let first = line.split_whitespace().next().unwrap_or("");
        let tool = first.rsplit(['/', '\\']).next().unwrap_or(first).to_ascii_lowercase();
        if line.is_empty()
            || tool.ends_with(".exe")
            || matches!(tool.as_str(), "g++" | "c++" | "gcc" | "cc" | "ar" | "objcopy")
            || line.starts_with(char::is_whitespace)
            || line.starts_with(">>>")
            || line.starts_with("In file included from ")
            || line.contains("-make: ")
            || line.contains("-make[")
            || line.starts_with("make: ")
            || line.starts_with("make[")
            || line.ends_with(" generated.")
        {
            continue;
        }
        // What only follows from a linker error: the compiler driver's line, and GNU ld's.
        if line.contains(": error: linker command failed with exit code") || line.contains(": error: ld returned ") {
            continue;
        }
        if let Some(sym) = line.split_once("error: undefined symbol: ").map(|(_, s)| s) {
            failures.push(Failure::Link {
                name: symbol_name(sym)?,
            });
            open = None;
            continue;
        }
        if let Some(sym) = line.split_once("undefined reference to `").map(|(_, s)| s) {
            failures.push(Failure::Link {
                name: symbol_name(sym.strip_suffix('\'')?)?,
            });
            open = None;
            continue;
        }
        if line.contains(": warning: ") {
            // The notes that follow are the warning's.
            open = None;
            continue;
        }
        if let Some((_, note)) = line.split_once(": note: ") {
            // gcc names the function of a wrong argument only here (`initializing argument 1 of ‘int32_t
            // h_isnull(const void*)’`; `verification\v37_x17` on Linux), its error the types.
            if let (Some(i), Some(of)) = (open, note.trim_start().strip_prefix("initializing argument "))
                && let Some(signature) = of.split(['\'', '‘', '’']).nth(1)
                && let Some(function) = signature.split('(').next().and_then(|s| s.split_whitespace().last())
                && let Failure::Compile { name, .. } = &mut failures[i]
            {
                *name = function.trim_start_matches(['*', '&']).to_string();
                continue;
            }
            // clang's reason an overload was not taken (gcc's `candidate: `); other notes (where a guessed name is
            // declared) are left.
            if let (Some(i), true) = (open, note.starts_with("candidate"))
                && let Failure::Compile { message, .. } = &mut failures[i]
            {
                message.push_str("; ");
                message.push_str(note);
            }
            continue;
        }
        // Any other line that speaks of an error is read as clang's, or gives up (`llvm-rc` writes "Error in").
        if !line.to_ascii_lowercase().contains("error") {
            continue;
        }
        let at = files
            .iter()
            .find_map(|f| line.strip_prefix(f)?.strip_prefix(':').map(|rest| (*f, rest)));
        let (file, rest) = at?;
        let (line_no, rest) = rest.split_once(':')?;
        let (_col, rest) = rest.split_once(':')?;
        let message = rest.strip_prefix(" error: ")?;
        let message = message.split_once("; did you mean").map_or(message, |(m, _)| m);
        // clang's `'h_over'`; gcc's, in a UTF-8 locale, `‘h_over(NULL)’`, with the arguments.
        let name = message.split(['\'', '‘', '’']).nth(1)?;
        failures.push(Failure::Compile {
            file: file.to_string(),
            line: line_no.parse().ok()?,
            name: symbol_name(name)?,
            message: message.to_string(),
        });
        open = Some(failures.len() - 1);
    }
    (!failures.is_empty()).then_some(failures)
}

/// A linker's symbol without its parameters; `None` for an empty one.
fn symbol_name(sym: &str) -> Option<String> {
    let name = sym.split_once('(').map_or(sym, |(n, _)| n).trim();
    (!name.is_empty()).then(|| name.to_string())
}

/// The C name `c_name` is the name a toolchain message gives: the same, the last part after `::` (`hns::twice`), or
/// the name before a template's `<` (`h_max<int32_t>`).
fn names_c(c_name: &str, name: &str) -> bool {
    let c_name = c_name.trim();
    c_name == name
        || c_name.rsplit_once("::").is_some_and(|(_, last)| last == name)
        || c_name.split_once('<').is_some_and(|(base, _)| base.trim() == name)
}

/// The procedure of a block that is not `DYNAMIC` whose C function `name` is, the first declared.
fn library_proc(fe: &Frontend, name: &str) -> Option<ProcId> {
    let p = &fe.program;
    (0..p.procs.len())
        .map(|i| ProcId(qb64rust_base::to_u32(i)))
        .find(|&id| {
            p.proc(id).external.as_ref().is_some_and(|e| {
                p.libraries[e.library.0 as usize].kind != LibraryKind::Dynamic && names_c(&e.c_name, name)
            })
        })
}

/// The errors of the program in the toolchain's `output`, or `None` when one of its errors is not one this module
/// reads and matches with a library procedure.
pub fn diagnostics(fe: &Frontend, output: &str) -> Option<Vec<Diagnostic>> {
    let names: Vec<&str> = fe.map.files().map(|(_, f)| f.name.as_str()).collect();
    let mut out: Vec<Diagnostic> = Vec::new();
    for failure in read(output, &names)? {
        let (Failure::Compile { name, .. } | Failure::Link { name }) = &failure;
        let id = library_proc(fe, name)?;
        let symbol = fe
            .program
            .symbols
            .iter()
            .find_map(|(_, s)| (s.kind == SymbolKind::Proc(id)).then_some(s))?;
        let basic = qb64rust_base::show_bytes(fe.map.text(symbol.def));
        let c_name = &fe.program.proc(id).external.as_ref()?.c_name;
        let d = match failure {
            Failure::Compile {
                file, line, message, ..
            } => {
                let (fid, f) = fe.map.files().find(|(_, f)| f.name == file)?;
                let on_line = |s: &Span| s.file == fid && f.line_col(s.start).0 == line;
                let span = match symbol.refs.iter().find(|s| on_line(s)) {
                    Some(&s) => s,
                    None => {
                        let (start, end) = f.line_content(line.checked_sub(1)?)?;
                        Span::new(fid, start, end)
                    }
                };
                Diagnostic::error(
                    span,
                    format!(
                        "the C++ compiler rejects the call of `{basic}` (`{c_name}` of `DECLARE LIBRARY`): {message}"
                    ),
                )
            }
            Failure::Link { .. } => Diagnostic::error(
                symbol.def,
                format!(
                    "`{basic}` is defined in no library or header the program uses (the linker finds no `{c_name}`)"
                ),
            ),
        };
        if !out.iter().any(|o| o.span == d.span && o.message == d.message) {
            out.push(d);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A missing header or library of the system packages, as `g++` and GNU ld word it (spec `compiler/toolchain`,
    /// "A missing header"); a missing file of the program's own is none of them.
    #[test]
    fn missing_system_files() {
        let header = "In file included from internal/c/libqb.cpp:12:\n\
                      internal/c/libqb/include/gl.h:3:10: fatal error: GL/glu.h: No such file or directory\n\
                      compilation terminated.\n";
        assert_eq!(missing_system_file(header).as_deref(), Some("GL/glu.h"));
        let library = "/usr/bin/ld: cannot find -lasound: No such file or directory\n\
                       collect2: error: ld returned 1 exit status\n";
        assert_eq!(missing_system_file(library).as_deref(), Some("-lasound"));
        assert_eq!(
            missing_system_file("x.h:1:10: fatal error: mine.h: No such file or directory\n"),
            None
        );
        assert_eq!(missing_system_file("/usr/bin/ld: cannot find -lmine\n"), None);
    }

    /// `verification\v37_x30` as the clone's clang printed it (2026-10-20), the build folder's path made relative.
    const X30: &str = "\
mingw32-make: Entering directory 'libqb'
In file included from out/v37_x30.exe.qb64rust/c/qbx.cpp:1606:
In file included from out/v37_x30.exe.qb64rust/c/../temp/main.txt:1:
v37_x30.bas:6:46: error: use of undeclared identifier 'h_none'
    6 | qbs_set(tqbs,qbs_add(qbs_str((int32)(((int32)h_none(1)))),qbs_new_txt(\" \")));
      |                                              ^~~~~~
1 error generated.
mingw32-make: *** [Makefile:496: out/v37_x30.exe.qb64rust/c/qbx.o] Error 1
";

    const X31: &str = "\
v37_x31.bas:6:46: error: use of undeclared identifier 'H_L'; did you mean 'HKL'?
    6 | qbs_set(tqbs,qbs_add(qbs_str((int32)(((int32)H_L(1)))),qbs_new_txt(\" \")));
      |                                              ^~~
c_compiler/include/minwindef.h:211:19: note: 'HKL' declared here
  211 |   DECLARE_HANDLE (HKL);
c_compiler/include/winnt.h:443:88: note: expanded from macro 'DECLARE_HANDLE'
1 error generated.
";

    const X17: &str = "\
v37_x17.bas:6:46: error: no matching function for call to 'h_isnull'
    6 | qbs_set(tqbs,qbs_add(qbs_str((int32)(((int32)h_isnull(5ll)))),qbs_new_txt(\" \")));
      |                                              ^~~~~~~~
out/v37_lib/v37h.h:43:9: note: candidate function not viable: no known conversion from 'long long' to 'const void *' for 1st argument
   43 | int32_t h_isnull(const void *p) { return p == NULL; }
1 error generated.
";

    const X04: &str = "\
bin/c++.exe  -m64 qbx.o internal/c/libqb/src/error_handle.o -o program.exe
ld.lld: error: undefined symbol: f
>>> referenced by out/v37_x04.exe.qb64rust/c/qbx.o:(QBMAIN(void*))
clang-23: error: linker command failed with exit code 1 (use -v to see invocation)
mingw32-make: *** [Makefile:510: out/v37_x04.exe.qb64rust/program.exe] Error 1
";

    fn compile(file: &str, line: u32, name: &str, message: &str) -> Failure {
        Failure::Compile {
            file: file.into(),
            line,
            name: name.into(),
            message: message.into(),
        }
    }

    #[test]
    fn reads_the_measured_outputs() {
        assert_eq!(
            read(X30, &["v37_x30.bas"]),
            Some(vec![compile(
                "v37_x30.bas",
                6,
                "h_none",
                "use of undeclared identifier 'h_none'"
            )])
        );
        assert_eq!(
            read(X31, &["v37_x31.bas"]),
            Some(vec![compile(
                "v37_x31.bas",
                6,
                "H_L",
                "use of undeclared identifier 'H_L'"
            )])
        );
        assert_eq!(
            read(X17, &["v37_x17.bas"]),
            Some(vec![compile(
                "v37_x17.bas",
                6,
                "h_isnull",
                "no matching function for call to 'h_isnull'; candidate function not viable: no known conversion \
                 from 'long long' to 'const void *' for 1st argument"
            )])
        );
        assert_eq!(
            read(X04, &["v37_x04.bas"]),
            Some(vec![Failure::Link { name: "f".into() }])
        );
    }

    #[test]
    fn reads_other_forms_of_the_linker() {
        let lld = "ld.lld: error: undefined symbol: hns::twice(int)\n>>> referenced by qbx.o\n";
        assert_eq!(
            read(lld, &["p.bas"]),
            Some(vec![Failure::Link {
                name: "hns::twice".into()
            }])
        );
        let ld =
            "qbx.o:qbx.cpp:(.text+0x1a): undefined reference to `c_none'\ncollect2: error: ld returned 1 exit status\n";
        assert_eq!(
            read(ld, &["p.bas"]),
            Some(vec![Failure::Link { name: "c_none".into() }])
        );
    }

    /// The system toolchain's output on Linux (task 3.5 of `m7-linux`, `verification/v37_x04`): `make`'s own lines
    /// and the commands it echoes, which start with `/usr/bin/g++` and name `error_handle.o`, are left.
    #[test]
    fn reads_the_linux_toolchain() {
        let out = "make: Entering directory 'libqb'\nOS: lnx\n\
                   /usr/bin/g++  -no-pie -m64 out/c/qbx.cpp -c -o out/c/qbx.o\n\
                   /usr/bin/g++  -no-pie ./internal/c/libqb/src/error_handle.o out/c/qbx.o -o 'out/program'\n\
                   make: Leaving directory 'libqb'\n\
                   /usr/bin/ld: out/c/qbx.o: in function `QBMAIN(void*)':\n\
                   qbx.cpp:(.text+0x293f): undefined reference to `f'\n\
                   collect2: error: ld returned 1 exit status\n\
                   make: *** [Makefile:517: exe] Error 1\n";
        assert_eq!(read(out, &["p.bas"]), Some(vec![Failure::Link { name: "f".into() }]));
        // gcc's quotes in a UTF-8 locale, and the arguments it writes after the name (`v37_x60`).
        let out = "p.bas: In function ‘void QBMAIN(void*)’:\n\
                   p.bas:6:55: error: call of overloaded ‘h_over(NULL)’ is ambiguous\n\
                   v37h.h:66:9: note: candidate: ‘int32_t h_over(int32_t)’\n";
        assert_eq!(
            read(out, &["p.bas"]),
            Some(vec![compile(
                "p.bas",
                6,
                "h_over",
                "call of overloaded ‘h_over(NULL)’ is ambiguous; candidate: ‘int32_t h_over(int32_t)’"
            )])
        );
        // A wrong argument: the function is named by the note that follows (`v37_x17`).
        let out = "p.bas:6:55: error: invalid conversion from ‘long long int’ to ‘const void*’ [-fpermissive]\n\
                   v37h.h:43:30: note:   initializing argument 1 of ‘int32_t h_isnull(const void*)’\n";
        assert_eq!(
            read(out, &["p.bas"]),
            Some(vec![compile(
                "p.bas",
                6,
                "h_isnull",
                "invalid conversion from ‘long long int’ to ‘const void*’ [-fpermissive]"
            )])
        );
    }

    /// An error this module does not read gives up, whatever else the output holds.
    #[test]
    fn gives_up_on_other_errors() {
        let in_cpp = "out/c/qbx.cpp:12:3: error: use of undeclared identifier 'x'\n";
        assert_eq!(read(in_cpp, &["p.bas"]), None);
        let in_header = format!("{X30}out/v37_lib/v37h.h:3:1: error: unknown type name 'foo'\n");
        assert_eq!(read(&in_header, &["v37_x30.bas"]), None);
        let rc = "llvm-rc: Error in ICON statement (ID 0): \nIncorrect icon/cursor Reserved field; should be 0.\n";
        assert_eq!(read(rc, &["p.bas"]), None);
        assert_eq!(
            read(&format!("{X30}{rc}"), &["v37_x30.bas"]),
            None,
            "beside a library error"
        );
        let fatal = "p.bas:3:10: fatal error: 'x.h' file not found\n";
        assert_eq!(read(fatal, &["p.bas"]), None);
        let unnamed = "p.bas:3:10: error: expected expression\n";
        assert_eq!(read(unnamed, &["p.bas"]), None);
    }

    /// A note after a warning is the warning's, not the error's before it.
    #[test]
    fn notes_of_a_warning_are_left() {
        let out = "p.bas:2:5: error: no matching function for call to 'g'\n\
                   p.bas:3:5: warning: unused variable 'v'\n\
                   h.h:1:1: note: candidate function not viable: requires 2 arguments\n";
        assert_eq!(
            read(out, &["p.bas"]),
            Some(vec![compile("p.bas", 2, "g", "no matching function for call to 'g'")])
        );
    }

    /// The longest file name is taken: `a.bas.bi` is not `a.bas`.
    #[test]
    fn file_names_by_their_longest_match() {
        let out = "a.bas.bi:2:5: error: use of undeclared identifier 'g'\n";
        assert_eq!(
            read(out, &["a.bas", "a.bas.bi"]),
            Some(vec![compile("a.bas.bi", 2, "g", "use of undeclared identifier 'g'")])
        );
    }

    /// The program of `verification\v37_x30` with a `DYNAMIC` block and a SUB of the program beside it.
    const PROGRAM: &str = "$CONSOLE:ONLY
DECLARE LIBRARY \"v37_lib/v37h\"
    FUNCTION h_none& (BYVAL v AS LONG)
END DECLARE
DECLARE DYNAMIC LIBRARY \"kernel32\"
    FUNCTION d_none& ()
END DECLARE
PRINT h_none&(1)
PRINT d_none&
mine
SUB mine
END SUB
";

    fn rendered(fe: &Frontend, output: &str) -> Option<Vec<String>> {
        let list = diagnostics(fe, output)?;
        Some(list.iter().map(|d| d.render(&fe.map)).collect())
    }

    /// Each failure stands at the call on clang's line, or at the declaration for the linker; a name of a `DYNAMIC`
    /// block or of the program's own SUB gives up.
    #[test]
    fn errors_at_the_call_and_at_the_declaration() {
        let fe = crate::frontend("p.bas", PROGRAM.as_bytes().to_vec());
        assert!(!fe.has_errors(), "{}", fe.render_diagnostics());
        let compile = "p.bas:8:46: error: use of undeclared identifier 'h_none'\n";
        assert_eq!(
            rendered(&fe, compile),
            Some(vec![
                "p.bas:8:7: error: the C++ compiler rejects the call of `h_none&` (`h_none` of `DECLARE LIBRARY`): \
                 use of undeclared identifier 'h_none'"
                    .to_string()
            ])
        );
        let link = "ld.lld: error: undefined symbol: h_none\n";
        assert_eq!(
            rendered(&fe, link),
            Some(vec![
                "p.bas:3:14: error: `h_none&` is defined in no library or header the program uses (the linker finds \
                 no `h_none`)"
                    .to_string()
            ])
        );
        assert_eq!(rendered(&fe, "ld.lld: error: undefined symbol: d_none\n"), None);
        assert_eq!(rendered(&fe, "ld.lld: error: undefined symbol: MINE\n"), None);
        assert_eq!(
            rendered(&fe, "p.bas:10:3: error: use of undeclared identifier 'mine'\n"),
            None
        );
    }

    #[test]
    fn c_names() {
        assert!(names_c("h_none", "h_none"));
        assert!(!names_c("h_l", "H_L"));
        assert!(names_c("hns::twice", "twice"));
        assert!(names_c("hns::twice", "hns::twice"));
        assert!(names_c("h_max<int32_t>", "h_max"));
        assert!(!names_c("h_maxi", "h_max"));
    }
}
