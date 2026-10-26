$Console:Only
' Slice (m3-sound-handles, task 3.1; from verification\v39_c_files_raw_midi): a WAV file the program writes,
' _SNDPLAYFILE and _SNDPLAYCOPY at volume 0, raw sound of samples of 0, _WAVE, and MIDI built in a string with the
' default sound bank (a note played at volume 0). The open of the file just written is tried again until it gives a
' handle, and its KILL until it raises no error: a file being played stays open until libqb's sound loop frees it.
' After the first _SNDPLAYFILE a handle is printed only as h > 0; _SNDRATE and _SNDRAWLEN only against limits.
DECLARE FUNCTION wav$ (frames AS LONG, rate AS LONG)
DECLARE FUNCTION midi$ ()
DECLARE FUNCTION opened& (f$, o$)
Dim Shared quiet As Integer, lasterr As Integer
On Error GoTo handler
Dim h As Long, r As Long, k As Long, i As Long, n As Long, d As Double, tries As Integer

Open "s129.wav" For Output As #1: Print #1, wav$(4000, 8000);: Close #1
h = opened&("s129.wav", "")
Print "a WAV file:"; h; _SndLen(h); _SndPlaying(h); _SndPaused(h)
k = opened&("s129.wav", "stream")
Print "streamed:"; k; _SndLen(k);
_SndClose k
k = opened&("S129.WAV", "noasync, nodecode")
Print "not decoded:"; k > 0; _SndLen(k);
k = _SndOpen("s129.wav", "memory")
Print "its name taken as data:"; k; _SndLen(k)

_SndPlayFile "s129.wav", , 0
_SndPlayFile "s129.wav", 1, 0
_SndPlayFile "no such file.wav", , 0
_SndPlayFile "", , 0
_SndPlayCopy h, 0
_SndPlayCopy h, 0, 1
_SndPlayCopy h, 0, 1, 2, 3
Print "played and copied at volume 0:"; _SndPlaying(h); Err

r = _SndOpenRaw
_SndVol r, 0
n = _SndRate
Print "a raw stream:"; r > 0; "rate over 0:"; n > 0; "nothing queued:"; _SndRawLen(r)
For i = 1 To n: _SndRaw 0, 0, r: Next
d = _SndRawLen(r)
Print "one second queued, over 0 and at most 1:"; d > 0; d <= 1
Dim b(1 To 1024) As Single
_SndRawBatch b(), 1, r
_SndRawBatch b(), 2, r, 100
_SndRawBatch b(), 3, r
d = _SndRawLen(r)
Print "and a batch:"; d > 0; d <= 1.1
_SndBal r, 1
_SndRawDone r
Print "done:"; Err

Print "the default stream:"; _SndRawLen;
_SndRaw 0
For i = 1 To n \ 2: _SndRaw 0, 0: Next
d = _SndRawLen
Print d > 0; d <= 1;
_SndRawBatch b()
_SndRawBatch b(512), 1
_SndRawDone
Print Err

Dim w(1 To 64) As _Byte, w1(1 To 1) As _Byte
_Wave 0, w()
_Wave 1, w(), 32
Sound 440, 1, 0, , 10
Print "a custom wave:"; Err
Print "an array of one, a frame count of 1, voice 4, and two frames:"
_Wave 0, w1()
_Wave 0, w(), 1
_Wave 4, w()
_Wave 3, w(63)
Print Err

Dim m$
m$ = midi$
h = _SndOpen(m$, "memory")
Print "a MIDI of one second:"; h > 0; _SndLen(h)
_MIDISoundBank "no such bank.sf2"
k = _SndOpen(m$, "memory")
Print "with a bank that is not there:"; k > 0; _SndLen(k); Err
_MIDISoundBank "no such bank.sf2", "memory, sf2"
k = _SndOpen(m$, "memory")
Print "with a bank of bad data:"; k > 0; _SndLen(k); Err
_MIDISoundBank ""
k = _SndOpen(m$, "memory")
Print "with the default bank again:"; k > 0; _SndLen(k); Err
_SndVol k, 0: _SndPlay k
Print "played at volume 0:"; _SndPlaying(k); _SndPaused(k);
_SndStop k
Print _SndPlaying(k); _SndPaused(k); _SndGetPos(k)

quiet = -1
Do
    lasterr = 0
    Kill "s129.wav"
    If lasterr = 0 Then Exit Do
    tries = tries + 1
    If tries > 100 Then Print "the file stays open": Exit Do
    _Delay .1
Loop
quiet = 0
Print "removed:"; _FileExists("s129.wav")
System

handler:
If quiet Then lasterr = Err Else Print "error"; Err
Resume Next

' _SNDOPEN of a file, tried again for up to five seconds: the open of a file just written can fail under load.
Function opened& (f$, o$)
    Dim t As Integer, x As Long
    For t = 1 To 50
        If Len(o$) Then x = _SndOpen(f$, o$) Else x = _SndOpen(f$)
        If x Then Exit For
        _Delay .1
    Next
    opened& = x
End Function

Function wav$ (frames As Long, rate As Long)
    Dim d As Long
    d = frames * 2
    wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(1) + MKL$(rate) + MKL$(rate * 2) + MKI$(2) + MKI$(16) + "data" + MKL$(d) + String$(d, 0)
End Function

' A MIDI file of one track: one note of two quarters at the default tempo (120), one second.
Function midi$
    Dim t As String
    t = Chr$(0) + Chr$(&H90) + Chr$(60) + Chr$(64)
    t = t + Chr$(&H81) + Chr$(&H40) + Chr$(&H80) + Chr$(60) + Chr$(0)
    t = t + Chr$(0) + Chr$(&HFF) + Chr$(&H2F) + Chr$(0)
    midi$ = "MThd" + Chr$(0) + Chr$(0) + Chr$(0) + Chr$(6) + Chr$(0) + Chr$(0) + Chr$(0) + Chr$(1) + Chr$(0) + Chr$(96) + "MTrk" + Chr$(0) + Chr$(0) + Chr$(0) + Chr$(Len(t)) + t
End Function
