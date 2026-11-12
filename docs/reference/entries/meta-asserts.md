# $ASSERTS

`$ASSERTS` turns on the `_ASSERT` checks of the whole program. Without it, `_ASSERT` does nothing. `$ASSERTS:CONSOLE`
also gives the program a console, for the messages.

## Syntax

```text
$ASSERTS
$ASSERTS:CONSOLE
```

## Behavior

`$ASSERTS` turns assertions on for the whole program, wherever it stands, and sets the `$IF` flag `_ASSERTS_` to 1.
<!-- src: language/asserts-and-debug "$ASSERTS"; tests\reference\examples\meta-asserts_1 -->

`$ASSERTS:CONSOLE` does the same and acts as `$CONSOLE` too.
<!-- src: language/asserts-and-debug "$ASSERTS" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-asserts_1 -->
```qb64
$ASSERTS
x = 5
_ASSERT x > 0, "x must be positive"
PRINT "The assertion held."
$IF _ASSERTS_ THEN
    PRINT "Assertions are on."
$END IF
END
```

Output:

```text
The assertion held.
Assertions are on.

Press any key to continue
```
<!-- end generated -->

## See also

`_ASSERT`, `$CONSOLE`, `$DEBUG`.
