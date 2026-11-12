# $USELIBRARY

`$USELIBRARY:'author/library'` includes the files of a library installed under the compiler's `libraries` folder.

## Syntax

```text
$USELIBRARY:'author/library'
```

## Behavior

It reads the descriptor `libraries\descriptors\author\library.ini` under the compiler's folder, and includes the files
its `[LIBRARY INCLUDES]` section names from `libraries\includes\author\library\`.
<!-- src: language/external-libraries "$USELIBRARY"; verification\v58_u00_one -->

`IncAtTop` files go before the program's first line, `IncAfterMain` after the main module's end, and `IncAtBottom`
after its last line.
<!-- src: language/external-libraries "$USELIBRARY" -->

With several libraries, the `IncAtTop` files go from the last library named to the first, the others from the first to
the last. A library named on two lines is included twice, unless its files start with `$INCLUDEONCE`.
<!-- src: language/external-libraries "The order of several libraries" -->

A library's file may itself use `$USELIBRARY`.
<!-- src: language/external-libraries "The order of several libraries" -->

Its effect depends on the libraries installed beside the compiler, so this page has no example.
<!-- src: language/external-libraries "$USELIBRARY" -->

## Errors

- A line with no quoted name, or an empty one, is the error "Expected $USELIBRARY:'author/library'".
  <!-- src: language/external-libraries "Errors of $USELIBRARY" -->
- A missing descriptor is "Library descriptor for '...' not found", and a missing file it names "Library source for
  '...' not found", with the key's name.
  <!-- src: language/external-libraries "Errors of $USELIBRARY" -->

## Availability

<!-- begin generated: availability -->
Not checked: no call-site program, example or probe uses it.
<!-- end generated -->

## See also

`$INCLUDE`, `$INCLUDEONCE`, `DECLARE LIBRARY`.
