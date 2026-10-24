$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a FUNCTION opened in an included file and closed in the main one.
PRINT f(2)
SYSTEM
'$INCLUDE:'v57_inc/function_open.bi'
    f = x * 3
END FUNCTION
