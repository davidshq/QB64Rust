' Catch: move the basket with the Left and Right arrow keys and catch the falling stars. Three missed
' stars end the game.
SCREEN _NEWIMAGE(480, 480, 32)
_TITLE "Catch"
RANDOMIZE TIMER
basket = 240
starX = RND * 440 + 20: starY = 0
speed = 3
score = 0: missed = 0
DO
    IF _KEYDOWN(19200) AND basket > 30 THEN basket = basket - 6
    IF _KEYDOWN(19712) AND basket < 450 THEN basket = basket + 6
    starY = starY + speed
    IF starY >= 440 THEN
        IF ABS(starX - basket) < 35 THEN
            score = score + 1
            speed = speed + 0.25
        ELSE
            missed = missed + 1
        END IF
        starX = RND * 440 + 20: starY = 0
    END IF
    CLS
    CIRCLE (starX, starY), 8, _RGB32(255, 255, 0)
    LINE (basket - 30, 440)-(basket + 30, 455), _RGB32(160, 100, 40), BF
    _PRINTSTRING (10, 10), "Caught:" + STR$(score) + "   Missed:" + STR$(missed)
    _DISPLAY
    _LIMIT 60
LOOP UNTIL missed = 3 OR _KEYDOWN(27)
_AUTODISPLAY
_PRINTSTRING (180, 230), "Game over! You caught" + STR$(score)
SLEEP
SYSTEM
