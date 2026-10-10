$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (107_swap_function_result)
PRINT f&
SYSTEM
FUNCTION f&
    DIM l AS LONG
    l = 4
    SWAP f&, l
END FUNCTION
