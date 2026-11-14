# DATE$

`DATE$` gives the system's date as `mm-dd-yyyy`.

## Syntax

<!-- begin generated: syntax -->
```text
DATE$ = dateToSet$
DATE$
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
DATE$ = text$
```

## Result

A STRING of 10 characters.

## Behavior

The date is the machine's, so a program checks its form: 10 characters with `-` at the third and sixth.
<!-- src: language/builtin-functions "Supported built-in functions"; tests\reference\examples\date-str_1 -->

`DATE$ = text$` compiles, and changes nothing: libqb's `sub_date` leaves the system's date as it was.
<!-- src: language/builtin-statements "DATE$ and TIME$ as statements"; verification\v58_s03_date_time; tests\corpus\slice\s159_builtins_last; tests\reference\examples\date-str_1 -->

## Errors

- A value that is not a string, two values, and `DATE$` with no `=` or nothing after it are compile errors.
  <!-- src: language/builtin-statements "DATE$ and TIME$ as statements"; verification\v58_x70_date_number; verification\v58_x72_date_two_values; verification\v58_x73_date_no_equals -->

## Availability

<!-- begin generated: availability -->
Supported. All 6 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:date-str_1 -->
```qb64
d$ = DATE$
PRINT "Ten characters, mm-dd-yyyy:"; LEN(d$); MID$(d$, 3, 1); MID$(d$, 6, 1)
DATE$ = "01-02-2003"
PRINT "DATE$ = changes nothing:"; DATE$ = d$ OR DATE$ <> "01-02-2003"
END
```

Output:

```text
Ten characters, mm-dd-yyyy: 10 --
DATE$ = changes nothing:-1 

Press any key to continue
```
<!-- end generated -->

## See also

[`TIME$`](time-str.md), [`TIMER`](timer.md).
