$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): IF in the main file, ELSE in an included file, END IF in the main file.
x = 2
IF x = 1 THEN
    PRINT "then"
'$INCLUDE:'v57_inc/else.bi'
    PRINT "else part"
END IF
PRINT "after"
SYSTEM
