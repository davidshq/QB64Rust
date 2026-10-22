$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x74_function_param_typo)
PRINT f(1)
SYSTEM
FUNCTION f (b AS nosuch)
    f = 1
END FUNCTION
