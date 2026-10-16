$CONSOLE:ONLY
' DECLARE LIBRARY with a header (v37h.h in this program's .files folder): every numeric type by value
' (BYVAL) and by reference, the conversion of a BYVAL argument, and what a parameter without BYVAL
' receives for each kind of argument (verification\v37_b, v37_c, x56, x58, x59, x63).
DEFLNG H
CONST zero = 0
DECLARE LIBRARY "v37h"
    FUNCTION h_b%% (BYVAL v AS _BYTE)
    SUB h_pb (v AS _BYTE)
    FUNCTION h_ub~%% (BYVAL v AS _UNSIGNED _BYTE)
    SUB h_pub (v AS _UNSIGNED _BYTE)
    FUNCTION h_i% (BYVAL v AS INTEGER)
    SUB h_pi (v AS INTEGER)
    FUNCTION h_ui~% (BYVAL v AS _UNSIGNED INTEGER)
    SUB h_pui (v AS _UNSIGNED INTEGER)
    FUNCTION h_l& (BYVAL v AS LONG)
    SUB h_pl (v AS LONG)
    FUNCTION h_ul~& (BYVAL v AS _UNSIGNED LONG)
    SUB h_pul (v AS _UNSIGNED LONG)
    FUNCTION h_ll&& (BYVAL v AS _INTEGER64)
    SUB h_pll (v AS _INTEGER64)
    FUNCTION h_ull~&& (BYVAL v AS _UNSIGNED _INTEGER64)
    SUB h_pull (v AS _UNSIGNED _INTEGER64)
    FUNCTION h_s! (BYVAL v AS SINGLE)
    SUB h_ps (v AS SINGLE)
    FUNCTION h_d# (BYVAL v AS DOUBLE)
    SUB h_pd (v AS DOUBLE)
    FUNCTION h_f## (BYVAL v AS _FLOAT)
    SUB h_pf (v AS _FLOAT)
    FUNCTION h_o%& (BYVAL v AS _OFFSET)
    SUB h_po (v AS _OFFSET)
    FUNCTION h_uo~%& (BYVAL v AS _UNSIGNED _OFFSET)
    SUB h_puo (v AS _UNSIGNED _OFFSET)
END DECLARE
DECLARE LIBRARY "v37h"
    FUNCTION h_isnull& (v AS LONG)
    FUNCTION h_isnull_o& ALIAS h_isnull (BYVAL v AS _OFFSET)
    FUNCTION h_peekl& (BYVAL p AS _OFFSET)
    FUNCTION h_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB h_tick
    FUNCTION h_ticks&
    FUNCTION h_dl ALIAS h_d (BYVAL v AS DOUBLE)
END DECLARE
TYPE t
    a AS LONG
    b AS LONG
END TYPE

DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER
DIM l AS LONG, ul AS _UNSIGNED LONG, ll AS _INTEGER64, ull AS _UNSIGNED _INTEGER64
DIM s AS SINGLE, d AS DOUBLE, f AS _FLOAT, o AS _OFFSET, uo AS _UNSIGNED _OFFSET

PRINT "byval:"; h_b%%(126); h_ub~%%(254); h_i%(32766); h_ui~%(65534); h_l&(-5); h_ul~&(4294967294)
PRINT "byval:"; h_ll&&(-9); h_ull~&&(18446744073709551614~&&); h_s!(1.5); h_d#(2.25); h_f##(3.125)
PRINT "byval offset:"; h_o%&(41); h_uo~%&(41)
PRINT "wrap:"; h_b%%(127); h_ub~%%(255); h_i%(32767); h_ui~%(65535); h_l&(2147483647)

b = 1: ub = 2: i = 3: ui = 4: l = 5: ul = 6: ll = 7: ull = 8: s = 9.5: d = 10.5: f = 11.5: o = 12: uo = 13
h_pb b: h_pub ub: h_pi i: h_pui ui: h_pl l: h_pul ul: h_pll ll: h_pull ull
h_ps s: h_pd d: h_pf f: h_po o: h_puo uo
PRINT "byref:"; b; ub; i; ui; l; ul; ll; ull; s; d; f; o; uo

' A BYVAL argument of another type is converted as for any call.
PRINT "convert:"; h_l&(2.6); h_l&(3.5); h_s!(7); h_d#(1 / 3)
' Results used in expressions; a result without a suffix under DEFLNG is truncated by its cast.
PRINT "expr:"; h_l&(1) + h_l&(2) * 10; LEN(STR$(h_d#(0.5))); h_dl(1.5); h_dl(1.7)

' What a parameter without BYVAL receives.
DIM x AS LONG, a(3) AS LONG, v AS t, k AS INTEGER, sg AS SINGLE
x = 10
h_pl x: PRINT "variable:"; x
h_pl (x): PRINT "parenthesised:"; x
h_pl x + 0: PRINT "expression:"; x
a(2) = 20: h_pl a(2): PRINT "element:"; a(2)
v.b = 30: h_pl v.b: PRINT "member:"; v.b
h_pl 5: PRINT "literal: done"
k = 40: h_pl k: PRINT "other type:"; k
sg = 1.5: h_ps sg: PRINT "single:"; sg
CALL h_pl(x): PRINT "call:"; x

' A BYVAL argument written 0 is NULL.
PRINT "null literal 0:"; h_isnull&(0); " null 1:"; h_isnull&(1); " null var:"; h_isnull&(x)
PRINT "byval offset 0:"; h_isnull_o&(0); h_isnull_o&((0)); h_isnull_o&(0&); h_isnull_o&(zero)
x = 1234: PRINT "peek:"; h_peekl&(_OFFSET(x))
PRINT "add:"; h_add&(2, 3)
h_tick: h_tick: CALL h_tick
PRINT "ticks:"; h_ticks&
inner
SYSTEM

SUB inner
    ' A block in a SUB; its names are global.
    DECLARE LIBRARY "v37h"
        FUNCTION h_add2& ALIAS h_add (BYVAL a AS LONG, BYVAL b AS LONG)
    END DECLARE
    PRINT "sub:"; h_add2&(20, 22); h_ticks&
END SUB
