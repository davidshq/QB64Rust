$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x29_lbound_function_call)
PRINT LBOUND(f(1))
SYSTEM
FUNCTION f (n)
    f = n
END FUNCTION
