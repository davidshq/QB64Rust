$CONSOLE:ONLY
' SWAP and the MID$ statement (spec language/builtin-statements "SWAP", "MID$ statement"): SWAP of every numeric
' type, of two types that differ only in signedness, strings, fixed-length strings of equal and unequal length, a
' string with a fixed-length one, TYPE variables and elements, elements, members, a variable with an element or a
' member, the same place twice, an element with a bad index (error 9; the exchange is then made with the array's
' first element, as the old compiler's program does), in a SUB; the MID$ statement with every start and length,
' fixed-length, element and member targets, its own value, raising arguments.
ON ERROR GOTO h
TYPE rec
    n AS LONG
    s AS STRING * 3
    d AS DOUBLE
END TYPE
DIM b1 AS _BYTE, b2 AS _BYTE, ub1 AS _UNSIGNED _BYTE, ub2 AS _UNSIGNED _BYTE
DIM i1 AS INTEGER, i2 AS INTEGER, ui1 AS _UNSIGNED INTEGER, ui2 AS _UNSIGNED INTEGER
DIM l1 AS LONG, l2 AS LONG, ul1 AS _UNSIGNED LONG, ul2 AS _UNSIGNED LONG
DIM q1 AS _INTEGER64, q2 AS _INTEGER64, uq1 AS _UNSIGNED _INTEGER64, uq2 AS _UNSIGNED _INTEGER64
DIM s1 AS SINGLE, s2 AS SINGLE, d1 AS DOUBLE, d2 AS DOUBLE, f1 AS _FLOAT, f2 AS _FLOAT
DIM o1 AS _OFFSET, o2 AS _OFFSET, uo1 AS _UNSIGNED _OFFSET, uo2 AS _UNSIGNED _OFFSET
DIM a AS STRING, c AS STRING, x1 AS STRING * 3, x2 AS STRING * 3, x5 AS STRING * 5
DIM r1 AS rec, r2 AS rec, arr(3) AS LONG, sarr(3) AS STRING, rarr(2) AS rec, k AS LONG
DIM s AS STRING, fx AS STRING * 5, d AS DOUBLE, n AS LONG

b1 = 1: b2 = 2: SWAP b1, b2: PRINT "_BYTE"; b1; b2
ub1 = 1: ub2 = 200: SWAP ub1, ub2: PRINT "_UNSIGNED _BYTE"; ub1; ub2
i1 = 1: i2 = 2: SWAP i1, i2: PRINT "INTEGER"; i1; i2
ui1 = 1: ui2 = 60000: SWAP ui1, ui2: PRINT "_UNSIGNED INTEGER"; ui1; ui2
l1 = 1: l2 = 2: SWAP l1, l2: PRINT "LONG"; l1; l2
ul1 = 1: ul2 = 4000000000: SWAP ul1, ul2: PRINT "_UNSIGNED LONG"; ul1; ul2
q1 = 1: q2 = 5000000000: SWAP q1, q2: PRINT "_INTEGER64"; q1; q2
uq1 = 1: uq2 = 9000000000: SWAP uq1, uq2: PRINT "_UNSIGNED _INTEGER64"; uq1; uq2
s1 = 1.5: s2 = 2.5: SWAP s1, s2: PRINT "SINGLE"; s1; s2
d1 = 1.5: d2 = 2.5: SWAP d1, d2: PRINT "DOUBLE"; d1; d2
f1 = 1.5: f2 = 2.5: SWAP f1, f2: PRINT "_FLOAT"; f1; f2
o1 = 1: o2 = 2: SWAP o1, o2: PRINT "_OFFSET"; o1; o2
uo1 = 1: uo2 = 2: SWAP uo1, uo2: PRINT "_UNSIGNED _OFFSET"; uo1; uo2
l1 = -1: ul1 = 2: SWAP l1, ul1: PRINT "LONG and _UNSIGNED LONG"; l1; ul1
b1 = -1: ub1 = 2: SWAP ub1, b1: PRINT "_UNSIGNED _BYTE and _BYTE"; ub1; b1
a = "left": c = "r": SWAP a, c: PRINT "STRING ["; a; "]["; c; "]"
x1 = "abc": x2 = "xy": SWAP x1, x2: PRINT "STRING * 3 ["; x1; "]["; x2; "]"
x1 = "abc": x5 = "vwxyz": SWAP x1, x5: PRINT "STRING * 3 and * 5 ["; x1; "]["; x5; "]"
a = "long one": x1 = "abc": SWAP a, x1: PRINT "STRING and STRING * 3 ["; a; "]["; x1; "]"
r1.n = 1: r1.s = "one": r1.d = 1.5: r2.n = 2: r2.s = "two": r2.d = 2.5
SWAP r1, r2: PRINT "TYPE"; r1.n; r1.s; r1.d; r2.n; r2.s; r2.d
arr(1) = 7: arr(2) = 9: SWAP arr(1), arr(2): PRINT "elements"; arr(1); arr(2)
k = 1: SWAP arr(k), arr(k + 1): PRINT "elements by variable"; arr(1); arr(2)
sarr(0) = "p": sarr(3) = "q": SWAP sarr(0), sarr(3): PRINT "string elements "; sarr(0); sarr(3)
l1 = 5: SWAP l1, arr(1): PRINT "variable and element"; l1; arr(1)
SWAP r1.n, l1: PRINT "member and variable"; r1.n; l1
SWAP r1.n, r2.n: PRINT "members"; r1.n; r2.n
x1 = "abc": SWAP r1.s, x1: PRINT "fixed member and variable ["; r1.s; "]["; x1; "]"
rarr(0).n = 10: rarr(1).n = 11: SWAP rarr(0), rarr(1): PRINT "TYPE elements"; rarr(0).n; rarr(1).n
SWAP rarr(0), r1: PRINT "TYPE element and variable"; rarr(0).n; r1.n
l1 = 3: SWAP l1, l1: PRINT "the same variable"; l1
a = "same": SWAP a, a: PRINT "the same string "; a
SWAP made1, made2: PRINT "variables made by SWAP"; made1; made2
arr(0) = 0: arr(1) = 1: l1 = 2: k = 9
PRINT "an element with a bad index": SWAP l1, arr(k): PRINT l1; arr(0); arr(1)
PRINT "a bad index first": SWAP arr(k), l1: PRINT l1; arr(0); arr(1)
insub

s = "abcdef": MID$(s, 3) = "XY": PRINT "no length ["; s; "]"
s = "abcdef": MID$(s, 3, 1) = "XY": PRINT "length 1 ["; s; "]"
s = "abcdef": MID$(s, 3, 5) = "XY": PRINT "length beyond the value ["; s; "]"
s = "abcdef": MID$(s, 5) = "VWXYZ": PRINT "value beyond the end ["; s; "]"
s = "abcdef": MID$(s, 5, 9) = "VWXYZ": PRINT "length and value beyond the end ["; s; "]"
s = "abcdef": MID$(s, 1) = "": PRINT "empty value ["; s; "]"
s = "abcdef": MID$(s, 6) = "Z": PRINT "last position ["; s; "]"
s = "abcdef": MID$(s, 3, 0) = "XY": PRINT "length 0 ["; s; "]"
s = "q": MID$(s, 1, 1) = "ZZ": PRINT "one byte ["; s; "]"
s = "abcdef": PRINT "start 7 (one past the end)": MID$(s, 7) = "Z": PRINT "["; s; "]"
s = "abcdef": PRINT "start 0": MID$(s, 0) = "Z": PRINT "["; s; "]"
s = "abcdef": PRINT "start -1": MID$(s, -1) = "Z": PRINT "["; s; "]"
s = "abcdef": PRINT "length -1": MID$(s, 2, -1) = "XYZ": PRINT "["; s; "]"
s = "": PRINT "empty target, start 1": MID$(s, 1) = "Z": PRINT "["; s; "]"
s = "abcdef": d = 2.5: MID$(s, d, d) = "XYZ": PRINT "start and length 2.5 ["; s; "]"
s = "abcdef": d = 3.5: MID$(s, d) = "XYZ": PRINT "start 3.5 ["; s; "]"
s = "abcdef": MID$(s, 2, 3) = s: PRINT "its own value ["; s; "]"
s = "abcdef": MID$(s, 1, 2) = MID$(s, 5, 2): PRINT "its own MID$ ["; s; "]"
fx = "abcde": MID$(fx, 4) = "XYZ": PRINT "fixed-length ["; fx; "]"
sarr(1) = "abcdef": n = 1: MID$(sarr(n), 2, 2) = "XY": PRINT "element ["; sarr(1); "]"
r1.s = "abc": MID$(r1.s, 2, 2) = "XY": PRINT "member ["; r1.s; "]"
s = "abcdef": PRINT "raising value": MID$(s, 2) = CHR$(-1): PRINT "["; s; "]"
s = "abcdef": PRINT "raising start": MID$(s, ASC("")) = "Z": PRINT "["; s; "]"
n = 9: sarr(0) = "abcdef": PRINT "element with a bad index": MID$(sarr(n), 1) = "Z": PRINT "["; sarr(0); "]"
s = "abcdef": MID$(s, 2, 2) = "X" + "Y" + "Z": PRINT "an expression ["; s; "]"
s = "abcdef": MID$ (s, 2) = "sp": PRINT "a blank before ( ["; s; "]"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT

SUB insub
    DIM p AS LONG, q AS LONG, t AS STRING
    STATIC st AS LONG
    p = 1: q = 2: st = 3
    SWAP p, q: SWAP q, st
    t = "local": MID$(t, 1, 1) = "L"
    PRINT "in a SUB"; p; q; st; t
END SUB
