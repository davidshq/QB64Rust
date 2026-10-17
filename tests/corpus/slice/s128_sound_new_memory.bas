$CONSOLE:ONLY
' Slice (m3-sound-handles, task 3.1; from verification\v39_b_new_mem): _SNDNEW and _MEMSOUND, the block of a new
' sound and of one decoded from memory. Every sound is silence, played only at volume 0. A decoded sound's samples
' are at the device's rate, so of its block only what does not depend on the rate is printed.
DECLARE FUNCTION wav$ (frames AS LONG, rate AS LONG, bits AS LONG, channels AS LONG)
ON ERROR GOTO handler
DIM n AS LONG, c AS LONG, h AS LONG, k AS LONG, m AS _MEM, mc AS _MEM

n = _SNDNEW(800, 1, 16, 8000)
m = _MEMSOUND(n)
PRINT "a new sound:"; n; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.SOUND = n; m.IMAGE; _MEMEXISTS(m); _SNDLEN(n)
PRINT "not playing, paused, at 0:"; _SNDPLAYING(n); _SNDPAUSED(n); _SNDGETPOS(n)

PRINT "its samples are 0:"; _MEMGET(m, m.OFFSET, INTEGER); _MEMGET(m, m.OFFSET + 1598, INTEGER)
_MEMPUT m, m.OFFSET + 2, 1234 AS INTEGER
c = _SNDCOPY(n)
mc = _MEMSOUND(c)
PRINT "a copy has the sample written:"; c; mc.SIZE; mc.TYPE; mc.ELEMENTSIZE; _MEMGET(mc, mc.OFFSET + 2, INTEGER); _SNDLEN(c)
_MEMPUT m, m.OFFSET + 2, 0 AS INTEGER

PRINT "channels, bits and rates:"
k = _SNDNEW(800, 2, 16, 8000): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE; _SNDLEN(k)
k = _SNDNEW(800, 1, 8, 8000): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE; _MEMGET(m, m.OFFSET, _UNSIGNED _BYTE)
k = _SNDNEW(800, 2, 32, 8000): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE; _SNDLEN(k)
k = _SNDNEW(800, 1, 32): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE
k = _SNDNEW(800): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE
k = _SNDNEW(800, , 16, 4000): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE; _SNDLEN(k)
k = _SNDNEW(4000, 1, 16, 8000): PRINT _SNDLEN(k); _SNDNEW(1, 1, 16, 8000) > 0

PRINT "arguments refused:";
k = _SNDNEW(0, 2, 32): m = _MEMSOUND(k): PRINT k; m.SIZE; _MEMEXISTS(m);
k = _SNDNEW(1024, 0, 32): m = _MEMSOUND(k): PRINT k; m.SIZE;
k = _SNDNEW(1024, 3, 32): PRINT k;
k = _SNDNEW(1024, 1, 0): PRINT k;
k = _SNDNEW(1024, 1, 24): PRINT k;
k = _SNDNEW(1024, 1, 16, 0): PRINT k;
k = _SNDNEW(-1024, 1, 16): PRINT k; ERR

h = _SNDOPEN(wav$(4000, 8000, 16, 1), "memory")
m = _MEMSOUND(h)
PRINT "decoded, 16 bits mono:"; m.SIZE = _SNDRATE; m.TYPE; m.ELEMENTSIZE; m.SOUND = h; _MEMGET(m, m.OFFSET + 100, INTEGER); _SNDLEN(h)
h = _SNDOPEN(wav$(4000, 8000, 8, 2), "memory")
m = _MEMSOUND(h)
PRINT "decoded, 8 bits stereo:"; m.SIZE = _SNDRATE; m.TYPE; m.ELEMENTSIZE; _SNDLEN(h)
m = _MEMSOUND(h, 0): PRINT "channel 0, 1, 2:"; m.SIZE = _SNDRATE;
m = _MEMSOUND(h, 1): PRINT m.SIZE = _SNDRATE;
m = _MEMSOUND(h, 2): PRINT m.SIZE; _MEMEXISTS(m)

h = _SNDOPEN(wav$(4000, 8000, 16, 1), "memory, stream")
m = _MEMSOUND(h): PRINT "a stream has no block:"; h > 0; m.SIZE; _SNDLEN(h)
h = _SNDOPEN(wav$(4000, 8000, 16, 1), "memory, nodecode")
m = _MEMSOUND(h): PRINT "nor a sound not decoded:"; h > 0; m.SIZE; _SNDLEN(h)

_SNDVOL n, 0
_SNDPLAY n
m = _MEMSOUND(n)
PRINT "a new sound playing at volume 0:"; _SNDPLAYING(n); _MEMEXISTS(m)
_SNDSTOP n
_SNDCLOSE n
PRINT "closed:"; _SNDLEN(n)
m = _MEMSOUND(0): PRINT "the block of handle 0:"; m.SIZE; ERR
SYSTEM

handler:
PRINT "error"; ERR
RESUME NEXT

FUNCTION wav$ (frames AS LONG, rate AS LONG, bits AS LONG, channels AS LONG)
    DIM d AS LONG, b AS LONG, z AS STRING
    b = (bits \ 8) * channels
    d = frames * b
    IF bits = 8 THEN z = STRING$(d, 128) ELSE z = STRING$(d, 0)
    wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(channels) + MKL$(rate) + MKL$(rate * b) + MKI$(b) + MKI$(bits) + "data" + MKL$(d) + z
END FUNCTION
