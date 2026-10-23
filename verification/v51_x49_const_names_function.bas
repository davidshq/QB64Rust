$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x49_const_names_function)
CONST c = f + 1
PRINT c
SYSTEM
FUNCTION f
    f = 2
END FUNCTION
