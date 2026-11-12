# _CONTROLCHR

`_CONTROLCHR OFF` makes text output show the characters 0 to 31 as symbols instead of acting on them. `_CONTROLCHR ON`
makes them act again.

## Syntax

```text
_CONTROLCHR {ON|OFF}
value = _CONTROLCHR
```

## Behavior

After `_CONTROLCHR OFF`, a character such as `CHR$(7)` or `CHR$(13)` is printed as its symbol; after `_CONTROLCHR ON`
it acts again, as a bell or a line end.
<!-- src: language/screen-output "Control characters" -->

The function gives -1 while the characters are shown as symbols, and 0 otherwise.
<!-- src: language/screen-output "Control characters"; tests\reference\examples\_controlchr_1 -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_controlchr_1 -->
```qb64
PRINT "_CONTROLCHR is"; _CONTROLCHR
_CONTROLCHR OFF
PRINT "after OFF:"; _CONTROLCHR
_CONTROLCHR ON
PRINT "after ON:"; _CONTROLCHR
END
```

Output:

```text
_CONTROLCHR is 0 
after OFF:-1 
after ON: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`PRINT`, `CHR$`.
