# 47. Review: the next big wins, and the gaps against other compilers

Written 2026-11-09, in answer to the user's question "What are the next big wins? What are the big gaps in
what we offer compared to QB64pe, to other modern BASIC languages, to FreeBASIC, and to other modern languages
generally?". It is a review, so it stays active until the user has answered it (`CLAUDE.md` working rule 7).
Nothing here is decided.

**Sources.** The state of the project comes from `STATUS.md`, `ROADMAP.md`, `SOMEDAY.md`, `DIVERGENCES-QB64PE.md`,
`UPSTREAM.md`, `MANUAL.md`, `vscode\README.md`, `crates\README.md` and the merged specs under `openspec\specs\`.
What FreeBASIC, the other BASICs and the modern languages offer comes from Claude's general knowledge, not from a
search or from reading their sources (FreeBASIC's clone is reference only, `CLAUDE.md` working rule 3). Study
`43` is the earlier answer on modern languages' tools; its lints are built since (`DECISIONS.md` 2026-11-06).

**Written the same day, by another session:** `study\44` (QB64 projects on GitHub, whose findings became the
OpenSpec change `community-programs` and nine fixes measured by `verification\v70`). Where this review and
study 44 overlap, study 44 has the measured figures and the detail. This one adds the comparison with QB64pe and
FreeBASIC, and the order of the wins.

Two measurements made for this review, on the host, release build at `ae2b5a2b`:

| Measurement | Result |
|---|---|
| `qb64rust -z` of a two-line program (check and C++ only) | 0.05 s |
| `qb64rust -x` of the same program, libqb's objects already built | 2.0 to 2.6 s |
| The executable of that program | 2.5 MB |

## 1. Where the project stands

- Language parity with QB64pe 4.7.0 is in effect complete: every corpus program the old compiler accepts
  compiles, upstream's tests pass at 401 of 404, and 3,890 verification programs are on `pass.list`. What is
  left is edge forms, each a row of `SOMEDAY.md` "Deferred QB64pe features".
- The tooling goes beyond QB64pe's IDE: a language server, a formatter, a debugger by BASIC line, unit tests in
  BASIC, statement coverage, value tracing, five lints and the fixes switch.
- Windows x64 and Linux x64 are released (`vscode-v0.3.0`). macOS is postponed. No milestone is open.
- The compiler is a Rust front end and C++ emitter over a copy of libqb. Everything a user sees at run time is
  libqb's, and the runtime ABI is QB64pe's.

## 2. The next big wins, in the recommended order

1. **Reach: make it findable and installable.** The purpose is "a QB64 for anyone", and today no one else can
   find or install it without help. The `release` group of `SOMEDAY.md`: the check on a machine with no QB64pe
   (P2), the Marketplace and Open VSX listing (P3), one install with no prerequisites (P3), the user page
   "Differences from QB64pe" (P3), and the F1 link to the wiki (`f1-help`, P2).
2. **Real programs, and what the ports found.** The suites prove "the same as QB64pe"; only Civil War Strategy
   proves "programs people wrote work". Two changes decided on 2026-11-09 cover most of this: `community-programs`
   (a front-end check of pinned community clones) and `showcase` (about 30 programs written here, each a recorded
   corpus program). The `real-programs` rows of `SOMEDAY.md` (recordings of `tests\qbasic_testcases`, more
   programs with a license) remain beside them. The ports already named the fixes that matter:
   `CALL INTERRUPT` (the World War 2 port's `MOUSE.BAS`), a clear message for a QuickBASIC 4.5 binary file
   (`qb45-binary`, P2), a warning for a bare `CURDIR$` (P2) and for a `GOSUB` routine that leaves its SUB early
   (`gosub-stack`, P2).
3. **Editor features whose data already exists.** `$INCLUDE` links and workspace symbols (`ls-navigation`, P2),
   signature help, semantic tokens, snippets, layout as you type, comments above a SUB shown in hover, quick
   fixes attached to diagnostics.
4. **Fuzz the parser and the checker.** The language server reads half-typed code all day, and only the corpus
   reaches the checker. "No panic, exact round trip" is the lexer and parser's property (`SOMEDAY.md`, P4).
5. **Measure a precompiled header for libqb's headers.** A check takes 0.05 s and a build 2 s or more, nearly all
   of it the C++ compiler on libqb's headers (`SOMEDAY.md`, P4).

## 3. Gaps against QB64pe

- **Platforms.** No macOS, no 32-bit Windows or Linux, no ARM. The runtime ABI's reliance on x86 memory order
  gates ARM and Apple silicon (`SOMEDAY.md`, "Redesign the runtime ABI").
- **One download.** QB64pe is one zip with its IDE, compiler, toolchain and offline help inside. Here it is VS
  Code, then a `.vsix` installed by hand, then a 190 MB toolchain download on Windows or the distribution's
  packages and `lldb-20` on Linux.
- **Help.** No help text at all: hover shows a signature. The wiki's license is not asked about
  (`STATUS.md`, "Queued for later milestones").
- **Following upstream.** `..\QB64pe-latest` is 38 commits past the reference `16f629784e`: sub-millisecond
  `TIMER` (pull request 798), window content scaling, mouse fixes, the s-buffer handle fix. Only the audio lock
  (D-062) and `_BIT` parameters were taken. There is no routine for reading upstream's new commits.
- **Giving back.** `UPSTREAM.md` holds 13 QB64pe bugs with fixes, none sent.
- **Language leftovers.** `$DEBUG` is a no-op (D-017), `CHAIN` to a program that exists is untested, and the
  member-array, `VARPTR` and `_MEM` edge forms of `SOMEDAY.md` are still "not supported yet".
- **IDE odds and ends.** The ASCII chart, the RGB mixer, export as HTML or forum markup, converting a QB 4.5
  binary file, purge of build files, run without keeping the executable.

## 4. Gaps against FreeBASIC

From general knowledge of FreeBASIC, not from its sources.

- **Native code generation.** FreeBASIC emits assembly or C and needs no 460 MB toolchain install; a hello world
  is tens of kilobytes. Ours is 2.5 MB and builds through clang or g++.
- **Separate compilation and libraries.** Several modules compiled and linked, static and shared library output,
  bindings to many C libraries. We have one main module plus `$INCLUDE`, and `DECLARE LIBRARY` for C.
- **Language.** Namespaces, types with methods, constructors and destructors, operator overloading, pointers,
  function pointers, `BYVAL` and default parameters for BASIC procedures, threads, a C-style preprocessor with
  macros, inline assembly, a `-lang qb` dialect beside its own.
- **Targets.** DOS, 32-bit, ARM, and historically more.
- **Where we lead.** QuickBASIC and QB64 fidelity measured against both compilers, built-in graphics, sound,
  images and fonts, and the editor, debugger, tests, coverage and lints.

## 5. Gaps against other modern BASICs

VB.NET, Xojo, Gambas, PureBasic, twinBASIC, B4X, BlitzMax, SmallBASIC and QBJS, from general knowledge.

- **A web target.** QBJS runs QB64-style programs in the browser, which is how hobby games get shared and
  shown now. We produce a desktop executable only.
- **GUI forms and a designer.** VB.NET, Xojo, Gambas and twinBASIC build windowed applications from forms. QB64
  programs draw their own screens.
- **Classes, collections, exceptions.** Methods on types, lists and dictionaries, `TRY`/`CATCH`.
- **Unicode strings.** Ours are CP437 bytes, as QB64's.
- **Mobile.** B4X and Xojo build for phones. Out of scope here, noted for completeness.

## 6. Gaps against modern languages generally

The tools of study `43`, less what is built since:

- **Editor:** quick fixes, inlay hints for implicit types and parameter names, doc comments in hover and
  signature help, semantic highlighting.
- **Workflow:** a dependency manifest for shared `.bi`/`.bm` libraries, run a selection or a scratch file, a
  timing profiler on the coverage instrumentation, `--watch`, a web playground.
- **Language:** modules or namespaces, `TRY`/`CATCH`, default and optional parameters, procedures as values,
  growable lists and maps, string interpolation.
- **Portability:** a WebAssembly or ARM target, which the runtime redesign row gates.

Every language item here, and in sections 4 and 5, is an extension QB64pe would reject. A program that used one
would no longer build with QB64pe. So each needs a decision and a switch, as the 40-character names row and the
PDS 7.1 table of `SOMEDAY.md` already say. That policy decision has not been made.

## 7. The panel's view

- **Pragmatic engineer:** reach first. A compiler nobody else can install has one user. The release group is
  mostly checks and metadata, minutes of Claude's time each, and the user's time to publish.
- **QB64 engineer:** then the items the ports found. Language extensions split the community unless they sit
  behind a switch that a new project turns on and a port leaves off.
- **Compiler engineer:** decide the extension policy once. After that the PDS 7.1 features are small changes
  through the parser, `sema`, the IR and the emitter, one OpenSpec change each.
- **Rust engineer:** fuzzing is cheap now, with the lossless tree's round-trip property, and finds panics before
  users do.
- **Test engineer:** more recorded real programs are the weakest evidence today; the suites all say the same
  thing about the same programs.
- **A new user**, added because reach is the subject: install friction and no help text matter more than any
  feature on these lists.

**Recommendation:** the next milestone is "first outside users": the `release` group, then the `real-programs`
group with the port-found fixes (`CALL INTERRUPT`, `qb45-binary`, the two warnings), then the P2 and P3 editor
rows. In parallel, one decision from the user: whether language extensions are allowed behind a switch, with the
PDS 7.1 table as the first case.

## 8. What the user decides

1. Is "first outside users" the next milestone, in the order above?
2. Are language extensions allowed behind a switch, PDS 7.1 first? Or is the dialect fixed at QB64pe's?
3. Should upstream's commits since `16f629784e` be reviewed as a routine, for example at each merge into `main`?
