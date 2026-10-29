$Console:Only
' Slice (m3-sound-handles, task 3.1; from verification\v39_a_handles): sound handles and their state, from WAV
' data built in a string. Every sound is silence (samples of 0), played only at volume 0. Only what is exact on any
' sound device is printed: lengths and positions of whole half-seconds or quarters, handle numbers only before the
' first handle is closed (libqb frees a closed handle later, in its sound loop).
DECLARE FUNCTION wav$ (frames AS LONG, rate AS LONG)
On Error GoTo handler
Dim h As Long, c As Long, k As Long, r As Long, w$, ten$
Dim i As Integer, d As Double, s As Single

Print "no file, and the empty name:"; _SndOpen("no such file.wav"); _SndOpen("")

' Half a second: 4000 frames, mono, 16 bits, 8000 Hz
w$ = wav$(4000, 8000)
h = _SndOpen(w$, "memory")
Print "opened from memory:"; h; _SndLen(h); _SndGetPos(h); "playing"; _SndPlaying(h); "paused"; _SndPaused(h)
c = _SndCopy(h)
Print "a copy:"; c; _SndLen(c); _SndGetPos(c); _SndPlaying(c); _SndPaused(c)

_SndSetPos h, .25
Print "position .25:"; _SndGetPos(h); _SndPlaying(h); _SndPaused(h)
_SndSetPos h, -1
Print "a negative position is ignored:"; _SndGetPos(h)
_SndSetPos h, 5
Print "one past the end stops the sound and leaves the position:"; _SndGetPos(h); _SndPlaying(h); _SndPaused(h)
_SndStop h
Print "stopped:"; _SndGetPos(h); _SndPlaying(h); _SndPaused(h)

Print "a handle that is not open:"; _SndLen(99); _SndPlaying(-1); _SndCopy(0); _SndGetPos(5); _SndPaused(7); Err
_SndPlay 0: _SndStop 99: _SndVol -1, 1: _SndSetPos 5, 1: _SndClose 42
_SndPause 9: _SndLoop 9: _SndBal 9, 1: _SndLimit 9, 1: _SndPlayCopy 9, 0
Print "no statement raised an error:"; Err

' Ten seconds, played at volume 0
ten$ = wav$(80000, 8000)
k = _SndOpen(ten$, "memory")
_SndVol k, 0
Print "ten seconds:"; k; _SndLen(k); "open"; _SndPlaying(k); _SndPaused(k)
_SndPlay k: Print "play"; _SndPlaying(k); _SndPaused(k)
_SndPause k: Print "pause"; _SndPlaying(k); _SndPaused(k)
_SndPlay k: Print "play"; _SndPlaying(k); _SndPaused(k)
_SndStop k: Print "stop"; _SndPlaying(k); _SndPaused(k); _SndGetPos(k)
_SndLoop k: Print "loop"; _SndPlaying(k); _SndPaused(k)
_SndPause k: Print "pause"; _SndPlaying(k); _SndPaused(k)
_SndStop k: Print "stop"; _SndPlaying(k); _SndPaused(k)

_SndBal k, 1: _SndBal k, , 2: _SndBal k, 1, 2, 3, 1: _SndVol k, .5: _SndVol k, 0
Print "balance and volume change nothing printed:"; _SndLen(k); _SndGetPos(k); _SndPlaying(k); _SndPaused(k)
' A limit is a time on the sound engine's clock (miniaudio's stop time), not a position in the sound, and _SNDLIMIT
' past the sound's length is ignored, so the limit of 1 stays set until _SNDSTOP, _SNDSETPOS or a _SNDPLAY of a playing
' sound. Wait until the clock is past it, so that _SNDPLAY stops at once on every machine (else it plays only on one
' that gets here in a second).
_Delay 1.5
_SndLimit k, 1
Print "a limit changes neither length nor position:"; _SndLen(k); _SndGetPos(k); _SndPlaying(k); _SndPaused(k)
_SndLimit k, 20
_SndPlay k: Print "a limit past the end is ignored, and the earlier one has passed:"; _SndLen(k); _SndPlaying(k); _SndPaused(k)
_SndStop k

_SndLimit k, 1
c = _SndCopy(k)
Print "a copy of a sound with a limit:"; c; _SndLen(c); _SndGetPos(c); _SndPlaying(c); _SndPaused(c)
_SndSetPos k, 2: _SndPlay k
c = _SndCopy(k)
Print "a copy of a playing sound:"; c; _SndLen(c); _SndPlaying(c); _SndPaused(c); "and the sound:"; _SndPlaying(k)
_SndStop k

r = _SndOpenRaw
Print "a raw handle:"; r; _SndLen(r); _SndGetPos(r); _SndPlaying(r); _SndPaused(r); _SndCopy(r)

' Arguments of other types, and CALL
i = k: d = k: s = 0
Call _SndVol(i, s)
Call _SndPlay(d)
Print "CALL:"; _SndPlaying(i);
Call _SndStop(k)
Call _SndSetPos(d, 3)
Print _SndPlaying(k); _SndGetPos(s + k); _SndLen(d)

_SndClose h
Print "closed, the functions give 0 at once:"; _SndLen(h); _SndPlaying(h); _SndPaused(h); _SndCopy(h); Err
_SndClose h
Print "closed again:"; Err
h = _SndOpen(w$, "memory")
Print "opened again:"; h > 0; _SndLen(h)
System

handler:
Print "error"; Err
Resume Next

Function wav$ (frames As Long, rate As Long)
    Dim d As Long
    d = frames * 2
    wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(1) + MKL$(rate) + MKL$(rate * 2) + MKI$(2) + MKI$(16) + "data" + MKL$(d) + String$(d, 0)
End Function
