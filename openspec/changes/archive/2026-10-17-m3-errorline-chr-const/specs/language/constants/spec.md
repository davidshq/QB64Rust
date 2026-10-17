# Spec Delta

## MODIFIED Requirements

### Requirement: CONST values
A constant's value SHALL be computed when compiling, as the old compiler's constant evaluator does: integer
arithmetic in 64 bits, `/` in floating point, `^` right-associative, comparisons giving -1 or 0, string `+` on
string constants. An integer result, and a floating-point result with an integer value within `_INTEGER64` range,
SHALL be typed `_INTEGER64`; any other floating-point result DOUBLE, also an integer power outside `_INTEGER64`
range (the old evaluator wraps it, `DIVERGENCES.md` D-007). A floating-point suffix on the name SHALL convert the
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
wrong kind) SHALL be a compile error, and so SHALL a `/` by 0 (the old evaluator gives 0, D-007); a function from
its list other than `CHR$` and a floating-point value the compiler cannot show to equal the old evaluator's `_FLOAT` result SHALL be
"not supported yet". A floating-point result outside `_INTEGER64` range SHALL be DOUBLE. `CHR$(code)` SHALL
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

### Requirement: Constants of the auto-included file
Every program SHALL know the constants QB64pe declares in its auto-included file `beforefirstline.bi` (the
Windows 64-bit branches), as if their `CONST` lines stood above the program's first line: visible in the main
module and in every procedure, each with the value and type the `CONST` evaluator gives its expression. A constant
whose expression the evaluator does not compute yet SHALL be reported "not supported yet" where it is used, and
SHALL NOT affect a program that does not use it.

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
- **WHEN** a program uses `_E`, whose `CONST` line needs `EXP`
- **THEN** the only diagnostic is "not supported yet" at that use

#### Scenario: A value made with CHR$
- **WHEN** `PRINT LEN(_STR_CRLF); ASC(_STR_CRLF, 2); ASC(_CHR_ESC)` runs
- **THEN** it prints ` 2  10  27 `

#### Scenario: Not used
- **WHEN** a program names no constant of the auto-included file
- **THEN** it compiles as before, with no diagnostic about them
