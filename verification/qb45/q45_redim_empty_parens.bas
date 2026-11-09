' QB 4.5 (beside q45_dim_empty_parens): REDIM a() with empty parentheses, and REDIM a AS LONG with none. QB64pe
' declares the scalar a and no array for both. The compile messages show which lines are errors.
OPEN "OUT.TXT" FOR OUTPUT AS #1
REDIM a()
REDIM c AS LONG
PRINT #1, "built"
CLOSE #1
SYSTEM
