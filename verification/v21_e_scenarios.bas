$CONSOLE:ONLY
' Verification (m2-numeric-types, task 1.5): the scenarios of the change's spec deltas that the old compiler can
' run, each line labelled with its requirement, so the expected outputs in the deltas are checked against it.
ON ERROR GOTO h
DIM b AS _UNSIGNED _BYTE: b = 255: PRINT "[numeric: unsigned byte]"; b; LEN(b)
x~% = 65535: PRINT "[numeric: suffix names the type]"; x~%
b~%% = 255: x~% = 65535: PRINT "[numeric: narrow operands widened]"; b~%% + 1; x~% * 2
DIM o AS _UNSIGNED _OFFSET: o = 7: PRINT "[numeric: offset division]"; o / 2
o = 5: PRINT "[numeric: offset division 5 / 2]"; o / 2
DIM so AS _OFFSET: so = 7: PRINT "[numeric: offset * 1.5, + 1.5, \ 2, MOD 4]"; so * 1.5; so + 1.5; so \ 2; so MOD 4
PRINT "[numeric: offset - 8, -offset, NOT offset]"; so - 8; -so; NOT so
DIM b3 AS _BIT * 3: b3 = 5: PRINT "[numeric: signed bit field]"; b3
DIM u3 AS _UNSIGNED _BIT * 3: u3 = 13: PRINT "[numeric: unsigned bit field]"; u3
DIM t AS _BIT: t = 1: PRINT "[numeric: single bit]"; t
DIM wa AS _BIT * 33, wb AS _BIT * 33: wa = 5: wb = -1: PRINT "[numeric: wide bit fields]"; wa
PRINT "[numeric: unsigned literal]"; 4294967295~&; 255~%%
d# = 2.5000001: x% = d#: l& = d#: PRINT "[numeric: INTEGER rounds the SINGLE value]"; x%; l&
x~% = -1: u~&& = -1: PRINT "[numeric: negative into unsigned]"; x~%; u~&&
b%% = 200: PRINT "[numeric: signed byte wraps]"; b%%
DIM ua(2) AS _UNSIGNED INTEGER: ua(1) = -1: PRINT "[arrays: unsigned elements]"; ua(1); ua(2)
TYPE nt
    sb AS _BYTE
    ub AS _UNSIGNED _BYTE
    ui AS _UNSIGNED INTEGER
    ul AS _UNSIGNED LONG
    uq AS _UNSIGNED _INTEGER64
    so AS _OFFSET
END TYPE
DIM v AS nt
v.sb = -1: v.ub = -1: v.ui = -1: v.ul = -1: v.uq = -1: v.so = -1
PRINT "[arrays: members of the new types]"; v.sb; v.ub; v.ui; v.ul; v.uq; v.so; LEN(v)
PRINT "[procedures: unsigned function and parameter]"; big~&(255)
n& = -1: show n&: PRINT "[procedures: same width, other signedness, after]"; n&
PRINT "[control-flow: unsigned byte passes its range]";
FOR fb~%% = 250 TO 255 STEP 3: PRINT fb~%%;: NEXT: PRINT fb~%%
CONST cu~% = 65535: PRINT "[constants: unsigned suffix]"; cu~%; cu~% + 1
nm%% = 1: nm~%% = 2: PRINT "[numeric: one name, two suffixes]"; nm%%; nm~%%
DIM u64 AS _UNSIGNED _BIT * 64: u64 = -1: PRINT "[numeric: unsigned 64-bit bit field]"; u64
DIM pad AS _BIT * 32, u3b AS _UNSIGNED _BIT * 3
u3b = 7: PRINT "[numeric: unsigned bit field in arithmetic]"; u3b * 1000000000; u3b > -1
PRINT "[numeric: hexadecimal literal]"; &HFF%%; &HFF%% + 0
DIM b16 AS _BIT * 16: d# = 2.5000001: ux~% = d#: b16 = d#: PRINT "[numeric: unsigned and bit targets]"; ux~%; b16
PRINT "[control-flow: unsigned counts down through 0]";
FOR fu~% = 2 TO 0 STEP -1: PRINT fu~%;: NEXT: PRINT fu~%
DIM sui AS _UNSIGNED INTEGER, sul AS _UNSIGNED LONG: sui = 65535: sul = 4294967295
PRINT "[control-flow: negative item against unsigned selectors]";
SELECT CASE sui
    CASE -1: PRINT " -1";
    CASE ELSE: PRINT " else";
END SELECT
SELECT CASE sul
    CASE -1: PRINT " -1"
    CASE ELSE: PRINT " else"
END SELECT
q&& = -1: showuo q&&: PRINT "[procedures: _INTEGER64 to _OFFSET, after]"; q&&
DIM bp AS _BIT * 5: bp = -16: setl bp: PRINT "[procedures: bit variable passes a copy]"; bp
DIM fs AS STRING * 4294967297: PRINT "[fixed: length read as 32 bits]"; LEN(fs)
DIM p$3: p$3 = "abcdef": p$ = "x": PRINT "[fixed: suffix form]"; p$3; LEN(p$3); p$
DIM fp AS STRING * 5: fp = "ab": setlong fp: PRINT "[fixed: passed to STRING, after]"; fp
fp = "ab": setlong (fp): PRINT "[fixed: in parentheses]"; fp
DIM rr AS rec5: rr.nm = "ab": setlong rr.nm: PRINT "[fixed: member]"; rr.nm
DIM ea(2) AS STRING * 5: ea(1) = "ab": setlong ea(1): PRINT "[fixed: element]"; ea(1)
DIM f AS STRING * 4: PRINT "[fixed: initial bytes]"; LEN(f); ASC(f, 1); ASC(f, 4)
f = "ab": PRINT "[fixed: short value padded]"; "["; f; "]"; ASC(f, 3)
f = "abcdef": PRINT "[fixed: long value cut]"; f
f = "ab": PRINT "[fixed: compared with its padding]"; f = "ab"; f = "ab  "
TYPE rec
    id AS LONG
    nm AS STRING * 6
END TYPE
TYPE rec5
    id AS LONG
    nm AS STRING * 5
END TYPE
DIM r AS rec: r.nm = "bob": PRINT "[fixed: member in the layout]"; LEN(r); "["; r.nm; "]"
DIM fa(2) AS STRING * 3: fa(1) = "x": PRINT "[fixed: array]"; ASC(fa(0), 1); "["; fa(1); "]"
DIM g AS STRING * 5: g = "ab": PRINT "[fixed: built-in argument]"; LEN(RTRIM$(g)); UCASE$(g)
u~& = 4294967295: PRINT "[builtin: unsigned argument]"; HEX$(u~&); STR$(u~&); ABS(u~&)
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT

FUNCTION big~& (x AS _UNSIGNED _BYTE)
big~& = x * 16843009
END FUNCTION

SUB showuo (x AS _UNSIGNED _OFFSET)
PRINT "[procedures: _INTEGER64 to _OFFSET]"; x
x = 7
END SUB

SUB setl (x AS LONG)
x = 5
END SUB

SUB setlong (t AS STRING)
PRINT "[fixed: STRING parameter sees]"; LEN(t)
t = "longer text"
END SUB

SUB show (x AS _UNSIGNED LONG)
PRINT "[procedures: same width, other signedness]"; x
x = 5
END SUB
