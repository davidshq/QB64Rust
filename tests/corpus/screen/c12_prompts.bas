' The steps prompt and state of the keys file (tools/screen/README.md). The program announces each wait for a
' key by counting in byte &H4F0 of conventional memory; each prompt of the keys file waits for one announcement,
' and state writes the screen as it is then. A loop that only looks for a key gets none, and a menu that
' announces after every arrow key gets its keys one at a time.
DIM k AS STRING, i AS INTEGER, eaten AS INTEGER, row AS INTEGER
SCREEN 13
LINE (20, 20)-(120, 80), 4, BF
GOSUB announce
DO: k = INKEY$: LOOP WHILE k = ""
CLS
PRINT "first: "; k
FOR i = 1 TO 100
    _DELAY .002
    IF INKEY$ <> "" THEN eaten = eaten + 1
NEXT i
PRINT "eaten:"; eaten
GOSUB announce
DO: k = INKEY$: LOOP WHILE k = ""
PRINT "second: "; k
row = 1
DO
    FOR i = 1 TO 4
        COLOR 7: IF i = row THEN COLOR 14
        LOCATE 5 + i, 3: PRINT "entry"; i
    NEXT i
    GOSUB announce
    DO: k = INKEY$: LOOP WHILE k = ""
    IF k = CHR$(0) + "P" THEN row = row + 1
LOOP UNTIL k = CHR$(13)
COLOR 15: LOCATE 11, 1: PRINT "chosen:"; row
SYSTEM

announce:
DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG
RETURN
