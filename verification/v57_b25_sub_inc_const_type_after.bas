$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): a SUB opened in an included file with a CONST in its part in the main file, and a TYPE of the main module after END SUB with STRING * of it.
s
DIM x AS tt
PRINT "len"; LEN(x)
SYSTEM
'$INCLUDE:'v57_inc/sub_open.bi'
    CONST n = 5
    PRINT "n"; n
END SUB
TYPE tt
    f AS STRING * n
END TYPE
