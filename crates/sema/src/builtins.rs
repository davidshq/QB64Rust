//! Built-in functions (design D2–D4 of `m2-core-builtins`): which are compiled, the rule each follows, and the
//! types of their results. The arguments are checked in `check\builtins.rs`; the emitter finds a call's rule here
//! ([`rule`]) and writes what the old compiler writes for it.
//!
//! The built-in table (`qb64rust_builtins`, generated from the old compiler's registrations) gives each function's
//! slots, optional mask and plain return type. The old compiler special-cases about a third of the functions in
//! `evaluatefunc` (`study\02` §5.5); each kind of special-casing is one [`Rule`]. The table is never edited: where it
//! is wrong, a rule overrides it, citing the measurement (`verification\v20_*`, `study\00` §5).
//!
//! Each call has a held type (the C++ type of the libqb call for the arguments' held types, so arithmetic on it is
//! computed as in the old compiler) and a believed type (the old compiler's, which `PRINT`, `HEX$` and later
//! operators see), as operators have ([`crate::Expr`]).

use crate::{Expr, ExprKind, PLACE_ONLY_UNREACHABLE, Ty};
use qb64rust_builtins::passing::plan;
use qb64rust_builtins::{Builtin, BuiltinId, find_function_form, find_statements, find_stub};

/// How the old compiler treats a built-in function: one variant per kind of special-casing, not per function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    /// Slots, optional mask and result from the table; each argument converted by its slot type ([`Slot`]).
    Plain,
    /// The result has the argument's type, the argument converted as its slot says (`ABS`).
    ResultOfArg,
    /// `INT`, `FIX`: the argument as it is, the result held and believed as the argument.
    IntFix,
    /// `SIN`, `COS`, `TAN`, `ATN`, `SQR`, `LOG`: believed SINGLE, DOUBLE or `_FLOAT` by the argument's believed type
    /// (`qb64pe.bas` 22071).
    FloatByArg,
    /// `EXP`: believed SINGLE for an integer of 16 bits or fewer and for SINGLE, else `_FLOAT` (`qb64pe.bas` 21066).
    Exp,
    /// `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND`: the result type fixed, a range-checked libqb entry chosen by the
    /// argument's believed type (`qb64pe.bas` 21116–21215).
    Convert(Ty),
    /// The result type overrides the table, no arguments to convert (`ERR` LONG, `ERL` DOUBLE; `LBOUND`/`UBOUND`
    /// `_INTEGER64`, measured `v18_e_bounds*`, checked as [`ExprKind::Bound`]).
    Fixed(Ty),
    /// `LEN`: a string expression's length, or a place's size (D4).
    Len,
    /// `VAL(s$[, type])` (D4).
    Val,
    /// `HEX$` (4 bits per digit), `OCT$` (3), `_BIN$` (1): the width of a negative value from the argument's type.
    Radix(u32),
    /// `ASC(s$[, position])`: two slots, against the table's one.
    Asc,
    /// `STRING$(n, code)` or `STRING$(n, s$)`: the second argument a number or a string (`qb64pe.bas` 21294).
    StringFill,
    /// `ENVIRON$(name$)` or `ENVIRON$(n)`: a string as it is, or a number converted as the table's LONG slot
    /// (`qb64pe.bas` 20952; libqb has an entry for each, `func_environ(qbs*)` and `func_environ(int32_t)`).
    StrOrIndex,
    /// `_RGB32`: the table's slots, of which the first n are needed; the call is written with the arguments given
    /// and nothing for the others (libqb has an entry for each number of arguments; measured,
    /// `verification\v28_x01`, `x45`, `x46`).
    GivenArgs(usize),
    /// `_MIN`, `_MAX`, `_CLAMP`: every argument a number as it is; the result is typed by what the arguments are
    /// believed (`check\ops.rs`, `widest`), and the call is written with that type as its C++ template argument,
    /// which converts each argument ([`cpp_arithmetic`]; design D6 of `m3-console-files`).
    Widest,
    /// `_ROL`, `_ROR`: two integers as they are; the bits turn within the width of the first one's believed type,
    /// and the result is the unsigned type of that width (`check\ops.rs`, `rotated`; design D7).
    Rotate,
    /// `_IIF`: no call but a choice that evaluates one of its parts, checked as [`ExprKind::Choice`].
    Choice,
    /// No call but an expression of its own, which the checker builds before it looks at the table (`_MEM`,
    /// `_MEMELEMENT`: [`ExprKind::MemOf`], [`ExprKind::MemAt`]; `_OFFSET`: [`ExprKind::AddressOf`]). Named bare,
    /// such a built-in is an arity error.
    Own,
    /// `MKI$`, `MKL$`, `MKS$`, `MKD$` (the type given) and `_MK$` (`None`: the type is its first argument): the
    /// bytes of a number as a string, libqb's `<t>2string` (design D5 of `m3-mem`; `qb64pe.bas` 21217–21255).
    /// The number is converted to an integer type as a store would, and passed to a float type as it is. `_MK$`'s
    /// first slot holds a 0 of the type named (`_BIT` and `_UNSIGNED _BIT` are one bit wide).
    Mk(Option<Ty>),
    /// `CVI`, `CVL`, `CVS`, `CVD` (the type given) and `_CV` (`None`, as for [`Rule::Mk`]): the number a string's
    /// bytes hold, libqb's `string2<t>`, typed as the type (a `_BIT` as an `_INTEGER64`).
    Cv(Option<Ty>),
}

/// Every built-in function `sema` compiles, as written (with its required suffix), and its rule. The coverage check
/// of tier 1 requires each to be called in a `slice.list` program and in a `typed` front-end test.
pub const SUPPORTED: &[(&str, Rule)] = &[
    ("INSTR", Rule::Plain),
    ("CHR$", Rule::Plain),
    // String functions (`v20_c_string_edges`).
    ("LEFT$", Rule::Plain),
    ("RIGHT$", Rule::Plain),
    ("MID$", Rule::Plain),
    ("SPACE$", Rule::Plain),
    ("STRING$", Rule::StringFill),
    ("LTRIM$", Rule::Plain),
    ("RTRIM$", Rule::Plain),
    ("_TRIM$", Rule::Plain),
    ("UCASE$", Rule::Plain),
    ("LCASE$", Rule::Plain),
    ("STR$", Rule::Plain),
    ("_TOSTR$", Rule::Plain),
    ("LEN", Rule::Len),
    ("ASC", Rule::Asc),
    ("VAL", Rule::Val),
    ("HEX$", Rule::Radix(4)),
    ("OCT$", Rule::Radix(3)),
    ("_BIN$", Rule::Radix(1)),
    // Math functions (`v20_b_result_types`, `v20_f_math_edges`).
    ("ABS", Rule::ResultOfArg),
    ("INT", Rule::IntFix),
    ("FIX", Rule::IntFix),
    ("SGN", Rule::Plain),
    ("SIN", Rule::FloatByArg),
    ("COS", Rule::FloatByArg),
    ("TAN", Rule::FloatByArg),
    ("ATN", Rule::FloatByArg),
    ("SQR", Rule::FloatByArg),
    ("LOG", Rule::FloatByArg),
    ("EXP", Rule::Exp),
    ("CINT", Rule::Convert(Ty::I16)),
    // LONG: the table's INTEGER is wrong (`qb64pe.bas` 21190, `v20_b_result_types`).
    ("CLNG", Rule::Convert(Ty::I32)),
    ("CSNG", Rule::Convert(Ty::F32)),
    ("CDBL", Rule::Convert(Ty::F64)),
    ("_ROUND", Rule::Convert(Ty::I64)),
    ("_ATAN2", Rule::Plain),
    ("_HYPOT", Rule::Plain),
    ("_PI", Rule::Plain),
    ("LBOUND", Rule::Fixed(Ty::I64)),
    ("UBOUND", Rule::Fixed(Ty::I64)),
    // `ERR` is LONG, not `_UNSIGNED LONG` as in the table (design D5 of `m2-procedures-and-errors`).
    ("ERR", Rule::Fixed(Ty::I32)),
    ("ERL", Rule::Fixed(Ty::F64)),
    // File functions (`verification\v22_b_funcs`); `FREEFILE` and `_CWD$` have no slot and are called bare.
    ("EOF", Rule::Plain),
    ("LOF", Rule::Plain),
    ("LOC", Rule::Plain),
    ("SEEK", Rule::Plain),
    ("FREEFILE", Rule::Plain),
    ("_FILEEXISTS", Rule::Plain),
    ("_DIREXISTS", Rule::Plain),
    ("_CWD$", Rule::Plain),
    // `RND` and `TIMER` (`verification\v22_d_random`): one optional slot each, so both are also called bare. `RND`
    // is a `float`; `TIMER` is believed SINGLE and held DOUBLE (libqb's `func_timer` returns `double`).
    ("RND", Rule::Plain),
    ("TIMER", Rule::Plain),
    // `verification\v22_d_shell`, `v22_d_environ`: the `SHELL` function is an `_INTEGER64`; `COMMAND$` has one
    // optional slot and is also called bare.
    ("SHELL", Rule::Plain),
    ("COMMAND$", Rule::Plain),
    ("ENVIRON$", Rule::StrOrIndex),
    // Functions by demand (`verification\v22_f_functions`): all believed `_FLOAT`. The `std::` ones are held by C++
    // overloading; libqb's own take and return a `double`.
    ("_ACOS", Rule::Plain),
    ("_ASIN", Rule::Plain),
    ("_SINH", Rule::Plain),
    ("_COSH", Rule::Plain),
    ("_TANH", Rule::Plain),
    ("_CEIL", Rule::Plain),
    ("_COT", Rule::Plain),
    ("_CSC", Rule::Plain),
    ("_SEC", Rule::Plain),
    ("_D2R", Rule::Plain),
    ("_R2D", Rule::Plain),
    ("_STRCMP", Rule::Plain),
    ("_STRICMP", Rule::Plain),
    // No slot, called bare only (`verification\v22_f_startdir`).
    ("_STARTDIR$", Rule::Plain),
    // The destination and source of text, and the console's handle (`verification\v26_b_dest_switch`): no slot,
    // called bare only, a LONG.
    ("_DEST", Rule::Plain),
    ("_SOURCE", Rule::Plain),
    ("_CONSOLE", Rule::Plain),
    // The window functions (design D6 of `m2-console-screenhide`, measured from the old compiler's C++): compiled
    // and never run yet ([`NEVER_RUN`]). No slot and called bare only, a LONG, but for the three below.
    ("_SCREENICON", Rule::Plain),
    ("_DESKTOPWIDTH", Rule::Plain),
    ("_DESKTOPHEIGHT", Rule::Plain),
    ("_SCREENX", Rule::Plain),
    ("_SCREENY", Rule::Plain),
    ("_WINDOWHASFOCUS", Rule::Plain),
    ("_TITLE$", Rule::Plain),
    // The table has no return type for it; the old compiler treats the value as a LONG everywhere but as the direct
    // argument of `HEX$`, `OCT$` and `_BIN$`, which is marked (`check\builtins.rs`).
    ("_SCREENEXISTS", Rule::Fixed(Ty::I32)),
    // `_NEWIMAGE(w, h[, mode])`; `_WINDOWHANDLE[(kind)]` is an `_UNSIGNED _OFFSET`, bare or with its argument.
    ("_NEWIMAGE", Rule::Plain),
    ("_WINDOWHANDLE", Rule::Plain),
    // The screen and keyboard functions (design D2 of `m3-window-output`, measured from the old compiler's C++;
    // `verification\v27_*`). `INKEY$` and `CSRLIN` have no slot and are called bare only; `POS(n)` is a LONG.
    ("INKEY$", Rule::Plain),
    ("CSRLIN", Rule::Plain),
    ("POS", Rule::Plain),
    // `POINT(x, y)` and `POINT(n)`: one entry with an optional second slot, a DOUBLE.
    ("POINT", Rule::Plain),
    // `SCREEN(row, column[, colorflag])`: an `_UNSIGNED LONG`.
    ("SCREEN", Rule::Plain),
    // `TAB(n)` and `SPC(n)` are functions that give a string, one LONG slot each: `PRINT`, `PRINT #` and `WRITE`
    // take them as the string items they are (design D2 of `m3-get-put-bload`; `verification\v28_a_tab_spc`).
    ("TAB", Rule::Plain),
    ("SPC", Rule::Plain),
    // `PEEK(offset)`: a LONG, in the segment `DEF SEG` set (`verification\v28_b_bload`).
    ("PEEK", Rule::Plain),
    // `VARPTR(a(i))` and `VARSEG(a(i))`: written by the compiler, an `_INTEGER64` to `PRINT` and to operators
    // against the table's LONG (`qb64pe.bas` 21328; measured, `verification\v28_d_varptr_index`); checked as
    // [`ExprKind::VarPtr`] and [`ExprKind::VarSeg`].
    ("VARPTR", Rule::Fixed(Ty::I64)),
    ("VARSEG", Rule::Fixed(Ty::I64)),
    // `_RGB32` with one to four LONG slots (libqb has an entry for each number of arguments) and `_RGBA32` with
    // four: an `_UNSIGNED LONG`. `_FULLSCREEN` has no slot and is called bare only, a LONG ([`NEVER_RUN`]).
    ("_RGB32", Rule::GivenArgs(1)),
    ("_RGBA32", Rule::Plain),
    ("_FULLSCREEN", Rule::Plain),
    // The date and the time as strings: no slot, called bare only. Civil War Strategy writes `DATE$` into its
    // history file, on a line `TAB` kept marked until now.
    ("DATE$", Rule::Plain),
    ("TIME$", Rule::Plain),
    // `PLAY[(voice)]`: the seconds of sound left to play in a voice, a DOUBLE; one optional slot, so it is also
    // called bare (design D1 of `m3-sound`; `verification\v29_x55`–`x60`).
    ("PLAY", Rule::Plain),
    // The choice, the least and greatest, and the shifts (design D6, D7 of `m3-console-files`;
    // `verification\v30_g_iif_min_max`, `v30_h_shifts`, `v30_result_types.txt`). `_SHL` and `_SHR` are plain: an
    // `_UNSIGNED _INTEGER64` and a LONG slot, the result an `_UNSIGNED _INTEGER64`.
    ("_IIF", Rule::Choice),
    ("_MIN", Rule::Widest),
    ("_MAX", Rule::Widest),
    ("_CLAMP", Rule::Widest),
    ("_SHL", Rule::Plain),
    ("_SHR", Rule::Plain),
    ("_ROL", Rule::Rotate),
    ("_ROR", Rule::Rotate),
    // The colour functions (design D1 of `m3-images-fonts-mouse`; `verification\v31_a_colours`, `v31_x01`,
    // `x27`–`x36`): each the plain call of its row. `_RGB` and `_RGBA` are `_UNSIGNED LONG` with an optional
    // handle; `_RED` to `_ALPHA` LONG with an optional handle; the `…32` parts LONG; `_HSB32` and `_HSBA32`
    // `_UNSIGNED LONG` of DOUBLE slots; `_HUE32`, `_SATURATION32`, `_BRIGHTNESS32` DOUBLE.
    ("_RGB", Rule::Plain),
    ("_RGBA", Rule::Plain),
    ("_RED", Rule::Plain),
    ("_GREEN", Rule::Plain),
    ("_BLUE", Rule::Plain),
    ("_ALPHA", Rule::Plain),
    ("_RED32", Rule::Plain),
    ("_GREEN32", Rule::Plain),
    ("_BLUE32", Rule::Plain),
    ("_ALPHA32", Rule::Plain),
    ("_HSB32", Rule::Plain),
    ("_HSBA32", Rule::Plain),
    ("_HUE32", Rule::Plain),
    ("_SATURATION32", Rule::Plain),
    ("_BRIGHTNESS32", Rule::Plain),
    // The image functions (same design, `v31_b_images`, `v31_c_save_load`): LONG each, but `PMAP`, a SINGLE.
    // `_LOADIMAGE` needs the image codec ([`crate::RuntimePart::ImageCodec`]).
    ("_LOADIMAGE", Rule::Plain),
    ("_COPYIMAGE", Rule::Plain),
    ("_WIDTH", Rule::Plain),
    ("_HEIGHT", Rule::Plain),
    ("_PIXELSIZE", Rule::Plain),
    ("_CLEARCOLOR", Rule::Plain),
    ("_BLEND", Rule::Plain),
    ("_PALETTECOLOR", Rule::Plain),
    ("_AUTODISPLAY", Rule::Plain),
    ("PMAP", Rule::Plain),
    // The font functions (same design, `v31_d_fonts`): LONG each. `_LOADFONT` and the `_U…` functions need the
    // font part ([`crate::RuntimePart::Font`]); `_UCHARPOS` takes a LONG array as its bytes ([`Slot::Block`]).
    ("_LOADFONT", Rule::Plain),
    ("_FONT", Rule::Plain),
    ("_FONTWIDTH", Rule::Plain),
    ("_FONTHEIGHT", Rule::Plain),
    ("_PRINTWIDTH", Rule::Plain),
    ("_PRINTMODE", Rule::Plain),
    ("_UFONTHEIGHT", Rule::Plain),
    ("_ULINESPACING", Rule::Plain),
    ("_UPRINTWIDTH", Rule::Plain),
    ("_UCHARPOS", Rule::Plain),
    ("_MAPUNICODE", Rule::Plain),
    // The mouse functions (same design, `v31_x01_accepted`, `x13`–`x16`, `x101`–`x103`): the event last taken
    // from libqb's queue. `_MOUSEINPUT`, `_MOUSEBUTTON`, `_MOUSEHIDDEN`, `_MOUSEDISABLED` LONG; `_MOUSEX`,
    // `_MOUSEY`, `_MOUSEWHEEL` and the movements DOUBLE. Read in tests from the mouse steps of a keys file
    // (`tools\screen\screen_state.cpp`, design D4).
    ("_MOUSEINPUT", Rule::Plain),
    ("_MOUSEX", Rule::Plain),
    ("_MOUSEY", Rule::Plain),
    ("_MOUSEBUTTON", Rule::Plain),
    ("_MOUSEWHEEL", Rule::Plain),
    ("_MOUSEMOVEMENTX", Rule::Plain),
    ("_MOUSEMOVEMENTY", Rule::Plain),
    ("_MOUSEHIDDEN", Rule::Plain),
    ("_MOUSEDISABLED", Rule::Plain),
    // An image of the desktop or of the clipboard (same design, `v31_x01_accepted`): LONG each, with libqb's
    // screen-image part ([`crate::RuntimePart::ScreenImage`]). Compiled and never run ([`NEVER_RUN`]).
    ("_SCREENIMAGE", Rule::Plain),
    ("_CLIPBOARDIMAGE", Rule::Plain),
    // Memory blocks (design D2 and D4 of `m3-mem`; `verification\v32_x01_accepted`): `_MEMNEW`, `_MEMIMAGE` and
    // `_MEMSOUND` give a block, which is only stored into a `_MEM` place (`check\mod.rs`, `assign`); `_MEMEXISTS`
    // takes one ([`Slot::Mem`]). `_MEMSOUND` needs libqb's audio part and no test runs it ([`NEVER_RUN`]).
    ("_MEMNEW", Rule::Plain),
    ("_MEMIMAGE", Rule::Plain),
    ("_MEMSOUND", Rule::Plain),
    ("_MEMEXISTS", Rule::Plain),
    // The block of a reference or of an address, and the address of a reference (design D3 and D6 of `m3-mem`).
    ("_MEM", Rule::Own),
    ("_MEMELEMENT", Rule::Own),
    ("_OFFSET", Rule::Own),
    // A value read from a block (design D4 of `m3-mem`), [`ExprKind::MemRead`].
    ("_MEMGET", Rule::Own),
    // A number and its bytes (design D5 of `m3-mem`; `verification\v32_c_mk_cv`, `v32_x97`–`x113`).
    ("MKI$", Rule::Mk(Some(Ty::I16))),
    ("MKL$", Rule::Mk(Some(Ty::I32))),
    ("MKS$", Rule::Mk(Some(Ty::F32))),
    ("MKD$", Rule::Mk(Some(Ty::F64))),
    ("_MK$", Rule::Mk(None)),
    ("CVI", Rule::Cv(Some(Ty::I16))),
    ("CVL", Rule::Cv(Some(Ty::I32))),
    ("CVS", Rule::Cv(Some(Ty::F32))),
    ("CVD", Rule::Cv(Some(Ty::F64))),
    ("_CV", Rule::Cv(None)),
    // Checksums, compression and base64 (design D4 of `m3-asserts-debug-zlib`; `verification\v33_b*`, `x25`–`x36`):
    // each the plain call of its row, with libqb's data-processing part ([`crate::RuntimePart::DataProcessing`]).
    // `_ADLER32` and `_CRC32` are `_UNSIGNED LONG`; `_DEFLATE$`'s level is a LONG slot, `_INFLATE$`'s size an
    // `_INTEGER64` one, both optional.
    ("_ADLER32", Rule::Plain),
    ("_CRC32", Rule::Plain),
    ("_DEFLATE$", Rule::Plain),
    ("_INFLATE$", Rule::Plain),
    ("_BASE64ENCODE$", Rule::Plain),
    ("_BASE64DECODE$", Rule::Plain),
];

/// The built-in functions whose result is a block of the predefined type `_MEM` (design D2 of `m3-mem`), as
/// written. The old compiler takes such a call only as the value stored into a place of type `_MEM` (measured,
/// `verification\v32_x02`, `x03`, `x80`, `x93`–`x96`).
pub const MEM_RESULTS: &[&str] = &["_MEM", "_MEMELEMENT", "_MEMNEW", "_MEMIMAGE", "_MEMSOUND"];

/// How the old compiler treats a built-in statement that is one call of the runtime (design D3 of
/// `m2-builtin-statements`): one variant per kind of special-casing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StmtRule {
    /// Slots and choices from the table entry and its template ([`stmt_slots`]); each argument converted by its
    /// slot type; passed as the old compiler's template rule says (`qb64rust_builtins::passing`).
    Plain,
    /// `CLOSE`: any number of file numbers, each a LONG value; one call per number, in order, or one call that
    /// closes every file when there is none (measured from the C++: `sub_close(n,1)`, `sub_close(NULL,0)`).
    Close,
    /// `SWAP a, b`: two places of one type (`qb64pe.bas` 11377; measured, `verification\v22_d_swap`): two strings of
    /// any kind, two places of the same `TYPE`, or two numbers of the same type but for their signedness; never a
    /// `_BIT`. The slots are the two places.
    Swap,
    /// `MID$(target$, start[, length]) = value$`: a string place, a LONG value, a LONG value or absent, a string
    /// value (measured, `verification\v22_d_mid`).
    MidAssign,
    /// Graphics `GET` and `PUT`: as [`StmtRule::Plain`], for the entries of the name whose call is libqb's
    /// `sub_graphics_…` only. The name's other entry is the file form ([`StmtRule::File`]).
    Graphics,
    /// `GET` and `PUT` on a file (design D2 of `m3-console-files`): the entry of the name whose call is `sub_get`
    /// or `sub_put`. Its slots are the template's: the file number, the position or nothing, and the variable or
    /// nothing ([`Slot::Bytes`]). The variable decides the runtime's entry: none, the file's record buffer; a
    /// variable-length string, the string itself; anything else, its bytes.
    File,
}

/// Every built-in statement `sema` compiles as a call of the runtime, as written, and its rule. A name stands for
/// every form the table has for it (`OPEN` two, `SHELL` three). The coverage check of tier 1 requires each to be
/// used in a `slice.list` program and in an `ir` front-end test.
pub const STATEMENTS: &[(&str, StmtRule)] = &[
    // The file-system statements and `ENVIRON` (`verification\v22_a_*`).
    ("KILL", StmtRule::Plain),
    ("MKDIR", StmtRule::Plain),
    ("RMDIR", StmtRule::Plain),
    ("CHDIR", StmtRule::Plain),
    ("NAME", StmtRule::Plain),
    ("ENVIRON", StmtRule::Plain),
    // Files (`verification\v22_b_*`): `OPEN` in both forms.
    ("OPEN", StmtRule::Plain),
    ("CLOSE", StmtRule::Close),
    ("SEEK", StmtRule::Plain),
    // `verification\v22_d_*`. `RANDOMIZE` without a seed is the same call without the argument: libqb asks for the
    // seed on the console (`verification\v22_e_randomize`).
    ("SWAP", StmtRule::Swap),
    ("MID$", StmtRule::MidAssign),
    ("RANDOMIZE", StmtRule::Plain),
    // `SHELL` in its three forms (`verification\v22_d_shell`).
    ("SHELL", StmtRule::Plain),
    // `verification\v26_b_dest_switch`, `v26_g_echo`: the handle a LONG value. `_ECHO` needs a program with a
    // console, checked after the walk (`check\mod.rs`).
    ("_DEST", StmtRule::Plain),
    ("_SOURCE", StmtRule::Plain),
    ("_ECHO", StmtRule::Plain),
    // The window statements (design D6 of `m2-console-screenhide`): compiled and never run yet
    // ([`NEVER_RUN`]).
    ("_SCREENHIDE", StmtRule::Plain),
    ("_SCREENSHOW", StmtRule::Plain),
    ("_SCREENICON", StmtRule::Plain),
    ("_TITLE", StmtRule::Plain),
    ("_ICON", StmtRule::Plain),
    ("_MOUSESHOW", StmtRule::Plain),
    ("_MOUSEHIDE", StmtRule::Plain),
    ("_MOUSECURSOR", StmtRule::Plain),
    ("_WINDOWSIZELIMIT", StmtRule::Plain),
    // Run by the screen programs of `m3-images-fonts-mouse` since then.
    ("_FREEIMAGE", StmtRule::Plain),
    // The screen statements (design D2 of `m3-window-output`: 455 forms read from the old compiler's C++; accepted
    // and rejected forms, `verification\v27_*`). Each is the plain call of its row and template, in every form.
    // Text screen: `VIEW` stands for both of its entries, `VIEW PRINT` and the graphics viewport.
    ("SCREEN", StmtRule::Plain),
    ("CLS", StmtRule::Plain),
    ("COLOR", StmtRule::Plain),
    ("LOCATE", StmtRule::Plain),
    ("WIDTH", StmtRule::Plain),
    ("VIEW", StmtRule::Plain),
    // Drawing. `PAINT`'s fill is a number or a tile string ([`Slot::ColorOrTile`]).
    ("PSET", StmtRule::Plain),
    ("PRESET", StmtRule::Plain),
    ("LINE", StmtRule::Plain),
    ("CIRCLE", StmtRule::Plain),
    ("PAINT", StmtRule::Plain),
    ("DRAW", StmtRule::Plain),
    ("WINDOW", StmtRule::Plain),
    ("PALETTE", StmtRule::Plain),
    ("PCOPY", StmtRule::Plain),
    // Pacing.
    ("_LIMIT", StmtRule::Plain),
    ("_DELAY", StmtRule::Plain),
    ("SLEEP", StmtRule::Plain),
    ("_DISPLAY", StmtRule::Plain),
    // Conventional memory (design D2 of `m3-get-put-bload`; `verification\v28_b_bload`, `v28_x26`–`x35`): `DEF`
    // stands for `DEF SEG [= segment]`, the one entry of that name with a call.
    ("DEF", StmtRule::Plain),
    ("POKE", StmtRule::Plain),
    ("BLOAD", StmtRule::Plain),
    ("BSAVE", StmtRule::Plain),
    // `_FULLSCREEN [mode][, _SMOOTH]`: compiled and never run ([`NEVER_RUN`]).
    ("_FULLSCREEN", StmtRule::Plain),
    // Graphics `GET` and `PUT` (design D5 of `m3-get-put-bload`; `verification\v28_x10`–`x25`): the array is
    // a slot of its own kind ([`Slot::Block`]).
    ("GET", StmtRule::Graphics),
    ("PUT", StmtRule::Graphics),
    // `GET` and `PUT` on a file open `FOR RANDOM` or `FOR BINARY`, the second row of each name (design D2 of
    // `m3-console-files`; `verification\v30_d_binary`, `v30_e_random`, `v30_x20`-`x48`), and `LOCK` and `UNLOCK`,
    // plain: a file number and one record, a range with `TO`, or neither (D4; `v30_i_lock`, `x50`-`x56`).
    ("GET", StmtRule::File),
    ("PUT", StmtRule::File),
    ("LOCK", StmtRule::Plain),
    ("UNLOCK", StmtRule::Plain),
    // `CHAIN name$`: one string, written like a SUB call (design D8 of `m3-console-files`;
    // `verification\v30_x95`-`x98`). Compiled and never run ([`NEVER_RUN`]).
    ("CHAIN", StmtRule::Plain),
    // Sound (design D1 of `m3-sound`; `verification\v29_x01`–`x86`): each the plain call of its row and template.
    // `SOUND` and `PLAY` need libqb's audio part ([`crate::RuntimePart::Audio`]); `BEEP` is never run
    // ([`NEVER_RUN`]).
    ("SOUND", StmtRule::Plain),
    ("PLAY", StmtRule::Plain),
    ("BEEP", StmtRule::Plain),
    // The image statements (design D1 of `m3-images-fonts-mouse`; `verification\v31_x01`, `x02`–`x109`): each the
    // plain call of its row and template. `_SAVEIMAGE` needs the image codec; `_MAPTRIANGLE`'s 3-D form (a z on
    // each destination point) is compiled and run by no test (design D3).
    ("_SAVEIMAGE", StmtRule::Plain),
    ("_PUTIMAGE", StmtRule::Plain),
    ("_MAPTRIANGLE", StmtRule::Plain),
    ("_CLEARCOLOR", StmtRule::Plain),
    ("_SETALPHA", StmtRule::Plain),
    ("_BLEND", StmtRule::Plain),
    ("_DONTBLEND", StmtRule::Plain),
    ("_PALETTECOLOR", StmtRule::Plain),
    ("_COPYPALETTE", StmtRule::Plain),
    ("_AUTODISPLAY", StmtRule::Plain),
    // The font statements (same design): each the plain call of its row and template. `_UPRINTSTRING` needs the
    // font part.
    ("_FONT", StmtRule::Plain),
    ("_FREEFONT", StmtRule::Plain),
    ("_PRINTSTRING", StmtRule::Plain),
    ("_PRINTMODE", StmtRule::Plain),
    ("_UPRINTSTRING", StmtRule::Plain),
    ("_MAPUNICODE", StmtRule::Plain),
    // It moves the machine's cursor: compiled, and run by no test (design D3).
    ("_MOUSEMOVE", StmtRule::Plain),
    // Compiled and run by no test (design D3): `_CLIPBOARDIMAGE = h` sets the clipboard (the screen-image part),
    // `_PRINTIMAGE` prints a page (the printer part), `_DEPTHBUFFER` acts on hardware images only.
    ("_CLIPBOARDIMAGE", StmtRule::Plain),
    ("_PRINTIMAGE", StmtRule::Plain),
    ("_DEPTHBUFFER", StmtRule::Plain),
    // Memory blocks (design D2 and D4 of `m3-mem`): each a plain call whose blocks are `_MEM` places
    // ([`Slot::Mem`]); `_MEMCOPY a, o, n TO b, p` by its template.
    ("_MEMFREE", StmtRule::Plain),
    ("_MEMCOPY", StmtRule::Plain),
    // `_ASSERT condition[, message$]` (design D3 of `m3-asserts-debug-zlib`; `verification\v33_a*`, `x17`–`x24`):
    // the plain call of its row, which checks nothing without `$ASSERTS` (the program's global `asserts`).
    ("_ASSERT", StmtRule::Plain),
];

/// The built-ins no test runs, as written. The coverage check of tier 1 wants each in a slice program as code that
/// is compiled and not reached, and in a call-site program (`tests\callsite`), which compares its call with the old
/// compiler's.
pub const NEVER_RUN: &[&str] = &[
    // The window built-ins: what they do shows only in a window, outside the state the screen check reads.
    "_SCREENHIDE",
    "_SCREENSHOW",
    "_SCREENICON",
    "_TITLE",
    "_ICON",
    "_MOUSESHOW",
    "_MOUSEHIDE",
    "_MOUSECURSOR",
    "_WINDOWSIZELIMIT",
    "_SCREENEXISTS",
    "_TITLE$",
    "_DESKTOPWIDTH",
    "_DESKTOPHEIGHT",
    "_SCREENX",
    "_SCREENY",
    "_WINDOWHANDLE",
    "_WINDOWHASFOCUS",
    "_FULLSCREEN",
    // `BEEP` rings the system's bell: it is not libqb's audio, so no volume silences it (design D6 of `m3-sound`).
    "BEEP",
    // `LPRINT` prints a page on the default printer (design D8 of `m3-console-files`; user, `DECISIONS.md`
    // 2026-10-13). It is no row of [`STATEMENTS`]: `PRINT` to the printer, with a syntax node of its own.
    "LPRINT",
    // `CHAIN` compiles another program: libqb starts `qb64pe.exe -c` beside the running program through `cmd`
    // (`tests\corpus\README.md`), which no test machine may be asked for (same decision).
    "CHAIN",
    // `_MOUSEMOVE` moves the machine's cursor; `_SCREENIMAGE` and `_CLIPBOARDIMAGE` read the desktop and the
    // clipboard, `_PRINTIMAGE` prints a page, `_DEPTHBUFFER` needs a hardware image (design D3 of
    // `m3-images-fonts-mouse`).
    "_MOUSEMOVE",
    "_SCREENIMAGE",
    "_CLIPBOARDIMAGE",
    "_PRINTIMAGE",
    "_DEPTHBUFFER",
    // `_MEMSOUND` needs a sound handle, which no supported statement gives (`_SNDOPEN` is not supported; design D7
    // of `m3-mem`).
    "_MEMSOUND",
];

/// One form of a supported built-in statement: its table entry and rule.
#[derive(Clone, Copy, Debug)]
pub struct StmtSupported {
    pub id: BuiltinId,
    pub rule: StmtRule,
    /// The name as written, with its required suffix.
    pub name: &'static str,
}

/// The forms of the supported statement a name as written stands for (`name` in upper case without suffix, `string`
/// when it carries `$`), in table order; empty for any other name.
pub fn stmt_lookup(name: &str, string: bool) -> Vec<StmtSupported> {
    STATEMENTS
        .iter()
        .filter(|&&(written, _)| {
            let bare = written.strip_suffix('$').unwrap_or(written);
            bare == name && (bare.len() != written.len()) == string
        })
        .flat_map(|&(written, rule)| {
            let forms = match rule {
                StmtRule::Plain => find_statements(name.as_bytes(), string),
                StmtRule::Graphics => find_statements(name.as_bytes(), string)
                    .into_iter()
                    .filter(|id| id.get().callname.starts_with("sub_graphics_"))
                    .collect(),
                StmtRule::File => find_statements(name.as_bytes(), string)
                    .into_iter()
                    .filter(|id| matches!(id.get().callname, "sub_get" | "sub_put"))
                    .collect(),
                // The statements the old compiler writes itself are named by their stub entry.
                StmtRule::Close | StmtRule::Swap | StmtRule::MidAssign => {
                    find_stub(name.as_bytes(), string).into_iter().collect()
                }
            };
            forms.into_iter().map(move |id| StmtSupported {
                id,
                rule,
                name: written,
            })
        })
        .collect()
}

/// Every form of every supported statement.
pub fn statements() -> Vec<StmtSupported> {
    STATEMENTS
        .iter()
        .flat_map(|&(written, _)| {
            let bare = written.strip_suffix('$').unwrap_or(written);
            stmt_lookup(bare, bare.len() != written.len())
        })
        .collect()
}

/// The supported statement form with this table entry.
pub fn stmt_find(id: BuiltinId) -> Option<StmtSupported> {
    statements().into_iter().find(|s| s.id == id)
}

/// One slot of a built-in statement: an argument or a choice of its template, in template order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StmtSlot {
    /// An argument of this slot type; `optional` when it may be left out.
    Arg { slot: Slot, optional: bool },
    /// A choice among `alts` alternatives; `optional` when it may be left out.
    Choice { alts: u8, optional: bool },
}

/// The slots of a [`StmtRule::Plain`] statement form: its template's arguments and choices, each argument typed by
/// the table (the table lists the types of the C arguments, [`qb64rust_builtins::passing`]). Empty for a stub
/// entry, whose rule says what its slots are.
pub fn stmt_slots(id: BuiltinId) -> Vec<StmtSlot> {
    let b = id.get();
    if b.callname == "sub_stub" {
        return Vec::new();
    }
    let plan = plan(&b.template());
    let mut c_arg = 0;
    plan.parts
        .iter()
        .map(|part| {
            let ty = part.passed.then(|| {
                c_arg += 1;
                b.arg_types.get(c_arg - 1).copied()
            });
            match part.alts {
                0 => {
                    let t = ty
                        .flatten()
                        .unwrap_or_else(|| unreachable!("`{}`: an argument without a table type", b.name));
                    StmtSlot::Arg {
                        slot: stmt_slot_type(b, c_arg - 1, t),
                        optional: part.optional,
                    }
                }
                alts => StmtSlot::Choice {
                    alts,
                    optional: part.optional,
                },
            }
        })
        .collect()
}

/// The type of C argument `k` of a statement entry: the table's, except where the libqb entry takes another type.
fn stmt_slot_type(b: &Builtin, k: usize, table: &str) -> Slot {
    match (b.callname, k) {
        // `sub_seek(int32 i, int64 pos)`: the table says LONG, but the old compiler passes the position as it is
        // and C++ keeps its 64 bits (measured from the C++, `verification\v22_a_args`).
        ("sub_seek", 1) => Slot::Int64,
        // `PAINT (x, y), fill`: a colour or a tile string; libqb has an entry for each (measured from the C++,
        // design D2 of `m3-window-output`).
        ("sub_paint", 2) => Slot::ColorOrTile,
        // `sub_get(int32 i, int64 offset, …)` and `sub_put`: the position is passed as it is, an `_INTEGER64`
        // too (measured from the C++: `sub_put( 1 ,*__INTEGER64_Q,…)`), a float rounded.
        ("sub_get" | "sub_put", 1) => Slot::Int64,
        // The array of graphics `GET` and `PUT`.
        _ if table.starts_with("offset+size (largest safe block") => Slot::Block,
        // The variable of `GET` and `PUT` on a file.
        _ if table.starts_with("offset+size (element size") => Slot::Bytes,
        _ => Slot::from_table(table),
    }
}

/// A supported built-in: its table entry and rule.
#[derive(Clone, Copy, Debug)]
pub struct Supported {
    pub id: BuiltinId,
    pub rule: Rule,
    /// The name as written, with its required suffix (`CHR$`).
    pub name: &'static str,
}

/// The supported built-in a name as written stands for (`name` in upper case without suffix, `string` when it
/// carries `$`); `None` for any other name.
pub fn lookup(name: &str, string: bool) -> Option<Supported> {
    SUPPORTED.iter().find_map(|&(written, rule)| {
        let (bare, dollar) = match written.strip_suffix('$') {
            Some(b) => (b, true),
            None => (written, false),
        };
        (bare == name && dollar == string).then(|| Supported {
            id: find_function_form(bare.as_bytes(), dollar).expect("every supported built-in is in the table"),
            rule,
            name: written,
        })
    })
}

/// Every supported built-in.
pub fn supported() -> impl Iterator<Item = Supported> {
    SUPPORTED.iter().map(|&(written, _)| {
        let bare = written.strip_suffix('$').unwrap_or(written);
        lookup(bare, bare.len() != written.len()).expect("listed")
    })
}

/// The rule of a supported built-in; `None` for one `sema` does not compile.
pub fn rule(id: BuiltinId) -> Option<Rule> {
    find(id).map(|s| s.rule)
}

/// The supported built-in with this table entry.
pub fn find(id: BuiltinId) -> Option<Supported> {
    supported().find(|s| s.id == id)
}

/// The number of argument slots a call of the built-in has in the typed tree: the table's, except where a rule says
/// otherwise.
pub fn slot_count(id: BuiltinId) -> usize {
    match rule(id) {
        Some(Rule::Asc) => 2,
        Some(
            Rule::Plain
            | Rule::ResultOfArg
            | Rule::IntFix
            | Rule::FloatByArg
            | Rule::Exp
            | Rule::Convert(_)
            | Rule::Fixed(_)
            | Rule::Len
            | Rule::Val
            | Rule::Radix(_)
            | Rule::StringFill
            | Rule::StrOrIndex
            | Rule::GivenArgs(_)
            | Rule::Widest
            | Rule::Rotate
            | Rule::Choice
            | Rule::Own
            | Rule::Mk(_)
            | Rule::Cv(_),
        )
        | None => id.get().arg_types.len(),
    }
}

/// The slots of a call: their types and whether each is optional.
pub fn slots(s: Supported) -> Vec<(Slot, bool)> {
    let b = s.id.get();
    match s.rule {
        Rule::Asc => vec![(Slot::Str, false), (Slot::Long, true)],
        Rule::GivenArgs(required) => b
            .arg_types
            .iter()
            .enumerate()
            .map(|(i, t)| (Slot::from_table(t), i >= required))
            .collect(),
        // Numbers as they are, as many as the table has arguments (its types for them are placeholders).
        Rule::Widest | Rule::Rotate | Rule::Choice => vec![(Slot::Float, false); b.arg_types.len()],
        // Checked as an expression of its own, by no slot.
        Rule::Own => Vec::new(),
        // Checked by their rule (`check\mem.rs`, `number_bytes`): a number or a string, and for `_MK$` and
        // `_CV` first a type.
        Rule::Mk(_) | Rule::Cv(_) => Vec::new(),
        Rule::Plain
        | Rule::ResultOfArg
        | Rule::IntFix
        | Rule::FloatByArg
        | Rule::Exp
        | Rule::Convert(_)
        | Rule::Fixed(_)
        | Rule::Len
        | Rule::Val
        | Rule::Radix(_)
        | Rule::StringFill
        | Rule::StrOrIndex => {
            let optional = b.optional.unwrap_or(&[]);
            b.arg_types
                .iter()
                .enumerate()
                .map(|(i, t)| (Slot::from_table(t), optional.get(i).copied().unwrap_or(false)))
                .collect()
        }
    }
}

/// The type of an argument slot, as decoded in the table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    /// Converted as a store into a LONG: a float rounded half to even to `_INTEGER64`, then the low 32 bits
    /// (measured `v20_a_slots`).
    Long,
    /// Converted as a store into an `_INTEGER64` (a float rounded half to even); only where a rule says so: the
    /// table has no such slot.
    Int64,
    /// The table's INTEGER, the condition of `_ASSERT`: the old compiler writes a float through
    /// `qbr_float_to_long` (narrowed to SINGLE, rounded half to even) and an integer as it is, and libqb takes an
    /// `int32`, so an integer keeps its low 32 bits (measured, `verification\v33_a08`: `1E+10` passes and
    /// `4294967296` fails). The conversion of `ON n GOTO`.
    Integer,
    /// Converted exactly to DOUBLE (C's implicit conversion of a `double` parameter).
    Double,
    /// Passed in its own type (`func_sqr(*__INTEGER_I)`, measured from the C++).
    Float,
    /// Cast to its believed type (`func_abs((int16)(…))`, `qbs_str((int16)(…))`).
    AnyNumeric,
    Str,
    /// Converted as a store into an `_UNSIGNED _INTEGER64` (a float rounded half to even to `_INTEGER64` first): the
    /// value `_SHL` and `_SHR` shift. The old compiler writes an integer as it is and a float through `qbr`, and
    /// C++ converts to libqb's `uint64` (measured, `verification\v30_h_shifts`: `_SHR(-1, 1)` is 2^63 - 1).
    UInt64,
    /// Converted as a store into an `_UNSIGNED LONG`: the colours libqb takes as `uint32` (`CLS`, `CIRCLE`,
    /// `PAINT`). The old compiler writes it as it writes a LONG slot, an integer as it is and a float through
    /// `qbr`, and C++ keeps the low 32 bits (measured, design D2 of `m3-window-output`).
    ULong,
    /// The fill of `PAINT`: a number (an integer as it is, a float rounded half to even to an `_INTEGER64`) or a
    /// string as it is. The emitter writes the old compiler's cast before it, `(uint32)` or `(qbs*)`, which picks
    /// one of libqb's two `sub_paint` and, for a number, keeps its low 32 bits. The old compiler's cast stands
    /// before the first operand of an expression only (`(uint32)*__LONG_L+ 1`); that gives the same 32 bits as a
    /// cast of the whole, which is what is written here: its infix operators at that level are `+`, `-`, `*` and
    /// the bitwise ones, and every other operation is a call (measured, design D2 of `m3-window-output`).
    ColorOrTile,
    /// The array of graphics `GET` and `PUT`: no value but the bytes of an array from one of its elements to
    /// its end, or of the whole array ([`crate::StmtArg::Block`]; design D5 of `m3-get-put-bload`).
    Block,
    /// The variable of `GET` and `PUT` on a file: no value but the bytes of one place
    /// ([`crate::StmtArg::Bytes`]) or of a whole array ([`crate::StmtArg::Block`] with no index); design D2 of
    /// `m3-console-files`.
    Bytes,
    /// Converted as a store into an `_OFFSET` (a float rounded half to even): the sizes and offsets of memory blocks
    /// (`func__memnew(qbr( 1.5E+0 ))`, measured, `verification\v32_x01_accepted`).
    Offset,
    /// A block: no value but a place of the predefined type `_MEM` (a variable, an element, a member), passed as its
    /// address (design D4 of `m3-mem`): an [`ExprKind::Load`] of the place in a function's slot, a
    /// [`crate::StmtArg::Place`] in a statement's. Anything else is the old compiler's "TYPE '_MEM' required for
    /// function" (measured, `verification\v32_x02`, `x67`–`x73`).
    Mem,
}

impl Slot {
    fn from_table(t: &str) -> Slot {
        match t {
            "LONG" => Slot::Long,
            // The condition of `_ASSERT`, the one supported built-in with such a slot.
            "INTEGER" => Slot::Integer,
            // The records of `LOCK` and `UNLOCK`.
            "_INTEGER64" => Slot::Int64,
            "_UNSIGNED LONG" => Slot::ULong,
            "_UNSIGNED _INTEGER64" => Slot::UInt64,
            "DOUBLE" => Slot::Double,
            // SINGLE (`POINT`): passed in its own type as a `_FLOAT` slot is; libqb's parameter is a `float` either
            // way and C++ converts (measured: `func_point(*__INTEGER64_Q,*__DOUBLE_D,0|1)`).
            "_FLOAT" | "SINGLE" => Slot::Float,
            "any-numeric" => Slot::AnyNumeric,
            "STRING" => Slot::Str,
            "_OFFSET" => Slot::Offset,
            "_MEM" => Slot::Mem,
            // The array of `_UCHARPOS`, the one function with such a slot.
            _ if t.starts_with("offset+size (largest safe block") => Slot::Block,
            other => unreachable!("slot type {other} of a supported built-in"),
        }
    }
}

/// The held and believed types of a call's result, from its (converted) arguments; `None` slots are absent.
pub(crate) fn result_types(s: Supported, args: &[Option<Expr>]) -> (Ty, Ty) {
    let b = s.id.get();
    let first = || {
        args[0]
            .as_ref()
            .expect("the first argument of a typed-by-argument built-in")
    };
    match s.rule {
        Rule::Plain | Rule::GivenArgs(_) => {
            let qb = table_ty(b);
            let held = if b.callname.starts_with("std::") {
                overload(args.iter().flatten().map(|a| a.ty))
            } else {
                held_override(b.callname).unwrap_or(qb)
            };
            (held, qb)
        }
        Rule::ResultOfArg => (first().qb, first().qb),
        Rule::IntFix => (first().ty, first().qb),
        Rule::FloatByArg => {
            let a = first();
            // A float keeps its rank; an integer of 16 bits or fewer is SINGLE, of 32 DOUBLE, wider `_FLOAT`; a
            // `_BIT * n` counts n bits (measured, `v21_d_builtins`: `SQR` of a `_BIT * 3` is SINGLE).
            let qb = match (a.qb.float_rank(), qb_bits(a.qb)) {
                (Some(1), _) => Ty::F32,
                (Some(2), _) => Ty::F64,
                (Some(_), _) => Ty::F80,
                (None, Some(b)) if b <= 16 => Ty::F32,
                (None, Some(b)) if b <= 32 => Ty::F64,
                (None, Some(_)) => Ty::F80,
                (None, None) => unreachable!("a numeric argument"),
            };
            // `std::sin` and the others follow C++ overloading; `func_sqr` and `func_log` take a `double`.
            let held = if b.callname.starts_with("std::") {
                overload([a.ty])
            } else {
                Ty::F64
            };
            (held, qb)
        }
        Rule::Exp => {
            if exp_single(first().qb) {
                (Ty::F64, Ty::F32)
            } else {
                (Ty::F80, Ty::F80)
            }
        }
        // `_ROUND` of an `_OFFSET` keeps its type (`qb64pe.bas` 21129; measured, `v21_d_builtins`).
        Rule::Convert(Ty::I64) if matches!(first().qb, Ty::Off | Ty::UOff) => (first().ty, first().qb),
        Rule::Convert(to) => (convert_held(to, first()), to),
        Rule::Fixed(t) => (t, t),
        Rule::Len | Rule::Asc => (Ty::I32, Ty::I32),
        Rule::Radix(_) | Rule::StringFill | Rule::StrOrIndex => (Ty::Str, Ty::Str),
        Rule::Val => unreachable!("typed by its type argument (check::builtins)"),
        Rule::Widest | Rule::Rotate | Rule::Choice => {
            unreachable!("typed by what its arguments are believed (check::builtins)")
        }
        Rule::Own => unreachable!("an expression of its own (check::mem)"),
        Rule::Mk(_) | Rule::Cv(_) => unreachable!("typed by their type (check::mem)"),
    }
}

/// The C++ arithmetic type the old compiler names for a type in a template argument (`std::min<int16_t>`,
/// `func__rol<uint8_t>`; its `Type_GetCppArithmeticType`): a `_BIT * n` by its storage, an `_OFFSET` as a pointer-
/// sized integer.
pub fn cpp_arithmetic(t: Ty) -> &'static str {
    match t {
        Ty::I8 => "int8_t",
        Ty::U8 => "uint8_t",
        Ty::I16 => "int16_t",
        Ty::U16 => "uint16_t",
        Ty::I32 => "int32_t",
        Ty::U32 => "uint32_t",
        Ty::I64 => "int64_t",
        Ty::U64 => "uint64_t",
        Ty::Off => "intptr_t",
        Ty::UOff => "uintptr_t",
        Ty::Bit { .. } => cpp_arithmetic(t.storage()),
        Ty::F32 => "float",
        Ty::F64 => "double",
        Ty::F80 => "long double",
        Ty::Str | Ty::User(_) => unreachable!("an arithmetic type, not {t:?}"),
        crate::place_only_types!() => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
    }
}

/// The held type of a plain function whose libqb entry returns another C++ type than the table's return type says
/// (the type the old compiler believes): `func_lof`, `func_loc` and `func_seek` return `int64` and are believed
/// LONG, so arithmetic on them is computed in 64 bits (libqb's prototypes; `study\00` §5, "File functions").
fn held_override(callname: &str) -> Option<Ty> {
    match callname {
        "func_lof" | "func_loc" | "func_seek" => Some(Ty::I64),
        "func_timer" => Some(Ty::F64),
        // Believed `_FLOAT`; libqb's prototypes take and return a `double` (`extended_math.h`).
        "func_cot" | "func_csc" | "func_sec" | "func_deg2rad" | "func_rad2deg" => Some(Ty::F64),
        _ => None,
    }
}

/// The C++ result type of a `std::` math function for arguments of these held types: `long double` if any is,
/// `float` if all are `float`, else `double` (integers convert to `double`).
fn overload(args: impl IntoIterator<Item = Ty>) -> Ty {
    let mut all_float = true;
    let mut any_long = false;
    for t in args {
        all_float &= t == Ty::F32;
        any_long |= t == Ty::F80;
    }
    if any_long {
        Ty::F80
    } else if all_float {
        Ty::F32
    } else {
        Ty::F64
    }
}

/// The held type of `CINT`, `CLNG`, `CSNG`, `CDBL`, `_ROUND` of an argument, by the libqb entry the old compiler
/// picks for the argument's believed type (`rounding.h`; measured from the C++): the argument itself where no entry
/// is called.
fn convert_held(to: Ty, a: &Expr) -> Ty {
    let wide_float = a.qb == Ty::F80;
    match (to, a.qb.is_float()) {
        // `func_cint_float` returns `int64`, `func_cint_double` `int32`; `func_cint_long`/`_ulong`/`_int64`/`_uint64`
        // `int16`.
        (Ty::I16, true) => {
            if wide_float {
                Ty::I64
            } else {
                Ty::I32
            }
        }
        // `func_clng_float` returns `int64`, `func_clng_double`, `_ulong`, `_int64` and `_uint64` `int32`.
        (Ty::I32, true) => {
            if wide_float {
                Ty::I64
            } else {
                Ty::I32
            }
        }
        (Ty::I16 | Ty::I32, false) => {
            if int_entry(to, a.qb).is_some() {
                to
            } else {
                a.ty
            }
        }
        // `CSNG` of a SINGLE and `CDBL` of a SINGLE or DOUBLE emit the argument itself; the entries return `double`;
        // an integer is `(double)(e)` (not narrowed, `v20_b_result_types`).
        (Ty::F32, true) if a.qb == Ty::F32 => a.ty,
        (Ty::F64, true) if a.qb != Ty::F80 => a.ty,
        (Ty::F32 | Ty::F64, _) => Ty::F64,
        // `func_round_*` return `int64`; an integer is emitted as it is.
        (Ty::I64, true) => Ty::I64,
        (Ty::I64, false) => a.ty,
        (
            Ty::I8
            | Ty::U8
            | Ty::U16
            | Ty::U32
            | Ty::U64
            | Ty::Off
            | Ty::UOff
            | Ty::Bit { .. }
            | Ty::F80
            | Ty::Str
            | Ty::User(_),
            _,
        ) => unreachable!("no conversion function to {to:?}"),
        (crate::place_only_types!(), _) => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
    }
}

/// The width in bits the old compiler sees in a value believed `t`: a `_BIT * n`'s n (its type's bits, not its
/// storage's), any other integer's [`Ty::int_bits`]; `None` for a float.
pub fn qb_bits(t: Ty) -> Option<u32> {
    match t {
        Ty::Bit { width, .. } => Some(u32::from(width)),
        Ty::I8
        | Ty::U8
        | Ty::I16
        | Ty::U16
        | Ty::I32
        | Ty::U32
        | Ty::I64
        | Ty::U64
        | Ty::Off
        | Ty::UOff
        | Ty::F32
        | Ty::F64
        | Ty::F80
        | Ty::Str
        | Ty::FixedStr(_)
        | Ty::User(_) => t.int_bits(),
    }
}

/// Whether `EXP` of a value believed `t` is SINGLE (`func_exp_single`): a SINGLE, or an integer of 16 bits or
/// fewer that is not a `_BIT` (`qb64pe.bas` 21066–21084; measured, `v21_d_builtins`: a `_BIT * 3` is `_FLOAT`).
pub fn exp_single(t: Ty) -> bool {
    match t {
        Ty::F32 => true,
        Ty::Bit { .. } | Ty::F64 | Ty::F80 => false,
        Ty::I8 | Ty::U8 | Ty::I16 | Ty::U16 | Ty::I32 | Ty::U32 | Ty::I64 | Ty::U64 | Ty::Off | Ty::UOff => {
            t.int_bits().is_some_and(|b| b <= 16)
        }
        Ty::Str | Ty::User(_) => unreachable!("a numeric argument"),
        crate::place_only_types!() => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
    }
}

/// The libqb entry `CINT` (`to` INTEGER) or `CLNG` (`to` LONG) calls for an integer believed `qb`, `None` where the
/// old compiler writes the value itself (`qb64pe.bas` 21172–21215): `CINT` checks the range of an unsigned value of
/// 16 bits or more and of a signed one wider than 16, `CLNG` of an unsigned one of 32 bits or more and of a signed
/// one wider than 32 (measured, `v21_d_builtins`: `CINT` of a 65535 `_UNSIGNED INTEGER` raises error 6).
pub fn int_entry(to: Ty, qb: Ty) -> Option<&'static str> {
    let bits = qb_bits(qb).unwrap_or_else(|| unreachable!("an integer argument, not {qb:?}"));
    let unsigned = qb.is_unsigned();
    match (to, unsigned) {
        (Ty::I16, true) if bits > 32 => Some("func_cint_uint64"),
        (Ty::I16, true) if bits > 15 => Some("func_cint_ulong"),
        (Ty::I16, false) if bits > 32 => Some("func_cint_int64"),
        (Ty::I16, false) if bits > 16 => Some("func_cint_long"),
        (Ty::I32, true) if bits > 32 => Some("func_clng_uint64"),
        (Ty::I32, true) if bits == 32 => Some("func_clng_ulong"),
        (Ty::I32, false) if bits > 32 => Some("func_clng_int64"),
        (Ty::I16 | Ty::I32, _) => None,
        _ => unreachable!("`CINT` or `CLNG` only, not {to:?}"),
    }
}

/// The table's return type of a plain function.
fn table_ty(b: &Builtin) -> Ty {
    match b.ret {
        Some("INTEGER") => Ty::I16,
        Some("LONG") => Ty::I32,
        // The `SCREEN` function (measured: `qbs_str((uint32)(func_screen(…)))`).
        Some("_UNSIGNED LONG") => Ty::U32,
        Some("_INTEGER64") => Ty::I64,
        // `_SHL`, `_SHR`.
        Some("_UNSIGNED _INTEGER64") => Ty::U64,
        Some("SINGLE") => Ty::F32,
        Some("DOUBLE") => Ty::F64,
        Some("_FLOAT") => Ty::F80,
        Some("STRING") => Ty::Str,
        Some("_UNSIGNED _OFFSET") => Ty::UOff,
        other => unreachable!("return type {other:?} of a plain supported built-in"),
    }
}

/// The width a negative value is printed with by `HEX$`, `OCT$` or `_BIN$` (D4, `qb64pe.bas` 20978–21062): from the
/// argument's believed type, 16 bits 4 hex digits and 32 bits 8; 64 bits 16 only for a place (variable, element,
/// member), 0 (the shortest form) for any other expression. For `OCT$` and `_BIN$` the number of bits. `None` for a
/// float argument (the `_float` entries take no width).
///
/// A `_BIT * n` (a place or a literal) has n bits, `HEX$` (n + 3) \ 4 digits, whether a place or not (measured,
/// `v21_d_builtins`: `HEX$` of a `_BIT * 3` holding -4 is `C`; the numeric-semantics scenario `HEX$(-1`5)` is `FF`).
pub fn radix_width(bits_per_digit: u32, arg: &Expr) -> Option<u32> {
    if let Ty::Bit { width, .. } = arg.qb {
        let bits = u32::from(width);
        return Some(if bits_per_digit == 4 { bits.div_ceil(4) } else { bits });
    }
    let bits = match qb_bits(arg.qb) {
        None => return None,
        Some(64) if matches!(arg.kind, ExprKind::Load(_)) => 64,
        Some(64) => 0,
        Some(b) => b,
    };
    Some(if bits_per_digit == 4 { bits / 4 } else { bits })
}

#[cfg(test)]
mod tests {
    use super::*;
    use qb64rust_base::{FileId, Span};

    #[test]
    fn every_row_is_in_the_table_once() {
        let mut names: Vec<&str> = SUPPORTED.iter().map(|(n, _)| *n).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), SUPPORTED.len(), "a row is listed twice");
        for s in supported() {
            let b = s.id.get();
            assert_eq!(b.musthave == Some("$"), s.name.ends_with('$'), "{}", s.name);
            assert_eq!(rule(s.id), Some(s.rule));
        }
    }

    #[test]
    fn every_statement_row_is_in_the_table_once() {
        // A name may have a row per rule (`GET`: the graphics form and the file form), never two of one rule.
        let mut rows: Vec<String> = STATEMENTS.iter().map(|(n, r)| format!("{n} {r:?}")).collect();
        rows.sort_unstable();
        rows.dedup();
        assert_eq!(rows.len(), STATEMENTS.len(), "a row is listed twice");
        for &(written, rule) in STATEMENTS {
            let bare = written.strip_suffix('$').unwrap_or(written);
            let forms = stmt_lookup(bare, bare.len() != written.len());
            assert!(!forms.is_empty(), "{written} has no statement entry in the table");
            for s in forms {
                // A name with two rules gives the forms of both; each form has the rule of its own row.
                if s.rule != rule {
                    continue;
                }
                assert_eq!(stmt_find(s.id).map(|f| f.rule), Some(rule), "{written}");
                // Every argument has a table type, and the table has no more types than C arguments and the mask.
                let slots = stmt_slots(s.id);
                let b = s.id.get();
                let p = plan(&b.template());
                let passed = p.parts.iter().filter(|part| part.passed).count();
                assert!(
                    b.arg_types.len() == passed || b.arg_types.len() == passed + usize::from(p.mask),
                    "{written}: {} table types for {passed} C arguments",
                    b.arg_types.len()
                );
                assert_eq!(slots.len(), p.parts.len());
            }
        }
        assert!(stmt_lookup("KILL", true).is_empty());
        assert!(stmt_lookup("NOPE", false).is_empty());
        let name = stmt_lookup("NAME", false);
        assert_eq!(name.len(), 1);
        assert_eq!(
            stmt_slots(name[0].id),
            [
                StmtSlot::Arg {
                    slot: Slot::Str,
                    optional: false
                },
                StmtSlot::Choice {
                    alts: 1,
                    optional: false
                },
                StmtSlot::Arg {
                    slot: Slot::Str,
                    optional: false
                },
            ]
        );
    }

    /// The slot kinds of the screen statements (design D2 of `m3-window-output`): `_UNSIGNED LONG` in the table is
    /// [`Slot::ULong`], SINGLE and `_FLOAT` are [`Slot::Float`], and the fill of `PAINT` takes a number or a string.
    #[test]
    fn screen_slot_kinds() {
        let args = |name: &str, form: usize| -> Vec<Slot> {
            stmt_slots(stmt_lookup(name, false)[form].id)
                .into_iter()
                .filter_map(|s| match s {
                    StmtSlot::Arg { slot, .. } => Some(slot),
                    StmtSlot::Choice { .. } => None,
                })
                .collect()
        };
        assert_eq!(args("CLS", 0), [Slot::Long, Slot::ULong, Slot::Long]);
        assert_eq!(
            args("CIRCLE", 0),
            [
                Slot::Float,
                Slot::Float,
                Slot::Float,
                Slot::ULong,
                Slot::Float,
                Slot::Float,
                Slot::Float
            ]
        );
        assert_eq!(
            args("PAINT", 0),
            [Slot::Float, Slot::Float, Slot::ColorOrTile, Slot::ULong, Slot::Str]
        );
        assert_eq!(args("_LIMIT", 0), [Slot::Double]);
        // `VIEW` is two statements: the graphics viewport (six LONG slots) and `VIEW PRINT` (two).
        assert_eq!(stmt_lookup("VIEW", false).len(), 2);
        assert_eq!(args("VIEW", 0), [Slot::Long; 6]);
        assert_eq!(args("VIEW", 1), [Slot::Long; 2]);
        // The functions: `POINT` takes its two arguments as they are, the second optional; `SCREEN` is unsigned.
        let point = lookup("POINT", false).unwrap();
        assert_eq!(slots(point), [(Slot::Float, false), (Slot::Float, true)]);
        assert_eq!(result_types(point, &[None, None]), (Ty::F64, Ty::F64));
        let screen = lookup("SCREEN", false).unwrap();
        assert_eq!(
            slots(screen),
            [(Slot::Long, false), (Slot::Long, false), (Slot::Long, true)]
        );
        assert_eq!(result_types(screen, &[None, None, None]), (Ty::U32, Ty::U32));
        assert_eq!(result_types(lookup("CSRLIN", false).unwrap(), &[]), (Ty::I32, Ty::I32));
        assert_eq!(result_types(lookup("INKEY", true).unwrap(), &[]), (Ty::Str, Ty::Str));
    }

    #[test]
    fn lookup_needs_the_suffix_as_written() {
        assert!(lookup("CHR", true).is_some());
        assert!(lookup("CHR", false).is_none());
        assert!(lookup("INSTR", false).is_some());
        assert!(lookup("INSTR", true).is_none());
        assert!(lookup("NOPE", false).is_none());
    }

    fn e(ty: Ty, qb: Ty, kind: ExprKind) -> Expr {
        Expr {
            span: Span::new(FileId(0), 0, 0),
            ty,
            qb,
            kind,
        }
    }

    #[test]
    fn std_overloads() {
        assert_eq!(overload([Ty::F32, Ty::F32]), Ty::F32);
        assert_eq!(overload([Ty::F32, Ty::I16]), Ty::F64);
        assert_eq!(overload([Ty::I64]), Ty::F64);
        assert_eq!(overload([Ty::F64, Ty::F80]), Ty::F80);
    }

    #[test]
    fn radix_widths() {
        let var = |t| e(t, t, ExprKind::Load(crate::Place::Var(crate::VarId(0))));
        let op = |t| e(Ty::I32, t, ExprKind::Int(0));
        assert_eq!(radix_width(4, &var(Ty::I16)), Some(4));
        assert_eq!(radix_width(4, &var(Ty::I32)), Some(8));
        assert_eq!(radix_width(4, &var(Ty::I64)), Some(16));
        assert_eq!(radix_width(3, &var(Ty::I64)), Some(64));
        assert_eq!(radix_width(4, &op(Ty::I64)), Some(0));
        assert_eq!(radix_width(1, &op(Ty::I16)), Some(16));
        assert_eq!(radix_width(4, &op(Ty::F32)), None);
    }

    #[test]
    fn new_types_special_cases() {
        let bit = |width, signed| Ty::Bit { width, signed };
        // `_BIT * n` counts its n bits, not its storage's.
        assert_eq!(qb_bits(bit(3, true)), Some(3));
        assert_eq!(qb_bits(Ty::U8), Some(8));
        assert_eq!(qb_bits(Ty::UOff), Some(64));
        assert_eq!(qb_bits(Ty::F32), None);
        // `HEX$`: (n + 3) \ 4 digits for a `_BIT * n`, place or not; 2 for a byte; 16 for a 64-bit place, else 0.
        let var = |t| e(Ty::I64, t, ExprKind::Load(crate::Place::Var(crate::VarId(0))));
        let op = |t| e(Ty::I64, t, ExprKind::Int(0));
        assert_eq!(radix_width(4, &var(bit(3, true))), Some(1));
        assert_eq!(radix_width(4, &op(bit(5, true))), Some(2));
        assert_eq!(radix_width(3, &var(bit(3, false))), Some(3));
        assert_eq!(radix_width(4, &var(Ty::U8)), Some(2));
        assert_eq!(radix_width(1, &var(Ty::I8)), Some(8));
        assert_eq!(radix_width(4, &var(Ty::Off)), Some(16));
        assert_eq!(radix_width(4, &op(Ty::U64)), Some(0));
        // `EXP`: SINGLE up to 16 bits, never for a `_BIT`.
        assert!(exp_single(Ty::U16) && exp_single(Ty::I8) && exp_single(Ty::F32));
        assert!(!exp_single(Ty::U32) && !exp_single(bit(3, true)) && !exp_single(Ty::F64));
        // `CINT`: unsigned from 16 bits, signed above 16; `CLNG`: unsigned at 32 or more, signed above 32.
        assert_eq!(int_entry(Ty::I16, Ty::U8), None);
        assert_eq!(int_entry(Ty::I16, Ty::U16), Some("func_cint_ulong"));
        assert_eq!(int_entry(Ty::I16, Ty::I16), None);
        assert_eq!(int_entry(Ty::I16, Ty::U32), Some("func_cint_ulong"));
        assert_eq!(int_entry(Ty::I16, Ty::UOff), Some("func_cint_uint64"));
        assert_eq!(int_entry(Ty::I16, bit(20, true)), Some("func_cint_long"));
        assert_eq!(int_entry(Ty::I16, bit(10, true)), None);
        assert_eq!(int_entry(Ty::I32, Ty::U16), None);
        assert_eq!(int_entry(Ty::I32, Ty::U32), Some("func_clng_ulong"));
        assert_eq!(int_entry(Ty::I32, bit(20, false)), None);
        assert_eq!(int_entry(Ty::I32, Ty::U64), Some("func_clng_uint64"));
        assert_eq!(int_entry(Ty::I32, Ty::Off), Some("func_clng_int64"));
        // The held type is the entry's (`int16`, `int32`) or the argument's own where none is called.
        assert_eq!(convert_held(Ty::I16, &var(Ty::U16)), Ty::I16);
        assert_eq!(convert_held(Ty::I16, &e(Ty::U8, Ty::U8, ExprKind::Int(0))), Ty::U8);
    }

    #[test]
    fn conversion_held_types() {
        let lit = |ty, qb| e(ty, qb, ExprKind::Int(0));
        // CINT of a DOUBLE: func_cint_double, int32; of a _FLOAT: int64; of a LONG: int16; of an INTEGER: itself.
        assert_eq!(convert_held(Ty::I16, &lit(Ty::F64, Ty::F64)), Ty::I32);
        assert_eq!(convert_held(Ty::I16, &lit(Ty::F80, Ty::F80)), Ty::I64);
        assert_eq!(convert_held(Ty::I16, &lit(Ty::I32, Ty::I32)), Ty::I16);
        assert_eq!(convert_held(Ty::I16, &lit(Ty::I16, Ty::I16)), Ty::I16);
        // CSNG of an integer is (double), not narrowed; of a SINGLE literal (held DOUBLE) the literal itself.
        assert_eq!(convert_held(Ty::F32, &lit(Ty::I32, Ty::I32)), Ty::F64);
        assert_eq!(convert_held(Ty::F32, &lit(Ty::F64, Ty::F32)), Ty::F64);
        assert_eq!(convert_held(Ty::F64, &lit(Ty::F32, Ty::F32)), Ty::F32);
        // CLNG of an _INTEGER64: func_clng_int64, int32; of an INTEGER: itself.
        assert_eq!(convert_held(Ty::I32, &lit(Ty::I64, Ty::I64)), Ty::I32);
        assert_eq!(convert_held(Ty::I32, &lit(Ty::I16, Ty::I16)), Ty::I16);
        assert_eq!(convert_held(Ty::I64, &lit(Ty::F32, Ty::F32)), Ty::I64);
    }
}
