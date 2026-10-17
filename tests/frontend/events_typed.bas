' TEST: typed
$CONSOLE:ONLY
' Event trapping in the typed tree (m3-events design D2): _FREETIMER a LONG; the operands of ON TIMER and ON KEY
' converted as the old compiler converts them (a timer number to LONG, rounded; seconds and a key to DOUBLE; a
' SUB's argument to _INTEGER64, rounded); KEY's own forms.
DIM t AS LONG
t = _FREETIMER
ON TIMER(t + 0.4, 2) GOSUB h
ON KEY(31.6) s 2.5
KEY 1, "x"
KEY OFF
END

h:
RETURN

SUB s (n AS LONG)
END SUB
