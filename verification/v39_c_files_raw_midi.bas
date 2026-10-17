$CONSOLE:ONLY
' Verification (m3-sound-handles, task 1.1): a WAV file the program writes, _SNDPLAYFILE, raw sound, _WAVE, and
' MIDI from memory with the default sound bank. Every sound is silence or is played at volume 0.
DECLARE FUNCTION wav$ (frames AS LONG, rate AS LONG)
DECLARE FUNCTION midi$ ()
ON ERROR GOTO handler
DIM h AS LONG, r AS LONG, k AS LONG, i AS LONG, n AS LONG, d AS DOUBLE

' 1. A WAV file written by the program
OPEN "v39c.wav" FOR OUTPUT AS #1: PRINT #1, wav$(4000, 8000);: CLOSE #1
h = _SNDOPEN("v39c.wav")
PRINT "1."; h; _SNDLEN(h); _SNDPLAYING(h); _SNDPAUSED(h);
k = _SNDOPEN("v39c.wav", "stream")
PRINT k; _SNDLEN(k);
k = _SNDOPEN("V39C.WAV", "noasync, nodecode")
PRINT k; _SNDLEN(k);
k = _SNDOPEN("v39c.wav", "memory")
PRINT k; _SNDLEN(k)

' 2. _SNDPLAYFILE at volume 0: which handle the next sound gets
k = _SNDOPENRAW: PRINT "2."; k;
_SNDPLAYFILE "v39c.wav", , 0
k = _SNDOPENRAW: PRINT k;
_SNDPLAYFILE "v39c.wav", 1, 0
_SNDPLAYFILE "no such file.wav", , 0
_SNDPLAYFILE "", , 0
k = _SNDOPENRAW: PRINT k; ERR

' 3. _SNDPLAYCOPY at volume 0
_SNDPLAYCOPY h, 0
_SNDPLAYCOPY h, 0, 1
_SNDPLAYCOPY h, 0, 1, 2, 3
k = _SNDOPENRAW: PRINT "3."; k; _SNDPLAYING(h); ERR

' 4. A raw stream at volume 0: one second of samples of 0
r = _SNDOPENRAW
_SNDVOL r, 0
n = _SNDRATE
PRINT "4."; n > 0; _SNDRAWLEN(r);
FOR i = 1 TO n: _SNDRAW 0, 0, r: NEXT
d = _SNDRAWLEN(r)
PRINT d > 0; d <= 1;
DIM b(1 TO 1024) AS SINGLE
_SNDRAWBATCH b(), 1, r
_SNDRAWBATCH b(), 2, r, 100
_SNDRAWBATCH b(), 3, r
d = _SNDRAWLEN(r)
PRINT d > 0; d <= 1.1;
_SNDBAL r, 1
_SNDRAWDONE r
PRINT _SNDRAWLEN(r) > 0; ERR

' 5. The default stream: samples of 0
PRINT "5."; _SNDRAWLEN;
_SNDRAW 0
FOR i = 1 TO n \ 2: _SNDRAW 0, 0: NEXT
d = _SNDRAWLEN
PRINT d > 0; d <= 1;
_SNDRAWBATCH b()
_SNDRAWDONE
PRINT ERR

' 6. _WAVE and SOUND with waveform 10 at volume 0
DIM w(1 TO 64) AS _BYTE
_WAVE 0, w()
_WAVE 1, w(), 32
SOUND 440, 1, 0, , 10
PRINT "6."; ERR;
DIM w1(1 TO 1) AS _BYTE
_WAVE 0, w1()
PRINT ERR;
_WAVE 0, w(), 1
PRINT ERR;
_WAVE 4, w()
PRINT ERR;
_WAVE 3, w(63)
PRINT ERR

' 7. MIDI from memory with the default bank, a bank that is not there, and the empty name
DIM m$
m$ = midi$
h = _SNDOPEN(m$, "memory")
PRINT "7."; h > 0; _SNDLEN(h);
_MIDISOUNDBANK "no such bank.sf2"
k = _SNDOPEN(m$, "memory")
PRINT k > 0; _SNDLEN(k); ERR;
_MIDISOUNDBANK "no such bank.sf2", "memory, sf2"
k = _SNDOPEN(m$, "memory")
PRINT k > 0; _SNDLEN(k); ERR;
_MIDISOUNDBANK ""
k = _SNDOPEN(m$, "memory")
PRINT k > 0; _SNDLEN(k); ERR
_SNDVOL k, 0: _SNDPLAY k
PRINT "8."; _SNDPLAYING(k); _SNDPAUSED(k);
_SNDSTOP k
PRINT _SNDPLAYING(k); _SNDPAUSED(k); _SNDGETPOS(k)

KILL "v39c.wav"
SYSTEM

handler:
PRINT "error"; ERR; "line"; _ERRORLINE;
RESUME NEXT

FUNCTION wav$ (frames AS LONG, rate AS LONG)
    DIM d AS LONG
    d = frames * 2
    wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(1) + MKL$(rate) + MKL$(rate * 2) + MKI$(2) + MKI$(16) + "data" + MKL$(d) + STRING$(d, 0)
END FUNCTION

' A MIDI file of one track: one note of two quarters at the default tempo (120), one second.
FUNCTION midi$
    DIM t AS STRING
    t = CHR$(0) + CHR$(&H90) + CHR$(60) + CHR$(64)
    t = t + CHR$(&H81) + CHR$(&H40) + CHR$(&H80) + CHR$(60) + CHR$(0)
    t = t + CHR$(0) + CHR$(&HFF) + CHR$(&H2F) + CHR$(0)
    midi$ = "MThd" + CHR$(0) + CHR$(0) + CHR$(0) + CHR$(6) + CHR$(0) + CHR$(0) + CHR$(0) + CHR$(1) + CHR$(0) + CHR$(96) + "MTrk" + CHR$(0) + CHR$(0) + CHR$(0) + CHR$(LEN(t)) + t
END FUNCTION
