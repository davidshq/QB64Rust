$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.3): fixed-length strings: initial bytes, assignment (padding,
' cutting), comparison, LEN, built-ins on them, the suffix form, members, arrays, storage classes. NUL bytes
' are never printed directly (ASC shows them), so the output holds no control bytes.
ON ERROR GOTO h
DIM f AS STRING * 4
PRINT "initial:"; LEN(f); ASC(f, 1); ASC(f, 4)
f = "ab"
PRINT "short: ["; f; "]"; ASC(f, 3); LEN(f)
f = "abcdef"
PRINT "long: ["; f; "]"
f = "ab"
PRINT "compare:"; f = "ab"; f = "ab  "; f < "ac"; f > "ab"; "ab  " = f
DIM s AS STRING
s = f
PRINT "into STRING: ["; s; "]"; LEN(s)
PRINT "concat: ["; f + "!"; "]"
f = f + "x"
PRINT "f = f + x: ["; f; "]"
f = "abcd"
f = MID$(f, 2)
PRINT "f = MID$(f, 2): ["; f; "]"
f = ""
PRINT "empty: ["; f; "]"; ASC(f, 1)
f = "ab"
MID$(f, 2, 1) = "x"
PRINT "MID$ statement: ["; f; "]"
MID$(f, 4) = "yz"
PRINT "MID$ statement past end: ["; f; "]"
DIM g AS STRING * 6
g = "xy"
f = g
PRINT "fixed from longer fixed: ["; f; "]"
g = f
PRINT "fixed from shorter fixed: ["; g; "]"
DIM k AS STRING * 5
k = "ab"
PRINT "built-ins:"; LEN(RTRIM$(k)); " ["; UCASE$(k); "] ["; LEFT$(k, 3); "] ["; RIGHT$(k, 2); "] ["; MID$(k, 2, 3); "]"; INSTR(k, " "); INSTR(k, "b"); ASC(k); ASC(k, 5)
PRINT "STR$/VAL: ["; LTRIM$(k); "]"; VAL(k); LEN(STR$(LEN(k)))
' Suffix form
DIM p$3
p$3 = "abcdef"
PRINT "DIM p$3: ["; p$3; "]"; LEN(p$3)
q$2 = "xyz"
PRINT "implicit q$2: ["; q$2; "]"; LEN(q$2)
p$ = "plain"
PRINT "p$ and p$3:["; p$; "] ["; p$3; "]"
CONST n = 3
DIM c AS STRING * n
c = "abcdef"
PRINT "STRING * CONST: ["; c; "]"
DIM u AS _UNSIGNED STRING * 3
u = "abcdef"
PRINT "_UNSIGNED STRING * 3: ["; u; "]"; LEN(u)
' Length limits
DIM big AS STRING * 100000
PRINT "100000:"; LEN(big); ASC(big, 100000)
big = "a"
PRINT "100000 after store:"; ASC(big, 1); ASC(big, 2); ASC(big, 100000)
' Members
TYPE rec
    id AS LONG
    nm AS STRING * 6
    z AS INTEGER
END TYPE
DIM r AS rec
PRINT "member initial:"; LEN(r); ASC(r.nm, 1); ASC(r.nm, 6); LEN(r.nm)
r.nm = "bob"
r.z = 7
PRINT "member: ["; r.nm; "]"; ASC(r.nm, 6); r.z
r.nm = "abcdefgh"
PRINT "member cut: ["; r.nm; "]"; r.z
' Arrays
DIM a(2) AS STRING * 3
a(1) = "x"
PRINT "array:"; ASC(a(0), 1); " ["; a(1); "]"; LEN(a(1))
a(2) = "abcdef"
PRINT "array cut: ["; a(2); "] ["; a(1); "]"
DIM ra(1) AS rec
ra(1).nm = "q"
PRINT "member of element: ["; ra(1).nm; "]"; ASC(ra(0).nm, 1)
' SELECT CASE
f = "ab"
SELECT CASE f
    CASE "ab": PRINT "case: ab"
    CASE "ab  ": PRINT "case: ab + 2 blanks"
    CASE ELSE: PRINT "case: else"
END SELECT
' Storage classes
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
