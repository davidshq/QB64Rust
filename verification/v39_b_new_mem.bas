$CONSOLE:ONLY
' Verification (m3-sound-handles, task 1.1): _SNDNEW and _MEMSOUND, of a new sound and of one decoded from memory.
' Every sound is silence and is played only at volume 0.
DECLARE FUNCTION wav$ (frames AS LONG, rate AS LONG, bits AS LONG, channels AS LONG)
ON ERROR GOTO handler
DIM n AS LONG, c AS LONG, h AS LONG, m AS _MEM, i AS LONG

' 1. A new sound: 800 frames, mono, 16 bits, 8000 Hz
n = _SNDNEW(800, 1, 16, 8000)
m = _MEMSOUND(n)
PRINT "1."; n; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.SOUND = n; m.IMAGE; _MEMEXISTS(m); _SNDLEN(n)
PRINT _SNDPLAYING(n); _SNDPAUSED(n); _SNDGETPOS(n)

' 2. Its samples are 0; one written is read back, also through a copy
PRINT "2."; _MEMGET(m, m.OFFSET, INTEGER); _MEMGET(m, m.OFFSET + 1598, INTEGER);
_MEMPUT m, m.OFFSET + 2, 1234 AS INTEGER
c = _SNDCOPY(n)
DIM mc AS _MEM
mc = _MEMSOUND(c)
PRINT c; mc.SIZE; mc.TYPE; mc.ELEMENTSIZE; _MEMGET(mc, mc.OFFSET + 2, INTEGER); _SNDLEN(c)
_MEMPUT m, m.OFFSET + 2, 0 AS INTEGER

' 3. Each channel count, bit depth and rate
DIM k AS LONG
k = _SNDNEW(800, 2, 16, 8000): m = _MEMSOUND(k): PRINT "3."; m.SIZE; m.TYPE; m.ELEMENTSIZE; _SNDLEN(k);
k = _SNDNEW(800, 1, 8, 8000): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE; _MEMGET(m, m.OFFSET, _UNSIGNED _BYTE);
k = _SNDNEW(800, 2, 32, 8000): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE; _SNDLEN(k);
k = _SNDNEW(800, 1, 32): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE;
k = _SNDNEW(800): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE;
k = _SNDNEW(800, , 16, 4000): m = _MEMSOUND(k): PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE; _SNDLEN(k)
k = _SNDNEW(4000, 1, 16, 8000): PRINT "  "; _SNDLEN(k); _SNDNEW(1, 1, 16, 8000) > 0

' 4. Arguments refused: each 0, and its block of size 0
PRINT "4.";
k = _SNDNEW(0, 2, 32): m = _MEMSOUND(k): PRINT k; m.SIZE; _MEMEXISTS(m);
k = _SNDNEW(1024, 0, 32): m = _MEMSOUND(k): PRINT k; m.SIZE;
k = _SNDNEW(1024, 3, 32): PRINT k;
k = _SNDNEW(1024, 1, 0): PRINT k;
k = _SNDNEW(1024, 1, 24): PRINT k;
k = _SNDNEW(1024, 1, 16, 0): PRINT k;
k = _SNDNEW(-1024, 1, 16): PRINT k; ERR

' 5. _MEMSOUND of a sound decoded from memory: 16 bits mono, 8 bits stereo
h = _SNDOPEN(wav$(4000, 8000, 16, 1), "memory")
m = _MEMSOUND(h)
PRINT "5."; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.SOUND = h; _MEMGET(m, m.OFFSET + 100, INTEGER);
h = _SNDOPEN(wav$(1000, 8000, 8, 2), "memory")
m = _MEMSOUND(h)
PRINT m.SIZE; m.TYPE; m.ELEMENTSIZE; _SNDLEN(h); _SNDRATE
m = _MEMSOUND(h, 0): PRINT "  "; m.SIZE;
m = _MEMSOUND(h, 1): PRINT m.SIZE;
m = _MEMSOUND(h, 2): PRINT m.SIZE; _MEMEXISTS(m)

' 6. A stream or an undecoded sound has no block
h = _SNDOPEN(wav$(4000, 8000, 16, 1), "memory, stream")
m = _MEMSOUND(h): PRINT "6."; h > 0; m.SIZE; _SNDLEN(h);
h = _SNDOPEN(wav$(4000, 8000, 16, 1), "memory, nodecode")
m = _MEMSOUND(h): PRINT h > 0; m.SIZE; _SNDLEN(h)

' 7. A new sound played at volume 0, then closed: its block stops being valid
_SNDVOL n, 0
_SNDPLAY n
m = _MEMSOUND(n)
PRINT "7."; _SNDPLAYING(n); _MEMEXISTS(m);
_SNDSTOP n
_SNDCLOSE n
PRINT _MEMEXISTS(m); _SNDLEN(n)
m = _MEMSOUND(0): PRINT m.SIZE; ERR
SYSTEM

handler:
PRINT "error"; ERR; "line"; _ERRORLINE
RESUME NEXT

FUNCTION wav$ (frames AS LONG, rate AS LONG, bits AS LONG, channels AS LONG)
    DIM d AS LONG, b AS LONG, z AS STRING
    b = (bits \ 8) * channels
    d = frames * b
    IF bits = 8 THEN z = STRING$(d, 128) ELSE z = STRING$(d, 0)
    wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(channels) + MKL$(rate) + MKL$(rate * b) + MKI$(b) + MKI$(bits) + "data" + MKL$(d) + z
END FUNCTION
