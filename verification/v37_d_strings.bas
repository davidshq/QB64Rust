$CONSOLE:ONLY
' DECLARE LIBRARY with a header: STRING parameters (char*) and STRING results (char* made a string).
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_len& (s AS STRING)
    SUB h_upper (s AS STRING)
    FUNCTION h_hello$
    FUNCTION h_empty$ ()
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
PRINT "result: "; h_hello$; "|"; h_empty$; "|"; LEN(h_hello$); UCASE$(h_hello$)
r$ = h_hello$ + "!": PRINT r$
SYSTEM
