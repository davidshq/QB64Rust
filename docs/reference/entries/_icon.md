# _ICON

`_ICON image, small` sets the window's icon from an image, or with no argument to the icon of the executable.

## Syntax

<!-- begin generated: syntax -->
```text
_ICON [mainImageHandle&[, smallImageHandle&]]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

`_ICON image&` sets both of the window's icons, the big one and the small one, from the image: after `_ICON` of a
40 × 40 image, both are 40 × 40. `_ICON big&, small&` sets each from its own image: 40 × 40 and 20 × 20. `_ICON`
with no argument gives the window the standard icons, 32 × 32 and 16 × 16. Before any `_ICON`, the window has
no icon of its own: Windows shows its default.
<!-- src: language/console-and-window "Window built-ins are compiled"; tests\sandbox\window_icon; tests\corpus\slice\s66_window_builtins -->

This runs only in the sandbox family, in the test VM, which reads the icons' sizes from Windows.
<!-- src: tests\sandbox\window_icon -->

## Errors

- A form QB64pe rejects is a compile error: three arguments or a string. For example:
  - `_ICON 1, 2, 3`: "this `_ICON` statement matches none of its forms".
  - `_ICON "a"`: "argument 1 of `_ICON` needs a number".
  <!-- src: language/console-and-window "Window built-ins are compiled"; verification\v26_x78_icon_three; verification\v26_x79_icon_string -->

## Availability

<!-- begin generated: availability -->
Supported. All 14 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _ICON -->

## See also

[`$EXEICON`](meta-exeicon.md), [`_TITLE`](_title.md).
