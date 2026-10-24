$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN of a label of a SUB, from the main module.
PRINT "ok"
SYSTEM
RUN inside
SUB s
inside:
    PRINT "in s"
END SUB
