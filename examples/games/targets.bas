' Targets: click each target with the mouse before it moves. You have 30 seconds.
SCREEN _NEWIMAGE(640, 480, 32)
_TITLE "Targets"
RANDOMIZE TIMER
score = 0
start = TIMER
MoveTarget tx, ty, shown
DO
    WHILE _MOUSEINPUT
        IF _MOUSEBUTTON(1) AND NOT down THEN
            IF (_MOUSEX - tx) ^ 2 + (_MOUSEY - ty) ^ 2 < 25 ^ 2 THEN
                score = score + 1
                MoveTarget tx, ty, shown
            END IF
        END IF
        down = _MOUSEBUTTON(1)
    WEND
    IF TIMER - shown > 1.5 THEN MoveTarget tx, ty, shown
    CLS
    CIRCLE (tx, ty), 25, _RGB32(255, 60, 60)
    CIRCLE (tx, ty), 15, _RGB32(255, 255, 255)
    CIRCLE (tx, ty), 5, _RGB32(255, 60, 60)
    remaining = 30 - INT(TIMER - start)
    _PRINTSTRING (10, 10), "Hits:" + STR$(score) + "   Seconds left:" + STR$(remaining)
    _DISPLAY
    _LIMIT 60
LOOP UNTIL remaining <= 0 OR _KEYDOWN(27)
_AUTODISPLAY
_PRINTSTRING (240, 230), "Time! Hits:" + STR$(score)
SLEEP
SYSTEM

' Puts the target somewhere new and notes when.
SUB MoveTarget (x, y, since)
    x = RND * 560 + 40
    y = RND * 400 + 40
    since = TIMER
END SUB
