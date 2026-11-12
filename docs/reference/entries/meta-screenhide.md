# $SCREENHIDE

`$SCREENHIDE` starts the program with its window hidden, and `$SCREENSHOW` with it shown.

## Syntax

```text
$SCREENHIDE
$SCREENSHOW
```

## Behavior

With several of them, the last one in the file counts, wherever it stands. Neither gives the program a console:
`$CONSOLE` does.
<!-- src: language/console-and-window "$SCREENHIDE and $SCREENSHOW"; verification\v26_x07_screenshow_after_hide -->

A program with `$CONSOLE` and `$SCREENHIDE` has a console and no window shown; its window's output still goes to the
hidden window.
<!-- src: language/console-and-window "The end of a program and errors with a hidden window"; verification\v58_s01_ports_window -->

They are accepted in any letter case, with blanks before and after, alone on their line.
<!-- src: language/console-and-window "Forms of the console and window metacommands" -->

## Errors

- Any other form, such as `$SCREENSHOW junk`, is a compile error.
  <!-- src: language/console-and-window "Forms of the console and window metacommands"; verification\v26_x32_screenshow_junk -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

<!-- probe: $SCREENHIDE -->

## See also

`_SCREENHIDE`, `$CONSOLE`.
