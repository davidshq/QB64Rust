$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): an omitted argument: MID$(a$, , 2).
u$ = MID$("abc", , 2)
PRINT "ok"
SYSTEM
