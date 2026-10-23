$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (31_function_argument)
DIM a(2) AS LONG
a(0) = 4
PRINT f(a())
SYSTEM
FUNCTION f (v AS LONG)
    f = v + 1
END FUNCTION
