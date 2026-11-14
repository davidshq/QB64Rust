# IOCTL

`IOCTL` and `IOCTL$` are not available. QB64pe lists them but rejects them, and QB64Rust does the same.

## Syntax

```text
IOCTL [#]filenumber, string$
IOCTL$([#]filenumber)
```

## Behavior

Every use of `IOCTL` and `IOCTL$` is a compile error.
<!-- src: language/builtin-functions "The old compiler's stubs" -->

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- `IOCTL` is not implemented in QB64pe
<!-- end generated -->

<!-- probe: IOCTL 1, "x" -->

## See also

[`OPEN`](open.md).
