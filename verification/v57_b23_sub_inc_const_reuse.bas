$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): a SUB opened in an included file, a CONST in its part in the main file, and the same name as a CONST of the main module after END SUB.
s
PRINT "main n"; n
SYSTEM
'$INCLUDE:'v57_inc/sub_open.bi'
    CONST n = 5
    PRINT "in s n"; n
END SUB
CONST n = 7
