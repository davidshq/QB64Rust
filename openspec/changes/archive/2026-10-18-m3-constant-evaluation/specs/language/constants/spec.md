# Spec Delta

## MODIFIED Requirements

### Requirement: CONST values
A constant's value SHALL be computed when compiling, as the old compiler's constant evaluator does: integer
arithmetic in 64 bits, `/` in floating point, `^` right-associative, comparisons giving -1 or 0, string `+` on
string constants. An integer result, and a floating-point result with an integer value within `_INTEGER64` range,
SHALL be typed `_INTEGER64`; any other floating-point result is a float typed by its text (below), also an integer
power outside `_INTEGER64` range (the old evaluator wraps it, `DIVERGENCES.md` D-007). A floating-point suffix on the name SHALL convert the
value to that type as an assignment does. An integer suffix, `_BIT` suffixes included, SHALL make the constant
behave as a literal with that suffix (`numeric-semantics`, "Integer literal typing") whose digits are the
evaluator's value, rounded half to even if it is a float, and for an unsigned suffix a negative value written as
its unsigned 64-bit value, as the old compiler writes it: `CONST b~%% = -1` prints 255, `b~%% + 0` is -1 and
`b~%% < 0` is false; `CONST c%% = 200` prints -56 and `c%% + 0` is 200; `` CONST j~`3 = -1 `` prints -1
(`DECISIONS.md`, 2026-10-08; `verification\v21_d_const`). An `_OFFSET` suffix makes it an `_INTEGER64` literal of its
signedness (`CONST k~%& = -1`: `k~%& + 0` is -1, as the old compiler writes `18446744073709551615ull`). A constant
used with an integer suffix other than its own SHALL be such a literal of the use's suffix, rounded half to even if
a float (measured, `verification\v21_g_const_suffix`: with `CONST u~& = 4294967295`, `u%` prints -1 and `u% + 0` is
4294967295; with `CONST n = -1`, `n~&` prints 4294967295 and `n~& < 0` is false); a constant used with a `_BIT`
suffix other than its own, or a `_BIT` constant with another suffix, SHALL be "not supported yet" (not measured). A
suffixed literal inside a `CONST` expression is its held value (`CONST r = 300~%%` is 300). An expression the old evaluator rejects (a variable, a string mixed
with a number, a string comparison, a function outside its list, integer division or `MOD` by 0, a suffix of the
wrong kind) SHALL be a compile error, and so SHALL a `/` by 0 (the old evaluator gives 0, D-007). Floating-point
values SHALL be computed as the old evaluator computes them, in `_FLOAT` (a 64-bit mantissa), each parenthesised
group, function result and the whole value printed to 19 significant digits and read back, so that a constant
computed from literals, constants and the operators has the old compiler's value to the bit. A floating-point
result whose 19-digit text has an exponent SHALL be typed `_FLOAT`, one without DOUBLE; an unsigned result
`_UNSIGNED _INTEGER64` (measured, `verification\v45_*`). `CHR$(code)` SHALL
give the one-byte string of the code, the code computed as a parenthesised group is and, when a float,
rounded half to even; a code outside 0 to 255 after the rounding, a string as the code, or another number of
codes than one SHALL be a compile error (measured, `verification\v43_c_chr_const`, `v43_x01`–`x28`).

#### Scenario: Float beyond _INTEGER64
- **WHEN** `CONST a = 1E+19 / 1: PRINT a` runs
- **THEN** it prints ` 1D+19 `

#### Scenario: Integer-valued constant is 64-bit
- **WHEN** `CONST i3 = 3, f2 = 4 / 2: PRINT i3 * 1000000000; f2 * 4611686018427387904` runs
- **THEN** it prints ` 3000000000 -9223372036854775808 ` (`3 * 1000000000` with the literal wraps at 32 bits)

#### Scenario: Right-associative power
- **WHEN** `CONST c = 2 ^ 3 ^ 2: PRINT c; 2 ^ 3 ^ 2` runs
- **THEN** it prints ` 512  64 ` (the run-time `^` is left-associative)

#### Scenario: Suffix rounds
- **WHEN** `CONST c% = 3.7: PRINT c%` runs
- **THEN** it prints ` 4 `

#### Scenario: Unsigned suffix
- **WHEN** `CONST u~% = 65535: PRINT u~%; u~% + 1` runs
- **THEN** it prints ` 65535  65536 `

#### Scenario: Suffix with a value out of range
- **WHEN** `CONST u~% = -1, c~%% = 300: PRINT u~%; u~% + 0; u~% < 0; c~%%; c~%% + 0` runs
- **THEN** it prints ` 65535 -1  0  44  300 `

#### Scenario: Bit suffix
- **WHEN** ``CONST i`3 = 5, j~`3 = -1: PRINT i`3; j~`3; i`3 + 0`` runs
- **THEN** it prints ` 5 -1  5 `

#### Scenario: Constant used with another suffix
- **WHEN** `CONST n = -1, s = 300, u~& = 4294967295: PRINT n~&; n~& + 0; s%%; s%% + 0; u%; u% + 0` runs
- **THEN** it prints ` 4294967295 -1  44  300 -1  4294967295 `

#### Scenario: Assignment to a constant
- **WHEN** a program has `CONST c = 1` and later `c = 2`
- **THEN** it is a compile error

#### Scenario: Division by zero
- **WHEN** `CONST c = 1 / 0` is compiled
- **THEN** it is a compile error (not "not supported yet")

#### Scenario: Power beyond _INTEGER64
- **WHEN** `CONST c = 2 ^ 70: PRINT c` runs
- **THEN** it prints ` 1.180591620717411D+21 `

#### Scenario: CHR$ in a CONST
- **WHEN** `CONST a = CHR$(65) + CHR$(65.5) + CHR$(66.5) + CHR$(-0.5): PRINT ASC(a, 2); ASC(a, 3); ASC(a, 4)`
  runs
- **THEN** it prints ` 66  66  0 `

#### Scenario: CHR$ of a code out of range
- **WHEN** `CONST a = CHR$(255.5)` or `CONST a = CHR$(-0.6)` is compiled
- **THEN** it is a compile error (not "not supported yet")

#### Scenario: Extended precision
- **WHEN** `CONST third## = 1## / 3##: PRINT _TOSTR$(third##)` runs
- **THEN** it prints `0.3333333333333333333`

#### Scenario: A rounded value in another operation
- **WHEN** `CONST t = 0.1 * 3: PRINT _TOSTR$(t)` runs
- **THEN** it prints what the old compiler's program prints (no "not supported yet")

#### Scenario: A float typed by its text
- **WHEN** `CONST c = 1E+30 / 3, d = 1 / 3: PRINT _TOSTR$(c); " "; _TOSTR$(d)` runs
- **THEN** it prints `3.333333333333333333F+29 0.3333333333333333` (`c` is `_FLOAT`, `d` DOUBLE)

#### Scenario: Unsigned lane
- **WHEN** `CONST a = 18446744073709551615~&&, b = a + 1, u = a - 1: PRINT a; b; u` runs
- **THEN** it prints ` 18446744073709551615  0  18446744073709551614 `

#### Scenario: Unsuffixed literal beyond _INTEGER64
- **WHEN** `CONST big~&& = 18446744073709551615: PRINT big~&&` runs
- **THEN** it prints ` 9223372036854775807 `, as with the old compiler (the literal is read as an `_INTEGER64`
  and clamped)

### Requirement: Constants of the auto-included file
Every program SHALL know the constants QB64pe declares in its auto-included file `beforefirstline.bi` (the
Windows 64-bit branches), as if their `CONST` lines stood above the program's first line: visible in the main
module and in every procedure, each with the value and type the `CONST` evaluator gives its expression, the
limits whose literals the old evaluator clamps included (`_INTEGER64_MIN` is -9223372036854775807,
`_UINTEGER64_MAX` 9223372036854775807). A constant whose expression has a form the compiler does not read SHALL be
reported "not supported yet" where it is used, and SHALL NOT affect a program that does not use it.

#### Scenario: _TRUE and _FALSE
- **WHEN** `PRINT _TRUE; _FALSE` runs
- **THEN** it prints `-1  0 `, as with the old compiler

#### Scenario: In a procedure
- **WHEN** a SUB runs `IF _TRUE THEN PRINT _SECS_IN_MIN * _MINS_IN_HOUR`
- **THEN** it prints ` 3600 `

#### Scenario: A constant computed from other constants
- **WHEN** `PRINT _ONE_MB` runs
- **THEN** it prints what the old compiler's program prints for it

#### Scenario: A value not computed yet
- **WHEN** a program uses an auto-included constant whose expression has a form the compiler does not read (none
  in QB64pe 4.7.0's file)
- **THEN** the only diagnostic is "not supported yet" at that use

#### Scenario: A value made with EXP
- **WHEN** `PRINT _TOSTR$(_E); _E` runs
- **THEN** it prints `2.718281828459045235 2.718281828459045 `

#### Scenario: A value made with CHR$
- **WHEN** `PRINT LEN(_STR_CRLF); ASC(_STR_CRLF, 2); ASC(_CHR_ESC)` runs
- **THEN** it prints ` 2  10  27 `

#### Scenario: The limits of the types
- **WHEN** `PRINT _INTEGER64_MIN; _UINTEGER64_MAX; _TOSTR$(_DOUBLE_MAX); " "; _TOSTR$(_FLOAT_MAX)` runs
- **THEN** it prints `-9223372036854775807  9223372036854775807 1.797693134862315D+308 1.189731495357231765F+4932`

#### Scenario: Not used
- **WHEN** a program names no constant of the auto-included file
- **THEN** it compiles as before, with no diagnostic about them

## ADDED Requirements

### Requirement: Functions in a CONST
Every function of the old evaluator's list SHALL be computed in a `CONST`, with the old evaluator's argument
counts, argument kinds and errors. A function the old compiler computes exactly SHALL have its value. Where the old
value depends on the CPU and system library (`SIN`, `LOG`, `ROOT` …), the constant SHALL have the value every
result that library could give leads to, or, when they lead to different constants, the value of correctly
rounded calls with a warning (`DIVERGENCES.md` D-026).

#### Scenario: Exact functions
- **WHEN** `CONST a = ABS(2), s = SGN(-20), i = INT(20.2), r = _ROUND(20.2), c = _CEIL(20.2), f = FIX(20.5)` and
  `PRINT a; s; i; r; c; f` run
- **THEN** it prints ` 2 -1  20  20  21  20 `

#### Scenario: Colour functions
- **WHEN** `CONST k = _RGB32(2, 3, 4), m = _RGBA32(2, 3, 4, 5), b = _BLUE32(22): PRINT HEX$(k); " "; HEX$(m); b`
  runs
- **THEN** it prints `FF020304 5020304  22 `

#### Scenario: Pi
- **WHEN** `CONST p = _PI, p2 = _PI(2): PRINT _TOSTR$(p); " "; _TOSTR$(p2)` runs
- **THEN** it prints `3.141592653589793 6.283185307179586`

#### Scenario: ASC
- **WHEN** `CONST a = ASC("\"), n = CHR$(ASC(CHR$(ASC("\")))): PRINT a; n` runs
- **THEN** it prints ` 92 \`

#### Scenario: A certified function
- **WHEN** `CONST s = SIN(.2), r = 20 ROOT 3: PRINT _TOSTR$(s); " "; _TOSTR$(r)` runs
- **THEN** it prints `0.1986693307950612 2.714417616594907`, as with the old compiler

#### Scenario: A value that cannot be certified
- **WHEN** a `CONST` uses a function whose possible results, within the bound of the old compiler's library, give
  two different constants (`CONST r = 20 ROOT 3`), and is compiled with `-w`
- **THEN** it compiles with the correctly rounded value and a warning at the `CONST` that QB64pe's value depends on the
  CPU and the system's library

#### Scenario: An argument the old evaluator rejects
- **WHEN** `CONST c = SQR(-1)` or `CONST c = _ARCSEC(0.5)` is compiled
- **THEN** it is a compile error (not "not supported yet")

### Requirement: Suffixed constants inside a CONST
A constant named with a type suffix inside a `CONST` expression SHALL be read as the old evaluator reads it: the
constant whose name it is, whatever the suffix, with the constant's own value and type.

#### Scenario: Suffix on a constant used in a CONST
- **WHEN** `CONST c% = 4 + 4: CONST d = c% * 2: PRINT d` runs
- **THEN** it prints ` 16 `

#### Scenario: Defined with a suffix, used with and without
- **WHEN** `const\const_replace` of the upstream tests is built and run
- **THEN** its output is the recorded one

### Requirement: Colour constants
`$COLOR:0` and `$COLOR:32` SHALL give a program the constants of QB64pe's `color0.bi` or `color32.bi`, as if the
file's lines stood above the program's first line, wherever the metacommand stands; both in one program SHALL be a
compile error, and so SHALL a name of the program that the file's constants take, as with the old compiler.

#### Scenario: Text-mode colours
- **WHEN** a program has `$COLOR:0`, `CONST newGreen = Green` and `PRINT Red; newGreen; Blue`
- **THEN** it prints ` 4  2  1 `

#### Scenario: 32-bit colours
- **WHEN** `auto_include\color32` of the upstream tests is built and run
- **THEN** its output is the recorded one

#### Scenario: Both sets
- **WHEN** a program has `$COLOR:0` and `$COLOR:32`
- **THEN** it is a compile error
