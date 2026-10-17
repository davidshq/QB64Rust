' TEST: cpp
$CONSOLE:ONLY
' Event trapping in C++ (m3-events design D4): the setup calls, the handlers numbered per event in source order
' (the main module, then the procedures), the cases of events() and the jumps into QBMAIN; a label handler after a
' loop, a SELECT and an error handler; a SUB with and without an argument; a handler set up in a SUB.
DIM SHARED n AS LONG
DIM i AS LONG
ON ERROR GOTO oops
ON TIMER(0.5) GOSUB tick
ON KEY(1) withArg n + 1
ON KEY(2) noArg
TIMER ON
FOR i = 1 TO 3
    SELECT CASE i
        CASE 1: n = n + 1
        CASE ELSE: n = n - 1
    END SELECT
NEXT
setup
KEY 1, "help"
END

oops:
RESUME NEXT

tick:
n = n + 1
RETURN

SUB withArg (k AS INTEGER)
    PRINT k
END SUB

SUB noArg
END SUB

SUB setup
    ON TIMER(1) GOSUB tick
END SUB
