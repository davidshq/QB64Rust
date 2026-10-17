' TEST: typed
$CONSOLE:ONLY
' The sound handle functions (m3-sound-handles task 2.1, design D1 and D4; verification\v39_x01_accepted): LONG
' handles and states, DOUBLE lengths and positions, each argument of any numeric type converted to its LONG,
' _UNSIGNED LONG or STRING slot. _SNDRATE and _SNDOPENRAW are called bare only, _SNDRAWLEN bare or with its handle;
' _SNDOPEN's and _SNDNEW's optional slots left out in turn; _MEMSOUND's block stored into a _MEM
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
DIM t AS STRING, fs AS STRING * 4, h AS LONG, r AS LONG, m AS _MEM
h = _SNDOPEN(t)
h = _SNDOPEN(t, "memory")
h = _SNDOPEN(fs, fs)
h = _SNDOPEN(t + "x", "stream, noasync")
PRINT _SNDOPEN(t); _SNDOPEN(t) + 1; HEX$(_SNDOPEN(t))
h = _SNDCOPY(h): h = _SNDCOPY(i): h = _SNDCOPY(q): h = _SNDCOPY(s): h = _SNDCOPY(d): h = _SNDCOPY(f): h = _SNDCOPY(u)
d = _SNDLEN(h)
PRINT _SNDLEN(i); _SNDLEN(q); _SNDLEN(s); _SNDLEN(f); _SNDLEN(u); _SNDLEN(1) + 1; _SNDLEN(1) / 3
d = _SNDGETPOS(h)
PRINT _SNDGETPOS(i); _SNDGETPOS(s); _SNDGETPOS(d); _SNDGETPOS(u)
l = _SNDPLAYING(h): l = _SNDPAUSED(h)
PRINT _SNDPLAYING(i); _SNDPAUSED(q); _SNDPLAYING(s); _SNDPAUSED(d); _SNDPLAYING(f); _SNDPAUSED(u)
PRINT HEX$(_SNDPLAYING(1)); _SNDPAUSED(1) + 1
l = _SNDRATE
PRINT _SNDRATE; _SNDRATE + 1; HEX$(_SNDRATE)
r = _SNDOPENRAW
PRINT _SNDOPENRAW; HEX$(_SNDOPENRAW)
d = _SNDRAWLEN
d = _SNDRAWLEN(r)
PRINT _SNDRAWLEN(i); _SNDRAWLEN(q); _SNDRAWLEN(s); _SNDRAWLEN(d); _SNDRAWLEN(u); _SNDRAWLEN + 1
h = _SNDNEW(800)
h = _SNDNEW(800, 1, 16, 8000)
h = _SNDNEW(800, , 16)
h = _SNDNEW(800, , , 8000)
h = _SNDNEW(q, s, d, f)
h = _SNDNEW(i, u, l, i)
PRINT _SNDNEW(1) + 1; HEX$(_SNDNEW(1))
m = _MEMSOUND(h)
m = _MEMSOUND(h, 0)
m = _MEMSOUND(s, d)
h = _sndopen(t): d = _sndlen(h)
