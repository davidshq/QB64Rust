$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): IF in the main file, END IF in an included file.
x = 1
IF x = 1 THEN
    PRINT "then"
'$INCLUDE:'v57_inc/endif.bi'
PRINT "after"
SYSTEM
