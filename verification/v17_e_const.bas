$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): CONST visibility and typing. `x * 1000000000` tells a 32-bit
' computation (wraps) from a 64-bit one; `x / 3` shows SINGLE or DOUBLE digits.
CONST c1 = 5
PRINT "c1:"; c1
late
PRINT "literal 3 * 1000000000:"; 3 * 1000000000
CONST i3 = 3
PRINT "i3 * 1000000000:"; i3 * 1000000000; " i3 / 7:"; i3 / 7
CONST l3 = 3000000000
PRINT "l3:"; l3; " l3 * 4:"; l3 * 4
CONST f2 = 4 / 2
PRINT "f2 = 4 / 2:"; f2; " f2 / 3:"; f2 / 3; " f2 * 1000000000:"; f2 * 1000000000
CONST c3 = 1 / 3, c3s! = 1 / 3, c3d# = 1 / 3
c = 1 / 3
PRINT "c3:"; c3; " c3s!:"; c3s!; " c3d#:"; c3d#; " c = 1 / 3:"; c
CONST h = 0.1
PRINT "h = 0.1:"; h; " h * 3:"; h * 3; " h / 3:"; h / 3
CONST hd = 0.1#
PRINT "hd = 0.1#:"; hd; " hd * 3:"; hd * 3
CONST e = 1E+30
PRINT "e = 1E+30:"; e
CONST e2 = 2.5E+10
PRINT "e2 = 2.5E+10:"; e2
CONST cmp = 3 > 2, cmpf = 2 > 3
PRINT "cmp:"; cmp; cmpf
CONST lg = 6 AND 3, nt = NOT 0, lf = 1.5 AND 3, xr = 5 XOR 3, eq = 5 EQV 3, im = 5 IMP 3, orr = 4 OR 1
PRINT "logic:"; lg; nt; lf; xr; eq; im; orr
CONST p = 2 ^ 0.5
PRINT "p = 2 ^ 0.5:"; p
CONST pw = 2 ^ 3 ^ 2, pw2 = (2 ^ 3) ^ 2, pw3 = 2 ^ 10
PRINT "pw:"; pw; pw2; pw3; " run time:"; 2 ^ 3 ^ 2
CONST rh% = 2.5, rh2% = 3.5, rh3& = -2.5, rh4% = 2.4999
PRINT "suffix rounding:"; rh%; rh2%; rh3&; rh4%
CONST neg = -5, sub2 = 2 - 3
PRINT "negatives:"; neg; sub2
CONST s = "x" + "y"
PRINT "s: "; s; LEN(s)
CONST ref = c1 * 2
PRINT "ref = c1 * 2:"; ref
CONST idv = 7 \ 2, md = -7 MOD 3, fl = 7.5 \ 2, fm = 7.5 MOD 2, idn = -7 \ 2
PRINT "idiv mod:"; idv; md; fl; fm; idn
CONST big2 = 2147483647 + 1, i16 = 32767 + 1, mx = 9223372036854775807
PRINT "big:"; big2; i16; mx
CONST mul = 3000000000 * 4
PRINT "mul:"; mul
CONST hx = &HFF, hx2 = &HFFFF, hx3 = &HFFFFFFFF
PRINT "hex:"; hx; hx2; hx3
CONST pr = 1 + 2 * 3, pr2 = (1 + 2) * 3, pr3 = -2 ^ 2, pr4 = 10 - 2 - 3
PRINT "precedence:"; pr; pr2; pr3; pr4
IF 1 THEN CONST inif = 4
PRINT "CONST in single-line IF:"; inif
FOR j = 1 TO 2: CONST infor = 6: NEXT
PRINT "CONST in FOR:"; infor
IF 0 THEN
    CONST skipped = 8
END IF
PRINT "CONST in a skipped IF block:"; skipped
SYSTEM

SUB late
    PRINT "late SUB sees c1:"; c1
    CONST pc = 9
    PRINT "late SUB constant pc:"; pc
END SUB
