# The call-site check

`callsite.py` checks, without building or running anything, that the new compiler calls libqb for a built-in as
the old compiler does: the same entry point, the same arguments and conversions, the same `passed` mask (OpenSpec
change `m2-builtin-statements`, design D8; spec `testing/call-site-check`). It is a trial; the verdict (kept or
dropped) is recorded in `DECISIONS.md` at the end of that change.

```
python tools\callsite\callsite.py                 # every program of tests\callsite
python tools\callsite\callsite.py --glob "kill*"  # some of them
python tools\callsite\callsite.py --verbose       # also print the normalised tokens of equal programs
python tools\callsite\callsite.py --self-test     # the normaliser and the seeded mistakes, no compiler needed
```

It needs the reference clone with `qb64pe.exe` (`--qb-root`, default `..\QB64pe`) and the release build of the
new compiler (`--qb64rust`, default `target\release\qb64rust.exe`). The exit code is 0 when every program compares
equal, 2 when one of the two compilers is missing (checked before the lock), 3 when another run is building in the clone: the tool takes the clone's lock first
(`tools\legacy_tests\clone_lock.py`), as the test runner does with either compiler, so it cannot run beside
tier 2; with `--wait` it waits for the other run to end. It empties the clone's `internal\temp`, which a `qb64rust` build's `make` reads too.

## A program

`tests\callsite\<name>.bas`: `$CONSOLE:ONLY`, a few `DIM` lines, then **the statement under test on the last line
with code before the closing `END`** (or `SYSTEM`). One program per statement form and argument type. The programs
are never run.

## What is compared

Both compilers write their C++ with `-z`: `qb64pe.exe` into the clone's git-ignored `internal\temp` (emptied
first, keeping the tracked `temp.bin`, as the corpus runner does; so the tool runs the programs one at a time),
`qb64rust.exe` into a scratch folder outside the repository, removed at the end. From each main-module fragment
(`main0.txt`) the tool takes the lines that follow a `#line N "<name>.bas"` marker for the statement's line N, and
normalises them:

1. **The statement frame is dropped**: `do{`, the event check `if(!qbevent)break;evnt(…);}while(r);`, the old
   compiler's `S_n:;` label, `qbs_cleanup(qbs_tmp_base,0);` and blank lines. The emitter's design calls these
   free.
2. **Numbered temporaries and labels are renumbered** in order of first use (`pass3`, `skip7`, `L_2`, `temp_…1`):
   the two compilers count them differently. User variables are compared as written: both compilers name them by
   the old scheme (`__STRING_S`), and renaming them would hide two swapped variables.
3. **Redundant parentheses are dropped**: around a whole call argument (`f((a+b),c)`), and around a single token
   or a negative number (`( 1 )`, `(-1)`). Spacing never counts (the text is split into C tokens).
4. **An explicit cast to an integer type around a whole call argument is dropped** (`func_chr(((int32)(qbr(x))))`
   against `func_chr(qbr(x))`). The new compiler's IR states every conversion and the emitter writes it; the old
   compiler leaves an integer argument to C++, which converts it to the parameter's type. Both are the same
   conversion **when the cast names the parameter's type**. The check cannot see a cast to a narrower type than the
   parameter (it does not read libqb's prototypes); that case is covered by a slice program with a large value.
   Casts anywhere else, and casts to a float type, are compared.
5. **The `ll` suffix of an integer literal that is a whole call argument is dropped** (`sub_seek(1,2ll)` against
   `sub_seek( 1 , 2 )`). The new compiler spells a literal of an `_INTEGER64` slot as a `long long`; the old
   compiler writes the digits and C++ converts them to the parameter's type. A suffix on an operand inside an
   argument (`a*2ll`) is compared.

6. **A whole call argument that is integer literals joined by `|` is replaced by its value** (`func_rnd(x,0|1)`
   against `func_rnd(x,1)`). The old compiler writes the `passed` mask of a function as its bits joined by `|`,
   the new compiler as one number. A wrong bit still gives another value.

No rule names a built-in. The two token lists must then be equal.

Output: one line per program, `equal`, `differ` (with both token lists) or `not compared` (a compiler rejected the
program; the reason is its first diagnostic), then a summary line. No local path is printed.

## What it does not compare

- **Array elements as arguments.** An element's index is written with other conversions by the two compilers
  (`array_check((*__LONG_N)-…` against `array_check((((int64)(*__LONG_N)))-…`, `1` against `1ll`), which is
  element addressing, not the built-in's call; the programs use variables, literals and expressions instead.
- **Arithmetic on operands of different widths inside an argument, and the store of a function's result into a
  narrower variable.** The new compiler writes each widening and narrowing (`((int32)(*__INTEGER_I))*2`,
  `func_lof(n)+1ll`, `*__LONG_N=((int32)(func_lof(1)))`) where the old compiler leaves it to C++. That is how an
  expression or a store is spelled, not the built-in's call, and rules for it would weaken the check everywhere
  (the same reasoning as for elements). An expression argument in the programs uses operands of one width
  (`n + 1`, `n * 2` with `n` a LONG), and a function's result is stored into a variable of its own type; the
  mixed forms are covered by the slice programs.
- **A number of another type passed where libqb takes a `double`.** The new compiler writes the conversion
  (`sub_randomize(((double)(5)),1)`), the old compiler leaves it to C++ (`sub_randomize( 5 ,1)`). A cast to a float
  type is compared on purpose (it is not the same conversion for a `long double` argument), so the programs pass a
  DOUBLE variable or a float literal to such a slot; integer seeds are covered by the slice programs.
- **Stores into a `_BIT` variable or a member.** The store's mask and a member's address are spelled differently
  (`&1` against `&0x1ull`, `((0+4))` against `+4`); a `READ` target in the programs is a plain variable of another
  type.
- **The cleanup of string temporaries.** Every `qbs_cleanup(qbs_tmp_base,0);` line is dropped as frame (rule 1),
  so a statement that forgets its cleanup compares equal.
- Anything outside the statement's own lines: declarations, the data fragment, procedures.
- Behaviour. A call that is the same may still be made at the wrong time; that is what the corpus is for.

## Self-test

`--self-test` runs the normaliser on recorded line pairs (what each compiler wrote for one statement), which must
compare equal, and makes three deliberate mistakes on the new compiler's lines, which must be reported: a missing
rounding call (`qbr(` removed), a wrong mask (the last number of the call changed) and swapped arguments. It also
checks pairs the normaliser must keep apart (a cast to `double`, a missing call, `NULL` against `0`, parentheses
that change precedence, an `ll` suffix on an operand or outside a call).
