' TEST: ir
' The pacing statements in the IR (m3-window-output task 5.1, design D2; each call measured from the old compiler's
' C++ and compared by tests\callsite): _LIMIT and _DELAY take one DOUBLE value (any number, converted exactly),
' SLEEP an optional LONG value (a float rounded), _DISPLAY nothing. _LIMIT, _DELAY and _DISPLAY have no template
' and are also read after CALL
DIM l AS LONG, i AS INTEGER, q AS _INTEGER64, sg AS SINGLE, d AS DOUBLE, f AS _FLOAT
_LIMIT 60
_LIMIT d
_LIMIT sg
_LIMIT q
_LIMIT f
_LIMIT 59.5
CALL _LIMIT(i)
_DELAY 1
_DELAY .01
_DELAY d
_DELAY f
CALL _DELAY(sg)
SLEEP
SLEEP 1
SLEEP l
SLEEP q
SLEEP sg
SLEEP 0.5
_DISPLAY
CALL _DISPLAY
SYSTEM
