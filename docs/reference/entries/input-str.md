# INPUT$

`INPUT$(n)` reads `n` characters from the keyboard, and `INPUT$(n, #f)` from a file, with no prompt and no line end
needed.

## Syntax

<!-- begin generated: syntax -->
```text
INPUT$(numberOfBytes%[, fileOrPortNumber])
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- `n`: how many characters.
- `f`: the file number, with or without `#`.

## Result

A STRING of `n` characters.

## Behavior

`INPUT$(n, #f)` gives the next `n` characters of a file open for input or binary: after `abcdef` is written,
`INPUT$(3, #1)` is `abc` and `INPUT$(2, 1)` is `de`.
<!-- src: language/builtin-functions "INPUT$ on a file"; tests\reference\examples\input-str_1 -->

`INPUT$(n)` reads the keyboard, or the program's redirected input. At the end of that input it raises error 62 with
what it read.
<!-- src: language/console-and-window "Console input at the end of the input" -->

## Errors

- Reading past the end of a file raises run-time error 62. A negative count, a file not open, or one not open for
  reading, raises QB64pe's errors.
  <!-- src: language/builtin-functions "INPUT$ on a file" -->
- `INPUT$(1, #)` with nothing after `#` is a compile error.
  <!-- src: language/builtin-functions "INPUT$ on a file" -->

## Differences

From QB64pe: `INPUT$(n)` reads only the keyboard there, so with redirected input it waits for ever. QB64Rust reads the
redirected input (`DIVERGENCES-QB64PE.md` D-048).
<!-- src: D-048 -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:input-str_1 -->
```qb64
OPEN "input-str_1.tmp" FOR OUTPUT AS #1
PRINT #1, "abcdef"
CLOSE #1
OPEN "input-str_1.tmp" FOR INPUT AS #1
PRINT INPUT$(3, #1); "|"; INPUT$(2, 1)
CLOSE #1
KILL "input-str_1.tmp"
END
```

Output:

```text
abc|de

Press any key to continue
```
<!-- end generated -->

## See also

`INKEY$`, `INPUT`, `OPEN`.
