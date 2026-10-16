' TEST: ir
$CONSOLE:ONLY
' DECLARE LIBRARY in the IR (m3-declare-library task 3.1): the blocks, each library procedure with its C name and
' BYVAL flags and no statements, and the arguments of its calls: a place (Ref), a copy (Temp), a BYVAL value
' (Value) and NULL (Null).
TYPE pt
    x AS LONG
    y AS LONG
END TYPE
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    FUNCTION h_d# (BYVAL v AS DOUBLE)
    FUNCTION h_isnull& (BYVAL p AS _OFFSET)
    FUNCTION h_ptsum& (p AS pt)
    FUNCTION h_hello$
    SUB h_pl (v AS LONG)
    SUB h_upper (s AS STRING)
END DECLARE
DECLARE DYNAMIC LIBRARY "kernel32"
    FUNCTION lstrlenA& (s AS STRING)
END DECLARE

DIM x AS LONG, p AS pt, s AS STRING, y AS DOUBLE
PRINT h_add&(x, 2.6); h_d#(y); h_isnull&(0); h_ptsum&(p); h_hello$; lstrlenA&(s)
h_pl x: h_pl (x): h_upper s: h_upper "lit"
bump

SUB bump
    x = h_add&(1, 2)
END SUB
