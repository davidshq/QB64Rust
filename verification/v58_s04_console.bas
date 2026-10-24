$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): the console's own statements in a console program: _CONSOLETITLE, _CONSOLECURSOR
' in each form, and _CONSOLEFONT compiled and never run (it changes the console the program shares, design D3).
_CONSOLETITLE "t"
_CONSOLETITLE ""
t$ = "title": _CONSOLETITLE t$ + "!"
_CONSOLECURSOR _HIDE
_CONSOLECURSOR _SHOW, 50
_CONSOLECURSOR _SHOW
_CONSOLECURSOR , 20
_CONSOLECURSOR _HIDE, 101
n = 25: _CONSOLECURSOR _SHOW, n
k = 0
IF k = 1 THEN
    _CONSOLEFONT "Consolas", 16
    _CONSOLEFONT "", 14
    _CONSOLEFONT t$, n
END IF
PRINT "ok"
SYSTEM
