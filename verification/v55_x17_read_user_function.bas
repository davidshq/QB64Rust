$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x17_read_user_function)
PRINT "ok"
SYSTEM
DATA 1
READ f(1)
FUNCTION f (n)
    f = n
END FUNCTION
