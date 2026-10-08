' TEST: typed
$CONSOLE:ONLY
' ON n GOTO / ON n GOSUB in the typed tree (m2-core-builtins task 7.1, design D8): n converted to LONG as the old
' compiler does (a float narrowed to SINGLE, then rounded half to even; an integer keeping its low 32 bits); the
' targets are labels of the same body
DIM i AS INTEGER, q AS _INTEGER64, d AS DOUBLE
ON i GOTO a, b
ON q GOSUB a
ON d GOTO b, a, b
ON 1.5 GOSUB b
a:
b:
RETURN
SUB s (n)
    ON n GOSUB c
    EXIT SUB
c: RETURN
END SUB
