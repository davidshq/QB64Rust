' TEST: check-fail
$CONSOLE:ONLY
' The ASC statement rejected (m4-qb64pe-sources task 3.1; verification\v52_x01-x08): a number as the place, a
' literal, a string position, a string value, three arguments, an expression as the place, a FUNCTION's name
' outside it; ASC(a$) alone is no statement
DIM a AS STRING, b AS STRING, n AS INTEGER
ASC(n) = 65
ASC("ab") = 65
ASC(a, "2") = 65
ASC(a) = "A"
ASC(a, 1, 2) = 65
ASC(a + b) = 65
ASC(f$) = 65
ASC(a)
FUNCTION f$
    f$ = "xy"
END FUNCTION
