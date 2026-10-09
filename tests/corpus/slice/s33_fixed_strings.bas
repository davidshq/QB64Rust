$CONSOLE:ONLY
' Slice program (m2-numeric-types, tasks 7.1 and 7.2): fixed-length strings, the scenarios of the
' fixed-length-strings spec delta and the forms measured by verification\v21_c_*: declarations (AS STRING * n with a
' number or a constant, _UNSIGNED STRING * n, the suffix form name$n in DIM and implicit, a length read in 32 bits),
' NUL bytes at the start (also a local on each call, DIVERGENCES-QB45.md Q-005), stores that pad and cut, comparisons
' and SELECT CASE with the padding, built-ins, members, array elements, members of elements, each storage class, and
' fixed strings passed to a STRING parameter by reference (also in parentheses). NUL bytes are shown with ASC, never
' printed. The MID$ statement and FUNCTIONs named name$n are not supported yet. No PRINT comma.
ON ERROR GOTO h
' Declarations and the initial bytes.
DIM f AS STRING * 4
PRINT "initial:"; LEN(f); ASC(f, 1); ASC(f, 4)
DIM w AS STRING * 4294967297
PRINT "4294967297:"; LEN(w)
CONST n = 3
DIM c AS STRING * n
c = "abcdef"
PRINT "STRING * CONST: ["; c; "]"; LEN(c)
DIM u AS _UNSIGNED STRING * 3
u = "abcdef"
PRINT "_UNSIGNED STRING * 3: ["; u; "]"; LEN(u)
DIM p$3
p$3 = "abcdef": p$ = "x"
PRINT "suffix: "; p$3; LEN(p$3); p$
q$2 = "xyz"
PRINT "implicit q$2: ["; q$2; "]"; LEN(q$2)
' Stores: padded with blanks, cut to n.
f = "ab"
PRINT "short: ["; f; "]"; ASC(f, 3); LEN(f)
f = "abcdef"
PRINT "long: ["; f; "]"
f = ""
PRINT "empty: ["; f; "]"; ASC(f, 1)
f = "ab"
f = f + "x"
PRINT "f = f + x: ["; f; "]"
f = "abcd"
f = MID$(f, 2)
PRINT "f = MID$(f, 2): ["; f; "]"
DIM g AS STRING * 6
g = "xy": f = g
PRINT "fixed from longer fixed: ["; f; "]"
g = f
PRINT "fixed from shorter fixed: ["; g; "]"
' Values: comparisons see the padding.
f = "ab"
PRINT "compare:"; f = "ab"; f = "ab  "; f < "ac"; f > "ab"; "ab  " = f
DIM s AS STRING
s = f
PRINT "into STRING: ["; s; "]"; LEN(s)
PRINT "concat: ["; f + "!"; "]"
SELECT CASE f
    CASE "ab": PRINT "case: ab"
    CASE "ab  ": PRINT "case: ab and 2 blanks"
    CASE ELSE: PRINT "case: else"
END SELECT
' Built-ins.
DIM k AS STRING * 5
k = "ab"
PRINT "built-ins:"; LEN(RTRIM$(k)); " ["; UCASE$(k); "] ["; LEFT$(k, 3); "] ["; RIGHT$(k, 2); "] ["; MID$(k, 2, 3); "]"
PRINT "more:"; INSTR(k, " "); INSTR(k, "b"); ASC(k); ASC(k, 5); " ["; LTRIM$(k); "]"; VAL(k); LEN(STR$(LEN(k)))
' A long one.
DIM big AS STRING * 100000
PRINT "100000:"; LEN(big); ASC(big, 100000)
big = "a"
PRINT "100000 after store:"; ASC(big, 1); ASC(big, 2); ASC(big, 100000)
' Members.
TYPE rec
    id AS LONG
    nm AS STRING * 6
    z AS INTEGER
END TYPE
DIM r AS rec
PRINT "member initial:"; LEN(r); ASC(r.nm, 1); ASC(r.nm, 6); LEN(r.nm)
r.nm = "bob": r.z = 7: r.id = 1
PRINT "member: ["; r.nm; "]"; ASC(r.nm, 6); r.z; r.id
r.nm = "abcdefgh"
PRINT "member cut: ["; r.nm; "]"; r.z
' Arrays.
DIM a(2) AS STRING * 3
a(1) = "x"
PRINT "array:"; ASC(a(0), 1); " ["; a(1); "]"; LEN(a(1))
a(2) = "abcdef"
PRINT "array cut: ["; a(2); "] ["; a(1); "]"
a(3) = "bad"
PRINT "after a bad index: ["; a(2); "]"
DIM ra(1) AS rec
ra(1).nm = "q": ra(1).z = 5
PRINT "member of element: ["; ra(1).nm; "]"; ASC(ra(0).nm, 1); ra(1).z
' Passed to a STRING parameter: by reference, cut and padded on the way back.
DIM f5 AS STRING * 5
f5 = "ab": setlong f5
PRINT "var: ["; f5; "]"
f5 = "abcde": setshort f5
PRINT "var, shorter: ["; f5; "]"
f5 = "ab": showlen f5
f5 = "ab": setlong (f5)
PRINT "(var): ["; f5; "]"
TYPE rec5
    id AS LONG
    nm AS STRING * 5
END TYPE
DIM r5 AS rec5
r5.nm = "ab": setlong r5.nm
PRINT "member: ["; r5.nm; "]"
r5.nm = "ab": setlong (r5.nm)
PRINT "(member): ["; r5.nm; "]"
DIM a5(2) AS STRING * 5
a5(1) = "ab": setlong a5(1)
PRINT "element: ["; a5(1); "]"
a5(1) = "ab": setlong (a5(1))
PRINT "(element): ["; a5(1); "]"
' Storage classes.
DIM SHARED sh AS STRING * 3
sh = "shared"
loc
loc
st
st
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT

SUB setlong (t AS STRING)
t = "longer text"
END SUB

SUB setshort (t AS STRING)
t = "x"
END SUB

SUB showlen (t AS STRING)
PRINT "STRING param sees: ["; t; "]"; LEN(t)
END SUB

SUB loc
DIM l AS STRING * 3
PRINT "local:"; ASC(l, 1); " shared: ["; sh; "]"
l = "zz"
END SUB

SUB st
STATIC t AS STRING * 3
PRINT "static:"; ASC(t, 1)
t = "s"
END SUB
