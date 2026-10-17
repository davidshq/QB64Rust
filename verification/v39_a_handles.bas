$CONSOLE:ONLY
' Verification (m3-sound-handles, task 1.1): sound handles and their state, from WAV data built in a string.
' Every sound is silence (samples of 0) and is played only at volume 0.
DECLARE FUNCTION wav$ (frames AS LONG, rate AS LONG)
ON ERROR GOTO handler
DIM h AS LONG, c AS LONG, k AS LONG, w$, long$

' 1. No file, and the empty name
PRINT "1."; _SNDOPEN("no such file.wav"); _SNDOPEN("")

' 2. A WAV from memory: 4000 frames, mono, 16 bits, 8000 Hz
w$ = wav$(4000, 8000)
h = _SNDOPEN(w$, "memory")
PRINT "2."; h; _SNDLEN(h); _SNDGETPOS(h); _SNDPLAYING(h); _SNDPAUSED(h)

' 3. A copy
c = _SNDCOPY(h)
PRINT "3."; c; _SNDLEN(c); _SNDGETPOS(c); _SNDPLAYING(c); _SNDPAUSED(c)

' 4. Position
_SNDSETPOS h, .25
PRINT "4."; _SNDGETPOS(h); _SNDPLAYING(h); _SNDPAUSED(h);
_SNDSETPOS h, 0.125: PRINT _SNDGETPOS(h);
_SNDSETPOS h, -1: PRINT _SNDGETPOS(h);
_SNDSETPOS h, 5: PRINT _SNDGETPOS(h); _SNDPLAYING(h); _SNDPAUSED(h);
_SNDSTOP h: PRINT _SNDGETPOS(h); _SNDPLAYING(h); _SNDPAUSED(h)

' 5. A handle that is not open: functions
PRINT "5."; _SNDLEN(99); _SNDPLAYING(-1); _SNDCOPY(0); _SNDGETPOS(5); _SNDPAUSED(7); ERR

' 6. A handle that is not open: statements, under the handler
_SNDPLAY 0: _SNDSTOP 99: _SNDVOL -1, 1: _SNDSETPOS 5, 1: _SNDCLOSE 42
_SNDPAUSE 9: _SNDLOOP 9: _SNDBAL 9, 1: _SNDLIMIT 9, 1: _SNDPLAYCOPY 9, 0
PRINT "6."; ERR

' 7. Play, pause, stop and loop on a sound of 10 seconds, at volume 0
long$ = wav$(80000, 8000)
k = _SNDOPEN(long$, "memory")
_SNDVOL k, 0
PRINT "7."; k; _SNDLEN(k); "open"; _SNDPLAYING(k); _SNDPAUSED(k);
_SNDPLAY k: PRINT "play"; _SNDPLAYING(k); _SNDPAUSED(k);
_SNDPAUSE k: PRINT "pause"; _SNDPLAYING(k); _SNDPAUSED(k);
_SNDPLAY k: PRINT "play"; _SNDPLAYING(k); _SNDPAUSED(k);
_SNDSTOP k: PRINT "stop"; _SNDPLAYING(k); _SNDPAUSED(k); _SNDGETPOS(k);
_SNDLOOP k: PRINT "loop"; _SNDPLAYING(k); _SNDPAUSED(k);
_SNDPAUSE k: PRINT "pause"; _SNDPLAYING(k); _SNDPAUSED(k);
_SNDSTOP k: PRINT "stop"; _SNDPLAYING(k); _SNDPAUSED(k)

' 8. Balance and volume change nothing printed
_SNDBAL k, 1: _SNDBAL k, , 2: _SNDBAL k, 1, 2, 3, 1: _SNDVOL k, .5: _SNDVOL k, 0
PRINT "8."; _SNDLEN(k); _SNDGETPOS(k); _SNDPLAYING(k); _SNDPAUSED(k)

' 9. A limit
_SNDLIMIT k, 1
PRINT "9."; _SNDLEN(k); _SNDGETPOS(k); _SNDPLAYING(k); _SNDPAUSED(k);
_SNDLIMIT k, 20: PRINT _SNDLEN(k);
_SNDPLAY k: PRINT _SNDPLAYING(k); _SNDPAUSED(k);
_SNDSTOP k: PRINT _SNDPLAYING(k); _SNDPAUSED(k)

' 10. A copy of a sound with a limit, and of a playing sound
_SNDLIMIT k, 1
c = _SNDCOPY(k)
PRINT "10."; c; _SNDLEN(c); _SNDGETPOS(c); _SNDPLAYING(c); _SNDPAUSED(c);
_SNDSETPOS k, 2: _SNDPLAY k
c = _SNDCOPY(k)
PRINT c; _SNDLEN(c); _SNDPLAYING(c); _SNDPAUSED(c);
_SNDSTOP k: PRINT _SNDPLAYING(k)

' 11. A raw handle
DIM r AS LONG
r = _SNDOPENRAW
PRINT "11."; r; _SNDLEN(r); _SNDGETPOS(r); _SNDPLAYING(r); _SNDPAUSED(r); _SNDCOPY(r)

' 12. CALL forms
CALL _SNDVOL(k, 0)
CALL _SNDPLAY(k)
PRINT "12."; _SNDPLAYING(k);
CALL _SNDSTOP(k)
CALL _SNDSETPOS(k, 3)
PRINT _SNDPLAYING(k); _SNDGETPOS(k)

' 13. Closed: the functions give 0 at once
_SNDCLOSE h
PRINT "13."; _SNDLEN(h); _SNDPLAYING(h); _SNDPAUSED(h); _SNDCOPY(h); ERR
_SNDCLOSE h
PRINT "14."; ERR
SYSTEM

handler:
PRINT "error"; ERR; "line"; _ERRORLINE
RESUME NEXT

FUNCTION wav$ (frames AS LONG, rate AS LONG)
    DIM d AS LONG
    d = frames * 2
    wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(1) + MKL$(rate) + MKL$(rate * 2) + MKI$(2) + MKI$(16) + "data" + MKL$(d) + STRING$(d, 0)
END FUNCTION
