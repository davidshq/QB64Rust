$CONSOLE:ONLY
' Verification (m3-events, task 1.1): SUB handlers and where a handler is set up. A SUB with an INTEGER
' argument (evaluated when the handler is set up, not when it runs; what the SUB does to it), one without, an
' _UNSIGNED INTEGER and an _INTEGER64 one given a negative value, a _BYTE one given 300; a GOSUB handler set
' up inside a SUB, which runs the main module's label; ON TIMER again, replacing the handler.
DIM SHARED x AS LONG
x = 3
ON TIMER(0.3) show x * 2
x = 100
GOSUB once
GOSUB once
setup
GOSUB once
ON TIMER(0.3) plain
GOSUB once
ON TIMER(0.3) unsigned16 -5
GOSUB once
ON TIMER(0.3) wide -5
GOSUB once
ON TIMER(0.3) small 300
GOSUB once
ON TIMER(0.3) wide 2.5
GOSUB once
ON TIMER(0.3) wide 3.5
GOSUB once
SYSTEM

once:
TIMER ON
_DELAY 0.45
TIMER OFF
RETURN

h:
PRINT "main label h"
RETURN

SUB show (n AS INTEGER)
    PRINT "show"; n
    n = n + 1
END SUB

SUB setup
    ON TIMER(0.3) GOSUB h
END SUB

SUB plain
    PRINT "plain"
END SUB

SUB unsigned16 (n AS _UNSIGNED INTEGER)
    PRINT "unsigned16"; n
END SUB

SUB wide (n AS _INTEGER64)
    PRINT "wide"; n
END SUB

SUB small (n AS _BYTE)
    PRINT "small"; n
END SUB
