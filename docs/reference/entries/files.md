# FILES

`FILES` prints a listing of a folder: its path, its entries and the free space.

## Syntax

```text
FILES [spec$]
```

## Behavior

`FILES` prints the listing libqb prints: the current folder, the entries `spec$` names (a folder, a pattern, or the
current folder when left out), and the free space.
<!-- src: language/file-io "FILES" -->

A `spec$` that names nothing raises QB64pe's error.
<!-- src: language/file-io "FILES" -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

<!-- probe: FILES "*.none" -->

## See also

`_FILES$`, `CHDIR`.
