# $INCLUDEONCE

`$INCLUDEONCE` in a file makes the compiler include that file only the first time an `$INCLUDE` names it. It lets
several files include the same file of declarations safely.

## Syntax

```text
$INCLUDEONCE
```

The line holds nothing else.

## Behavior

The first `$INCLUDE` of the file includes it whole. Every later `$INCLUDE` of it is empty. Without
`$INCLUDEONCE`, each `$INCLUDE` compiles the file's text again.
<!-- src: compiler/pipeline "Included files"; verification\v16_m7_twice; tests\upstream\compile_tests\include_once\test -->

The line may stand anywhere in the file, not only on its first line.
<!-- src: verification\v16_m7_twice -->

The file is the same file when it is named another way, as `v16_inc/once.bi` and `./v16_inc/once.bi` are.
<!-- src: verification\v16_m7_twice -->

A library named twice with `$USELIBRARY` whose files hold `$INCLUDEONCE` is read once.
<!-- src: language/external-libraries "The order of several libraries"; verification\v70_i1_uselibrary_twice_includeonce; verification\v70_i2_uselibrary_nested_includeonce -->

## Errors

These are compile errors:

- A word or a tab after `$INCLUDEONCE`: "`$INCLUDEONCE` must stand alone on its line", with QB64pe's "Syntax error"
  as a note.
  <!-- src: verification\v26_x61_includeonce_word; verification\v26_x59_includeonce_tab_after -->
- `$INCLUDEONCE` after a statement and a colon: "a metacommand must stand at the start of a line".
  <!-- src: verification\v26_x47_includeonce_after_colon -->
- A tab before `$INCLUDEONCE`: "a metacommand must not have a tab before it".
  <!-- src: verification\v26_x42_tab_before_includeonce -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:meta-includeonce_1 -->
```qb64
'$INCLUDE: 'palette.bi'
'$INCLUDE: 'palette.bi'
PRINT "Red is"; RED; "and blue is"; BLUE
END
```

The file `palette.bi`:

```qb64
' palette.bi: included twice, compiled once.
$INCLUDEONCE
CONST RED = 4
CONST BLUE = 1
```

Output:

```text
Red is 4 and blue is 1 

Press any key to continue
```
<!-- end generated -->

## See also

`$INCLUDE`, `$IF`, `$USELIBRARY`.
