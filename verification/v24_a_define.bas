$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): _DEFINE with each type, a range and a list. LEN of a variable
' shows its type: 2 INTEGER, 4 LONG or SINGLE, 8 DOUBLE.
_DEFINE a AS INTEGER: a1 = 2.5: PRINT "INTEGER"; a1; LEN(a1)
_DEFINE b AS LONG: b1 = 2.5: PRINT "LONG"; b1; LEN(b1)
_DEFINE c AS SINGLE: c1 = 2.5: PRINT "SINGLE"; c1; LEN(c1)
_DEFINE d AS DOUBLE: d1 = 1# / 3: PRINT "DOUBLE"; d1; LEN(d1)
_DEFINE e AS _FLOAT: e1 = 2.5: PRINT "_FLOAT"; e1; LEN(e1)
_DEFINE f AS _BYTE: f1 = -1: PRINT "_BYTE"; f1; LEN(f1)
_DEFINE g AS _UNSIGNED _BYTE: g1 = -1: PRINT "_UNSIGNED _BYTE"; g1; LEN(g1)
_DEFINE h AS _UNSIGNED INTEGER: h1 = -1: PRINT "_UNSIGNED INTEGER"; h1; LEN(h1)
_DEFINE i AS _UNSIGNED LONG: i1 = -1: PRINT "_UNSIGNED LONG"; i1; LEN(i1)
_DEFINE j AS _INTEGER64: j1 = 5000000000.5: PRINT "_INTEGER64"; j1; LEN(j1)
_DEFINE k AS _UNSIGNED _INTEGER64: k1 = -1: PRINT "_UNSIGNED _INTEGER64"; k1; LEN(k1)
_DEFINE l AS _OFFSET: l1 = -1: PRINT "_OFFSET"; l1; LEN(l1)
_DEFINE m AS _UNSIGNED _OFFSET: m1 = 5: PRINT "_UNSIGNED _OFFSET"; m1; LEN(m1)
_DEFINE n AS STRING: n1 = "abc": PRINT "STRING "; n1; LEN(n1)
_DEFINE o-q, s AS LONG
o1 = 2.5: p1 = 2.5: q1 = 2.5: r1 = 2.5: s1 = 2.5: PRINT "o-q, s AS LONG: o1 to s1"; LEN(o1); LEN(p1); LEN(q1); r1; LEN(s1)
_Define U As _Unsigned _Byte: u = -1: PRINT "the delta's scenario: u"; u
DEFINT A-Z
_DEFINE v AS DOUBLE
v1 = 1# / 3: w1 = 2.5: PRINT "_DEFINE v after DEFINT A-Z: v1, w1"; v1; w1
SYSTEM
