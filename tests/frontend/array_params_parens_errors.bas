' TEST: check-fail
$CONSOLE:ONLY
' What stands between an array parameter's parentheses is skipped (array_params_typed.bas), but a second `(` and
' the word AS there are errors of the old compiler: "Syntax error - too many opening brackets", "Syntax error -
' check your brackets" (measured, verification\v24_x304, x305).
SYSTEM
SUB nested (a(b(2)) AS LONG)
END SUB
SUB typed (a(1 AS LONG))
END SUB
SUB open (a(1
END SUB
