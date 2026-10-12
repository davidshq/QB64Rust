$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM a&(5), then REDIM a%(6): two arrays.
REDIM a&(5)
REDIM a%(6)
PRINT UBOUND(a&); UBOUND(a%)
SYSTEM
