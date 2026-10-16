' TEST: typed
$CONSOLE:ONLY
' DECLARE LIBRARY in sema (m3-declare-library task 2.1, 2.3; measured, verification\v37_*): each form of block and
' declaration entered with its C name and BYVAL flags; a call before the block, from a SUB, of a block in a SUB and
' in an included file; the arguments: a place by reference (ref), a copy (temp), a BYVAL value converted as for an
' assignment (value), and the BYVAL arguments the old compiler writes NULL: 0, (0), 0& and a CONST 0, not 1 - 1.
CONST zero = 0
TYPE pt
    x AS LONG
    y AS LONG
END TYPE
PRINT early&(1)
DEFLNG D, H
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION early& ALIAS h_l (BYVAL v AS LONG)
    FUNCTION h_d (BYVAL v AS DOUBLE)
    FUNCTION h_dp# ALIAS "h_d" (BYVAL dv)
    FUNCTION maxl& ALIAS "h_max<int32_t>" (BYVAL a AS LONG, BYVAL b AS LONG)
    FUNCTION twice& ALIAS "hns::twice" (BYVAL v AS LONG)
    FUNCTION h_b%% (BYVAL v AS _BYTE)
    FUNCTION h_isnull& (BYVAL p AS _OFFSET)
    FUNCTION h_ptsum& (p AS pt)
    FUNCTION Fast_Sqrt& (BYVAL val AS LONG)
    FUNCTION h.dot& ALIAS h_l (BYVAL v AS LONG)
    SUB h_pl (v AS LONG)
    SUB h_upper (s AS STRING)
    SUB h_tick
    FUNCTION h_ticks&
    FUNCTION h_nullstr$
END DECLARE
DECLARE STATIC LIBRARY "./v37s"
    FUNCTION s_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
DECLARE DYNAMIC LIBRARY "kernel32"
    FUNCTION lstrlenA& (s AS STRING)
END DECLARE
DECLARE CUSTOMTYPE LIBRARY "v37_lib/v37h"
    FUNCTION addll&& ALIAS h_add (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
DECLARE LIBRARY "", "./v37c"
END DECLARE
DECLARE LIBRARY
    FUNCTION isdigit& (BYVAL c AS LONG)
END DECLARE
'$INCLUDE:'inc/library.bi'

DIM x AS LONG, a(3) AS LONG, p AS pt, k AS INTEGER, s AS STRING, y AS DOUBLE
PRINT h_d(1.7); h_dp#(2); maxl&(3, 9); twice&(21); h_b%%(300); isdigit&(48); inc_add&(1, 2)
PRINT h_isnull&(0); h_isnull&((0)); h_isnull&(0&); h_isnull&(zero); h_isnull&(1 - 1); h_isnull&(_OFFSET(x))
PRINT early&(2.6); h.dot&(y); h_ptsum&(p); Fast_Sqrt&(2000); s_add&(1, 2); lstrlenA&("seven!!"); addll&&(3, 4)
h_pl x: h_pl (x): h_pl a(2): h_pl p.x: h_pl k: h_pl 5
CALL h_pl(x): h_pl (x)
h_upper s: h_upper "lit": h_upper s + "t"
h_tick: CALL h_tick
PRINT h_ticks&; h_nullstr$
in_sub

SUB in_sub
    DECLARE LIBRARY "v37_lib/v37h"
        FUNCTION h_l2& ALIAS h_l (BYVAL v AS LONG)
    END DECLARE
    PRINT h_l2&(5); early&(3)
END SUB
