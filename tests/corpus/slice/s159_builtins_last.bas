$CONSOLE:ONLY
' Slice (m4-builtins-remaining, task 3.1): the built-ins no program used before, each that a console program can
' run (verification\v58_f01-f13, s01-s04, s06b, i01): the hyperbolic and gradian functions, the bit functions, the
' MBF conversions, _COMPILERVERSION$, _ENVIRONCOUNT, _UPTIME, LPOS, the lock keys (0 with no window), _BLINK as a
' function and a statement, INP, OUT and WAIT on the ports libqb
' emulates and one it does not, DATE$ = and TIME$ = (nothing changes), the console statements, INSTR with its start
' left out, FILES of the folder's a.txt (the path and the free space masked by the .normalize rules), INPUT$ on
' that file (one character, none, past its end: error 62), and in code
' that is not reached the built-ins never run (design D3) and _CINP and _CONSOLEINPUT, which read the runner's
' console (a key event waiting there changed a recording).
ON ERROR GOTO handler
PRINT _ASINH(1); _ACOSH(2); _ATANH(.5)
s! = 1.5: d# = 1.5: f## = 1.5: i% = 2
PRINT _ASINH(s!); _ACOSH(d#); _ATANH(f## / 3); _ASINH(i%)
PRINT _D2G(90); _G2D(100); _R2G(1); _G2R(200); _D2G(s!)
PRINT _SETBIT(5, 1); _RESETBIT(5, 0); _TOGGLEBIT(5, 2)
b%% = 1: q&& = 0: ub~%% = 1: PRINT _SETBIT(b%%, 7); _TOGGLEBIT(q&&, 63); _RESETBIT(ub~%%, 0); _SETBIT(5, 40)
PRINT CVSMBF(MKSMBF$(1.5)); CVDMBF(MKDMBF$(-2.25#)); LEN(MKSMBF$(1)); LEN(MKDMBF$(1))
PRINT ASC(MKSMBF$(1.5), 4); ASC(MKDMBF$(1.5#), 8)
x! = CVSMBF("ab"): PRINT x!
PRINT LEN(_COMPILERVERSION$) > 0; LEFT$(_COMPILERVERSION$, 8)
PRINT _ENVIRONCOUNT > 0; _UPTIME > 0; LPOS(0)
x = LPOS(4): PRINT x
PRINT _CAPSLOCK; _NUMLOCK; _SCROLLLOCK
PRINT _BLINK
_BLINK OFF: PRINT _BLINK
_BLINK ON
OUT &H3C0, 0: PRINT _BLINK
k = INP(&H60): u = INP(&H379): OUT &H378, 1: PRINT k = INP(&H60); u
WAIT &H378, 1
x = INP(70000): PRINT x
d$ = DATE$: DATE$ = "01-02-2003": PRINT DATE$ = d$
t$ = LEFT$(TIME$, 5): TIME$ = "12:34:56": PRINT LEFT$(TIME$, 5) >= t$ OR t$ = "23:59"
_CONSOLETITLE "slice": _CONSOLECURSOR _HIDE: _CONSOLECURSOR _SHOW, 50: _CONSOLECURSOR
PRINT INSTR(, "abab", "b"); INSTR(, "hello", "zz"); INSTR(2, "abab", "b")
FILES "*.txt"
OPEN "a.txt" FOR INPUT AS #1
PRINT INPUT$(1, #1); "|"; LEN(INPUT$(0, 1)); "|"
s$ = INPUT$(5, #1): PRINT LEN(s$)
CLOSE #1
IF k = 1 THEN
    PRINT _CONSOLEINPUT; _CINP; _CINP(0); _CINP(1)
    _SCREENCLICK 10, 20: _SCREENPRINT "abc": _NOTIFYPOPUP "t", "m"
    c~& = _COLORCHOOSERDIALOG: a$ = _INPUTBOX$("t")
    _CAPSLOCK ON: _NUMLOCK OFF: _SCROLLLOCK _TOGGLE
    _CONSOLEFONT "Consolas", 16
END IF
PRINT "done"
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT
