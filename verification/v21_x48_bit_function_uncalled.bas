$CONSOLE:ONLY
' Verification (m2-numeric-types, task 4.1): does the old compiler accept this? (bit_function_uncalled: a `_BIT`
' FUNCTION declared, never called)
PRINT "ok"
SYSTEM
FUNCTION fb` (x)
fb` = x
END FUNCTION
