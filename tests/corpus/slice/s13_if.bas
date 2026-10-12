$CONSOLE:ONLY
' IF (spec language/control-flow, "IF"): block IF with ELSEIF chains and ELSE, nesting, conditions as numbers
' (true when not zero), single-line IF with and without ELSE (the statements after ELSE belong to the ELSE part),
' a single-line IF inside a single-line IF, IF ... GOTO and IF ... THEN GOTO.
PRINT "ELSEIF chain"
pick 1
pick 2
pick 3
pick 7
PRINT "conditions are numbers"
IF 5 THEN PRINT "  5 is true"
IF -1 THEN PRINT "  -1 is true"
IF 0.4 THEN PRINT "  0.4 is true" ELSE PRINT "  0.4 is false"
IF 0 THEN PRINT "  0 is true" ELSE PRINT "  0 is false"
IF 2 AND 1 THEN PRINT "  2 AND 1 is true" ELSE PRINT "  2 AND 1 is false"
c$ = "b"
IF c$ = "b" THEN PRINT "  c$ is b"
IF c$ <> "b" THEN PRINT "  c$ is not b" ELSE PRINT "  c$ is still b"
PRINT "single-line IF with ELSE"
IF 0 THEN PRINT "a" ELSE PRINT "b": PRINT "c"
IF 1 THEN PRINT "a": PRINT "a2" ELSE PRINT "b": PRINT "c"
PRINT "d"
PRINT "single-line IF without ELSE"
IF 0 THEN PRINT "never": PRINT "never 2"
PRINT "  after"
PRINT "single-line IF inside a single-line IF"
IF 1 THEN IF 0 THEN PRINT "  inner then" ELSE PRINT "  inner else"
IF 0 THEN IF 1 THEN PRINT "  inner then" ELSE PRINT "  which else?"
PRINT "  after"
PRINT "nested block IF"
a = 1: b = 0
IF a THEN
    IF b THEN
        PRINT "  a and b"
    ELSE
        PRINT "  a, not b"
    END IF
    PRINT "  still in a"
ELSEIF b THEN
    PRINT "  b only"
END IF
IF b THEN
    PRINT "  b"
ELSEIF a THEN
    IF a = 1 THEN PRINT "  a is 1" ELSE PRINT "  a is not 1"
END IF
PRINT "block IF, false, no ELSE"
IF a = 2 THEN
    PRINT "  never"
END IF
PRINT "  after END IF"
PRINT "IF ... GOTO and IF ... THEN GOTO"
n = 0
again:
n = n + 1
PRINT "  pass"; n
IF n < 3 GOTO again
IF n = 3 THEN GOTO jumped
PRINT "  not printed"
jumped:
PRINT "  after the jump"
IF n = 4 THEN GOTO again ELSE PRINT "  no jump"
SYSTEM

SUB pick (x)
    IF x = 1 THEN
        PRINT "  one"
    ELSEIF x = 2 THEN
        PRINT "  two"
    ELSEIF x > 2 AND x < 5 THEN
        PRINT "  three or four"
    ELSE
        PRINT "  other"; x
    END IF
END SUB
