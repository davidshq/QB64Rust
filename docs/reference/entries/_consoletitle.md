# _CONSOLETITLE

`_CONSOLETITLE` sets the title of the console's window, `_CONSOLECURSOR` shows or hides the console's cursor, and
`_CONSOLEFONT` sets its font.

## Syntax

```text
_CONSOLETITLE title$
_CONSOLECURSOR {_SHOW|_HIDE} [, size]
_CONSOLEFONT fontname$, size
```

## Behavior

The three statements act on the console of a program with `$CONSOLE` or `$CONSOLE:ONLY`, as QB64pe's do.
<!-- src: language/console-and-window "The console's own statements and functions"; tests\reference\examples\_consoletitle_1 -->

`_CONSOLEFONT` changes the font of the console window the program shares with others, so it runs only in the
sandbox family, in the test VM. There `_CONSOLEFONT "Consolas", 20` and `_CONSOLEFONT "", 12`, the size alone,
raise no error and the program goes on. Nothing in BASIC reads the font back.
<!-- src: language/console-and-window "The console's own statements and functions"; tests\sandbox\consolefont_err -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_consoletitle_1 -->
```qb64
_CONSOLETITLE "Report"
_CONSOLECURSOR _HIDE
PRINT "The console's title is set and its cursor hidden."
_CONSOLECURSOR _SHOW
END
```

Output:

```text
The console's title is set and its cursor hidden.

Press any key to continue
```
<!-- end generated -->

## See also

[`$CONSOLE`](meta-console.md), [`_TITLE`](_title.md).
