$CONSOLE:ONLY
' Slice program (m2-numeric-types, tasks 4.3 and 8.1-8.5): the new numeric types and fixed-length strings
' everywhere else: static arrays and TYPE members of them, parameters and FUNCTION results (an unsigned FUNCTION, a
' FUNCTION f$n, a STRING * n parameter), passing by reference across signedness and between _INTEGER64 and _OFFSET
' (variables, elements, members; a copy in parentheses), a _BIT variable passed as a copy, FOR with each new type as
' the variable (the hidden type by width), SELECT CASE with each new type as the selector (read at each test or
' copied), CONST with the new suffixes and constants used with another suffix, and the special-cased built-ins with
' arguments of the new types. The scenarios of the spec deltas arrays-and-types, procedures, control-flow, constants
' and builtin-functions that the old compiler can run; the measurements are verification\v21_b_passing,
' v21_c_fixed_args, v21_d_*, v21_f_fixed_param and v21_g_*. No PRINT comma.
ON ERROR GOTO h

' Arrays of the new types (scenario "Unsigned elements").
DIM u(2) AS _UNSIGNED INTEGER: u(1) = -1: PRINT u(1); u(2)
DIM sb(-1 TO 1) AS _BYTE, ub(3) AS _UNSIGNED _BYTE, ul(1) AS _UNSIGNED LONG
DIM uq(1) AS _UNSIGNED _INTEGER64, so(1) AS _OFFSET, uo(1) AS _UNSIGNED _OFFSET
sb(-1) = 127: sb(-1) = sb(-1) + 1: ub(3) = ub(3) - 1: ul(0) = -2: uq(1) = -3: so(0) = -4: uo(1) = -5
PRINT "elements:"; sb(-1); sb(0); ub(3); ul(0); uq(1); so(0); uo(1)
PRINT "element arithmetic:"; ub(3) + ul(0); uq(1) * 2; so(0) / 3
DIM b2%%(2), ul2~&(1 TO 2)
b2%%(1) = 255: ul2~&(2) = -1
PRINT "suffixed arrays:"; b2%%(1); ul2~&(2); LBOUND(ul2~&); UBOUND(b2%%)
PRINT "bad index:"; ub(4)
DIM fa(2) AS STRING * 4
fa(1) = "abcdefg": fa(2) = "x"
PRINT "fixed elements: ["; fa(1); "]["; fa(2); "]"; LEN(fa(1)); ASC(fa(0), 1)

' TYPE members of the new types (scenario "Members of the new types").
TYPE nt
    b AS _BYTE
    ub AS _UNSIGNED _BYTE
    ui AS _UNSIGNED INTEGER
    ul AS _UNSIGNED LONG
    uq AS _UNSIGNED _INTEGER64
    o AS _OFFSET
END TYPE
TYPE wrap
    f AS STRING * 3
    n AS nt
    uo AS _UNSIGNED _OFFSET
END TYPE
DIM v AS nt
v.b = -1: v.ub = -1: v.ui = -1: v.ul = -1: v.uq = -1: v.o = -1
PRINT "members:"; v.b; v.ub; v.ui; v.ul; v.uq; v.o; LEN(v)
v.b = 200: v.ub = 300: v.ui = 70000: v.ul = 2.5: v.uq = 1E+19: v.o = 3.5
PRINT "member stores:"; v.b; v.ub; v.ui; v.ul; v.uq; v.o
DIM w(1) AS wrap
w(1).f = "hello": w(1).n.ul = 4294967295: w(1).n.b = -128: w(1).uo = -1
PRINT "members of elements: "; w(1).f; w(1).n.ul; w(1).n.b; w(1).uo; LEN(w(1)); w(0).n.uq

' Parameters and FUNCTION results.
PRINT "big~&:"; big~&(255)
PRINT "fs$5: ["; fs$5("ab"); "]"; LEN(fs$5("ab"))
PRINT "fs$ and fs: ["; fs$("abcdefgh"); "]["; fs("q"); "]"
PRINT "fz$3:"; LEN(fz$3); ASC(fz$3, 1)
s$ = "hello world"
sp s$
PRINT "after sp: "; s$
DIM fx AS STRING * 8
fx = "fixed"
sp fx
PRINT "after sp of a STRING * 8: ["; fx; "]"

' By reference across signedness (scenarios "Same width, other signedness", "_INTEGER64 to an _OFFSET parameter").
n& = -1: showul n&: PRINT " after:"; n&
q&& = -1: showuo q&&: PRINT " after:"; q&&
DIM sbv AS _BYTE: sbv = -1: showub sbv: PRINT " after:"; sbv
DIM uiv AS _UNSIGNED INTEGER: uiv = 65535: showsi uiv: PRINT " after:"; uiv
DIM uov AS _UNSIGNED _OFFSET: uov = 5: showsq uov: PRINT " after:"; uov
u(1) = 65535: showsi u(1): PRINT " after:"; u(1)
u(1) = 65535: showsi (u(1)): PRINT " after:"; u(1)
v.ul = 9: showsl v.ul: PRINT " after:"; v.ul
w(1).n.ub = 200: showsb w(1).n.ub: PRINT " after:"; w(1).n.ub
uq(0) = 3: showso uq(0): PRINT " after:"; uq(0)
' A _BIT variable is a copy (scenario "Bit variable passes a copy").
DIM b5 AS _BIT * 5
b5 = -16: showsl b5: PRINT " after:"; b5

' FOR with each new type (scenarios "Unsigned byte passes its range", "Unsigned variable counts down through 0").
FOR b~%% = 250 TO 255 STEP 3: PRINT b~%%;: NEXT: PRINT b~%%
FOR ui~% = 2 TO 0 STEP -1: PRINT ui~%;: NEXT: PRINT ui~%
FOR y%% = 125 TO 127: PRINT y%%;: NEXT: PRINT y%%
FOR l~& = 4294967294 TO 4294967295: PRINT l~&;: NEXT: PRINT l~&
FOR o%& = -1 TO 1: PRINT o%&;: NEXT: PRINT o%&
FOR uo~%& = 1 TO 2: PRINT uo~%&;: NEXT: PRINT uo~%&
c = 0
FOR qq~&& = 1 TO 18446744073709551615~&&: c = c + 1: NEXT
PRINT "~&& to 2^64-1 passes:"; c; qq~&&
FOR k~%% = 1 TO 2.6: PRINT k~%%;: NEXT: PRINT k~%%

' SELECT CASE with each new type (scenario "Negative item against unsigned selectors").
DIM sui AS _UNSIGNED INTEGER, sul AS _UNSIGNED LONG, suq AS _UNSIGNED _INTEGER64, ssb AS _BYTE
sui = 65535: sul = 4294967295: suq = 18446744073709551615~&&: ssb = -1
PRINT "~% 65535:";
SELECT CASE sui
    CASE -1: PRINT " -1"
    CASE ELSE: PRINT " else"
END SELECT
PRINT "~& max:";
SELECT CASE sul
    CASE -1: PRINT " -1"
    CASE ELSE: PRINT " else"
END SELECT
PRINT "~&& max, 0 TO -1:";
SELECT CASE suq
    CASE 0 TO -1: PRINT " 0 TO -1"
    CASE ELSE: PRINT " else"
END SELECT
PRINT "%% -1:";
SELECT CASE ssb
    CASE 255: PRINT " 255"
    CASE -1: PRINT " -1"
END SELECT
PRINT "element ~% 65535:";
SELECT CASE u(1)
    CASE IS > 65534: PRINT " > 65534"
    CASE ELSE: PRINT " else"
END SELECT
PRINT "member ~%% + 0, 2.5:";
v.ub = 2
SELECT CASE v.ub + 0
    CASE 2.5: PRINT " 2.5"
    CASE ELSE: PRINT " else"
END SELECT
PRINT "~& + 1~&&:";
SELECT CASE sul + 1~&&
    CASE 4294967296: PRINT " 4294967296"
    CASE ELSE: PRINT " else"
END SELECT

' CONST with the new suffixes (scenarios "Unsigned suffix", "Suffix with a value out of range", "Bit suffix").
CONST cu~% = 65535
PRINT "CONST ~%:"; cu~%; cu~% + 1
CONST du~% = -1, dc~%% = 300
PRINT "out of range:"; du~%; du~% + 0; du~% < 0; dc~%%; dc~%% + 0
CONST i`3 = 5, j~`3 = -1
PRINT "bit:"; i`3; j~`3; i`3 + 0
' Constants used with another suffix (v21_g_const_suffix).
CONST bigc = 4294967295, negc = -1, flc = 2.5, smallc = 300
PRINT "plain as new:"; bigc~&; negc~&; negc~%%; smallc%%; flc~%; negc~&&; negc%&; negc~%&
PRINT "+ 0:"; negc~& + 0; negc~%% + 0; smallc%% + 0; negc~&& + 0; negc~& < 0
CONST nb~%% = -1, nu~& = 4294967295
PRINT "new as other:"; nb%; nb&; nb!; nu&; nu%; nu% + 0; nb~&; nu~%%; nu~%% + 0

' Built-ins (scenarios "Unsigned argument", "VAL with an unsigned type").
ulv~& = 4294967295: PRINT HEX$(ulv~&); STR$(ulv~&); ABS(ulv~&)
PRINT VAL("-1", _UNSIGNED LONG); VAL("300", _UNSIGNED _BYTE); VAL("-1", _BYTE); VAL("-1", _OFFSET)
DIM b3 AS _BIT * 3: b3 = -4
PRINT "HEX$: "; HEX$(sbv); " "; HEX$(uiv); " "; HEX$(uq(1)); " "; HEX$(uq(1) + 0); " "; HEX$(b3); " "; HEX$(-1`5)
PRINT "OCT$ _BIN$: "; OCT$(sbv); " "; _BIN$(b3); " "; OCT$(v.ub)
PRINT "ABS SGN:"; ABS(sbv); ABS(uiv); ABS(b3); SGN(uiv); SGN(so(0)); SGN(b3)
PRINT "INT FIX:"; INT(uiv); FIX(uq(1)); INT(b3); INT(uiv) + 0
PRINT "CINT:"; CINT(sbv); CINT(b3); CINT(uiv)
PRINT "CLNG:"; CLNG(uiv); CLNG(sul)
PRINT "CLNG ~&&:"; CLNG(uq(1))
PRINT "_ROUND:"; _ROUND(ub(3)); _ROUND(uo(1)); _ROUND(b3)
PRINT "CSNG CDBL:"; CSNG(sul); CDBL(uq(1))
v.ub = 2: uiv = 2: sul = 2
PRINT "SQR:"; SQR(v.ub); SQR(uiv); SQR(sul); SQR(b3 + 6)
PRINT "EXP:"; EXP(v.ub); EXP(sul); EXP(b3)
PRINT "slots: ["; LEFT$("abcdef", v.ub); "] ["; CHR$(65 + v.ub); "]"
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT

FUNCTION big~& (x AS _UNSIGNED _BYTE)
    big~& = x * 16843009
END FUNCTION

FUNCTION fs$5 (x AS STRING)
    fs$5 = x
END FUNCTION

FUNCTION fz$3
END FUNCTION

SUB sp (t AS STRING * 5)
    PRINT "["; t; "]"; LEN(t); LEN(t + "!")
    t = "much longer text"
END SUB

SUB showsb (x AS _BYTE)
    PRINT "_BYTE param:"; x;: x = -2
END SUB
SUB showub (x AS _UNSIGNED _BYTE)
    PRINT "_UNSIGNED _BYTE param:"; x;: x = 254
END SUB
SUB showsi (x AS INTEGER)
    PRINT "INTEGER param:"; x;: x = -3
END SUB
SUB showsl (x AS LONG)
    PRINT "LONG param:"; x;: x = 5
END SUB
SUB showul (x AS _UNSIGNED LONG)
    PRINT "_UNSIGNED LONG param:"; x;: x = 5
END SUB
SUB showsq (x AS _INTEGER64)
    PRINT "_INTEGER64 param:"; x;: x = -6
END SUB
SUB showso (x AS _OFFSET)
    PRINT "_OFFSET param:"; x;: x = -8
END SUB
SUB showuo (x AS _UNSIGNED _OFFSET)
    PRINT "_UNSIGNED _OFFSET param:"; x;: x = 7
END SUB
