# $DEBUG

`$DEBUG` marks a program for QB64pe's IDE debugger. Outside that IDE the program runs as without it.

## Syntax

```text
$DEBUG
```

## Behavior

`$DEBUG` sets the `$IF` flags `_DEBUG_` and `_SOCKETS_` to 1, and the program links QB64pe's sockets part, as QB64pe's
does.
<!-- src: language/asserts-and-debug "$DEBUG"; tests\reference\examples\meta-debug_1 -->

No code for QB64pe's IDE debugger is written: the program behaves as QB64pe's does when it runs outside the IDE. With
`-w`, the compiler warns "$DEBUG features only work from the IDE".
<!-- src: language/asserts-and-debug "$DEBUG" -->

## Differences

From QB64pe: its program includes the IDE's debugger, vWatch, so it can be debugged from QB64pe's IDE, and vWatch's
names are taken: a SUB named `vwatch_stoptimers` is "Name already in use". A program built by QB64Rust cannot be
debugged from QB64pe's IDE, and those names are free (`DIVERGENCES-QB64PE.md` D-017).
<!-- src: D-017 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-debug_1 -->
```qb64
$DEBUG
$IF _DEBUG_ THEN
    PRINT "_DEBUG_ is on."
$END IF
PRINT "Outside the IDE, the program runs as without $DEBUG."
END
```

Output:

```text
_DEBUG_ is on.
Outside the IDE, the program runs as without $DEBUG.

Press any key to continue
```
<!-- end generated -->

## See also

`$ASSERTS`, `$CHECKING`.
