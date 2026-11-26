' Bouncing ball: a ball moves across the window and bounces off its edges. Press Esc to stop.
SCREEN _NEWIMAGE(640, 480, 32)
_TITLE "Bouncing ball"
x = 320: y = 240
dx = 4: dy = 3
radius = 20
DO
    CLS
    x = x + dx
    y = y + dy
    IF x < radius OR x > 640 - radius THEN dx = -dx
    IF y < radius OR y > 480 - radius THEN dy = -dy
    DrawBall x, y, radius
    _DISPLAY
    _LIMIT 60
LOOP UNTIL INKEY$ = CHR$(27)
SYSTEM

' Draws a filled ball with its center at (x, y).
SUB DrawBall (x, y, r)
    CIRCLE (x, y), r, _RGB32(255, 200, 0)
    PAINT (x, y), _RGB32(255, 200, 0), _RGB32(255, 200, 0)
END SUB
