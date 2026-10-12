$CONSOLE:ONLY
' Verification (m2-numeric-types, D1): does the old compiler accept this? (bit_function_param)
PRINT f(3)
FUNCTION f (x AS _BIT * 3)
f = x
END FUNCTION
