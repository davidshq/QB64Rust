$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN of a main-module label from a SUB, and from the main module.
PRINT "ok"
SYSTEM
lab:
RUN lab
SUB s
    RUN lab
END SUB
