' TEST: typed
$CONSOLE:ONLY
' Default types (m2-defxxx-common-arrays task 2.1; verification\v24_a_*): a variable before and after a statement
' (the plain name is the name plus the default at its line), a variable declared with AS, an array, DIM and DIM
' SHARED without a type, a FOR variable, a CONST (not typed by the default), ranges written backwards and the
' underscore entry, _DEFINE with an unsigned type, a fixed-length string and a _BIT, a statement inside a block,
' a procedure above and below a statement (parameter, local, result), a statement inside a SUB that goes on
' applying below it, a FUNCTION found by its plain name where the default has changed
SUB above (p)
    v = 2.5
END SUB
a = 1.5
DIM d AS LONG
DEFINT A-Z
a = 7
d = 100000
b = a! + a
DIM t(3), plain
DIM SHARED g
t(1) = plain + g
FOR i = 1 TO 2: NEXT
CONST ck = 2.5
c = ck
__u = 1
DEFDBL _, z-x
x = 1: y = 2: z = 3: w = 4: __v = 5
_DEFINE u AS _UNSIGNED _BYTE
u1 = -1
_DEFINE f-g AS STRING * 5
f1 = "abcdefgh"
_Define B As _Bit * 4
b2 = 7
IF 0 THEN
    DEFSTR S
END IF
s = "text"
DEFLNG F
r# = fn(3) + fn&(4)
DEFSNG F
r# = fn(5)
below 1
inner 2
last 3
m = 1
SYSTEM
DEFLNG F
FUNCTION fn (n)
    fn = n * 100000
END FUNCTION
SUB below (p)
    v = 2.5
    SHARED g
    STATIC st
END SUB
SUB inner (p)
    u = p
    DEFDBL A-Z
    v = p
END SUB
SUB last (p)
    v = p
END SUB
