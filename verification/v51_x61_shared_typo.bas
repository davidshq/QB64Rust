$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x61_shared_typo)
DIM b AS INTEGER
s
PRINT "ok"
SYSTEM
SUB s
    SHARED b AS _Bytef
END SUB
