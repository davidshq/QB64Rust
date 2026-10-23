$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x49_function_abs_percent)
PRINT "ok"
SYSTEM
FUNCTION abs% (a)
    abs% = a
END FUNCTION
