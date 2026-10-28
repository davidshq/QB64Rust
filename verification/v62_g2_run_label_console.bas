$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 14, the output of a $CONSOLE:ONLY program after RUN label
' from the main module (the state is kept in the process's environment, which RUN does not clear).
IF ENVIRON$("V62_G2_RAN") = "" THEN PRINT "one"
again:
IF ENVIRON$("V62_G2_RAN") = "" THEN ENVIRON "V62_G2_RAN=1": RUN again
PRINT "two"
SYSTEM
