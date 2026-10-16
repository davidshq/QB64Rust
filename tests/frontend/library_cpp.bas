' TEST: cpp
$CONSOLE:ONLY
' The C++ of DECLARE LIBRARY (m3-declare-library tasks 3.2, 3.3; design D3; compared with the old compiler's C++
' of verification\v37_b to v37_h). The test takes "v37_lib/v37h" as a header, "./v37c" as a C library and
' "./v37cpp" as a C++ library without one, and a DYNAMIC library as "<name>.dll" (frontend.rs, fixed_libraries).
' A header named by two blocks is included once (D-023); a block with a header writes no prototype, one without
' writes one per declaration, extern "C" for a C name; a DYNAMIC block a handle and a function pointer per
' declaration, loaded at the start with errors 259 and 260 (a second block of the same DLL shares its handle,
' another DLL whose handle name is taken gets "_2"); a CUSTOMTYPE block a pointer per declaration, set at the
' start, and without a header an extern "C" declaration of a C name, one with nm's parameters of a C++ name.
' Calls: a place by its pointer, a copy as &(passN=...), a STRING as its characters, a TYPE as its bytes, a BYVAL
' value with no cast (an integer to a DOUBLE stays an integer; a DOUBLE to a LONG is qbr), NULL, a FUNCTION's
' result cast to its type, a STRING result copied into a new string.
TYPE pt
    x AS LONG
    y AS LONG
END TYPE
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_l& (BYVAL v AS LONG)
    FUNCTION h_b%% (BYVAL v AS _BYTE)
    FUNCTION h_d# (BYVAL v AS DOUBLE)
    FUNCTION h_f## (BYVAL v AS _FLOAT)
    FUNCTION h_uo~%& (BYVAL v AS _UNSIGNED _OFFSET)
    FUNCTION h_isnull& (BYVAL p AS _OFFSET)
    FUNCTION h_ptsum& (p AS pt)
    FUNCTION maxl& ALIAS "h_max<int32_t>" (BYVAL a AS LONG, BYVAL b AS LONG)
    FUNCTION twice& ALIAS "hns::twice" (BYVAL v AS LONG)
    FUNCTION h_len& (s AS STRING)
    FUNCTION h_hello$
    SUB h_pl (v AS LONG)
    SUB h_upper (s AS STRING)
    SUB h_tick
END DECLARE
DECLARE LIBRARY "./v37c"
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB c_inc (v AS LONG)
    FUNCTION c_len& (s AS STRING)
    FUNCTION c_half# (BYVAL v AS DOUBLE)
    FUNCTION c_hello$
    FUNCTION c_big&& (BYVAL v AS _INTEGER64, BYVAL u AS _UNSIGNED _OFFSET, p AS pt)
END DECLARE
DECLARE LIBRARY "./v37cpp"
    FUNCTION cpp_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB cpp_tick
END DECLARE
DECLARE DYNAMIC LIBRARY "./v37dll"
    FUNCTION d_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB d_inc (v AS LONG)
    FUNCTION d_hello$
    FUNCTION ticks& ALIAS d_ticks
END DECLARE
DECLARE DYNAMIC LIBRARY "kernel32"
    FUNCTION lstrlenA& (s AS STRING)
END DECLARE
DECLARE CUSTOMTYPE LIBRARY "v37_lib/v37h"
    FUNCTION addll&& ALIAS h_add (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB ct_pl ALIAS h_pl (v AS LONG)
END DECLARE
DECLARE CUSTOMTYPE LIBRARY "./v37c"
    FUNCTION ct_add& ALIAS c_twice (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
DECLARE CUSTOMTYPE LIBRARY "./v37cpp"
    FUNCTION ct_cpp& ALIAS cpp_ticks
END DECLARE
DECLARE DYNAMIC LIBRARY "kernel32"
    FUNCTION GetTickCount&
END DECLARE
DECLARE DYNAMIC LIBRARY "./kernel32"
    FUNCTION k_own&
END DECLARE
' No name: the C runtime, no prototype (v37_a).
DECLARE LIBRARY
    FUNCTION isdigit& (BYVAL c AS LONG)
END DECLARE

DIM x AS LONG, a(3) AS LONG, p AS pt, q(2) AS pt, k AS INTEGER, s AS STRING, fx AS STRING * 6, y AS DOUBLE
x = h_l&(2.6) + h_l&(y) + h_b%%(300) + h_d#(1) + h_d#(x) + h_f##(1.5) + h_uo~%&(41)
x = h_isnull&(0) + h_isnull&(1 - 1) + h_isnull&(_OFFSET(x)) + h_ptsum&(p) + h_ptsum&(q(1))
x = maxl&(3, 9) + twice&(21) + h_len&(s) + h_len&("abc") + h_len&(s + "xy") + h_len&(fx)
PRINT h_hello$; LEN(h_hello$)
h_pl x: h_pl (x): h_pl a(2): h_pl p.y: h_pl k: h_pl 5
h_upper s: h_upper fx: h_tick
x = c_add&(2, 3) + c_len&("abcd") + c_half#(5) + c_big&&(4000000000, 7, p) + cpp_add&(20, 22)
c_inc x: cpp_tick: PRINT c_hello$
x = d_add&(2, 3) + ticks& + lstrlenA&("seven!!"): d_inc x: PRINT d_hello$
x = addll&&(3, 4) + ct_add&(5, 6) + isdigit&(48): ct_pl x
x = ct_cpp& + GetTickCount& + k_own&
in_sub

SUB in_sub
    DECLARE LIBRARY "v37_lib/v37h"
        FUNCTION h_l2& ALIAS h_l (BYVAL v AS LONG)
    END DECLARE
    PRINT h_l2&(5)
END SUB
