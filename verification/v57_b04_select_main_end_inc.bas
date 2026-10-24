$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): SELECT CASE in the main file, END SELECT in an included file.
x = 2
SELECT CASE x
    CASE 2
        PRINT "two"
'$INCLUDE:'v57_inc/endselect.bi'
PRINT "after"
SYSTEM
