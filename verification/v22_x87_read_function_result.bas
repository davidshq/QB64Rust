$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (87_read_function_result)
PRINT f&
SYSTEM
DATA 7
FUNCTION f&
    READ f&
END FUNCTION
