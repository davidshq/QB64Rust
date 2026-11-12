# DEFINT

`DEFINT`, `DEFLNG`, `DEFSNG`, `DEFDBL` and `DEFSTR` set the type of names by their first letter. After
`DEFINT A-Z`, a name with no suffix and no `AS` clause is an INTEGER.

## Syntax

```text
DEFINT letters
DEFLNG letters
DEFSNG letters
DEFDBL letters
DEFSTR letters
```

## Arguments

- `letters`: single letters and ranges, separated by commas: `A-Z`, `i`, `a-c, x`.

## Behavior

The five statements give the type INTEGER, LONG, SINGLE, DOUBLE and STRING to names that start with the letters
named. Without any of them, such a name is a SINGLE.
<!-- src: language/default-types "Default type by first letter"; verification\v24_a_letters -->

Letters are read without regard to case, and a range may be written backwards: `DEFINT z-w` covers w to z. `_` is
a letter of its own, after Z, for names that start with an underscore.
<!-- src: language/default-types "Default type by first letter"; verification\v24_a_letters; verification\v24_a_underscore -->

A name with a suffix, or declared with `AS`, keeps its own type: after `DEFINT A-Z`, `s!` is still a SINGLE.
<!-- src: language/default-types "Default type by first letter"; verification\v24_a_letters -->

The statement applies to every line below it in the file, whether or not it runs, and in whatever block or
procedure it stands. One inside a SUB goes on applying after the SUB ends. A procedure above the statement keeps
the earlier default.
<!-- src: language/default-types "Default types apply from the statement on"; verification\v24_a_position; verification\v24_a_procs_subs -->

A later statement for the same letter replaces the earlier one: after `DEFINT A-Z` and `DEFDBL D`, `d` is a DOUBLE.
<!-- src: language/default-types "Default types apply from the statement on"; verification\v24_a_letters -->

A name without a suffix means the name with the default type at its line. A variable `a` used before `DEFINT A-Z`
is `a!`, and `a` after it is `a%`, another variable. A variable declared with `AS` is found by its bare name
whatever the default.
<!-- src: language/default-types "Default types apply from the statement on"; verification\v24_a_position -->

The default type is the type of variables, arrays, `FOR` variables, parameters and FUNCTION results that have no
suffix and no `AS`. It does not type a `CONST`, which takes the type of its value.
<!-- src: language/default-types "What a default type gives its type to"; verification\v24_a_decls -->

## Errors

These are compile errors:

- No letters, a digit or a word of two letters, as in `DEFINT`, `DEFINT 1` or `DEFINT ab`: "expected a letter or a
  range of letters (`a-z`)".
  <!-- src: language/default-types "Default type by first letter"; verification\v24_x13_defint_alone; verification\v24_x14_defint_digit; verification\v24_x15_defint_word -->
- A range with no end, `DEFINT a-`: "expected a letter after `-`".
  <!-- src: language/default-types "Default type by first letter"; verification\v24_x16_defint_open_range -->
- An `AS` clause, as in `DEFINT a AS LONG`: "expected `-` or `,` after the letter, not `AS`: only `_DEFINE` takes a
  type".
  <!-- src: language/default-types "_DEFINE and DEFtype without their letters"; verification\v24_x19_defint_as; verification\v57_e05_deflng_as -->
- A `FOR` variable typed STRING by `DEFSTR`: "the `FOR` variable `s` must be a number, not a string".
  <!-- src: language/default-types "What a default type gives its type to" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:defint_1 -->
```qb64
x = 2.5: PRINT "Before DEFINT, x ="; x
DEFINT A-Z
y = 2.5: PRINT "After DEFINT A-Z, y ="; y; "in"; LEN(y); "bytes"
DEFDBL D
d = 1 / 3: PRINT "After DEFDBL D, d ="; d
DEFSTR S
s = "text": PRINT "After DEFSTR S, s = "; s
w! = 2.5: PRINT "A suffix wins: w! ="; w!
END
```

Output:

```text
Before DEFINT, x = 2.5 
After DEFINT A-Z, y = 2 in 2 bytes
After DEFDBL D, d = .3333333333333333 
After DEFSTR S, s = text
A suffix wins: w! = 2.5 

Press any key to continue
```
<!-- end generated -->

## See also

`_DEFINE`, `DIM`, `INTEGER`, `STRING`.
