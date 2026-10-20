' The step digest of the keys file (tools/screen/README.md): a line with a hash of the state that a state step
' would write at that moment, in its place among the states. The screen changes between the two digests.
DIM k AS STRING
SCREEN 12
LINE (10, 10)-(200, 100), 4, BF
PRINT "first"
GOSUB announce
DO: k = INKEY$: LOOP WHILE k = ""
CIRCLE (300, 200), 50, 14
PRINT "second: "; k
GOSUB announce
DO: k = INKEY$: LOOP WHILE k = ""
PRINT "third: "; k
GOSUB announce
DO: k = INKEY$: LOOP WHILE k = ""
SYSTEM

announce:
DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG
RETURN
