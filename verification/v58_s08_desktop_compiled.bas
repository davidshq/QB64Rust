$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _SCREENCLICK, _SCREENPRINT and _NOTIFYPOPUP in each form, compiled and never
' run (they act on the user's desktop, design D3).
k = 0
IF k = 1 THEN
    _SCREENCLICK 10, 20
    _SCREENCLICK 10, 20, 2
    x = 1.5: _SCREENCLICK x, x, x
    _SCREENPRINT "abc"
    s$ = "d": _SCREENPRINT s$ + "e"
    _NOTIFYPOPUP
    _NOTIFYPOPUP "t"
    _NOTIFYPOPUP "t", "m"
    _NOTIFYPOPUP "t", "m", "warning"
    _NOTIFYPOPUP , "m"
    _NOTIFYPOPUP , , "info"
    _NOTIFYPOPUP s$, , "error"
END IF
PRINT "ok"
SYSTEM
