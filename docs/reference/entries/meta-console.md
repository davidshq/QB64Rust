# $CONSOLE

`$CONSOLE` gives a program a console beside its window. `$CONSOLE:ONLY` gives it a console and no window, so that it
runs as a command-line program.

## Syntax

```text
$CONSOLE
$CONSOLE:ONLY
```

## Behavior

A program has a console when `$CONSOLE` or `$CONSOLE:ONLY` stands anywhere in its compiled code: on any line, in a
SUB, in an included file. One in a `$IF` branch that is not compiled does not count.
<!-- src: language/console-and-window "A program with a console" -->

With `$CONSOLE`, output still goes to the window until `_DEST _CONSOLE` makes the console the destination.
<!-- src: language/console-and-window "_DEST and _SOURCE" -->

`$CONSOLE:ONLY` makes the console the destination and the source at its line, and the program has no window. `_DEST`
and `_CONSOLE` then both give -10.
<!-- src: language/console-and-window "$CONSOLE:ONLY"; tests\reference\examples\meta-console_1 -->

The `$IF` flag `_CONSOLE_` is 1 with `$CONSOLE` and 2 with `$CONSOLE:ONLY` alone.
<!-- src: compiler/pipeline "Precompiler flags" -->

## Errors

- Any other form, such as `$CONSOLE:OFF`, `$CONSOLE : ONLY`, a word after it, a statement on its line or a tab before
  it, is a compile error.
  <!-- src: language/console-and-window "Forms of the console and window metacommands" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-console_1 -->
```qb64
PRINT "A $CONSOLE:ONLY program has no window: this goes to the console."
PRINT "_DEST is"; _DEST; "and _CONSOLE is"; _CONSOLE
END
```

Output:

```text
A $CONSOLE:ONLY program has no window: this goes to the console.
_DEST is-10 and _CONSOLE is-10 

Press any key to continue
```
<!-- end generated -->

## See also

[`_CONSOLE`](_console.md), [`_ECHO`](_echo.md), [`_DEST`](_dest.md), [`$SCREENHIDE`](meta-screenhide.md).
