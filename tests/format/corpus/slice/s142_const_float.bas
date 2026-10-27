$Console:Only
' The CONST evaluator's floats and lanes (m3-constant-evaluation; verification\v45_a_precision, v45_a_lanes,
' v45_a_auto, v45_b_conv, v45_b_asc_suffix): _FLOAT precision and the type a float constant takes from its text,
' the unsigned lane, a float in the integer lanes by qbr, integer-valued floats near 2^63, suffixed constants inside
' a CONST, ASC, and the auto-included constants that were not computed before (_E, the limits of the floats). Not
' here: the old evaluator's clamps beyond _INTEGER64 (unsuffixed literals, 9.3E+18 / 1, the limits _INTEGER64_MIN,
' _OFFSET_MIN, _UINTEGER64_MAX, _UOFFSET_MAX), fixed by QB64PE-DIVERGENCES.md D-027 (const-rulings; the values are in
' tests\frontend\const_rulings_typed.bas and the CLI test const_rulings_run).
Const third## = 1## / 3##
Const c = 1E+30 / 3
Const d = 1 / 3
Const e = 1E-10 / 3
Const t = 0.1 * 3
Const t2## = 0.1 * 3
Const big = 1E+19 / 1
Print _ToStr$(third##); " "; _ToStr$(c); " "; _ToStr$(d); " "; _ToStr$(e)
Print _ToStr$(t); " "; _ToStr$(t2##); " "; _ToStr$(big)
Print c; d; big

Const a = 18446744073709551615~&&
Const b = a + 1
Const u = a - 1
Const c8% = 4 + 4
Const d8 = c8% * 2
Print a; b; u
Print d8

Const i1 = 2.5 \ 1, i2 = 3.5 \ 1, i3 = -2.5 \ 1, i4 = 2.4999 \ 1, i5 = 7.5 Mod 4, i6 = -7.5 Mod 4
Const u1 = 2.5 And 255~&&, u2 = 3.5 Or 0~&&, u3 = 1E+19 Or 0~&&, u4 = -1.5 Or 0~&&
Const s1 = 1E+19 Or 0, s2 = -1E+19 Or 0, s3 = 1E+30 Or 0
Const n1 = -18446744073709551615~&&, n2 = Not 0~&&, n3 = Not 2.5
Const g2 = 9.2E+18 / 1
Const h = 1E+18 / 1, k = 1E-5 / 1, k2 = 0.0001 / 1, k3 = 123456789012345678 / 10
Print i1; i2; i3; i4; i5; i6
Print u1; u2; u3; u4
Print s1; s2; s3
Print n1; n2; n3
Print g2
Print h; _ToStr$(k); " "; _ToStr$(k2); " "; _ToStr$(k3)

Const a1 = Asc("A"), a2 = Asc("abc", 2), s = "xyz", a3 = Asc(s), a4 = Asc(s, 3)
Const a5 = Asc("\"), a6 = Chr$(Asc(Chr$(Asc("\")))), a7 = Asc("abc", 2.5)
Const cc% = 8
Const dd = cc% * 2, ee = cc& + 1, ff = cc! / 2, gg = cc## * 1, hh = cc~&& - 9, kk = cc&& + cc
Const uu~& = 4294967295
Const vv = uu% + 1, ww = uu~& + 1
Const tt$ = "ab"
Const tt2 = tt$ + "c", tt3 = tt + "d"
Const fl# = 0.1
Const fl2 = fl# * 3, fl3 = fl% * 3
Print a1; a2; a3; a4; a5; a6; a7
Print dd; ee; ff; gg; hh; kk
Print vv; ww; tt2; " "; tt3
Print _ToStr$(fl2); " "; _ToStr$(fl3)

Print _ToStr$(_E); _E
Print _ToStr$(_DOUBLE_MAX); " "; _ToStr$(_DOUBLE_MIN)
Print _ToStr$(_FLOAT_MIN_FRAC); " "; _ToStr$(_FLOAT_MAX); " "; _ToStr$(_FLOAT_MIN)
System
