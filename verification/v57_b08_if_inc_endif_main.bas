$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): IF in an included file, END IF in the main file.
x = 1
'$INCLUDE:'v57_inc/if.bi'
    PRINT "then"
END IF
PRINT "after"
SYSTEM
