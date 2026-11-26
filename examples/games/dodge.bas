' Dodge: rocks fall from the top. Move with the Left and Right arrow keys and keep away from them for
' as long as you can.
CONST ROCKS = 8
SCREEN _NEWIMAGE(480, 480, 32)
_TITLE "Dodge"
RANDOMIZE TIMER
DIM rx(1 TO ROCKS), ry(1 TO ROCKS), rspeed(1 TO ROCKS)
FOR i = 1 TO ROCKS
    rx(i) = RND * 460 + 10
    ry(i) = -RND * 480
    rspeed(i) = RND * 3 + 2
NEXT
player = 240
start = TIMER
hit = 0
DO
    IF _KEYDOWN(19200) AND player > 12 THEN player = player - 5
    IF _KEYDOWN(19712) AND player < 468 THEN player = player + 5
    CLS
    FOR i = 1 TO ROCKS
        ry(i) = ry(i) + rspeed(i)
        IF ry(i) > 490 THEN ry(i) = -10: rx(i) = RND * 460 + 10
        CIRCLE (rx(i), ry(i)), 10, _RGB32(150, 150, 150)
        IF ABS(rx(i) - player) < 20 AND ABS(ry(i) - 450) < 20 THEN hit = -1
    NEXT
    LINE (player - 10, 440)-(player + 10, 460), _RGB32(0, 200, 255), BF
    _PRINTSTRING (10, 10), "Time:" + STR$(INT(TIMER - start))
    _DISPLAY
    _LIMIT 60
LOOP UNTIL hit OR _KEYDOWN(27)
_AUTODISPLAY
_PRINTSTRING (150, 230), "Hit! You lasted" + STR$(INT(TIMER - start)) + " seconds."
SLEEP
SYSTEM
