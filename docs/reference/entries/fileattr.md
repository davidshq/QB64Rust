# FILEATTR

`FILEATTR` is not available. QB64pe lists it but rejects it, and QB64Rust does the same.

## Syntax

```text
FILEATTR(filenumber, attribute)
```

## Behavior

Every use of `FILEATTR` is a compile error: "`FILEATTR` is not implemented in QB64pe".
<!-- src: language/builtin-functions "The old compiler's stubs" -->

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- `FILEATTR` is not implemented in QB64pe
<!-- end generated -->

<!-- probe: x = FILEATTR(1, 1) -->

## See also

[`OPEN`](open.md), [`LOF`](lof.md).
