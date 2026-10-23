' TEST: check-fail
$CONSOLE:ONLY
' The forms of SOUND, PLAY and BEEP the old compiler rejects (m3-sound tasks 2.1 and 2.3; verification\v29_x*, the 36
' rejected programs, one line each but x80, which is the last PLAY: line with GOTO PLAY). Each is an error that is
' not "not supported yet"; GOTO BEEP and GOTO SOUND are "Invalid label!" (x78, x79; m4-names-implicit-arrays task
' 2.4). PLAY ON, PLAY OFF and PLAY STOP are no event switches (x61-x63)
DIM i AS INTEGER, l AS LONG, d AS DOUBLE, t AS STRING
SOUND 440
SOUND 440, 1, .5, 0, 4, .5, 1, 2
SOUND 440, 1, WAIT
SOUND WAIT RESUME
SOUND t, 1
SOUND 440, t
SOUND , 1
SOUND 440, , .5
SOUND = 1
l = SOUND
SOUND(440, 1)
DIM wait AS LONG
SOUND 440, 1, , , , ,
PLAY t, t, t, t, t
PLAY
PLAY 1
PLAY , t
PLAY(t, t)
d = PLAY()
d = PLAY(t)
d = PLAY(0, 1)
PLAY ON
PLAY OFF
PLAY STOP
PLAY: l = 1
PLAY = 1
PLAY(0)
CALL PLAY(0)
PLAY:
BEEP 1
BEEP = 1
l = BEEP
BEEP()
BEEP: l = 1
GOTO BEEP
SOUND: l = 1
GOTO SOUND
GOTO PLAY
SYSTEM
