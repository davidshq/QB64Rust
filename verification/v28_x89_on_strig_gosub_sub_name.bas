$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): the main module's first line is the name of a SUB and a colon; the last line
' before the SUB is not reached. Does that line make the first one a label (no "sub one" printed)?
DIM n AS INTEGER
one:
PRINT "main"
SYSTEM
two:
ON STRIG(0) GOSUB one
SUB one
    PRINT "sub one"
END SUB
