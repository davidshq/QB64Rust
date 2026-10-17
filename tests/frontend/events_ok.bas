' TEST: check-ok
$CONSOLE:ONLY
' Event trapping, the forms the old compiler accepts (m3-events task 2.2, design D1 and D2; measured,
' verification\v42_x01, x06-x08, x12, x37-x39, x47, x50, x51, x53-x55, x57, x60): handlers with a label of the
' main module, also set up in a SUB or a FUNCTION and with a label of the same name in a SUB; SUBs with an integer
' parameter of every kind or none; the switches, _FREETIMER and KEY's own forms.
DIM k AS INTEGER, a AS STRING, t AS LONG, d AS DOUBLE
t = _FREETIMER
ON TIMER(t, 0.5) GOSUB h
ON TIMER(2) GOSUB h
ON TIMER(d) GOSUB h
ON KEY(1) GOSUB h
ON KEY(d) GOSUB h
ON TIMER(1) plain
ON TIMER(1) i8 5
ON TIMER(1) u16 -5
ON TIMER(1) i32 (5)
ON TIMER(1) i64 2.5
ON TIMER(1) u64 k * 2
ON TIMER(1) offs 5
ON TIMER(1) uoffs 5
ON TIMER(1) suffix 5
ON KEY(15) i32 LEN(a)
ON TIMER(1) GOSUB noreturn
TIMER ON
TIMER OFF
TIMER STOP
TIMER(t) ON
TIMER(t) STOP
TIMER(t) OFF
TIMER(t) FREE
KEY(1) ON
KEY(1) OFF
KEY(1) STOP
KEY (2) ON
KEY(0) STOP
KEY ON
KEY OFF
KEY LIST
KEY 15, CHR$(0) + CHR$(30)
KEY k, a
KEY 3, "a" + "b"
setup
d = f
END

h:
RETURN

noreturn:
PRINT

SUB setup
    ON TIMER(1) GOSUB h
    ON KEY(1) GOSUB h
h:
END SUB

FUNCTION f
    ON TIMER(1) GOSUB h
END FUNCTION

SUB plain
END SUB

SUB i8 (n AS _BYTE)
END SUB

SUB u16 (n AS _UNSIGNED INTEGER)
END SUB

SUB i32 (n AS LONG)
END SUB

SUB i64 (n AS _INTEGER64)
END SUB

SUB u64 (n AS _UNSIGNED _INTEGER64)
END SUB

SUB offs (n AS _OFFSET)
END SUB

SUB uoffs (n AS _UNSIGNED _OFFSET)
END SUB

SUB suffix (n%)
END SUB
