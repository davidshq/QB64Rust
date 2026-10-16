' _LIMIT, _DELAY, SLEEP and _DISPLAY: nothing in the state depends on how long anything took.
DIM i AS INTEGER
SCREEN 7
FOR i = 1 TO 10
    PSET (i * 10, 50), i
    _LIMIT 60
NEXT i
_DELAY .01
_DISPLAY
LINE (10, 60)-(100, 70), 4, BF
_DISPLAY
' A key ends SLEEP only when it comes while SLEEP waits, and the keys file gives the next key only when the
' key buffer is empty: so the buffer is emptied first. The time is the longest SLEEP can take, not what it takes.
DO: LOOP UNTIL INKEY$ = ""
SLEEP 2
PRINT "after SLEEP"
SYSTEM
