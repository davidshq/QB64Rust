# _NOTIFYPOPUP

`_NOTIFYPOPUP title$, message$, icon$` shows a notification of the system.

## Syntax

<!-- begin generated: syntax -->
```text
_NOTIFYPOPUP [title$][, message$][, iconType$]
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Behavior

The notification is shown and the program goes on at once: two `_NOTIFYPOPUP`s, one `"info"` and one
`"warning"`, raise no error. Nothing in BASIC reads the notification back.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled"; tests\sandbox\notifypopup_err -->

It shows a real notification, so it runs only in the sandbox family, in the test VM, not on the host.
<!-- src: language/console-and-window "Dialogs and the window's place are compiled" -->

## Errors

- A form QB64pe rejects is a compile error: a number, four arguments, or the name as a function. For example:
  - `x = _NOTIFYPOPUP("a")`: "`_NOTIFYPOPUP(...)`: `_NOTIFYPOPUP` is a built-in statement".
  - `_NOTIFYPOPUP 1`: "argument 1 of `_NOTIFYPOPUP` needs a string".
  - `_NOTIFYPOPUP "a", "b", "c", "d"`: "this `_NOTIFYPOPUP` statement matches none of its forms".
  <!-- src: language/console-and-window "Dialogs and the window's place are compiled"; verification\v58_x58_notifypopup_number; verification\v58_x104_notifypopup_fn; verification\v58_x59_notifypopup_four_args -->

## Availability

<!-- begin generated: availability -->
Supported. All 7 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: IF 0 THEN _NOTIFYPOPUP "t", "m" -->

## See also

[`_MESSAGEBOX`](_messagebox.md).
