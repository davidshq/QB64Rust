$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): WHILE in an included file, WEND in the main file.
i = 0
'$INCLUDE:'v57_inc/while.bi'
    i = i + 1
WEND
PRINT "after"; i
SYSTEM
