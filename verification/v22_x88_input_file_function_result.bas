$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (88_input_file_function_result)
OPEN "v22_x88.tmp" FOR OUTPUT AS #1: PRINT #1, "7": CLOSE
OPEN "v22_x88.tmp" FOR INPUT AS #1
PRINT f&
CLOSE: KILL "v22_x88.tmp"
SYSTEM
FUNCTION f&
    INPUT #1, f&
END FUNCTION
