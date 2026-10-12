' TEST: cpp
$CONSOLE:ONLY
' Array parameters in the emitter (m2-defxxx-common-arrays task 7.1, design D7): an array parameter is the
' descriptor of the array passed (ptrszint*), so the procedure indexes, allocates and reads the bounds of the
' caller's array through it, and nothing is made or released for it on a call; a string array parameter gets no
' copy of a string argument. The argument is the caller's descriptor, its own parameter as it is. ERASE of a
' parameter tests the descriptor's flag: a static array is cleared, a dynamic one released
DIM x(1 TO 3) AS LONG, s(2) AS STRING
REDIM d(2) AS LONG
grow d(), s()
grow x(), s()
SYSTEM
SUB grow (a() AS LONG, t() AS STRING)
    REDIM _PRESERVE a(10) AS LONG
    a(1) = UBOUND(a)
    t(1) = "a"
    wipe a(), t()
END SUB
SUB wipe (a() AS LONG, t() AS STRING)
    ERASE a, t
END SUB
