$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): DO in an included file, LOOP in the main file.
i = 0
'$INCLUDE:'v57_inc/do.bi'
    i = i + 1
LOOP UNTIL i = 3
PRINT "after"; i
SYSTEM
