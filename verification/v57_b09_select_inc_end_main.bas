$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): SELECT CASE in an included file, END SELECT in the main file.
x = 2
'$INCLUDE:'v57_inc/select.bi'
    CASE 2
        PRINT "two"
END SELECT
PRINT "after"
SYSTEM
