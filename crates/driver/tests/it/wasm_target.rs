//! The wasm target's rejections, one test per scenario of spec `compiler/wasm-target` ("Console programs only",
//! "What the target does not have is rejected"), each also checked for Windows and Linux, where nothing is
//! reported (design D5 of `wasm-console`); and the sizes of its 4-byte pointer in the C++ ("A pointer is 4 bytes on
//! this target", "The C++ is the same as for Linux"; OpenSpec change `wasm-pointer-width`).

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

/// "A window program runs in a page" (OpenSpec change `wasm-window`): a program without `$CONSOLE:ONLY` is taken.
#[test]
fn a_window_program() {
    rejected("PRINT \"hi\"\nSLEEP\n", &[]);
    rejected("$CONSOLE\nPRINT \"hi\"\n", &[]);
    rejected("$CONSOLE:ONLY\nPRINT \"hi\"\n", &[]);
}

/// "What the target does not have is rejected": the words a page has no form of, at their lines; sound only in a
/// `$CONSOLE:ONLY` program, whose Node has no Web Audio (OpenSpec change `wasm-window`).
#[test]
fn window_words() {
    rejected(
        "SCREEN 12\nh& = _WINDOWHANDLE\n_SCREENCLICK 1, 1\n",
        &[
            "2: `_WINDOWHANDLE` is not available on the wasm target",
            "3: `_SCREENCLICK` is not available on the wasm target",
        ],
    );
    rejected(
        "a$ = _INPUTBOX$(\"t\", \"m\", \"\")\n_MESSAGEBOX \"t\", \"m\"\n",
        &[
            "1: `_INPUTBOX$` is not available on the wasm target",
            "2: `_MESSAGEBOX` is not available on the wasm target",
        ],
    );
    rejected(
        "SCREEN 12\nSUB _GL\nEND SUB\n",
        &["2: `SUB _GL` is not available on the wasm target"],
    );
    // A `$CONSOLE:ONLY` program keeps them, as the console target always had: it never shows a window.
    rejected(
        "$CONSOLE:ONLY\nh& = _WINDOWHANDLE\n_MESSAGEBOX \"t\", \"m\"\nSUB _GL\nEND SUB\n",
        &[],
    );
    rejected("SOUND 440, 1\nPLAY \"C\"\n", &[]);
    rejected(
        "$CONSOLE:ONLY\nSOUND 440, 1\n",
        &["2: `SOUND` is not available on the wasm target yet: libqb's audio part is not built for it"],
    );
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

/// The C++ fragments of `src` for `platform`, which must check without an error.
fn fragments(platform: Platform, src: &str) -> Vec<(String, String)> {
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
    qb64rust_driver::emit(&fe, &resolved)
        .files
        .into_iter()
        .map(|(name, text)| (name.to_string(), text))
        .collect()
}

/// The two size checks at the start of `global.txt` (design D4 of `wasm-pointer-width`), which hold the pointer's
/// size and so differ between the wasm target and the others.
fn is_size_check(line: &str) -> bool {
    line.starts_with("static_assert(sizeof(ptrszint)==") || line.starts_with("static_assert(sizeof(mem_block)==")
}

/// `text` with each run of digits written as `#`: two lines that are equal so differ only in numbers.
fn without_numbers(text: &str) -> String {
    let mut out = String::new();
    let mut in_number = false;
    for c in text.chars() {
        if c.is_ascii_digit() {
            if !in_number {
                out.push('#');
            }
            in_number = true;
        } else {
            out.push(c);
            in_number = false;
        }
    }
    out
}

/// "A pointer is 4 bytes on this target" and "Layouts with pointers on this target": what the first step of the
/// wasm target rejected for a pointer's size is built, with the sizes of a 4-byte pointer in its C++. An audio
/// word that takes an `_OFFSET` is still rejected, for its part of libqb.
#[test]
fn pointer_sized_types() {
    for src in [
        "$CONSOLE:ONLY\nDIM p AS _OFFSET\nDIM u AS _UNSIGNED _OFFSET\nq%& = 1\nPRINT LEN(p)\n",
        "$CONSOLE:ONLY\nDIM m AS _MEM\nm = _MEMNEW(16)\nPRINT LEN(m)\n_MEMFREE m\n",
        "$CONSOLE:ONLY\nTYPE t\n    o AS _OFFSET\nEND TYPE\nDIM v AS t\ns v.o\nSUB s (x AS _OFFSET)\nEND SUB\n",
        "$CONSOLE:ONLY\nf! = 2\nPRINT _CAST(_OFFSET, f!)\nPRINT _CV(_OFFSET, _MK$(_OFFSET, 5))\n",
        "$CONSOLE:ONLY\n$UNSTABLE:TYPEFIELDS\nTYPE t\n    s AS STRING\n    n(1 TO 3) AS LONG\n    a(0 TO 1) AS STRING\n\
         _DYNAMIC d(1 TO 2) AS LONG\n    _DYNAMIC e(1 TO 2) AS STRING\nEND TYPE\nDIM v AS t\n",
    ] {
        rejected(src, &[]);
    }
    rejected(
        "$CONSOLE:ONLY\nDIM m AS _MEM\nh& = 1\nm = _MEMSOUND(h&, 0)\n",
        &["4: `_MEMSOUND` is not available on the wasm target yet"],
    );
    // An array's storage follows the pointer's size: an `_INTEGER64` array is an `_OFFSET` array's storage at 8
    // bytes only, and a `LONG` array at 4 bytes only (`verification\v24_x179_param_offset_to_integer64`).
    let arrays = |ty: &str| format!("$CONSOLE:ONLY\nDIM q(3) AS {ty}\nf q()\nSUB f (a() AS _OFFSET)\nEND SUB\n");
    let wide = check(Platform::Wasm, &arrays("_INTEGER64"));
    assert!(
        wide.contains("cannot be passed to an array parameter of type _OFFSET"),
        "{wide}"
    );
    assert_eq!(check(Platform::Wasm, &arrays("LONG")), "");
    assert_eq!(check(Platform::Linux, &arrays("_INTEGER64")), "");
    let narrow = check(Platform::Linux, &arrays("LONG"));
    assert!(
        narrow.contains("cannot be passed to an array parameter of type _OFFSET"),
        "{narrow}"
    );
    // The size checks carry the platform's sizes.
    let src = "$CONSOLE:ONLY\nPRINT 1\n";
    let global = |platform| {
        let files = fragments(platform, src);
        files.into_iter().find(|(n, _)| n == "global.txt").unwrap().1
    };
    let wasm = global(Platform::Wasm);
    assert!(wasm.contains("static_assert(sizeof(ptrszint)==4,"), "{wasm}");
    assert!(wasm.contains("static_assert(sizeof(mem_block)==40,"), "{wasm}");
    for host in [Platform::Windows, Platform::Linux] {
        let text = global(host);
        assert!(text.contains("static_assert(sizeof(ptrszint)==8,"), "{text}");
        assert!(text.contains("static_assert(sizeof(mem_block)==56,"), "{text}");
    }
    // `LEN` of the spec's `TYPE` of a LONG, an `_OFFSET` and a `STRING * 2`, of an `_OFFSET` and of a `_MEM`: the
    // checker folds each to a number, which the C++ prints.
    let sizes = "$CONSOLE:ONLY\nTYPE t\n    a AS LONG\n    p AS _OFFSET\n    s AS STRING * 2\nEND TYPE\nDIM v AS t\n\
                 DIM p AS _OFFSET, m AS _MEM\nx& = LEN(v) * 1000000 + LEN(p) * 1000 + LEN(m)\n";
    let all = |platform| {
        fragments(platform, sizes)
            .into_iter()
            .map(|(_, t)| t)
            .collect::<String>()
    };
    assert!(all(Platform::Wasm).contains("10004040"), "LEN on wasm: 10, 4 and 40");
    assert!(all(Platform::Linux).contains("14008056"), "LEN on Linux: 14, 8 and 56");
}

/// "The C++ is the same as for Linux", scenario "Same fragments": three console programs without `DECLARE LIBRARY`
/// give the wasm target Linux's fragments, byte for byte, apart from the size checks.
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
        let (linux, wasm) = (fragments(Platform::Linux, src), fragments(Platform::Wasm, src));
        assert_eq!(linux.len(), wasm.len());
        for ((name, l), (_, w)) in linux.iter().zip(&wasm) {
            let rest = |text: &str| -> Vec<String> {
                text.lines()
                    .filter(|line| !is_size_check(line))
                    .map(str::to_string)
                    .collect()
            };
            assert!(rest(l) == rest(w), "program {i}: {name} differs");
            let checks = |text: &str| text.lines().filter(|line| is_size_check(line)).count();
            assert_eq!(checks(l), checks(w), "program {i}: {name}");
        }
    }
}

/// "The C++ is the same as for Linux", scenario "A program with an _OFFSET": the fragments of a program with an
/// `_OFFSET`, a `_MEM`, a `STRING` member and an array of a `TYPE` that holds a pointer differ from Linux's only
/// in numbers, and they do differ.
#[test]
fn fragments_with_pointers_differ_only_in_sizes() {
    let src = "$CONSOLE:ONLY\nTYPE t\n    a AS LONG\n    p AS _OFFSET\n    s AS STRING\n    b AS INTEGER\nEND TYPE\n\
               DIM v AS t, items(3) AS t, m AS _MEM, blocks(2) AS _MEM\nDIM p AS _OFFSET\n\
               v.p = 5: v.b = 2: items(1) = v: items(2).b = 7\nm = _MEM(v.a): blocks(1) = m\n\
               p = m.OFFSET + 4\nPRINT LEN(p); blocks(1).SIZE; items(2).b; HEX$(p)\n";
    let (linux, wasm) = (fragments(Platform::Linux, src), fragments(Platform::Wasm, src));
    assert_eq!(linux.len(), wasm.len());
    let mut differing = 0;
    for ((name, l), (_, w)) in linux.iter().zip(&wasm) {
        assert_eq!(l.lines().count(), w.lines().count(), "{name}");
        for (a, b) in l.lines().zip(w.lines()) {
            if a != b {
                differing += 1;
                assert_eq!(without_numbers(a), without_numbers(b), "{name}");
            }
        }
    }
    assert!(differing > 2, "the sizes of a 4-byte pointer are in the C++");
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
