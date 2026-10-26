$Console:Only
' Slice (m3-sound-handles, task 3.1; from verification\v39_b_new_mem): _SNDNEW and _MEMSOUND, the block of a new
' sound and of one decoded from memory. Every sound is silence, played only at volume 0. A decoded sound's samples
' are at the device's rate, so of its block only what does not depend on the rate is printed.
DECLARE FUNCTION wav$ (frames AS LONG, rate AS LONG, bits AS LONG, channels AS LONG)
On Error GoTo handler
Dim n As Long, c As Long, h As Long, k As Long, m As _MEM, mc As _MEM

n = _SndNew(800, 1, 16, 8000)
m = _MemSound(n)
Print "a new sound:"; n; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.SOUND = n; m.IMAGE; _MemExists(m); _SndLen(n)
Print "not playing, paused, at 0:"; _SndPlaying(n); _SndPaused(n); _SndGetPos(n)

Print "its samples are 0:"; _MemGet(m, m.OFFSET, Integer); _MemGet(m, m.OFFSET + 1598, Integer)
_MemPut m, m.OFFSET + 2, 1234 As Integer
c = _SndCopy(n)
mc = _MemSound(c)
Print "a copy has the sample written:"; c; mc.SIZE; mc.TYPE; mc.ELEMENTSIZE; _MemGet(mc, mc.OFFSET + 2, Integer); _SndLen(c)
_MemPut m, m.OFFSET + 2, 0 As Integer

Print "channels, bits and rates:"
k = _SndNew(800, 2, 16, 8000): m = _MemSound(k): Print m.SIZE; m.TYPE; m.ELEMENTSIZE; _SndLen(k)
k = _SndNew(800, 1, 8, 8000): m = _MemSound(k): Print m.SIZE; m.TYPE; m.ELEMENTSIZE; _MemGet(m, m.OFFSET, _Unsigned _Byte)
k = _SndNew(800, 2, 32, 8000): m = _MemSound(k): Print m.SIZE; m.TYPE; m.ELEMENTSIZE; _SndLen(k)
k = _SndNew(800, 1, 32): m = _MemSound(k): Print m.SIZE; m.TYPE; m.ELEMENTSIZE
k = _SndNew(800): m = _MemSound(k): Print m.SIZE; m.TYPE; m.ELEMENTSIZE
k = _SndNew(800, , 16, 4000): m = _MemSound(k): Print m.SIZE; m.TYPE; m.ELEMENTSIZE; _SndLen(k)
k = _SndNew(4000, 1, 16, 8000): Print _SndLen(k); _SndNew(1, 1, 16, 8000) > 0

Print "arguments refused:";
k = _SndNew(0, 2, 32): m = _MemSound(k): Print k; m.SIZE; _MemExists(m);
k = _SndNew(1024, 0, 32): m = _MemSound(k): Print k; m.SIZE;
k = _SndNew(1024, 3, 32): Print k;
k = _SndNew(1024, 1, 0): Print k;
k = _SndNew(1024, 1, 24): Print k;
k = _SndNew(1024, 1, 16, 0): Print k;
k = _SndNew(-1024, 1, 16): Print k; Err

h = _SndOpen(wav$(4000, 8000, 16, 1), "memory")
m = _MemSound(h)
Print "decoded, 16 bits mono:"; m.SIZE = _SndRate; m.TYPE; m.ELEMENTSIZE; m.SOUND = h; _MemGet(m, m.OFFSET + 100, Integer); _SndLen(h)
h = _SndOpen(wav$(4000, 8000, 8, 2), "memory")
m = _MemSound(h)
Print "decoded, 8 bits stereo:"; m.SIZE = _SndRate; m.TYPE; m.ELEMENTSIZE; _SndLen(h)
m = _MemSound(h, 0): Print "channel 0, 1, 2:"; m.SIZE = _SndRate;
m = _MemSound(h, 1): Print m.SIZE = _SndRate;
m = _MemSound(h, 2): Print m.SIZE; _MemExists(m)

h = _SndOpen(wav$(4000, 8000, 16, 1), "memory, stream")
m = _MemSound(h): Print "a stream has no block:"; h > 0; m.SIZE; _SndLen(h)
h = _SndOpen(wav$(4000, 8000, 16, 1), "memory, nodecode")
m = _MemSound(h): Print "nor a sound not decoded:"; h > 0; m.SIZE; _SndLen(h)

_SndVol n, 0
_SndPlay n
m = _MemSound(n)
Print "a new sound playing at volume 0:"; _SndPlaying(n); _MemExists(m)
_SndStop n
_SndClose n
Print "closed:"; _SndLen(n)
m = _MemSound(0): Print "the block of handle 0:"; m.SIZE; Err
System

handler:
Print "error"; Err
Resume Next

Function wav$ (frames As Long, rate As Long, bits As Long, channels As Long)
    Dim d As Long, b As Long, z As String
    b = (bits \ 8) * channels
    d = frames * b
    If bits = 8 Then z = String$(d, 128) Else z = String$(d, 0)
    wav$ = "RIFF" + MKL$(36 + d) + "WAVE" + "fmt " + MKL$(16) + MKI$(1) + MKI$(channels) + MKL$(rate) + MKL$(rate * b) + MKI$(b) + MKI$(bits) + "data" + MKL$(d) + z
End Function
