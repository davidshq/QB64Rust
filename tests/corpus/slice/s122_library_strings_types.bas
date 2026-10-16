$CONSOLE:ONLY
' DECLARE LIBRARY with a header (v37h.h in this program's .files folder): STRING parameters (char*),
' STRING results (char* made a string, NULL as ""), and a TYPE passed as void* (verification\v37_d,
' v37_e, x39).
TYPE pt
    x AS LONG
    y AS LONG
END TYPE
DECLARE LIBRARY "v37h"
    FUNCTION h_len& (s AS STRING)
    SUB h_upper (s AS STRING)
    FUNCTION h_hello$
    FUNCTION h_empty$ ()
    FUNCTION h_nullstr$
    FUNCTION h_ptsum& (p AS pt)
    SUB h_ptset (p AS pt)
END DECLARE
TYPE t
    f AS STRING * 4
    g AS STRING * 4
END TYPE

DIM a AS STRING, fx AS STRING * 6, v AS t
a = "hello"
PRINT "len:"; h_len&(a); h_len&("abc"); h_len&(""); h_len&(a + "xy")
PRINT "len left$:"; h_len&(LEFT$("abcdef", 3))
a = "ab" + CHR$(0) + "cd": PRINT "len nul:"; h_len&(a)
fx = "ab": PRINT "len fixed:"; h_len&(fx)
a = "mixed Case"
h_upper a: PRINT "upper var: "; a
fx = "abc": h_upper fx: PRINT "upper fixed: "; fx
b$ = "xyz": c$ = b$: h_upper b$: PRINT "upper copy: "; b$; " "; c$
PRINT "result: "; h_hello$; "|"; h_empty$; "|"; h_nullstr$; "|"; LEN(h_hello$); UCASE$(h_hello$)
r$ = h_hello$ + "!": PRINT r$

DIM p AS pt, q(2) AS pt
p.x = 3: p.y = 4
PRINT "udt:"; h_ptsum&(p)
h_ptset p: PRINT "udt set:"; p.x; p.y
q(1).x = 5: q(1).y = 6: PRINT "udt element:"; h_ptsum&(q(1))
SYSTEM
