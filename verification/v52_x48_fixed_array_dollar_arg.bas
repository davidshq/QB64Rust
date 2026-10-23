$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x48_fixed_array_dollar_arg)
' name$(i) of a STRING * n array passed to a FUNCTION taking a string (QB64pe hash.bas).
DIM a(2) AS STRING * 3
a(1) = "ab"
PRINT f(a$(1))
SYSTEM
FUNCTION f (s$)
    f = LEN(s$)
END FUNCTION
