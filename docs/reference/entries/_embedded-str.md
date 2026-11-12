# _EMBEDDED$

`_EMBEDDED$("handle")` gives the bytes of a file that `$EMBED` built into the executable.

## Syntax

<!-- begin generated: syntax -->
```text
_EMBEDDED$(handle)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING.

## Behavior

It gives every byte of the file, whatever their values and however many: a file of the 256 byte values gives a string
of 256 bytes from 0 to 255.
<!-- src: language/embedded-files "_EMBEDDED$"; tests\corpus\slice\s138_embed; tests\reference\examples\_embedded-str_1 -->

Each call gives a new string, so changing one does not change the next.
<!-- src: language/embedded-files "_EMBEDDED$"; tests\corpus\slice\s138_embed; tests\reference\examples\_embedded-str_1 -->

## Errors

- The argument must be one string literal holding a handle named by a `$EMBED` line above the call. Anything else is a
  compile error.
  <!-- src: language/embedded-files "_EMBEDDED$" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_embedded-str_1 -->
```qb64
$EMBED:'./bytes.bin','bytes'
b$ = _EMBEDDED$("bytes")
PRINT "Bytes:"; LEN(b$); "first"; ASC(b$, 1); "last"; ASC(b$, LEN(b$))
MID$(b$, 1, 1) = "x"
PRINT "A new string at each call:"; ASC(_EMBEDDED$("bytes"), 1)
END
```

Output:

```text
Bytes: 256 first 0 last 255 
A new string at each call: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`$EMBED`.
