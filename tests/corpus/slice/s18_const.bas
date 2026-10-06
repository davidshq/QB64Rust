$CONSOLE:ONLY
' CONST (spec language/constants, "CONST visibility", "CONST values"): main constants seen in later SUBs, a SUB
' constant shadowing a main constant and reusing a main variable's name, constants defined whatever the control
' flow around their line (skipped IF block, single-line IF, FOR body), the same CONST twice with an equal value,
' a plain constant used with a numeric suffix; typing (integer results and integer-valued floats are _INTEGER64,
' other floats DOUBLE), suffixes rounding half to even, ^ right-associative, the evaluator's operators, &HFFFF,
' string constants, a constant from constants, a plain constant stored in an INTEGER.
' `x * 1000000000` tells a 32-bit computation (wraps) from a 64-bit one; 2^62 tells _INTEGER64 from DOUBLE.
CONST c1 = 5
PRINT "c1:"; c1
CONST k = 1
v = 7
late
PRINT "main k after late:"; k; " v:"; v
PRINT "literal 3 * 1000000000:"; 3 * 1000000000
CONST i3 = 3, f2 = 4 / 2
PRINT "i3, f2:"; i3 * 1000000000; f2 * 4611686018427387904
CONST h2 = 2.5 * 2, sm = 2.5
PRINT "h2 = 2.5 * 2:"; h2 * 4611686018427387904; " sm = 2.5:"; sm * 4611686018427387904
CONST l3 = 3000000000
PRINT "l3:"; l3; l3 * 4
CONST c3 = 1 / 3, c3s! = 1 / 3, c3d# = 1 / 3
c = 1 / 3
PRINT "1 / 3:"; c3; c3s!; c3d#; c
CONST tenth = 0.1, e30 = 1E+30, e10 = 2.5E+10
PRINT "floats:"; tenth; tenth * 3; tenth / 3; e30; e10
CONST cmp = 3 > 2, cmpf = 2 > 3
PRINT "comparisons:"; cmp; cmpf
CONST lg = 6 AND 3, nt = NOT 0, lf = 1.5 AND 3, xr = 5 XOR 3, eq = 5 EQV 3, im = 5 IMP 3, orr = 4 OR 1
PRINT "logic:"; lg; nt; lf; xr; eq; im; orr
CONST p = 2 ^ 0.5
PRINT "2 ^ 0.5:"; p
CONST pw = 2 ^ 3 ^ 2, pw2 = (2 ^ 3) ^ 2, pw3 = 2 ^ 10
PRINT "power:"; pw; pw2; pw3; " at run time:"; 2 ^ 3 ^ 2
CONST rnd1% = 3.7, rh% = 2.5, rh2% = 3.5, rh3& = -2.5, rh4% = 2.4999
PRINT "suffixes round:"; rnd1%; rh%; rh2%; rh3&; rh4%
CONST neg = -5, sub2 = 2 - 3
PRINT "negatives:"; neg; sub2
CONST s = "x" + "y", s2$ = "z"
PRINT "strings: "; s; s2$
CONST ref = c1 * 2
PRINT "c1 * 2:"; ref
CONST idv = 7 \ 2, md = -7 MOD 3, fl = 7.5 \ 2, fm = 7.5 MOD 2, idn = -7 \ 2
PRINT "\ and MOD:"; idv; md; fl; fm; idn
CONST big2 = 2147483647 + 1, i16 = 32767 + 1, mx = 9223372036854775807, mul = 3000000000 * 4
PRINT "64-bit:"; big2; i16; mx; mul
CONST hx = &HFF, hx2 = &HFFFF, hx3 = &HFFFFFFFF
PRINT "hex:"; hx; hx2; hx3
CONST pr = 1 + 2 * 3, pr2 = (1 + 2) * 3, pr3 = -2 ^ 2, pr4 = 10 - 2 - 3
PRINT "precedence:"; pr; pr2; pr3; pr4
PRINT "with a numeric suffix:"; c1%; c1&; c1!; c1#
CONST c1 = 5
PRINT "the same CONST again:"; c1
IF 1 THEN CONST inif = 4
PRINT "in a single-line IF:"; inif
FOR j = 1 TO 2: CONST infor = 6: NEXT
PRINT "in a FOR body:"; infor
IF 0 THEN
    CONST skipped = 8
END IF
PRINT "in a skipped IF block:"; skipped
CONST big = 40000, half = 2.5
DIM i AS INTEGER
i = big
PRINT "stored in an INTEGER:"; i;
i = half
PRINT i
SYSTEM

SUB late
    PRINT "late sees c1:"; c1
    CONST k = 2
    PRINT "late's k:"; k
    CONST v = 9
    PRINT "late's v:"; v
END SUB
