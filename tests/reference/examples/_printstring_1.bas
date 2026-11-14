$CONSOLE:ONLY
img& = _NEWIMAGE(60, 20, 32)
_DEST img&
_PRINTSTRING (2, 2), "Hi"
w = _PRINTWIDTH("Hi")
_DEST _CONSOLE
PRINT "The text is"; w; "pixels wide and draws"; Lit&(img&); "white pixels."
END

FUNCTION Lit& (img&)
    _SOURCE img&
    n& = 0
    FOR y = 0 TO _HEIGHT(img&) - 1
        FOR x = 0 TO _WIDTH(img&) - 1
            IF POINT(x, y) = _RGB32(255, 255, 255) THEN n& = n& + 1
        NEXT x
    NEXT y
    Lit& = n&
END FUNCTION
