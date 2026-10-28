$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 14, how many RUNs from a SUB the program takes before it runs
' out of stack. The count is kept in the process's environment, which RUN does not clear; it is printed every
' 100 runs up to 5,000 and every 5,000 after, and the program ends at 100,000.
c = VAL(ENVIRON$("V62_G1_COUNT")) + 1
ENVIRON "V62_G1_COUNT=" + LTRIM$(STR$(c))
IF c MOD 5000 = 0 OR (c < 5000 AND c MOD 100 = 0) THEN PRINT c
IF c < 100000 THEN s
PRINT "done"; c
SYSTEM

SUB s
    DIM t AS STRING
    t = SPACE$(100)
    RUN
END SUB
