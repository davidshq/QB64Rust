$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 14, the output of a $CONSOLE:ONLY program after RUN of a line
' number from a SUB, and after RUN with no argument (the state is kept in the process's environment, which RUN does
' not clear).
r$ = ENVIRON$("V62_G3_RUNS")
PRINT "start "; r$
IF r$ = "" THEN ENVIRON "V62_G3_RUNS=1": s
100 PRINT "at 100 "; ENVIRON$("V62_G3_RUNS")
IF ENVIRON$("V62_G3_RUNS") = "1" THEN ENVIRON "V62_G3_RUNS=2": RUN
PRINT "end"
SYSTEM

SUB s
    RUN 100
END SUB
