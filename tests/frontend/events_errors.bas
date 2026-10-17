' TEST: check-fail
$CONSOLE:ONLY
' Event trapping, the forms the old compiler rejects (m3-events task 2.2, design D1 and D2), each a real error
' (measured, verification\v42_x02-x05, x09, x10, x13-x22, x24-x36, x41-x45, x48, x49, x52, x58). The SUB
' with a TYPE parameter passes the old compiler's checks and its C++ fails to build (x42).
TYPE t
    a AS LONG
END TYPE
' Labels and SUBs (x02, x09, x10, x13, x52).
ON TIMER(1) GOSUB g
ON TIMER(1) GOSUB nowhere
ON TIMER(1) nosuch
ON TIMER(1) f
ON TIMER(1) h
' The SUB's parameter and argument (x03-x05, x14, x24, x35, x36, x41, x42, x48, x58).
ON TIMER(1) needsarg
ON TIMER(1) noparams 5
ON TIMER(1) floatparam 5
ON TIMER(1) arrayparam 5
ON TIMER(1) twoparams 5
ON TIMER(1) needsarg "a"
ON TIMER(1) needsarg 1, 2
ON TIMER(1) stringparam 1
ON TIMER(1) typeparam 1
ON KEY(1) needsarg
ON TIMER(1) suffixparam 5
' The operands (x27-x30, x33, x34).
ON TIMER() GOSUB h
ON TIMER(1, 2, 3) GOSUB h
ON KEY(1, 2) GOSUB h
ON TIMER GOSUB h
ON TIMER("a") GOSUB h
ON KEY("a") GOSUB h
' The target (x31, x32).
ON TIMER(1)
ON TIMER(1) GOSUB
' Switches outside the template (x15, x16, x49).
KEY(1) FREE
TIMER(1, 2) ON
TIMER(1)
' KEY's own forms (x25, x26, x43-x45).
KEY ON 1
KEY LIST 2
KEY OFF 2
KEY 1, 5
KEY
' Events the old compiler does not have (x17-x22).
ON PEN GOSUB h
ON COM(1) GOSUB h
ON UEVENT GOSUB h
PEN ON
UEVENT ON
COM(1) ON
END

h:
RETURN

SUB s
g:
END SUB

FUNCTION f
END FUNCTION

SUB needsarg (n AS LONG)
END SUB

SUB noparams
END SUB

SUB floatparam (n AS SINGLE)
END SUB

SUB arrayparam (n() AS LONG)
END SUB

SUB twoparams (n AS LONG, m AS LONG)
END SUB

SUB stringparam (n AS STRING)
END SUB

SUB typeparam (n AS t)
END SUB

SUB suffixparam (n#)
END SUB
