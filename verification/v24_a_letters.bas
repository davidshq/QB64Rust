$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DEFINT, DEFLNG, DEFSNG, DEFDBL, DEFSTR: ranges, single letters,
' letter case, a range written backwards, a letter named twice, a later statement for the same letter, suffix
' and AS against the default. LEN of a variable shows its type: 2 INTEGER, 4 LONG or SINGLE, 8 DOUBLE.
a1 = 2.5: PRINT "no statement: a1"; a1; LEN(a1)
DEFINT A-Z
x1 = 2.5: PRINT "DEFINT A-Z: x1, x1 / 2, LEN"; x1; x1 / 2; LEN(x1)
DIM d1 AS DOUBLE: d1 = 1.5: s1! = 2.5: n1$ = "q": PRINT "suffix and AS win:"; d1; s1!; n1$; LEN(d1); LEN(s1!)
DEFDBL D
i2 = 1.5: d2 = 1.5: PRINT "DEFDBL D after DEFINT A-Z: i2, d2"; i2; d2; LEN(i2); LEN(d2)
DEFSNG A-Z
DEFLNG a-c, x
b3 = 100000: d3 = 1.5: x3 = 70000: c3 = 2.5
PRINT "DEFLNG a-c, x: b3, d3, x3, c3"; b3; d3; x3; c3; LEN(b3); LEN(d3); LEN(x3); LEN(c3)
DEFSNG A-Z
DEFINT z-w
v4 = 2.5: w4 = 2.5: x4 = 2.5: y4 = 2.5: z4 = 2.5: PRINT "DEFINT z-w: v4 to z4"; v4; w4; x4; y4; z4
DEFSNG A-Z
DEFINT k, K, k-m, l
k5 = 2.5: l5 = 2.5: m5 = 2.5: n5 = 2.5: PRINT "DEFINT k, K, k-m, l: k5 to n5"; k5; l5; m5; n5
DEFSNG A-Z
DEFSTR S
s6 = "text": PRINT "DEFSTR S: "; s6 + "!"; LEN(s6)
DEFSNG S
DEFDBL e-e
e7 = 1# / 3: f7 = 1# / 3: PRINT "DEFDBL e-e: e7, f7"; e7; f7
DEFSNG A-Z
DEFLNG A-Z: DEFINT Q: q8 = 2.5: r8 = 2.5: PRINT "two statements on one line: q8, r8"; LEN(q8); LEN(r8)
defsng a-z
defdbl t
t9 = 1# / 3: PRINT "lower-case defdbl t: t9"; t9
SYSTEM
