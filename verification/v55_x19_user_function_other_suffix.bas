$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x19_user_function_other_suffix)
f&(3) = 5
PRINT f&(3); f(2)
SYSTEM
FUNCTION f (n)
    f = n * 2
END FUNCTION
