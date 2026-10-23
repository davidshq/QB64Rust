' TEST: typed
$CONSOLE:ONLY
' Declared names with blanks around their dots (m4-names-implicit-arrays task 3.1, design D7): the old compiler
' drops the blanks as it reads the line, so each is the name without them (measured, verification\v47_x17, x18,
' v55_x59 to x65): a variable, an array, a list, a suffix, a shared variable, a SUB called with and without CALL
' and a STATIC SUB, a FUNCTION called with and without blanks.
DIM a . b AS LONG, c . d AS INTEGER
DIM e . f(3) AS LONG
DIM g . h$
DIM SHARED k . m AS LONG
a.b = 3: c.d = 2: e.f(1) = 4: g.h$ = "s": k.m = 5
PRINT a.b; a . b; c . d; e . f(1); g . h$
s . t 3
CALL s . t(4)
PRINT u.v(3); u . v(4)
w . x
SYSTEM

SUB s . t (n)
    PRINT n; k.m
END SUB

FUNCTION u . v (n)
    u.v = n * 2
END FUNCTION

SUB w . x STATIC
    y = y + 1
END SUB
