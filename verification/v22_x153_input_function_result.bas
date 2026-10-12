$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (153_input_function_result)
PRINT f&
SYSTEM
FUNCTION f&
    INPUT f&
END FUNCTION
