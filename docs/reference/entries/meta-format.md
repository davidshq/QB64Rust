# $FORMAT

`'$FORMAT:OFF` and `'$FORMAT:ON`, in comments, mark lines that the formatter (`qb64rust -y`) leaves as written. To the
compiler they are comments.

## Syntax

```text
'$FORMAT:OFF
'$FORMAT:ON
```

## Behavior

The lines between them keep their own indentation and spacing; only the letter case of keywords and names is applied
to them, as `qb64pe -y` applies it.
<!-- src: tests\corpus\slice\s160_layout_text -->

The formatter's two rewrites, `OneStatementPerLine` and `BlockIf`, leave these lines alone too: `x = 0: y = 0`
between them stays one line. `'$FORMAT:OFF` does not make `-y` warn that a rewrite has no effect.
<!-- src: compiler/format "Lines the rewrites leave alone"; compiler/format "A rewrite setting without effect" -->

To the compiler, `'$Format:Off` is an ordinary comment.
<!-- src: compiler/pipeline "Comment metacommands are never ignored" -->

## Availability

<!-- begin generated: availability -->
Not checked: no call-site program, example or probe uses it.
<!-- end generated -->

## See also

[`REM`](rem.md), [`$INCLUDE`](meta-include.md).
