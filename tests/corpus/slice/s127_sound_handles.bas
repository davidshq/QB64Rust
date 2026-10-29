$CONSOLE:ONLY
' Slice (m3-sound-handles, task 3.1; from verification\v39_a_handles): sound handles and their state, from WAV
' data built in a string. Every sound is silence (samples of 0), played only at volume 0. Only what is exact on any
' sound device is printed: lengths and positions of whole half-seconds or quarters, handle numbers only before the
' first handle is closed (libqb frees a closed handle later, in its sound loop).
DECLARE FUNCTION wav$ (frames AS LONG, rate AS LONG)
ON ERROR GOTO handler
DIM h AS LONG, c AS LONG, k AS LONG, r AS LONG, w$, ten$
DIM i AS INTEGER, d AS DOUBLE, s AS SINGLE

PRINT "no file, and the empty name:"; _SNDOPEN("no such file.wav"); _SNDOPEN("")

' Half a second: 4000 frames, mono, 16 bits, 8000 Hz
w$ = wav$(4000, 8000)
h = _SNDOPEN(w$, "memory")
PRINT "opened from memory:"; h; _SNDLEN(h); _SNDGETPOS(h); "playing"; _SNDPLAYING(h); "paused"; _SNDPAUSED(h)
c = _SNDCOPY(h)
PRINT "a copy:"; c; _SNDLEN(c); _SNDGETPOS(c); _SNDPLAYING(c); _SNDPAUSED(c)

_SNDSETPOS h, .25
PRINT "position .25:"; _SNDGETPOS(h); _SNDPLAYING(h); _SNDPAUSED(h)
_SNDSETPOS h, -1
PRINT "a negative position is ignored:"; _SNDGETPOS(h)
_SNDSETPOS h, 5
PRINT "one past the end stops the sound and leaves the position:"; _SNDGETPOS(h); _SNDPLAYING(h); _SNDPAUSED(h)
_SNDSTOP h
PRINT "stopped:"; _SNDGETPOS(h); _SNDPLAYING(h); _SNDPAUSED(h)

PRINT "a handle that is not open:"; _SNDLEN(99); _SNDPLAYING(-1); _SNDCOPY(0); _SNDGETPOS(5); _SNDPAUSED(7); ERR
_SNDPLAY 0: _SNDSTOP 99: _SNDVOL -1, 1: _SNDSETPOS 5, 1: _SNDCLOSE 42
_SNDPAUSE 9: _SNDLOOP 9: _SNDBAL 9, 1: _SNDLIMIT 9, 1: _SNDPLAYCOPY 9, 0
PRINT "no statement raised an error:"; ERR

' Ten seconds, played at volume 0
ten$ = wav$(80000, 8000)
k = _SNDOPEN(ten$, "memory")
_SNDVOL k, 0
PRINT "ten seconds:"; k; _SNDLEN(k); "open"; _SNDPLAYING(k); _SNDPAUSED(k)
_SNDPLAY k: PRINT "play"; _SNDPLAYING(k); _SNDPAUSED(k)
_SNDPAUSE k: PRINT "pause"; _SNDPLAYING(k); _SNDPAUSED(k)
_SNDPLAY k: PRINT "play"; _SNDPLAYING(k); _SNDPAUSED(k)
_SNDSTOP k: PRINT "stop"; _SNDPLAYING(k); _SNDPAUSED(k); _SNDGETPOS(k)
_SNDLOOP k: PRINT "loop"; _SNDPLAYING(k); _SNDPAUSED(k)
_SNDPAUSE k: PRINT "pause"; _SNDPLAYING(k); _SNDPAUSED(k)
_SNDSTOP k: PRINT "stop"; _SNDPLAYING(k); _SNDPAUSED(k)

_SNDBAL k, 1: _SNDBAL k, , 2: _SNDBAL k, 1, 2, 3, 1: _SNDVOL k, .5: _SNDVOL k, 0
PRINT "balance and volume change nothing printed:"; _SNDLEN(k); _SNDGETPOS(k); _SNDPLAYING(k); _SNDPAUSED(k)
' A limit is a time on the sound engine's clock (miniaudio's stop time), not a position in the sound, and _SNDLIMIT
' past the sound's length is ignored, so the limit of 1 stays set until _SNDSTOP, _SNDSETPOS or a _SNDPLAY of a playing
' sound. Wait until the clock is past it, so that _SNDPLAY stops at once on every machine (else it plays only on one
' that gets here in a second).
_DELAY 1.5
_SNDLIMIT k, 1
PRINT "a limit changes neither length nor position:"; _SNDLEN(k); _SNDGETPOS(k); _SNDPLAYING(k); _SNDPAUSED(k)
_SNDLIMIT k, 20
_SNDPLAY k: PRINT "a limit past the end is ignored, and the earlier one has passed:"; _SNDLEN(k); _SNDPLAYING(k); _SNDPAUSED(k)
_SNDSTOP k

_SNDLIMIT k, 1
c = _SNDCOPY(k)
PRINT "a copy of a sound with a limit:"; c; _SNDLEN(c); _SNDGETPOS(c); _SNDPLAYING(c); _SNDPAUSED(c)
_SNDSETPOS k, 2: _SNDPLAY k
c = _SNDCOPY(k)
PRINT "a copy of a playing sound:"; c; _SNDLEN(c); _SNDPLAYING(c); _SNDPAUSED(c); "and the sound:"; _SNDPLAYING(k)
_SNDSTOP k

r = _SNDOPENRAW
PRINT "a raw handle:"; r; _SNDLEN(r); _SNDGETPOS(r); _SNDPLAYING(r); _SNDPAUSED(r); _SNDCOPY(r)

' Arguments of other types, and CALL
i = k: d = k: s = 0
CALL _SNDVOL(i, s)
CALL _SNDPLAY(d)
PRINT "CALL:"; _SNDPLAYING(i);
CALL _SNDSTOP(k)
CALL _SNDSETPOS(d, 3)
PRINT _SNDPLAYING(k); _SNDGETPOS(s + k); _SNDLEN(d)

_SNDCLOSE h
PRINT "closed, the functions give 0 at once:"; _SNDLEN(h); _SNDPLAYING(h); _SNDPAUSED(h); _SNDCOPY(h); ERR
_SNDCLOSE h
PRINT "closed again:"; ERR
h = _SNDOPEN(w$, "memory")
PRINT "opened again:"; h > 0; _SNDLEN(h)
SYSTEM

handler:
PRINT "error"; ERR
RESUME NEXT

FUNCTION wav$ (frames AS LONG, rate AS LONG)
    DIM d AS LONG
    d = frames * 2
    wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(1) + MKL$(rate) + MKL$(rate * 2) + MKI$(2) + MKI$(16) + "data" + MKL$(d) + STRING$(d, 0)
END FUNCTION
