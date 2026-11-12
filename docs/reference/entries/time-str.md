# TIME$

`TIME$` gives the system's time as `hh:mm:ss`.

## Syntax

<!-- begin generated: syntax -->
```text
TIME$ = timeToSet$
TIME$
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
TIME$ = text$
```

## Result

A STRING of 8 characters.

## Behavior

The time is the machine's, so a program checks its form: 8 characters with `:` at the third and sixth.
<!-- src: language/builtin-functions "Supported built-in functions"; tests\reference\examples\time-str_1 -->

`TIME$ = text$` compiles, and changes nothing: libqb's `sub_time` leaves the system's time as it was.
<!-- src: language/builtin-statements "DATE$ and TIME$ as statements"; verification\v58_s03_date_time; tests\corpus\slice\s159_builtins_last; tests\reference\examples\time-str_1 -->

## Errors

- A value that is not a string is a compile error.
  <!-- src: language/builtin-statements "DATE$ and TIME$ as statements"; verification\v58_x71_time_number -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:time-str_1 -->
```qb64
t$ = TIME$
PRINT "Eight characters, hh:mm:ss:"; LEN(t$); MID$(t$, 3, 1); MID$(t$, 6, 1)
TIME$ = "12:34:56"
unchanged = (LEFT$(TIME$, 5) <> "12:34") OR (LEFT$(t$, 5) = "12:34")
PRINT "TIME$ = changes nothing:"; unchanged
END
```

Output:

```text
Eight characters, hh:mm:ss: 8 ::
TIME$ = changes nothing:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`DATE$`](date-str.md), [`TIMER`](timer.md).
