$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _CONSOLEINPUT and _CINP (bare, with 0 and with 1) in a console program whose
' standard input is the null device: nothing waits.
PRINT _CONSOLEINPUT; _CINP; _CINP(0); _CINP(1)
PRINT _CONSOLEINPUT; _CINP
x&& = _CINP: PRINT x&&
SYSTEM
