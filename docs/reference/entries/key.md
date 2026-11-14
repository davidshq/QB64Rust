# KEY

`ON KEY(n) GOSUB label` runs a handler when key `n` is pressed, and `KEY(n) ON`, `OFF` and `STOP` switch it. `KEY n,
text$` sets a soft key, and `KEY ON`, `KEY OFF` and `KEY LIST` show, hide and list them.

## Syntax

<!-- begin generated: syntax -->
```text
KEY(number) {ON|OFF|STOP}
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

```text
ON KEY(n) GOSUB label
ON KEY(n) subname [argument]
KEY n, text$
KEY {ON|OFF|LIST}
```

## Arguments

- `n`: 1 to 10 are F1 to F10, 11 to 14 the arrows, 15 to 25 user keys, 30 and 31 F11 and F12. `KEY(0)` switches every
  key.

## Behavior

A trapped key runs its handler when pressed in the program's window, and does not reach `INKEY$`. A key switched off
reaches `INKEY$` instead.
<!-- src: language/events "Key handlers"; tests\corpus\screen\k01_on_key -->

`KEY 15, CHR$(0) + CHR$(30)` defines user key 15 as the A key with no shift flags, for `ON KEY(15)`.
<!-- src: language/events "The KEY statements of their own"; tests\corpus\screen\k01_on_key -->

`KEY LIST` prints the soft keys F1 to F12 and their texts: after `KEY 1, "abc"`, the first line is `F1  abc`.
<!-- src: language/events "The KEY statements of their own"; tests\corpus\slice\s134_events_timer; tests\reference\examples\key_1 -->

`KEY ON` is the soft-key line, not an event switch; `KEY(0) ON` switches every key's event.
<!-- src: language/events "The KEY statements of their own" -->

`KEY(n) OFF` and `STOP` act as for `TIMER`.
<!-- src: language/events "Event switches" -->

## Errors

- A key number outside 1 to 31 raises error 5, in `ON KEY`, `KEY(n) ON` and `KEY n, text$`. The number is rounded: `ON
  KEY(0.7)` raises it, `ON KEY(31.6)` does not.
  <!-- src: language/events "Key handlers"; tests\corpus\slice\s134_events_timer; tests\reference\examples\key_1 -->
- `KEY ON 1`, and anything else after `KEY ON`, `KEY OFF` or `KEY LIST`, is a compile error, as is `KEY(1) FREE`.
  <!-- src: language/events "The KEY statements of their own"; language/events "Rejected handler forms" -->
- A handler QB64pe rejects is a compile error: a label that is not in the main module, a name that is no SUB, a SUB
  with more than one parameter or one not of an integer type, or an argument given to a SUB without a parameter or
  left out for one with it. For example:
  - `KEY(1) FREE`: "this `KEY` statement matches none of its forms".
  - `KEY ON 1`: "expected the end of the statement after `KEY ON`".
  - `KEY 1, 5`: "the text of `KEY n, text$` must be a string".
  <!-- src: language/events "Rejected handler forms"; verification\v42_x15_key_free; verification\v42_x25_key_on_extra; verification\v42_x26_key_number_text -->
- A missing `)` before `GOSUB` is a compile error with the note "Expected )".
  <!-- src: language/events "An event handler without its closing parenthesis" -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:key_1 -->
```qb64
ON ERROR GOTO trap
KEY 1, "abc"
KEY LIST
PRINT "KEY 40:";
KEY 40, "x"
PRINT
END
trap:
PRINT " error"; ERR;
RESUME NEXT
```

Output:

```text
F1  abc
F2  
F3  
F4  
F5  
F6  
F7  
F8  
F9  
F10 
F11 
F12 
KEY 40: error 5 

Press any key to continue
```
<!-- end generated -->

## See also

[`TIMER`](timer.md), [`ON`](on.md), [`INKEY$`](inkey-str.md), [`_KEYHIT`](_keyhit.md).
