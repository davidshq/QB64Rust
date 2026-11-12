# $UNSTABLE

`$UNSTABLE:TYPEFIELDS` turns on a QB64pe feature that is not settled yet: the `_STATIC` and `_DYNAMIC` markers of
`TYPE` member arrays. A `_DYNAMIC` member array can be given new bounds with `REDIM`.

## Syntax

```text
$UNSTABLE:TYPEFIELDS
$UNSTABLE:MIDI
$UNSTABLE:HTTP
```

## Behavior

`$UNSTABLE:TYPEFIELDS` allows the markers in the `TYPE` blocks below it in the file, also when it stands after the
first statement. A `TYPE` block above it is compiled as without it.
<!-- src: language/unstable-features "$UNSTABLE flags"; tests\reference\examples\meta-unstable_1 -->

The flag is read in any letter case, with blanks after the colon, and may stand more than once. In a `$IF` branch
that is not compiled it has no effect.
<!-- src: language/unstable-features "$UNSTABLE flags" -->

`$UNSTABLE:MIDI` and `$UNSTABLE:HTTP` are accepted and change nothing, with the warning "No longer required, feature
is stable now".
<!-- src: language/unstable-features "$UNSTABLE flags" -->

## Errors

- Any other flag, as `$UNSTABLE:FOO`: "unrecognized `$UNSTABLE` flag "FOO"". An empty flag and `$UNSTABLE` without a
  colon are compile errors too.
  <!-- src: language/unstable-features "$UNSTABLE flags" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-unstable_1 -->
```qb64
$UNSTABLE:TYPEFIELDS
TYPE Bag
    items(1 TO 2) _DYNAMIC AS LONG
END TYPE
DIM b AS Bag
PRINT "items runs from"; LBOUND(b.items); "to"; UBOUND(b.items)
REDIM b.items(1 TO 5) AS LONG
b.items(5) = 42
PRINT "After REDIM:"; UBOUND(b.items); b.items(5)
END
```

Output:

```text
items runs from 1 to 2 
After REDIM: 5  42 

Press any key to continue
```
<!-- end generated -->

## See also

`TYPE`, `REDIM`.
