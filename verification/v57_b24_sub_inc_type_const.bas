$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): a SUB opened in an included file, a CONST and a TYPE with STRING * of it in its part in the main file.
s
SYSTEM
'$INCLUDE:'v57_inc/sub_open.bi'
    CONST n = 5
    TYPE tt
        f AS STRING * n
    END TYPE
    DIM x AS tt
    PRINT "len"; LEN(x)
END SUB
