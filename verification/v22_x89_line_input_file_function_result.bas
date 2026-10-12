$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (89_line_input_file_function_result)
OPEN "v22_x89.tmp" FOR OUTPUT AS #1: PRINT #1, "seven": CLOSE
OPEN "v22_x89.tmp" FOR INPUT AS #1
PRINT f$
CLOSE: KILL "v22_x89.tmp"
SYSTEM
FUNCTION f$
    LINE INPUT #1, f$
END FUNCTION
