$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x47_function_len_dollar)
PRINT "ok"
SYSTEM
FUNCTION len$ (a$)
    len$ = a$
END FUNCTION
