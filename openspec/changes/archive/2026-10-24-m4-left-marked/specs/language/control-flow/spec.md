# Spec Delta

## MODIFIED Requirements

### Requirement: FOR loops
`FOR v = a TO b [STEP s]` SHALL evaluate `a`, `b` and `s` once, in that order, assign `a` to `v`, and run the body
while `v` has not passed `b` (above `b` for a step of 0 or more, below it for a negative step). `NEXT` SHALL add
`s` to the current value of `v` (changes made in the body count). The comparison SHALL use a value wider than `v`,
after `v` has been assigned, as in the old compiler: `a`, `b` and `s` converted with rounding to a type wider
than `v` (8-bit `_BYTE` or `_UNSIGNED _BYTE` → INTEGER, 16-bit INTEGER or `_UNSIGNED INTEGER` → LONG, every
32-bit and 64-bit integer type, signed or unsigned, and `_OFFSET` → `_INTEGER64`, SINGLE → DOUBLE, DOUBLE and
`_FLOAT` → `_FLOAT`), and the direction taken from the sign of `s` at the header. `v` SHALL be a numeric scalar
variable, or a numeric member of a `TYPE` variable reached through members only (`r.v`, `r.n.v`), as in the old
compiler; a string, a `_BIT` or `_BIT * n` variable, a constant, an array element, a member of an array element
(`a(i).v`), a string member and a whole member array SHALL be a compile error, and so SHALL a string `a`, `b` or
`s`. An element of a member array (`r.m(i)`, `ro.n.m(i)`) SHALL be the variable as in the old compiler: the element
SHALL be found again, its indexes evaluated, wherever the old compiler's loop names it (at the start, and at each
`NEXT` for the step and for the test), so that a change of an index in the body moves the loop to another
element (`verification\v56_f3`). An error raised there SHALL not stop the test, which takes the value, and SHALL
be serviced at the `FOR` line before the body, so that `RESUME NEXT` goes on into the body, as in the old
compiler (`v60_c3`; its store out of range is `DIVERGENCES.md` D-004).

#### Scenario: A member as the variable
- **WHEN** a `TYPE t` has `v AS LONG`, and `DIM r AS t: FOR r.v = 1 TO 2: PRINT r.v: NEXT` runs
  (`verification\v17_c_for_type_member`)
- **THEN** it prints ` 1 ` and ` 2 `

#### Scenario: Members measured
- **WHEN** the programs `verification\v56_f1_for_member` and `v56_f4_for_nested_member` run
- **THEN** each prints what the old compiler's program printed

#### Scenario: Members rejected or marked
- **WHEN** `FOR ar(i).v = 1 TO 3` (`verification\v56_f2`), `FOR r.m = 1 TO 2` or `FOR rs.z = 1 TO 2` for a
  `STRING * 2` member is compiled
- **THEN** each is a compile error that is not "not supported yet"

#### Scenario: An element of a member array
- **WHEN** `i = 1: FOR r.m(i) = 1 TO 3: PRINT r.m(1); r.m(2);: i = 2: NEXT: PRINT r.m(1); r.m(2)` runs, and
  `FOR r.m(idx(3)) = 1 TO 3` with a FUNCTION `idx` that counts its calls (`verification\v56_f3`)
- **THEN** the first prints ` 1  0  1  1  1  2  1  3  1  4 ` and the second runs `idx` 7 times, as the old
  compiler's program

#### Scenario: Limits rounded to the wider type
- **WHEN** `FOR i% = 1 TO 2.6: PRINT i%;: NEXT: PRINT i%` runs
- **THEN** it prints ` 1  2  3  4 `

#### Scenario: _INTEGER64 variable at its maximum
- **WHEN** `FOR q&& = 9223372036854775800 TO 9223372036854775807 STEP 5` runs a body capped at five passes
- **THEN** the third pass sees `-9223372036854775806` (the value wraps and the loop does not end by itself)

#### Scenario: Limits evaluated once
- **WHEN** `e = 3: FOR m = 1 TO e: e = 10: PRINT m;: NEXT` runs
- **THEN** it prints ` 1  2  3 `

#### Scenario: Body changes the variable
- **WHEN** `FOR k = 1 TO 5: k = k + 1: PRINT k;: NEXT` runs
- **THEN** it prints ` 2  4  6 `

#### Scenario: Loop that does not run
- **WHEN** `FOR m = 3 TO 1: PRINT "never": NEXT: PRINT m` runs
- **THEN** it prints ` 3 ` only

#### Scenario: INTEGER variable passes its range
- **WHEN** `FOR i% = 32760 TO 32767 STEP 4: NEXT: PRINT i%` runs
- **THEN** it prints `-32768` and the loop ends (the wider value 32768 is past the limit)

#### Scenario: SINGLE steps
- **WHEN** `FOR s! = 0 TO 1 STEP 0.1: c = c + 1: NEXT: PRINT s!; c` runs
- **THEN** it prints ` 1  10 `

#### Scenario: Unsigned byte passes its range
- **WHEN** `FOR b~%% = 250 TO 255 STEP 3: PRINT b~%%;: NEXT: PRINT b~%%` runs
- **THEN** it prints ` 250  253  0 ` (the wider value 256 is past the limit; the store wraps to 0)

#### Scenario: Unsigned variable counts down through 0
- **WHEN** `FOR u~% = 2 TO 0 STEP -1: PRINT u~%;: NEXT: PRINT u~%` runs
- **THEN** it prints ` 2  1  0  65535 ` (the wider value -1 is past the limit)

#### Scenario: Bit variable
- **WHEN** `DIM t AS _BIT * 8: FOR t = 1 TO 3: NEXT` is compiled
- **THEN** it is a compile error
