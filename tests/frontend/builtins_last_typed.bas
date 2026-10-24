' TEST: typed
' The functions no program used before m4-builtins-remaining (its design D2; verification\v58_f01-f15, i01): the
' hyperbolic and gradian functions with a _FLOAT slot and result, each argument passed as the plain rule converts
' it; _SETBIT and kin (an _UNSIGNED _INTEGER64 of an _UNSIGNED _INTEGER64 and a LONG slot); the MBF conversions;
' LPOS, INP, INPUT$ (a count and a file number, with or without #; the console form compiled), _COMPILERVERSION$,
' _ENVIRONCOUNT, _UPTIME; the lock keys, _CINP (bare and with its optional slot),
' _CONSOLEINPUT; the display functions, _DISPLAY and _BLINK as functions; INSTR with its first argument left out
' (the slot left empty, D5); the dialogs, compiled in code that is not reached (D3).
$CONSOLE:ONLY
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, u AS _UNSIGNED _INTEGER64, b AS _BYTE
DIM f AS SINGLE, d AS DOUBLE, x AS _FLOAT, s AS STRING, fx AS STRING * 4
PRINT _ASINH(f); _ACOSH(d); _ATANH(x); _ASINH(i); _ACOSH(1.5); _ATANH(.5)
PRINT _D2G(f); _G2D(d); _R2G(x); _G2R(l); _D2G(90)
PRINT _SETBIT(b, 7); _RESETBIT(i, 0); _TOGGLEBIT(q, 63); _SETBIT(u, l); _SETBIT(5, 1); _TOGGLEBIT(f, d)
PRINT MKSMBF$(f); MKDMBF$(d); MKSMBF$(i); MKDMBF$(x); CVSMBF(s); CVDMBF(fx); CVSMBF(MKSMBF$(1.5))
PRINT LPOS(0); LPOS(l); INP(&H3C9); INP(d)
PRINT INPUT$(3, #1); INPUT$(l, 1); INPUT$(2, #l); INPUT$(d)
PRINT _COMPILERVERSION$; _ENVIRONCOUNT; _UPTIME
PRINT _CAPSLOCK; _NUMLOCK; _SCROLLLOCK; _CINP; _CINP(0); _CINP(l); _CONSOLEINPUT
PRINT _SCALEDWIDTH; _SCALEDHEIGHT; _SMOOTH; _DESKTOPREFRESHRATE; _DISPLAY; _BLINK
PRINT INSTR(, "ab", "b"); INSTR(, s, fx); INSTR(2, s, "b"); INSTR(s, "b")
IF l = 1 THEN
    PRINT _COLORCHOOSERDIALOG; _COLORCHOOSERDIALOG("t"); _COLORCHOOSERDIALOG("t", &HFF0000); _COLORCHOOSERDIALOG(, 5)
    PRINT _INPUTBOX$; _INPUTBOX$("t"); _INPUTBOX$("t", "m"); _INPUTBOX$("t", "m", "d"); _INPUTBOX$(, , "d")
END IF
SYSTEM
