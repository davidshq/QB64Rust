$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a SUB opened in an included file and closed in the main one.
s
SYSTEM
'$INCLUDE:'v57_inc/sub_open.bi'
    PRINT "in s, main file"
END SUB
