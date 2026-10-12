$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Data"): RESTORE to a label before, between and after DATA lines, to a
' label on the DATA line itself, to a label with no DATA after it, to a label in a procedure, from a procedure to
' a main-module label, to a line number; the same label used by GOTO.
ON ERROR GOTO h
DIM s AS STRING
before:
DATA d1, d2
between:
PRINT "start"
DATA d3
online: DATA d4, d5
100 DATA d6
after:
PRINT "RESTORE before": RESTORE before: READ s: PRINT s
PRINT "RESTORE between": RESTORE between: READ s: PRINT s
PRINT "RESTORE online": RESTORE online: READ s: PRINT s
PRINT "RESTORE 100": RESTORE 100: READ s: PRINT s
PRINT "RESTORE after (the next DATA is in a procedure)": RESTORE after: READ s: PRINT s
PRINT "RESTORE insub (a label in SUB p)": RESTORE insub: READ s: PRINT s
PRINT "RESTORE last (no DATA after it)": RESTORE last: s = "old": READ s: PRINT "["; s; "]"
PRINT "RESTORE forward, then all": RESTORE: FOR i = 1 TO 8: READ s: PRINT s; " ";: NEXT: PRINT
p
PRINT "back in main": READ s: PRINT s
GOTO between2
between2:
PRINT "RESTORE to a label GOTO also uses": RESTORE between2: s = "old": READ s: PRINT "["; s; "]"
PRINT "RESTORE upper case": RESTORE BEFORE: READ s: PRINT s
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT

SUB p
    DIM s AS STRING
    PRINT "in p: RESTORE between (main-module label)": RESTORE between: READ s: PRINT s
    insub:
    DATA s1
    PRINT "in p: RESTORE insub": RESTORE insub: READ s: PRINT s
    PRINT "in p: RESTORE": RESTORE: READ s: PRINT s
END SUB

SUB q
    last:
END SUB
