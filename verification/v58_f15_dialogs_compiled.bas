$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): every form of _COLORCHOOSERDIALOG and _INPUTBOX$, compiled and never run (each waits
' for a person, design D3).
k = 0
IF k = 1 THEN
    c~& = _COLORCHOOSERDIALOG
    c~& = _COLORCHOOSERDIALOG("t")
    c~& = _COLORCHOOSERDIALOG("t", &HFF0000)
    c~& = _COLORCHOOSERDIALOG(, 5)
    d# = _COLORCHOOSERDIALOG("t", 1.5)
    a$ = _INPUTBOX$
    a$ = _INPUTBOX$("t")
    a$ = _INPUTBOX$("t", "m")
    a$ = _INPUTBOX$("t", "m", "d")
    a$ = _INPUTBOX$(, , "d")
    a$ = _INPUTBOX$(, "m")
    PRINT _INPUTBOX$("t"); _COLORCHOOSERDIALOG
END IF
PRINT "ok"
SYSTEM
