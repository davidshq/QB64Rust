$CONSOLE:ONLY
' Slice (m4-builtins-param-types, design D2): _CAST as a BYVAL argument of DECLARE LIBRARY keeps its cast, to a
' parameter of a wider type and to one of another kind (v37h.h in this program's .files folder, and the C runtime
' with no header); the overload a header's C++ name takes for a cast argument (h_over: 1 for int32_t, 2 for double).
DECLARE LIBRARY "v37h"
    FUNCTION h_l& (BYVAL v AS LONG)
    FUNCTION h_d# (BYVAL v AS DOUBLE)
    FUNCTION h_ll&& (BYVAL v AS _INTEGER64)
    FUNCTION over& ALIAS h_over (BYVAL v AS DOUBLE)
END DECLARE
DECLARE LIBRARY
    FUNCTION labs& (BYVAL v AS LONG)
END DECLARE
DIM k AS LONG, sg AS SINGLE, d AS DOUBLE, q AS _INTEGER64
k = 300: sg = -6.75: d = 7.5: q = 4294967297
PRINT h_l&(_CAST(_BYTE, k)); h_l&(_CAST(_UNSIGNED _BYTE, k)); h_l&(_CAST(INTEGER, k * 300))
PRINT h_d#(_CAST(LONG, sg)); h_d#(_CAST(INTEGER, d)); h_ll&&(_CAST(LONG, q))
PRINT labs(_CAST(_BYTE, k)); labs(_CAST(LONG, sg))
PRINT over&(_CAST(LONG, d)); over&(_CAST(INTEGER, sg)); over&(k); over&(d)
SYSTEM
