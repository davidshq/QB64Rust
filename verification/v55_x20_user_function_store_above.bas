$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x20_user_function_store_above)
PRINT "ok"
SYSTEM
FUNCTION f (n)
    f = n * 2
END FUNCTION
SUB s
    f(3) = 5
END SUB
