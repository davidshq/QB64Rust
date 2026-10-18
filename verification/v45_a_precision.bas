$CONSOLE:ONLY
' Verification (m3-constant-evaluation, before its proposal): the precision of the old evaluator's floats and the type a
' float constant takes from its text (an exponent: _FLOAT, none: DOUBLE)
CONST third## = 1## / 3##
CONST c = 1E+30 / 3
CONST d = 1 / 3
CONST e = 1E-10 / 3
CONST w = 2 ^ 0.5
CONST t = 0.1 * 3
CONST t2## = 0.1 * 3
CONST big = 1E+19 / 1
PRINT _TOSTR$(third##); " "; _TOSTR$(c); " "; _TOSTR$(d); " "; _TOSTR$(e); " "; _TOSTR$(w)
PRINT _TOSTR$(t); " "; _TOSTR$(t2##); " "; _TOSTR$(big)
PRINT c; d; big
SYSTEM
