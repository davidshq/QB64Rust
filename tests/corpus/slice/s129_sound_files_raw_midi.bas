$CONSOLE:ONLY
' Slice (m3-sound-handles, task 3.1; from verification\v39_c_files_raw_midi): a WAV file the program writes,
' _SNDPLAYFILE and _SNDPLAYCOPY at volume 0, raw sound of samples of 0, _WAVE, and MIDI built in a string with the
' default sound bank (a note played at volume 0). The open of the file just written is tried again until it gives a
' handle, and its KILL until it raises no error: a file being played stays open until libqb's sound loop frees it.
' After the first _SNDPLAYFILE a handle is printed only as h > 0; _SNDRATE and _SNDRAWLEN only against limits.
DECLARE FUNCTION wav$ (frames AS LONG, rate AS LONG)
DECLARE FUNCTION midi$ ()
DECLARE FUNCTION opened& (f$, o$)
DIM SHARED quiet AS INTEGER, lasterr AS INTEGER
ON ERROR GOTO handler
DIM h AS LONG, r AS LONG, k AS LONG, i AS LONG, n AS LONG, d AS DOUBLE, tries AS INTEGER

OPEN "s129.wav" FOR OUTPUT AS #1: PRINT #1, wav$(4000, 8000);: CLOSE #1
h = opened&("s129.wav", "")
PRINT "a WAV file:"; h; _SNDLEN(h); _SNDPLAYING(h); _SNDPAUSED(h)
k = opened&("s129.wav", "stream")
PRINT "streamed:"; k; _SNDLEN(k);
_SNDCLOSE k
k = opened&("S129.WAV", "noasync, nodecode")
PRINT "not decoded:"; k > 0; _SNDLEN(k);
k = _SNDOPEN("s129.wav", "memory")
PRINT "its name taken as data:"; k; _SNDLEN(k)

_SNDPLAYFILE "s129.wav", , 0
_SNDPLAYFILE "s129.wav", 1, 0
_SNDPLAYFILE "no such file.wav", , 0
_SNDPLAYFILE "", , 0
_SNDPLAYCOPY h, 0
_SNDPLAYCOPY h, 0, 1
_SNDPLAYCOPY h, 0, 1, 2, 3
PRINT "played and copied at volume 0:"; _SNDPLAYING(h); ERR

r = _SNDOPENRAW
_SNDVOL r, 0
n = _SNDRATE
PRINT "a raw stream:"; r > 0; "rate over 0:"; n > 0; "nothing queued:"; _SNDRAWLEN(r)
FOR i = 1 TO n: _SNDRAW 0, 0, r: NEXT
d = _SNDRAWLEN(r)
PRINT "one second queued, over 0 and at most 1:"; d > 0; d <= 1
DIM b(1 TO 1024) AS SINGLE
_SNDRAWBATCH b(), 1, r
_SNDRAWBATCH b(), 2, r, 100
_SNDRAWBATCH b(), 3, r
d = _SNDRAWLEN(r)
PRINT "and a batch:"; d > 0; d <= 1.1
_SNDBAL r, 1
_SNDRAWDONE r
PRINT "done:"; ERR

PRINT "the default stream:"; _SNDRAWLEN;
_SNDRAW 0
FOR i = 1 TO n \ 2: _SNDRAW 0, 0: NEXT
d = _SNDRAWLEN
PRINT d > 0; d <= 1;
_SNDRAWBATCH b
_SNDRAWBATCH b(512), 1
_SNDRAWDONE
PRINT ERR

DIM w(1 TO 64) AS _BYTE, w1(1 TO 1) AS _BYTE
_WAVE 0, w()
_WAVE 1, w(), 32
SOUND 440, 1, 0, , 10
PRINT "a custom wave:"; ERR
PRINT "an array of one, a frame count of 1, voice 4, and two frames:"
_WAVE 0, w1()
_WAVE 0, w(), 1
_WAVE 4, w()
_WAVE 3, w(63)
PRINT ERR

DIM m$
m$ = midi$
h = _SNDOPEN(m$, "memory")
PRINT "a MIDI of one second:"; h > 0; _SNDLEN(h)
_MIDISOUNDBANK "no such bank.sf2"
k = _SNDOPEN(m$, "memory")
PRINT "with a bank that is not there:"; k > 0; _SNDLEN(k); ERR
_MIDISOUNDBANK "no such bank.sf2", "memory, sf2"
k = _SNDOPEN(m$, "memory")
PRINT "with a bank of bad data:"; k > 0; _SNDLEN(k); ERR
_MIDISOUNDBANK ""
k = _SNDOPEN(m$, "memory")
PRINT "with the default bank again:"; k > 0; _SNDLEN(k); ERR
_SNDVOL k, 0: _SNDPLAY k
PRINT "played at volume 0:"; _SNDPLAYING(k); _SNDPAUSED(k);
_SNDSTOP k
PRINT _SNDPLAYING(k); _SNDPAUSED(k); _SNDGETPOS(k)

quiet = -1
DO
    lasterr = 0
    KILL "s129.wav"
    IF lasterr = 0 THEN EXIT DO
    tries = tries + 1
    IF tries > 100 THEN PRINT "the file stays open": EXIT DO
    _DELAY .1
LOOP
quiet = 0
PRINT "removed:"; _FILEEXISTS("s129.wav")
SYSTEM

handler:
IF quiet THEN lasterr = ERR ELSE PRINT "error"; ERR
RESUME NEXT

' _SNDOPEN of a file, tried again for up to five seconds: the open of a file just written can fail under load.
FUNCTION opened& (f$, o$)
    DIM t AS INTEGER, x AS LONG
    FOR t = 1 TO 50
        IF LEN(o$) THEN x = _SNDOPEN(f$, o$) ELSE x = _SNDOPEN(f$)
        IF x THEN EXIT FOR
        _DELAY .1
    NEXT
    opened& = x
END FUNCTION

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
