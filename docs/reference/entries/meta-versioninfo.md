# $VERSIONINFO

`$VERSIONINFO:key=value` sets one field of the executable's version resource, such as `CompanyName` or `FileVersion#`.

## Syntax

```text
$VERSIONINFO:key=value
```

## Behavior

The key stands between the line's first `:` and the first `=` after it, blanks trimmed, in any letter case. The value
is the rest of the line, trimmed, each `"` read as `'`, and one pair of enclosing `'` removed.
<!-- src: language/console-and-window "$VERSIONINFO"; verification\v40_d_forms -->

The last line of a key counts. A line in a `$IF` branch not taken does not count.
<!-- src: language/console-and-window "$VERSIONINFO"; verification\v40_e_repeat -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

<!-- probe: $VERSIONINFO:Comments=reference -->

## See also

[`$EXEICON`](meta-exeicon.md).
