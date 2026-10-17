' TEST: ir
$CONSOLE:ONLY
' Event trapping in the IR (m3-events design D3): one OnEvent per handler with its operands converted (a timer
' number LONG, seconds DOUBLE, a key DOUBLE, a SUB's argument _INTEGER64), the main module's label also from a SUB;
' the switches as built-in calls; KEY's own forms.
DIM t AS SINGLE, x AS INTEGER
t = _FREETIMER
ON TIMER(t, 1.5) GOSUB h
ON TIMER(2) GOSUB h
ON KEY(3) s x
ON KEY(1.7) plain
TIMER(t) ON
TIMER STOP
KEY(0) OFF
KEY ON
KEY LIST
KEY 15, CHR$(0) + CHR$(30)
setup
END

h:
RETURN

SUB s (n AS INTEGER)
END SUB

SUB plain
END SUB

SUB setup
    ON TIMER(t + 0.5, 1) GOSUB h
END SUB
