//! The wasm target's rejections, one test per scenario of spec `compiler/wasm-target` ("Console programs only",
//! "What the target does not have is rejected", "Pointer-sized types wait for their width"), each also checked
//! for Windows and Linux, where nothing is reported (design D5 of `wasm-console`).

use qb64rust_base::Platform;
use qb64rust_base::fixes::Fixes;
use qb64rust_driver::frontend_at;

/// The rendered diagnostics of `source` checked for `platform`, with no file read.
fn check(platform: Platform, source: &str) -> String {
    let fe = frontend_at(
        platform,
        "p.bas",
        source.as_bytes().to_vec(),
        |_| Box::new(qb64rust_syntax::NoLoader),
        None,
        Fixes::OFF,
    );
    fe.render_diagnostics()
}

/// `source` gives exactly the errors `expected` on the wasm target, each as `<line>: <message>`, and none on the
/// two desktop platforms.
fn rejected(source: &str, expected: &[&str]) {
    let out = check(Platform::Wasm, source);
    let errors: Vec<&str> = out.lines().filter(|l| l.contains("error")).collect();
    assert_eq!(errors.len(), expected.len(), "{out}");
    for want in expected {
        let (line, msg) = want.split_once(": ").unwrap();
        assert!(
            errors
                .iter()
                .any(|e| e.starts_with(&format!("p.bas:{line}:")) && e.contains(msg)),
            "{want} not in\n{out}"
        );
    }
    for host in [Platform::Windows, Platform::Linux] {
        let out = check(host, source);
        assert!(!out.contains("wasm"), "{host:?}:\n{out}");
    }
}

/// "Console programs only", scenario "A window program".
#[test]
fn a_window_program() {
    rejected(
        "PRINT \"hi\"\nSLEEP\n",
        &["1: the wasm target builds only `$CONSOLE:ONLY` programs so far"],
    );
    rejected("$CONSOLE\nPRINT \"hi\"\n", &["1: builds only `$CONSOLE:ONLY` programs"]);
    rejected("$CONSOLE:ONLY\nPRINT \"hi\"\n", &[]);
}

/// "What the target does not have is rejected", scenarios "SHELL" and "The same program for the host": the error at
/// the word's line.
#[test]
fn shell() {
    rejected(
        "$CONSOLE:ONLY\nPRINT 1\nPRINT 2\nSHELL \"dir\"\n",
        &["4: `SHELL` is not available on the wasm target"],
    );
    rejected(
        "$CONSOLE:ONLY\nSHELL _HIDE \"dir\"\nr&& = SHELL(\"dir\")\nr&& = _SHELLHIDE(\"dir\")\n",
        &[
            "2: `SHELL` is not available on the wasm target",
            "3: `SHELL` is not available on the wasm target",
            "4: `_SHELLHIDE` is not available on the wasm target",
        ],
    );
}

/// The other words of the table: `RUN` with a file (not `RUN` alone), `CHAIN`, the network functions,
/// `DECLARE DYNAMIC LIBRARY`.
#[test]
fn the_other_words() {
    rejected(
        "$CONSOLE:ONLY\nIF 0 THEN RUN\nRUN \"other.bas\"\n",
        &["3: `RUN` is not available on the wasm target"],
    );
    rejected(
        "$CONSOLE:ONLY\nCHAIN \"other\"\n",
        &["2: `CHAIN` is not available on the wasm target"],
    );
    rejected(
        "$CONSOLE:ONLY\nh = _OPENHOST(\"TCP/IP:8080\")\nc = _OPENCLIENT(\"TCP/IP:80:localhost\")\nx = _OPENCONNECTION(h)\n",
        &[
            "2: `_OPENHOST` is not available on the wasm target",
            "3: `_OPENCLIENT` is not available on the wasm target",
            "4: `_OPENCONNECTION` is not available on the wasm target",
        ],
    );
    rejected(
        "$CONSOLE:ONLY\nDECLARE DYNAMIC LIBRARY \"kernel32\"\n    FUNCTION GetTickCount& ()\nEND DECLARE\n",
        &["2: `DECLARE DYNAMIC LIBRARY` is not available on the wasm target"],
    );
    // `$DEBUG`, which brings in the network part through QB64pe's `afterlastline.bm`, and the clipboard.
    rejected(
        "$CONSOLE:ONLY\n$DEBUG\nPRINT 1\n",
        &["2: `$DEBUG` is not available on the wasm target"],
    );
    rejected(
        "$CONSOLE:ONLY\n_CLIPBOARD$ = \"a\"\nPRINT _CLIPBOARD$\n",
        &[
            "2: `_CLIPBOARD$` is not available on the wasm target",
            "3: `_CLIPBOARD$` is not available on the wasm target",
        ],
    );
}

/// "Parts of libqb the target does not build yet": one program for each part of `WASM_PARTS_NOT_YET`, by the
/// built-in table's `dependency` (the whole-family run failed in C++ for each: `verification\v29_x01_sound_two`,
/// `v47_b_devices`).
#[test]
fn parts_not_built_yet() {
    rejected(
        "$CONSOLE:ONLY\nSOUND 440, 1\nPLAY \"c\"\nh& = _SNDOPEN(\"a.wav\")\n",
        &[
            "2: `SOUND` is not available on the wasm target yet: libqb's audio part is not built for it",
            "3: `PLAY` is not available on the wasm target yet: libqb's audio part is not built for it",
            "4: `_SNDOPEN` is not available on the wasm target yet: libqb's audio part is not built for it",
        ],
    );
    rejected(
        "$CONSOLE:ONLY\nPRINT STICK(0)\nON STRIG(0) GOSUB h\nPRINT _DEVICES\nEND\nh: RETURN\n",
        &[
            "2: `STICK` is not available on the wasm target yet: libqb's device-input part is not built for it",
            "3: `ON STRIG` is not available on the wasm target yet: libqb's device-input part is not built for it",
            "4: `_DEVICES` is not available on the wasm target yet: libqb's device-input part is not built for it",
        ],
    );
    rejected(
        "$CONSOLE:ONLY\ni& = _CLIPBOARDIMAGE\n",
        &[
            "2: `_CLIPBOARDIMAGE` is not available on the wasm target yet: libqb's screen-image part is not built for it",
        ],
    );
}

/// "Pointer-sized types wait for their width", scenarios "A variable of the type" and "A built-in that returns one".
#[test]
fn pointer_sized_types() {
    rejected(
        "$CONSOLE:ONLY\nDIM p AS _OFFSET\n",
        &["2: `_OFFSET` is not available on the wasm target yet"],
    );
    rejected(
        "$CONSOLE:ONLY\nDIM u AS _UNSIGNED _OFFSET\nq%& = 1\n",
        &[
            "2: `_UNSIGNED _OFFSET` is not available on the wasm target yet",
            "3: `_OFFSET` is not available on the wasm target yet",
        ],
    );
    rejected(
        "$CONSOLE:ONLY\nDIM m AS _MEM\nm = _MEMNEW(16)\n",
        &[
            "2: `_MEM` is not available on the wasm target yet",
            "3: `_MEMNEW` is not available on the wasm target yet",
        ],
    );
    // A member, a parameter, and a built-in that gives an `_OFFSET` with no variable of the type.
    rejected(
        "$CONSOLE:ONLY\nTYPE t\n    o AS _OFFSET\nEND TYPE\nPRINT _WINDOWHANDLE\nSUB s (x AS _OFFSET)\nEND SUB\n",
        &[
            "3: `_OFFSET` is not available on the wasm target yet",
            "5: `_WINDOWHANDLE` is not available on the wasm target yet",
            "6: `_OFFSET` is not available on the wasm target yet",
        ],
    );
    // Values of the type that no declaration gives: `_CAST`, `_CV`, and a library function named with `~%&`.
    // Each was built by the first whole-family run, and failed in C++ (`verification\v49_a_cast`, `v32_c_mk_cv`,
    // `v37_a_noname`).
    rejected(
        "$CONSOLE:ONLY\nf! = 2\nPRINT _CAST(_OFFSET, f!)\nPRINT _CV(_OFFSET, \"12345678\")\n\
         DECLARE LIBRARY\n    FUNCTION strlen~%& (s AS STRING)\nEND DECLARE\nPRINT strlen~%&(\"four\")\n",
        &[
            "3: `_OFFSET` is not available on the wasm target yet",
            "4: `_OFFSET` is not available on the wasm target yet",
            "8: `_UNSIGNED _OFFSET` is not available on the wasm target yet",
        ],
    );
    // Member arrays of variable-length strings, fixed and `_DYNAMIC`, whose elements are pointers. A single `STRING`
    // member, a numeric member array and a numeric `_DYNAMIC` member are built.
    rejected(
        "$CONSOLE:ONLY\n$UNSTABLE:TYPEFIELDS\nTYPE t\n    s AS STRING\n    n(1 TO 3) AS LONG\n    a(0 TO 1) AS STRING\n\
         _DYNAMIC d(1 TO 2) AS LONG\n    _DYNAMIC e(1 TO 2) AS STRING\nEND TYPE\nDIM v AS t\n",
        &[
            "6: `STRING` is not available on the wasm target yet: a member array of variable-length strings",
            "8: `STRING` is not available on the wasm target yet: a member array of variable-length strings",
        ],
    );
}

/// "The C++ is the same as for Linux", scenario "Same fragments": three console programs without `DECLARE LIBRARY`
/// give the wasm target Linux's fragments, byte for byte.
#[test]
fn same_fragments_as_linux() {
    let programs = [
        // Numbers of each type, strings, a SUB and a FUNCTION.
        "$CONSOLE:ONLY\nDIM b AS _BYTE, i AS INTEGER, l AS LONG, q AS _INTEGER64\nDIM s AS SINGLE, d AS DOUBLE, f AS _FLOAT\n\
         b = -5: i = 300: l = 70000: q = 2 ^ 40: s = 1 / 3: d = 1 / 3: f = 1 / 3\nPRINT b; i; l; q, s; d; f\n\
         a$ = \"hello\" + STR$(l): PRINT LEFT$(a$, 3); UCASE$(a$); LEN(a$)\nshow 2\nPRINT twice(21)\nSYSTEM 3\n\
         SUB show (n AS INTEGER)\nPRINT \"n=\"; n\nEND SUB\nFUNCTION twice& (x AS LONG)\ntwice& = 2 * x\nEND FUNCTION\n",
        // A file written and read, standard input.
        "$CONSOLE:ONLY\nOPEN \"t.txt\" FOR OUTPUT AS #1\nPRINT #1, \"one\"; 2\nCLOSE #1\nOPEN \"t.txt\" FOR INPUT AS #1\n\
         LINE INPUT #1, l$\nCLOSE #1\nPRINT l$\nINPUT \"name\"; n$\nPRINT \"hi \"; n$\nKILL \"t.txt\"\n",
        // ON ERROR, a run-time error, TIMER and ON TIMER.
        "$CONSOLE:ONLY\nON ERROR GOTO handler\nx = 1 / 0\nPRINT \"after\"\nt = TIMER\nON TIMER(1) GOSUB tick\nTIMER ON\n\
         _LIMIT 10\nERROR 5\nEND\nhandler:\nPRINT \"error\"; ERR\nRESUME NEXT\ntick:\nRETURN\n",
    ];
    for (i, src) in programs.iter().enumerate() {
        let fragments = |platform: Platform| {
            let fe = frontend_at(
                platform,
                "p.bas",
                src.as_bytes().to_vec(),
                |_| Box::new(qb64rust_syntax::NoLoader),
                None,
                Fixes::OFF,
            );
            assert_eq!(
                fe.diagnostics.error_count(),
                0,
                "{platform:?}: {}",
                fe.render_diagnostics()
            );
            let resolved = qb64rust_driver::Resolved {
                platform,
                ..qb64rust_driver::Resolved::default()
            };
            qb64rust_driver::emit(&fe, &resolved).files
        };
        let (linux, wasm) = (fragments(Platform::Linux), fragments(Platform::Wasm));
        assert_eq!(linux.len(), wasm.len());
        for ((name, l), (_, w)) in linux.iter().zip(&wasm) {
            assert!(l == w, "program {i}: {name} differs");
        }
    }
}

/// "The target's name in a program", scenario "A branch for the target": the branch compiled is the target's, and
/// a `$IF LINUX` branch is not compiled on the wasm target.
#[test]
fn a_branch_for_the_target() {
    let src =
        "$CONSOLE:ONLY\n$IF WASM THEN\nSHELL \"x\"\n$ELSE\nPRINT 1\n$END IF\n$IF LINUX THEN\nSHELL \"y\"\n$END IF\n";
    let out = check(Platform::Wasm, src);
    let errors: Vec<&str> = out.lines().filter(|l| l.contains("error")).collect();
    assert_eq!(errors.len(), 1, "{out}");
    assert!(errors[0].starts_with("p.bas:3:"), "{out}");
}
