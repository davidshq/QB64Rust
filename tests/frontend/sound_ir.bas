' TEST: ir
$CONSOLE:ONLY
' SOUND, PLAY and BEEP in the IR (m3-sound task 2.1, design D1 to D3): every form the old compiler accepts
' (verification\v29_x*, each call compared by tests\callsite). SOUND has seven slots and the choice WAIT or RESUME;
' a trailing comma is the statement without it; the name and a colon is the statement with no argument (BEEP:,
' SOUND:). PLAY takes one to four strings, a fixed-length string among them. The settings line names the audio
' part of the runtime, which SOUND and PLAY need
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, t AS STRING, u AS _UNSIGNED LONG
DIM fs AS STRING * 4
SOUND 440, 1
SOUND
SOUND 440, 1, .5
SOUND 440, 1, , 0
SOUND 440, 1, , , 4
SOUND 440, 1, , , , .5
SOUND 440, 1, , , , , 1
SOUND 440, 1, .5, 0, 4, .5, 1
SOUND WAIT
SOUND RESUME
SOUND 440, 1 WAIT
SOUND 440, 1, .5 RESUME
SOUND 440, 1, .5, 0, 4, .5, 1 WAIT
SOUND i, l
SOUND q, d
SOUND u, s
SOUND 440, 1, d, q, s, i, d
SOUND f, f, f, f, f, f, f
SOUND (440), (1)
SOUND 440, 1,
SOUND 37 + 50 * RND, .03
CALL SOUND(440, 1)
sound 440, 1 wait
Sound Resume
SOUND: l = 1
SOUND WAIT: SOUND RESUME: l = 2
SOUND:
PLAY "c"
PLAY t
PLAY t, t
PLAY t, , t
PLAY t, , , t
PLAY t, t, t, t
PLAY t,
PLAY "c" + t + LEFT$(t, 2)
PLAY (t)
CALL PLAY(t)
PLAY fs
PLAY t, fs
play t
BEEP
CALL BEEP
BEEP: l = 3
BEEP:
beep
IF l THEN BEEP ELSE BEEP
IF l THEN BEEP: BEEP
IF l THEN SOUND 440, 1 ELSE PLAY t
CALL one
SYSTEM

SUB one
    SOUND 440, 1: PLAY "c": BEEP
    PRINT PLAY(0)
END SUB
