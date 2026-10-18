$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): ASC in a CONST, and constants used with a suffix inside a CONST
CONST a1 = ASC("A"), a2 = ASC("abc", 2), s = "xyz", a3 = ASC(s), a4 = ASC(s, 3)
CONST a5 = ASC("\"), a6 = CHR$(ASC(CHR$(ASC("\")))), a7 = ASC("abc", 2.5)
CONST c% = 8
CONST d = c% * 2, e = c& + 1, f = c! / 2, g = c## * 1, h = c~&& - 9, k = c&& + c
CONST u~& = 4294967295
CONST v = u% + 1, w = u~& + 1
CONST t$ = "ab"
CONST t2 = t$ + "c", t3 = t + "d"
CONST fl# = 0.1
CONST fl2 = fl# * 3, fl3 = fl% * 3
PRINT a1; a2; a3; a4; a5; a6; a7
PRINT d; e; f; g; h; k
PRINT v; w; t2; " "; t3
PRINT _TOSTR$(fl2); " "; _TOSTR$(fl3)
SYSTEM
