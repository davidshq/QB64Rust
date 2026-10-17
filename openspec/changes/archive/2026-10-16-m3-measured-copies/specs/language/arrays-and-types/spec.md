## MODIFIED Requirements

### Requirement: Whole-array assignment at run time
Before copying, a whole-array assignment SHALL check that both arrays are allocated and that in every dimension they
have the same lower bound and the same number of elements; if not, it SHALL raise error 9 and copy nothing, except
for two array variables of an owner type with other bounds (requirement "Whole-array assignment of owner types").
A static array after `ERASE` SHALL count as allocated. A target with other bounds SHALL NOT be given the source's.

#### Scenario: Other bounds
- **WHEN** `b() = a()` runs for `a(0 TO 5)` and `b(1 TO 6)`, also `c(0 TO 4)`, `d(0 TO 6)`, and two arrays of two
  dimensions whose second differs, under a handler that prints `ERR` and resumes next
- **THEN** the handler prints 9 for each and the target's elements keep their values

#### Scenario: Not allocated
- **WHEN** the target or the source is a dynamic array after `ERASE`, or a `$DYNAMIC` array after `ERASE`
- **THEN** error 9 is raised and nothing is copied

#### Scenario: A static array after ERASE
- **WHEN** `ERASE st: st() = a()` runs for two static arrays of the same bounds
- **THEN** no error is raised and `st` holds the elements of `a`

### Requirement: Whole-array assignment of owner types
Below `$UNSTABLE:TYPEFIELDS` a whole-array assignment of arrays of a type with dynamic member arrays SHALL copy each
element as a whole-value copy does: each dynamic member of the target then has the bounds and elements of the
source's, strings and nested owner types included, for a `DIM` and a `REDIM` target alike. For two array
variables with other bounds or shape it SHALL raise error 9 and then, as the old compiler does, copy the target's
number of elements, element `i` of the target from element `i` of the source in order, the target keeping its
bounds, when the source has at least that many elements; when it has fewer, or either is not allocated, nothing
SHALL be copied (the old compiler reads past the source's block, `DIVERGENCES.md` D-022).

#### Scenario: Members with their own bounds
- **WHEN** `dst() = src()` runs where `src(1).v` was given the bounds 3 to 5 and `src(1).v(4) = 44`, then
  `src(1).v(4) = -1`
- **THEN** `LBOUND(dst(1).v)`, `UBOUND(dst(1).v)` and `dst(1).v(4)` are 3, 5 and 44

#### Scenario: Other bounds, as many or fewer elements in the target
- **WHEN** `d() = s()` runs for `s(1 TO 2)` and `d(0 TO 1)`, also `d0(0 TO 0)`, `r(5 TO 6)` given by `REDIM`, and a
  `n(0 TO 1, 0 TO 1)` from an `m(1 TO 4, 0 TO 0)`, under a handler that prints `ERR`
- **THEN** the handler prints 9 for each, and each target holds the source's first elements in order with its own
  bounds (`verification\v38_d`)

#### Scenario: Other bounds copy nothing
- **WHEN** `e() = s()` runs for `s(1 TO 2)` and `e(1 TO 3)` of an owner type, under a handler that prints `ERR`
- **THEN** the handler prints 9 and every element of `e` keeps its values (the old compiler copies three elements,
  reading past `s`, `DIVERGENCES.md` D-022)
