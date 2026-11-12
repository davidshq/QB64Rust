# $EXEICON

`$EXEICON:'file.ico'` gives the program's executable an icon, and sets the window's icon to it as `_ICON` does.

## Syntax

```text
$EXEICON:'file'
```

## Behavior

The name stands between the line's first two single quotes; what follows is dropped. Where the line stands, it is the
statement `_ICON` with no argument.
<!-- src: language/console-and-window "$EXEICON" -->

A name starting with `./` or `.\` is looked for in the main source file's folder. Any other relative name is looked
for from the compiler's folder.
<!-- src: language/console-and-window "$EXEICON"; verification\v29_e01_dot_slash; verification\v29_e02_plain_path -->

A line in a `$IF` branch not taken does not count.
<!-- src: language/console-and-window "$EXEICON" -->

## Errors

- A line without a name between single quotes, a second `$EXEICON` line, a name whose file is not there, and a file
  that is no icon are compile errors at the line.
  <!-- src: language/console-and-window "Errors of $EXEICON"; verification\v29_e03_missing_plain_name; verification\v29_e05_twice; verification\v29_e06_bare -->
- A name with a drive and no slash after it (`C:icon.ico`) is reported "not supported yet".
  <!-- src: language/console-and-window "Errors of $EXEICON" -->

## Differences

From QB64pe: for a file that is not there, its message names the folder it looked in. QB64Rust's gives the name as
the program writes it: "the file `./nothere.ico` of `$EXEICON` is not found" (`DIVERGENCES-QB64PE.md` D-015).
<!-- src: D-015 -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## See also

`_ICON`, `$VERSIONINFO`.
