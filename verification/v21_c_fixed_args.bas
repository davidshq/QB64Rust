$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.3): fixed-length strings as arguments: to a STRING parameter
' (do the procedure's changes reach the argument, cut and padded?), from a variable, a member, an element and in
' parentheses; and parameters declared STRING * n or with the suffix form.
ON ERROR GOTO h
DIM f AS STRING * 5
f = "ab"
setlong f
PRINT "var to STRING, longer: ["; f; "]"
f = "abcde"
setshort f
PRINT "var to STRING, shorter: ["; f; "]"
f = "ab"
showlen f
f = "ab"
setlong (f)
PRINT "(var) to STRING: ["; f; "]"
TYPE rec
    id AS LONG
    nm AS STRING * 5
END TYPE
DIM r AS rec
r.nm = "ab"
setlong r.nm
PRINT "member to STRING: ["; r.nm; "]"
DIM a(2) AS STRING * 5
a(1) = "ab"
setlong a(1)
PRINT "element to STRING: ["; a(1); "]"
' Parameters declared STRING * n
f = "ab"
fixparam f
PRINT "var to STRING * 5: ["; f; "]"
DIM g AS STRING * 3
g = "xyz"
fixparam g
PRINT "STRING * 3 var to STRING * 5: ["; g; "]"
DIM s AS STRING
s = "hello world"
fixparam s
PRINT "STRING var to STRING * 5: ["; s; "]"
fixparam "lit"
fixsuffix f
PRINT "var to t$5: ["; f; "]"
PRINT "FUNCTION fs$5: ["; fs$5("ab"); "]"; LEN(fs$5("ab"))
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT

SUB setlong (t AS STRING)
t = "longer text"
END SUB

SUB setshort (t AS STRING)
t = "x"
END SUB

SUB showlen (t AS STRING)
PRINT "STRING param sees: ["; t; "]"; LEN(t)
END SUB

SUB fixparam (t AS STRING * 5)
PRINT "STRING * 5 param: ["; t; "]"; LEN(t)
t = "changed!"
END SUB

SUB fixsuffix (t$5)
PRINT "t$5 param: ["; t$5; "]"; LEN(t$5)
t$5 = "q"
END SUB

FUNCTION fs$5 (x AS STRING)
fs$5 = x
END FUNCTION
