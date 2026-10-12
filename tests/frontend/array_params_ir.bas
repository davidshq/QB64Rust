' TEST: ir
$CONSOLE:ONLY
' Array parameters in the IR (m2-defxxx-common-arrays task 7.1; spec compiler/pipeline "Array storage in the IR"):
' an array parameter is an array with storage "param" and the number of dimensions of what is passed to it; its
' argument is Array with the caller's array, static or dynamic or the caller's own parameter; a REDIM or DIM of it
' is an Allocate (which raises 10 for a static argument when it runs), its ERASE an EraseParam, which clears a
' static argument and releases a dynamic one. No descriptor appears
DIM x(1 TO 3) AS LONG, m(2, 4) AS LONG
REDIM d(2) AS LONG
grow d()
grow x()
PRINT last&(m())
SYSTEM
SUB grow (a() AS LONG)
    REDIM _PRESERVE a(10) AS LONG
    a(1) = UBOUND(a)
    wipe a()
END SUB
SUB wipe (a() AS LONG)
    ERASE a
END SUB
FUNCTION last& (t() AS LONG)
    last& = t(UBOUND(t, 1), UBOUND(t, 2))
END FUNCTION
