$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): FOR in an included file, NEXT in the main file.
'$INCLUDE:'v57_inc/for.bi'
    PRINT "body"; i
NEXT
PRINT "after"
SYSTEM
