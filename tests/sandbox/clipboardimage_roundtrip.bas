' _CLIPBOARDIMAGE = an image, then _CLIPBOARDIMAGE read: the image read back (tests/sandbox/README.md).
OPEN "out.txt" FOR OUTPUT AS #1

img& = _NEWIMAGE(4, 4, 32)
_DEST img&
FOR y = 0 TO 3
    FOR x = 0 TO 3
        PSET (x, y), _RGB32(x * 60, y * 60, 100 + x + y * 4)
    NEXT
NEXT
_DEST 0
_CLIPBOARDIMAGE = img&

r& = _CLIPBOARDIMAGE
PRINT #1, "an image read:"; r& < -1
IF r& < -1 THEN
    PRINT #1, "size:"; _WIDTH(r&); _HEIGHT(r&)
    _SOURCE r&
    FOR y = 0 TO _HEIGHT(r&) - 1
        FOR x = 0 TO _WIDTH(r&) - 1
            c~& = POINT(x, y)
            PRINT #1, _RED32(c~&); _GREEN32(c~&); _BLUE32(c~&); "|";
        NEXT
        PRINT #1, ""
    NEXT
END IF
CLOSE #1
SYSTEM
