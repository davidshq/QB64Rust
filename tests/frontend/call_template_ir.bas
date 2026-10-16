' TEST: ir
' A built-in statement that has a template, after CALL (m3-sound task 2.1): the old compiler reads the template's
' forms between the parentheses (verification\v26_x74, v27_x02, v28_x65, x66, x70, v29_x27, x54), and with no
' parentheses the form with no argument. Each is the same built-in operation as the statement without CALL
' (compared by tests\callsite, the programs named *_call*)
DIM l AS LONG, t AS STRING, a(1 TO 100) AS INTEGER
SCREEN 12
CALL CLS
CALL CLS(1)
CALL COLOR(1, 2)
CALL LOCATE(1, 2)
CALL LOCATE(, 2)
CALL WIDTH(80)
CALL SCREEN(12)
CALL PSET((1, 2), 3)
CALL PRESET((1, 2))
CALL LINE((1, 2)-(3, 4))
CALL LINE((1, 2)-(3, 4), 5, BF)
CALL LINE(-(3, 4))
CALL CIRCLE((1, 2), 3)
CALL PAINT((1, 2))
CALL PALETTE
CALL PALETTE(1, 2)
CALL SLEEP
CALL SLEEP(1)
CALL VIEW
CALL VIEW((1, 2)-(3, 4))
CALL WINDOW
CALL BLOAD(t, l)
CALL PUT((1, 2), a)
CALL _FULLSCREEN
CALL _ICON
CALL _ICON(1)
CALL _ICON(1, 2)
CALL _MOUSECURSOR(1, (2, 3))
CALL _MOUSEHIDE(_HIDE)
CALL _FREEIMAGE
CALL _FREEIMAGE(1)
CALL _SCREENHIDE
CALL _SCREENSHOW
CALL _SCREENICON
CALL _WINDOWSIZELIMIT((1, 2)-(3, 4))
CALL SOUND(440, 1)
CALL SOUND(440, 1, .5, 0, 4, .5, 1)
CALL SOUND(440, 1 WAIT)
CALL SOUND(WAIT)
CALL SOUND
CALL PLAY(t)
CALL PLAY(t, t)
CALL SHELL("echo x")
CALL SHELL(_HIDE "echo x")
CALL RANDOMIZE(5)
CALL RANDOMIZE(USING 5)
CALL _MOUSESHOW("a")
CALL BLOAD("f", 1)
IF l THEN CALL CLS ELSE CALL CLS(1)
CALL CLS: CALL LOCATE(1, 2)
SYSTEM
