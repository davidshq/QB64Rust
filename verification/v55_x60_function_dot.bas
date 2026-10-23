$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x60_function_dot)
PRINT a.b(3); a . b(4)
SYSTEM
FUNCTION a . b (n)
    a.b = n * 2
END FUNCTION
