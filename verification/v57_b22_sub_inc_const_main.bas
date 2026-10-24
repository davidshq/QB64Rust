$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): a SUB opened in an included file, a CONST in its part in the main file, used there and after END SUB.
s
PRINT "main n"; n
SYSTEM
'$INCLUDE:'v57_inc/sub_open.bi'
    CONST n = 5
    DIM t AS STRING * n
    PRINT "in s n"; n; LEN(t)
END SUB
PRINT "after END SUB n"; n
