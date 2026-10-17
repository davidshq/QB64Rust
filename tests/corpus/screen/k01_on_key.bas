' Screen program (m3-events, task 3.3): ON KEY in a window, given its keys by the keys file. F1, the up arrow
' and a user key (KEY 15 with flags 0 and the scancode of A) run their handlers; a key switched off reaches
' INKEY$ instead; the soft-key line of KEY ON and the list of KEY LIST.
DIM SHARED got AS INTEGER
DIM k AS STRING
KEY 15, CHR$(0) + CHR$(30)
KEY 2, "two"
ON KEY(1) GOSUB f1
ON KEY(11) GOSUB up
ON KEY(15) GOSUB user
KEY(1) ON
KEY(11) ON
KEY(15) ON
KEY ON
PRINT "waiting for F1"
GOSUB announce
DO: _LIMIT 100: LOOP UNTIL got = 1
GOSUB announce
DO: _LIMIT 100: LOOP UNTIL got = 2
GOSUB announce
DO: _LIMIT 100: LOOP UNTIL got = 3
KEY(1) OFF
PRINT "F1 is off"
GOSUB announce
DO: k = INKEY$: _LIMIT 100: LOOP WHILE k = ""
PRINT "INKEY$ got"; LEN(k); "bytes:"; ASC(k, LEN(k))
KEY OFF
KEY LIST
SYSTEM

f1:
got = got + 1
PRINT "F1 handler"
RETURN

up:
got = got + 1
PRINT "up handler"
RETURN

user:
got = got + 1
PRINT "user key handler"
RETURN

announce:
DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG
RETURN
