# _ACCEPTFILEDROP

`_ACCEPTFILEDROP` lets the user drop files on the program's window. `_TOTALDROPPEDFILES` gives how many were dropped,
`_DROPPEDFILE$` gives each one's name, and `_FINISHDROP` clears the list.

## Syntax

```text
_ACCEPTFILEDROP [{ON|OFF}]
accepting = _ACCEPTFILEDROP
n = _TOTALDROPPEDFILES
name$ = _DROPPEDFILE$[(index)]
_FINISHDROP
```

## Behavior

With no file dropped, `_ACCEPTFILEDROP` and `_TOTALDROPPEDFILES` give 0 and `_DROPPEDFILE$` "". `_DROPPEDFILE` without
its `$` is accepted too.
<!-- src: language/console-and-window "Files dropped on the window"; tests\corpus\slice\s155_console_on_off; tests\reference\examples\_acceptfiledrop_1 -->

Only a user can drop a file, so no test does.
<!-- src: language/console-and-window "Files dropped on the window" -->

## Errors

- A number after `_ACCEPTFILEDROP`, an argument to the function, `_DROPPEDFILE$` with a string, and `_FINISHDROP` with
  an argument are compile errors.
  <!-- src: language/console-and-window "Files dropped on the window"; verification\v53_x21_acceptfiledrop_number; verification\v53_x18_droppedfile_string; verification\v53_x23_finishdrop_arg -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_acceptfiledrop_1 -->
```qb64
PRINT "Accepting, files dropped:"; _ACCEPTFILEDROP; _TOTALDROPPEDFILES
PRINT "["; _DROPPEDFILE$; "]"
_FINISHDROP
END
```

Output:

```text
Accepting, files dropped: 0  0 
[]

Press any key to continue
```
<!-- end generated -->

## See also

`_FILEEXISTS`.
