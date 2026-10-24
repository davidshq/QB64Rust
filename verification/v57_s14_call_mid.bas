$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): the stub CALL MID$(a$, 1, 1, "x").
a$ = "abc"
CALL MID$(a$, 1, 1, "x")
PRINT a$
PRINT "ok"
SYSTEM
