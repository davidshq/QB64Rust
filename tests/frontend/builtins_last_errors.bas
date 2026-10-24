' TEST: check-fail
' The forms of the built-ins of m4-builtins-remaining the old compiler rejects (verification\v58_x01-x111): a wrong
' count of arguments, a string for a number or a number for a string, a function as a statement, a statement's
' form the template does not allow, INSTR with another argument than its first left out, DATE$ = with no string.
' Each is an error of the program, none "not supported yet".
$CONSOLE:ONLY
DIM l AS LONG, s AS STRING
IF l = 1 THEN
    x = _ASINH(1, 2)
    x = _ASINH("a")
    x = _ASINH
    x = _SETBIT(1)
    x = _SETBIT("a", 1)
    s = MKSMBF$("a")
    x = CVSMBF(1)
    x = INP
    x = INP("a")
    OUT 1
    OUT "a", 1
    WAIT 1
    x = LPOS
    x = _CAPSLOCK(1)
    _CAPSLOCK 1
    _CAPSLOCK
    _NUMLOCK TOGGLE
    x = _CINP(1, 2)
    x = _CONSOLEINPUT(1)
    _CONSOLETITLE
    _CONSOLETITLE 1
    _CONSOLECURSOR 1
    _CONSOLEFONT "a"
    x = _SCALEDWIDTH(1)
    _GLRENDER
    _GLRENDER 1
    _ALLOWFULLSCREEN 1
    _ALLOWFULLSCREEN _STRETCH, _STRETCH
    _FPS "a"
    _FPS 30, 1
    x = _DISPLAY(1)
    _BLINK
    _BLINK 1
    x = _BLINK(1)
    _SCREENCLICK 10
    _SCREENCLICK "a", 1
    _SCREENPRINT
    _SCREENPRINT 1
    _NOTIFYPOPUP 1
    _NOTIFYPOPUP "a", "b", "c", "d"
    x = _COLORCHOOSERDIALOG("t", "u")
    x = _COLORCHOOSERDIALOG(1, 2, 3)
    s = _INPUTBOX$(1)
    s = _INPUTBOX$("a", "b", "c", "d")
    s = _COMPILERVERSION$(1)
    x = _ENVIRONCOUNT(1)
    x = _UPTIME(1)
    FILES 1
    FILES "a", "b"
    DATE$ = 5
    TIME$ = 1
    DATE$ = "a", "b"
    DATE$ "x"
    DATE$ =
    x = INSTR(, "a")
    x = INSTR(1, , "a")
    x = INSTR(, , "a")
    _INPUTBOX$ "t"
    _SCALEDWIDTH
    _ENVIRONCOUNT
    _CONSOLECURSOR _SHOW, 50, 1
    _ASINH 1
END IF
