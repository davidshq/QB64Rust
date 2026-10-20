' TEST: check-fail
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' The forms the old compiler rejects for a type with a _DYNAMIC member (m3-typefields design D4 last sentence,
' task 5.2): REDIM of the member without TO and without AS (verification\v35_x18), with another type (x19), LEN
' of a variable and of an element (x20, x64), _OFFSET of a variable (x68), in COMMON as a variable and as an array
' (x39, x63), the member alone as a value (x41), PUT and GET of one value, on a random and on a binary file (x43, x46, x50, x54), and PUT of a
' whole array of a type with a _DYNAMIC member of STRING (x48), and DIM of the member (x37, found by
' verification-compare). Each is an error of the program, none "not supported yet"; the messages are new.
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
TYPE s
    w(1) _DYNAMIC AS STRING
END TYPE
COMMON SHARED c AS t
COMMON SHARED ca() AS t
DIM x AS t
REDIM p(1) AS t
REDIM q(1) AS s
REDIM x.v(5)
REDIM x.v(0 TO 5) AS INTEGER
DIM x.v(3) AS LONG
PRINT LEN(x)
PRINT LEN(p(0))
PRINT _OFFSET(x)
PRINT x.v
OPEN "typefields_rejected.tmp" FOR RANDOM AS #1 LEN = 8
PUT #1, 1, x
CLOSE #1
OPEN "typefields_rejected.tmp" FOR BINARY AS #1
PUT #1, 1, x
PUT #1, 1, p(0)
GET #1, 1, p(0)
PUT #1, 1, q()
CLOSE #1
SYSTEM
