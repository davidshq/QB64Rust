$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): the lock-key statements in each form, compiled and never run (they change the
' machine's keyboard state, design D3).
k = 0
IF k = 1 THEN
    _CAPSLOCK ON
    _CAPSLOCK OFF
    _CAPSLOCK _TOGGLE
    _NUMLOCK ON
    _NUMLOCK OFF
    _NUMLOCK _TOGGLE
    _SCROLLLOCK ON
    _SCROLLLOCK OFF
    _SCROLLLOCK _TOGGLE
END IF
PRINT "ok"
SYSTEM
