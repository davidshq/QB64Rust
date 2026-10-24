$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): EXIT FOR in an included file, inside a FOR of the main file.
FOR i = 1 TO 5
    PRINT "i"; i
'$INCLUDE:'v57_inc/exit_for.bi'
NEXT
PRINT "after"
SYSTEM
