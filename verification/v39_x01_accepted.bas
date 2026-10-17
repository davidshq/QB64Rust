$CONSOLE:ONLY
' Verification (m3-sound-handles, task 1.1): forms of the sound handle built-ins the old compiler accepts, as
' code that is not reached.
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
DIM t AS STRING, fs AS STRING * 4, h AS LONG, r AS LONG, m AS _MEM
DIM w(1 TO 64) AS _BYTE, b(1 TO 16) AS SINGLE, w2(3, 3) AS _BYTE
PRINT "ok"
SYSTEM
' f01 sndopen
h = _SNDOPEN(t)
h = _SNDOPEN("a.wav")
h = _SNDOPEN(t, "memory")
h = _SNDOPEN(fs, fs)
h = _SNDOPEN(t + "x", "stream, noasync")
PRINT _SNDOPEN(t); _SNDOPEN(t) + 1; HEX$(_SNDOPEN(t))
' f02 sndcopy
h = _SNDCOPY(h)
h = _SNDCOPY(i): h = _SNDCOPY(q): h = _SNDCOPY(s): h = _SNDCOPY(d): h = _SNDCOPY(f): h = _SNDCOPY(u)
PRINT _SNDCOPY(1) + 1; HEX$(_SNDCOPY(1))
' f03 sndlen sndgetpos
d = _SNDLEN(h)
PRINT _SNDLEN(i); _SNDLEN(q); _SNDLEN(s); _SNDLEN(d); _SNDLEN(f); _SNDLEN(u)
PRINT _SNDLEN(1) + 1; _SNDLEN(1) / 3
d = _SNDGETPOS(h)
PRINT _SNDGETPOS(i); _SNDGETPOS(q); _SNDGETPOS(s); _SNDGETPOS(d); _SNDGETPOS(f); _SNDGETPOS(u)
' f04 sndplaying sndpaused
l = _SNDPLAYING(h): l = _SNDPAUSED(h)
PRINT _SNDPLAYING(i); _SNDPAUSED(q); _SNDPLAYING(s); _SNDPAUSED(d); _SNDPLAYING(f); _SNDPAUSED(u)
PRINT HEX$(_SNDPLAYING(1)); _SNDPAUSED(1) + 1
' f05 sndrate sndopenraw sndrawlen
l = _SNDRATE
PRINT _SNDRATE; _SNDRATE + 1; HEX$(_SNDRATE)
r = _SNDOPENRAW
PRINT _SNDOPENRAW; HEX$(_SNDOPENRAW)
d = _SNDRAWLEN
d = _SNDRAWLEN(r)
PRINT _SNDRAWLEN; _SNDRAWLEN(i); _SNDRAWLEN(q); _SNDRAWLEN(s); _SNDRAWLEN(d); _SNDRAWLEN(u); _SNDRAWLEN + 1
' f06 sndnew
h = _SNDNEW(800)
h = _SNDNEW(800, 1)
h = _SNDNEW(800, 1, 16)
h = _SNDNEW(800, 1, 16, 8000)
h = _SNDNEW(800, , 16)
h = _SNDNEW(800, , , 8000)
h = _SNDNEW(800, 2, , 8000)
h = _SNDNEW(q, s, d, f)
h = _SNDNEW(i, u, l, i)
PRINT _SNDNEW(1) + 1; HEX$(_SNDNEW(1))
' f07 memsound
m = _MEMSOUND(h)
m = _MEMSOUND(h, 0)
m = _MEMSOUND(s, d)
' s01 sndclose sndplay sndpause sndstop sndloop
_SNDCLOSE h
_SNDCLOSE i: _SNDCLOSE q: _SNDCLOSE s: _SNDCLOSE d: _SNDCLOSE f: _SNDCLOSE u
_SNDPLAY h
_SNDPLAY s
_SNDPAUSE h
_SNDPAUSE d
_SNDSTOP h
_SNDSTOP q
_SNDLOOP h
_SNDLOOP u
' s02 sndvol sndsetpos sndlimit
_SNDVOL h, 0
_SNDVOL i, d
_SNDVOL q, f
_SNDSETPOS h, .25
_SNDSETPOS q, s
_SNDSETPOS d, i
_SNDLIMIT h, 1
_SNDLIMIT s, u
' s03 sndbal
_SNDBAL h, 1
_SNDBAL h, 1, 2
_SNDBAL h, 1, 2, 3
_SNDBAL h, 1, 2, 3, 1
_SNDBAL h, , 2
_SNDBAL h, , , 3
_SNDBAL h, , , , 1
_SNDBAL h, 1, , 3
_SNDBAL i, s, d, q, f
' s04 sndplaycopy
_SNDPLAYCOPY h
_SNDPLAYCOPY h, 0
_SNDPLAYCOPY h, 0, 1
_SNDPLAYCOPY h, 0, 1, 2
_SNDPLAYCOPY h, 0, 1, 2, 3
_SNDPLAYCOPY h, , 1
_SNDPLAYCOPY h, , , , 3
_SNDPLAYCOPY d, i, q, u, f
' s05 sndplayfile
_SNDPLAYFILE t
_SNDPLAYFILE t, 1
_SNDPLAYFILE t, , 0
_SNDPLAYFILE t, 1, 0
_SNDPLAYFILE fs, s, d
' s06 sndraw
_SNDRAW 0
_SNDRAW 0, 0
_SNDRAW 0, 0, r
_SNDRAW 0, , r
_SNDRAW s, d, q
_SNDRAW i, u
' s07 sndrawbatch
_SNDRAWBATCH b()
_SNDRAWBATCH b(1)
_SNDRAWBATCH b(), 1
_SNDRAWBATCH b(), 1, r
_SNDRAWBATCH b(), 1, r, 8
_SNDRAWBATCH b(), , r
_SNDRAWBATCH b(), , , 8
_SNDRAWBATCH b(3), 2, r, 4
_SNDRAWBATCH b(), s, d, f
' s08 sndrawdone
_SNDRAWDONE
_SNDRAWDONE r
_SNDRAWDONE s
' s09 midisoundbank
_MIDISOUNDBANK t
_MIDISOUNDBANK ""
_MIDISOUNDBANK t, "memory, sf2"
_MIDISOUNDBANK fs, fs
' s10 wave
_WAVE 0, w()
_WAVE 0, w(1)
_WAVE 0, w(1), 32
_WAVE 0, w(), 32
_WAVE i, w(), s
_WAVE d, w(5), q
_WAVE u, w2()
_WAVE 1, w2(1, 1), 4
' c01 call forms
CALL _SNDPLAY(h)
CALL _SNDCLOSE(h)
CALL _SNDVOL(h, 0)
CALL _SNDBAL(h, 1)
CALL _SNDBAL(h, , 2)
CALL _SNDPLAYCOPY(h, 0)
CALL _SNDPLAYFILE(t, , 0)
CALL _SNDRAW(0, 0, r)
CALL _SNDRAWBATCH(b(), 1, r)
CALL _SNDRAWDONE
CALL _SNDRAWDONE(r)
CALL _MIDISOUNDBANK(t)
CALL _WAVE(0, w())
CALL _SNDSETPOS(h, 1)
CALL _SNDLIMIT(h, 1)
' n01 lower case
h = _sndopen(t): _sndplay h: d = _sndlen(h)
