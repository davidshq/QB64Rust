$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.2): _BIT variables in each storage class (STATIC, local, DIM
' SHARED) and a _BIT array (bit-packed, study\02 section 1.7).
ON ERROR GOTO h
DIM SHARED sh AS _BIT * 4
sh = 9
PRINT "shared 9:"; sh
st
st
lo
lo
showsh
DIM a(3) AS _BIT * 3
a(1) = 5
a(2) = -1
PRINT "array:"; a(0); a(1); a(2); a(3)
DIM ua(9) AS _UNSIGNED _BIT
ua(3) = 1: ua(4) = 2: ua(5) = -1
PRINT "unsigned bit array:"; ua(2); ua(3); ua(4); ua(5); ua(6)
DIM wa(2) AS _BIT * 63
wa(1) = -1
PRINT "_BIT * 63 array:"; wa(0); wa(1); wa(2)
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT

SUB st
STATIC b AS _BIT * 4
b = b + 5
PRINT "static:"; b
END SUB

SUB lo
DIM b AS _BIT * 4
PRINT "local:"; b;
b = 7
PRINT b
END SUB

SUB showsh
PRINT "shared in SUB:"; sh
END SUB
