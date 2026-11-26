' Flying through stars: each star moves out from the middle of the window, faster as it comes near.
' Press Esc to stop.
CONST STARS = 200
SCREEN _NEWIMAGE(800, 600, 32)
_TITLE "Flying through stars"
DIM sx(1 TO STARS), sy(1 TO STARS), sz(1 TO STARS)
RANDOMIZE TIMER
FOR i = 1 TO STARS
    NewStar sx(i), sy(i), sz(i)
    sz(i) = RND * 100 + 1
NEXT
DO
    CLS
    FOR i = 1 TO STARS
        sz(i) = sz(i) - 1
        IF sz(i) < 1 THEN NewStar sx(i), sy(i), sz(i)
        ' The nearer the star, the farther from the middle and the brighter.
        px = 400 + sx(i) / sz(i) * 100
        py = 300 + sy(i) / sz(i) * 100
        b = 255 - sz(i) * 2
        IF b < 40 THEN b = 40
        PSET (px, py), _RGB32(b, b, b)
    NEXT
    _DISPLAY
    _LIMIT 60
LOOP UNTIL INKEY$ = CHR$(27)
SYSTEM

' A new star far away, somewhere ahead.
SUB NewStar (x, y, z)
    x = RND * 800 - 400
    y = RND * 600 - 300
    z = 100
END SUB
