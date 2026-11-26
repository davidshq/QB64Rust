' A piano: the keys A S D F G H J K play the notes of a scale, and the window shows which one.
' Press Esc to stop.
SCREEN _NEWIMAGE(640, 240, 32)
_TITLE "Piano"
keys$ = "ASDFGHJK"
DIM hertz(1 TO 8)
FOR i = 1 TO 8
    READ hertz(i)
NEXT
DATA 262,294,330,349,392,440,494,523
DrawKeys 0
DO
    k$ = UCASE$(INKEY$)
    n = INSTR(keys$, k$)
    IF k$ <> "" AND n > 0 THEN
        DrawKeys n
        SOUND hertz(n), 3
        DrawKeys 0
    END IF
    _LIMIT 60
LOOP UNTIL k$ = CHR$(27)
SYSTEM

' Draws the eight keys with their letters; the key `lit` (1 to 8) is drawn yellow, none when it is 0.
SUB DrawKeys (lit)
    SHARED keys$
    CLS
    FOR i = 1 TO 8
        x = (i - 1) * 80
        c~& = _RGB32(240, 240, 240)
        IF i = lit THEN c~& = _RGB32(255, 220, 0)
        LINE (x + 2, 20)-(x + 77, 220), c~&, BF
        COLOR _RGB32(0, 0, 0), c~&
        _PRINTSTRING (x + 36, 190), MID$(keys$, i, 1)
    NEXT
    COLOR _RGB32(255, 255, 255), _RGB32(0, 0, 0)
    _DISPLAY
END SUB
