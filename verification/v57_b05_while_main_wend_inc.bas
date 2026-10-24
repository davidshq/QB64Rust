$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): WHILE in the main file, WEND in an included file.
i = 0
WHILE i < 2
    i = i + 1
'$INCLUDE:'v57_inc/wend.bi'
PRINT "after"; i
SYSTEM
