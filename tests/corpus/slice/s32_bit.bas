$CONSOLE:ONLY
' Slice program (m2-numeric-types, task 6.1): stores into _BIT and _BIT * n scalars (the mask for unsigned ones,
' the sign extension from bit n-1 for signed ones) at each width, from integers and floats; _BIT variables in
' each storage class; a _BIT variable passed to an integer parameter (always a copy); _BIT values in arithmetic,
' comparisons and STR$. Every _BIT * n wider than 32 bits is declared after a _BIT * 32 pad, so the old compiler's
' overlap (DIVERGENCES.md D-009) hits nothing printed; _BIT arrays are not supported yet. No PRINT comma.
DIM SHARED sh AS _BIT * 4
DIM b1 AS _BIT, u1 AS _UNSIGNED _BIT, b3 AS _BIT * 3, u3 AS _UNSIGNED _BIT * 3
DIM b7 AS _BIT * 7, u7 AS _UNSIGNED _BIT * 7, b16 AS _BIT * 16, u16 AS _UNSIGNED _BIT * 16
DIM b17 AS _BIT * 17, u20 AS _UNSIGNED _BIT * 20, b31 AS _BIT * 31, u31 AS _UNSIGNED _BIT * 31
DIM b32 AS _BIT * 32, u32 AS _UNSIGNED _BIT * 32
DIM pad1 AS _BIT * 32, b33 AS _BIT * 33
DIM pad2 AS _BIT * 32, u33 AS _UNSIGNED _BIT * 33
DIM pad3 AS _BIT * 32, b40 AS _BIT * 40
DIM pad4 AS _BIT * 32, u40 AS _UNSIGNED _BIT * 40
DIM pad5 AS _BIT * 32, b63 AS _BIT * 63
DIM pad6 AS _BIT * 32, b64 AS _BIT * 64
DIM pad7 AS _BIT * 32, u64 AS _UNSIGNED _BIT * 64
DIM q AS _INTEGER64, ul AS _UNSIGNED LONG, d AS DOUBLE, fl AS _FLOAT
' Integer stores: the low n bits, then the sign.
b1 = 1: PRINT "b1:"; b1;: b1 = 2: PRINT b1;: b1 = 3: PRINT b1;: b1 = -2: PRINT b1
u1 = 1: PRINT "u1:"; u1;: u1 = 2: PRINT u1;: u1 = -1: PRINT u1
b3 = 3: PRINT "b3:"; b3;: b3 = 4: PRINT b3;: b3 = 5: PRINT b3;: b3 = 8: PRINT b3;: b3 = -5: PRINT b3;: b3 = 13: PRINT b3
u3 = 7: PRINT "u3:"; u3;: u3 = 8: PRINT u3;: u3 = 13: PRINT u3;: u3 = -1: PRINT u3
b7 = 63: PRINT "b7:"; b7;: b7 = 64: PRINT b7;: b7 = 200: PRINT b7;: b7 = -129: PRINT b7
u7 = 127: PRINT "u7:"; u7;: u7 = 128: PRINT u7;: u7 = -1: PRINT u7
b16 = 32767: PRINT "b16:"; b16;: b16 = 32768: PRINT b16;: b16 = 65535: PRINT b16;: b16 = 70000: PRINT b16
u16 = 65535: PRINT "u16:"; u16;: u16 = 65536: PRINT u16;: u16 = -1: PRINT u16
b17 = 65536: PRINT "b17:"; b17;: b17 = 131071: PRINT b17;: b17 = -65537: PRINT b17
u20 = 1048575: PRINT "u20:"; u20;: u20 = 1048576: PRINT u20;: u20 = -2: PRINT u20
b31 = 1073741823: PRINT "b31:"; b31;: b31 = 1073741824: PRINT b31;: b31 = -1: PRINT b31
u31 = 2147483647: PRINT "u31:"; u31;: u31 = -1: PRINT u31
b32 = 2147483647: PRINT "b32:"; b32;: b32 = 4294967295: PRINT b32;: b32 = 4294967296: PRINT b32
u32 = 4294967295: PRINT "u32:"; u32;: u32 = -1: PRINT u32;: u32 = 4294967298: PRINT u32
b33 = 4294967295: PRINT "b33:"; b33;: b33 = 8589934591: PRINT b33;: b33 = -4294967297: PRINT b33
u33 = 8589934591: PRINT "u33:"; u33;: u33 = -1: PRINT u33
b40 = 549755813887: PRINT "b40:"; b40;: b40 = 549755813888: PRINT b40;: b40 = 1E+12: PRINT b40
u40 = -1: PRINT "u40:"; u40;: u40 = 1099511627776: PRINT u40
b63 = -1: PRINT "b63:"; b63;: b63 = 4611686018427387904: PRINT b63
b64 = -1: PRINT "b64:"; b64;: b64 = 9223372036854775807: PRINT b64
u64 = 18446744073709551615~&&: PRINT "u64:"; u64; u64 + 0; u64 > 0
' Stores from other integer types.
q = 4294967298: b7 = q: PRINT "from _INTEGER64:"; b7;: q = -129: b7 = q: PRINT b7
ul = 4294967295: b7 = ul: PRINT " from _UNSIGNED LONG:"; b7;: u3 = ul: PRINT u3
b3 = -3: u7 = b3: PRINT "from _BIT * 3:"; u7;: b16 = b3: PRINT b16
' Stores from floats: rounded half to even from _FLOAT, whatever the width, then masked.
b3 = 2.5: PRINT "floats:"; b3;: b3 = 3.5: PRINT b3;: b3 = -0.5: PRINT b3;: b3 = -1.5: PRINT b3
u3 = 6.5: PRINT u3;: u3 = 7.5: PRINT u3
d = 2.5000001: b16 = d: b17 = d: b31 = d: b32 = d: PRINT " 2.5000001#:"; b16; b17; b31; b32
d = 16777217.5#: b17 = d: b31 = d: b32 = d: PRINT " 16777217.5#:"; b17; b31; b32
d = 65537.5#: b17 = d: PRINT " 65537.5#:"; b17
u20 = -1.5: PRINT " u20:"; u20;: u20 = 1048575.5: PRINT u20
fl = 549755813887.5##: b40 = fl: PRINT " b40:"; b40;: u40 = -2.5: PRINT u40
u64 = 1.8E+19: PRINT " u64:"; u64
' Arithmetic: a _BIT * n of up to 32 bits counts as its 32-bit storage.
b3 = -3: u3 = 7
PRINT "arith:"; b3 + 1; b3 * b3; u3 + 1; u3 * 1000000000; b3 - u3; -b3; NOT u3; b3 = -3; u3 > b3
u32 = 4294967295: PRINT "u32:"; u32 + 1; u32 * 2; u32 > 0; u32 > -1
b1 = -1: PRINT "b1:"; b1 + b1; NOT b1; b1 AND 5
IF b1 THEN PRINT "b1 is true"
PRINT "STR$: ["; STR$(b3); "] ["; STR$(u3); "] ["; STR$(u64); "]"
b3 = b3 + 2: PRINT "b3 + 2:"; b3;: b3 = b3 + 2: PRINT b3
' Storage classes.
sh = 9: PRINT "shared:"; sh
st
st
lo
lo
showsh
' Passed to integer parameters: always a copy, whatever the parameter's type.
DIM b5 AS _BIT * 5, ub5 AS _UNSIGNED _BIT * 5
b5 = -16: showsl b5: PRINT " after:"; b5
b5 = 7: showsi b5: PRINT " after:"; b5
ub5 = 31: showul ub5: PRINT " after:"; ub5
ub5 = 31: showsl ub5: PRINT " after:"; ub5
b40 = -1: showsq b40: PRINT " after:"; b40
SYSTEM

SUB st
STATIC b AS _BIT * 4
b = b + 5
PRINT "static:"; b
END SUB

SUB lo
DIM b AS _BIT * 4, u AS _UNSIGNED _BIT * 4
PRINT "local:"; b; u;
b = 7: u = 17
PRINT b; u
END SUB

SUB showsh
PRINT "shared in SUB:"; sh
END SUB

SUB showsi (x AS INTEGER)
PRINT "INTEGER param:"; x;: x = 99
END SUB
SUB showsl (x AS LONG)
PRINT "LONG param:"; x;: x = 5
END SUB
SUB showul (x AS _UNSIGNED LONG)
PRINT "_UNSIGNED LONG param:"; x;: x = 5
END SUB
SUB showsq (x AS _INTEGER64)
PRINT "_INTEGER64 param:"; x;: x = 6
END SUB
