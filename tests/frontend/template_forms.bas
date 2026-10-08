' TEST: parse-ok
' Built-in statements read by their template (m2-parser-breadth task 7.5, design D6): every form parses; sema marks
' them "not supported yet". The old compiler accepts this file (`qb64pe.exe -z`, 2026-10-07). With variables B and
' BF, `LINE …, B` takes B as the colour and `LINE …, BF, BF` the second BF as the box word (measured M5).
DIM a(100) AS INTEGER
DIM m AS _MEM
SCREEN 12
SCREEN _NEWIMAGE(640, 480, 32), , 1, 0
B = 7: BF = 9
LINE (0, 0)-(9, 9), , BF
LINE (0, 0)-STEP(5, 5), 3, B
LINE -(20, 20), B
LINE (1, 1)-(2, 2), BF, BF
LINE (1, 1)-(2, 2), 4, , &HFF00
PSET (1, 1)
PSET STEP(1, 1), 2
PRESET (3, 3)
CIRCLE (50, 50), 20, 4, , , 0.5
CIRCLE STEP(0, 0), 5
PAINT (5, 5), 2, 4
GET (0, 0)-(9, 9), a()
PUT (10, 10), a(), PSET
PUT (10, 10), a(), XOR
COLOR 15, 1
COLOR , 2
LOCATE 5, 10
LOCATE , 1
CLS
CLS 2
VIEW PRINT 1 TO 20
VIEW (10, 10)-(100, 100), 1, 2
WINDOW SCREEN(0, 0)-(10, 10)
WINDOW
PALETTE 1, 0
DEF SEG = 0
DEF SEG
OPEN "data.txt" FOR INPUT AS #1
OPEN "data.bin" FOR BINARY ACCESS READ SHARED AS 2 LEN = 128
OPEN "O", #3, "out.txt"
GET #2, 1, v%
PUT #2, , v%
PUT 2, 5, v%
SEEK #1, 10
LOCK #1, 1 TO 10
UNLOCK 1
NAME "a.txt" AS "b.txt"
TIME$ = "12:00:00"
DATE$ = "01-01-2026"
_CLIPBOARD$ = "text"
PLAY "cde"
SOUND 440, 18
WIDTH 80, 25
RANDOMIZE TIMER
RANDOMIZE USING 5
SHELL _HIDE "dir"
SLEEP 1
_PUTIMAGE (0, 0)-(10, 10), 1, 2
_PRINTSTRING (1, 1), "hi"
_FULLSCREEN _SQUAREPIXELS, _SMOOTH
_MEMCOPY m, m.OFFSET, 4 TO m, m.OFFSET + 4
