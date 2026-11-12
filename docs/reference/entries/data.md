# DATA

`DATA` holds values written into the program, which `READ` reads in order: `DATA Davout, 26000`.

## Syntax

```text
DATA item [, item]...
```

## Behavior

The items of every `DATA` statement form one list for the whole program, in order: main module, procedures and
included files, each in its place. A `DATA` counts wherever it stands, also in a block that never runs and after
`END`.
<!-- src: language/data-read "The program's data"; tests\reference\examples\data_1 -->

Blanks around an unquoted item are dropped. A quoted item keeps its blanks and commas: `"Lannes, Jean"` is one item.
An empty item stays an empty item.
<!-- src: language/data-read "The program's data"; tests\reference\examples\data_1 -->

In an unquoted item, `REM` is part of the item, not a comment.
<!-- src: verification\v16_m2_data_rem -->

## Errors

- Text after a closing quote, other than blanks, is a compile error.
  <!-- src: language/data-read "The program's data" -->

## Availability

<!-- begin generated: availability -->
Supported. All 2 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:data_1 -->
```qb64
FOR i = 1 TO 3
    READ name$, men
    PRINT name$; men
NEXT i
DATA Davout, 26000
DATA "Lannes, Jean", 20000
DATA Soult, 30000
END
```

Output:

```text
Davout 26000 
Lannes, Jean 20000 
Soult 30000 

Press any key to continue
```
<!-- end generated -->

## See also

`READ`, `RESTORE`.
