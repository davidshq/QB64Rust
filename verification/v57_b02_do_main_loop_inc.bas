$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): DO in the main file, LOOP in an included file.
i = 0
DO
    i = i + 1
'$INCLUDE:'v57_inc/loop.bi'
PRINT "after"; i
SYSTEM
