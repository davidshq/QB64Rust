' TEST: typed
$CONSOLE:ONLY
' The $ name of a STRING * n array or variable (m4-qb64pe-sources task 5.1; measured in verification\v52_x39-x49):
' a$(i) and UBOUND(a$) are the STRING * 4 array a, as a value and as a place; s$ is the STRING * 4 variable s, and
' a DIM s$ or REDIM s$ after it declares nothing new (x69), nor does STATIC w$ after STATIC w AS STRING * 2 in a
' SUB (x70); in a SUB, h$(i) and p$ are the DIM SHARED ones; b$(i) beside an AS STRING array b is that array (as
' before)
REDIM a(1 TO 3) AS STRING * 4
DIM s AS STRING * 4, b(2) AS STRING
DIM SHARED h(5) AS STRING * 8, p AS STRING * 1
x$ = a$(2)
a$(1) = "zz"
n = UBOUND(a$)
x$ = s$
s$ = "x"
DIM s$
REDIM s$
s$ = "v"
b$(1) = "q"
t
SUB t
    h$(1) = "x"
    p$ = "\"
    PRINT LEN(h$(1)); p$
    STATIC w AS STRING * 2
    STATIC w$
    w$ = "ab"
END SUB
