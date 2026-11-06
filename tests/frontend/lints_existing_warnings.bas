' TEST: warnings
' The `warnings` mode shows a warning the compiler gave before the lints: a chain of `^` in a `CONST`.
$CONSOLE:ONLY
CONST big = 2 ^ 3 ^ 2
PRINT big
