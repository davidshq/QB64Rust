' TEST: check-fail
$CONSOLE:ONLY
' The forms of the sound handle built-ins the old compiler rejects (m3-sound-handles task 2.1; verification\v39_x02-
' x54, x56, x61-x70, x73, x75, x78; one line each). Each is an error that is not "not supported yet": the number of
' arguments, a string for a number and the other way, a function named bare that needs its argument, a statement
' used as a function, the arrays of _WAVE and _SNDRAWBATCH (design D2), and $MIDISOUNDFONT: (D3)
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, t AS STRING, u AS _UNSIGNED LONG
DIM h AS LONG, r AS LONG, bv AS _BYTE, sv AS SINGLE, m AS _MEM
DIM w(1 TO 64) AS _BYTE, b(1 TO 16) AS SINGLE, iw(1 TO 8) AS INTEGER, uw(1 TO 8) AS _UNSIGNED _BYTE
DIM dd(1 TO 8) AS DOUBLE, ss(1 TO 8) AS STRING, lw(1 TO 8) AS LONG, bt(1 TO 8) AS _BIT, fl(1 TO 8) AS _FLOAT
TYPE tp
    v AS SINGLE
END TYPE
TYPE tw
    m(1 TO 8) AS _BYTE
END TYPE
TYPE te
    v AS _BYTE
END TYPE
DIM ut(1 TO 4) AS tp, x AS tw, e(1 TO 8) AS te
h = _SNDOPEN()
h = _SNDOPEN(1)
h = _SNDOPEN(t, "", 1)
h = _SNDOPEN
h = _SNDPLAYING
d = _SNDLEN(t)
h = _SNDCOPY
d = _SNDGETPOS()
l = _SNDRATE(1)
l = _SNDRATE()
r = _SNDOPENRAW(1)
d = _SNDRAWLEN()
d = _SNDRAWLEN(1, 2)
h = _SNDNEW()
h = _SNDNEW(1, 2, 3, 4, 5)
m = _MEMSOUND
_SNDPLAY
_SNDPLAY h, 1
_SNDPLAY t
l = _SNDPLAY(h)
_SNDPLAY = 1
_SNDPLAY: PRINT 1
_SNDVOL h
_SNDVOL (h, 1)
_SNDBAL h
_SNDBAL h, 1, 2, 3, 4, 5
_SNDLIMIT h
_SNDSETPOS h
_SNDCLOSE
_SNDPLAYCOPY h, 1, 2, 3, 4, 5
_SNDPLAYFILE
_SNDPLAYFILE 1
_SNDPLAYFILE t, 1, 1, 1
_SNDRAW
_SNDRAW t
_SNDRAW 0, 0, r, 1
_SNDRAWDONE r, 1
_MIDISOUNDBANK
_MIDISOUNDBANK 1
_MIDISOUNDBANK t, t, t
CALL _SNDPLAY h
CALL _SNDRATE
CALL _SNDRAWDONE()
_WAVE 0
_WAVE 0, w(), 1, 2
_WAVE 0, iw()
_WAVE 0, uw()
_WAVE 0, b()
_WAVE 0, bt()
_WAVE 0, ss()
_WAVE 0, bv
_WAVE 0, 5
_WAVE 0, x.m()
_WAVE 0, e(1).v
CALL _WAVE(0, 5)
_SNDRAWBATCH
_SNDRAWBATCH b(), 1, 2, 3, 4
_SNDRAWBATCH dd()
_SNDRAWBATCH fl()
_SNDRAWBATCH lw()
_SNDRAWBATCH w()
_SNDRAWBATCH ut()
_SNDRAWBATCH sv
$MIDISOUNDFONT:DEFAULT
$midisoundfont:default
$MIDISOUNDFONT: "a.sf2"
