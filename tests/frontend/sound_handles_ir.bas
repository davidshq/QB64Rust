' TEST: ir
$CONSOLE:ONLY
' The sound handle statements in the IR (m3-sound-handles task 2.1, design D1 and D2): every form the old compiler
' accepts (verification\v39_x01_accepted, x55, x57-x60, x71, x72; each call compared by tests\callsite). Every
' optional slot left out in turn, a numeric type of each kind in a slot, CALL with parentheses, the name and a colon.
' The arrays of _WAVE (_BYTE) and _SNDRAWBATCH (SINGLE) are whole, by name alone, from an element, of two
' dimensions, with a suffix, or an array parameter. The settings line names the audio part of the runtime
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
DIM t AS STRING, fs AS STRING * 4, h AS LONG, r AS LONG
DIM w(1 TO 64) AS _BYTE, b(1 TO 16) AS SINGLE, w2(3, 3) AS _BYTE, z%%(1 TO 8), z!(1 TO 8)
_SNDCLOSE h
_SNDCLOSE i: _SNDCLOSE q: _SNDCLOSE s: _SNDCLOSE d: _SNDCLOSE f: _SNDCLOSE u
_SNDPLAY h
_SNDPLAY s
_SNDPAUSE d
_SNDSTOP q
_SNDLOOP u
_SNDVOL h, 0
_SNDVOL i, d
_SNDVOL q, f
_SNDSETPOS h, .25
_SNDSETPOS d, i
_SNDLIMIT h, 1
_SNDLIMIT s, u
_SNDBAL h, 1
_SNDBAL h, 1, 2
_SNDBAL h, 1, 2, 3
_SNDBAL h, 1, 2, 3, 1
_SNDBAL h, , 2
_SNDBAL h, , , 3
_SNDBAL h, , , , 1
_SNDBAL i, s, d, q, f
_SNDPLAYCOPY h
_SNDPLAYCOPY h, 0
_SNDPLAYCOPY h, 0, 1, 2, 3
_SNDPLAYCOPY h, , 1
_SNDPLAYCOPY h, , , , 3
_SNDPLAYCOPY d, i, q, u, f
_SNDPLAYFILE t
_SNDPLAYFILE t, 1
_SNDPLAYFILE t, , 0
_SNDPLAYFILE fs, s, d
_SNDRAW 0
_SNDRAW 0, 0
_SNDRAW 0, 0, r
_SNDRAW 0, , r
_SNDRAW s, d, q
_SNDRAWBATCH b()
_SNDRAWBATCH b
_SNDRAWBATCH b(3), 2, r, 4
_SNDRAWBATCH b(), , r
_SNDRAWBATCH b(), , , 8
_SNDRAWBATCH b(), s, d, f
_SNDRAWBATCH z!()
_SNDRAWDONE
_SNDRAWDONE r
_SNDRAWDONE s
_SNDRAWDONE: PRINT 1
_MIDISOUNDBANK t
_MIDISOUNDBANK ""
_MIDISOUNDBANK t, "memory, sf2"
_MIDISOUNDBANK fs, fs
_WAVE 0, w()
_WAVE 0, w
_WAVE 0, w(1), 32
_WAVE i, w(), s
_WAVE d, w(5), q
_WAVE u, w2()
_WAVE 1, w2(1, 1), 4
_WAVE 0, z%%()
CALL _SNDPLAY(h)
CALL _SNDBAL(h, , 2)
CALL _SNDPLAYFILE(t, , 0)
CALL _SNDRAWBATCH(b(), 1, r)
CALL _SNDRAWDONE
CALL _SNDRAWDONE(r)
CALL _WAVE(0, w())
_sndplay h

SUB p (a() AS _BYTE, c() AS SINGLE)
    _WAVE 0, a()
    _WAVE 0, a(1), 4
    _SNDRAWBATCH c()
    _SNDRAWBATCH c(1), 1
END SUB
