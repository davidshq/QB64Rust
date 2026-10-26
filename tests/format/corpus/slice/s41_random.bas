$Console:Only
' RANDOMIZE, RND and TIMER (spec language/builtin-statements "RANDOMIZE"; language/builtin-functions "Built-ins
' without parentheses"): the sequence without RANDOMIZE; RANDOMIZE n, which depends on what was drawn before, and
' RANDOMIZE USING n, which does not; a seed of every numeric type, a float, a negative one; RND bare, with 1, 0 and
' negative arguments; RND is SINGLE in arithmetic; TIMER bare and with an argument, only in comparisons; a raising
' seed.
On Error GoTo h
Dim b As _Byte, i As Integer, l As Long, q As _Integer64, s As Single, d As Double, f As _Float
Dim uq As _Unsigned _Integer64, x As Double, t As Double
Print "no RANDOMIZE:"; Rnd; Rnd; Rnd
Randomize 5: Print "RANDOMIZE 5:"; Rnd; Rnd; Rnd
Randomize 5: Print "RANDOMIZE 5 again:"; Rnd; Rnd; Rnd
Randomize Using 5: Print "RANDOMIZE USING 5:"; Rnd; Rnd; Rnd
Randomize Using 5: Print "RANDOMIZE USING 5 again:"; Rnd; Rnd; Rnd
Randomize (5): Print "RANDOMIZE (5):"; Rnd
b = 5: Randomize Using b: Print "_BYTE 5:"; Rnd
i = 5: Randomize Using i: Print "INTEGER 5:"; Rnd
l = 5: Randomize Using l: Print "LONG 5:"; Rnd
q = 5: Randomize Using q: Print "_INTEGER64 5:"; Rnd
uq = 5: Randomize Using uq: Print "_UNSIGNED _INTEGER64 5:"; Rnd
s = 5: Randomize Using s: Print "SINGLE 5:"; Rnd
d = 5: Randomize Using d: Print "DOUBLE 5:"; Rnd
f = 5: Randomize Using f: Print "_FLOAT 5:"; Rnd
Randomize Using 5.5: Print "5.5:"; Rnd
Randomize Using -5: Print "-5:"; Rnd
Randomize Using 0: Print "0:"; Rnd
Randomize Using 1E+300: Print "1E+300:"; Rnd
q = 5000000000: Randomize Using q: Print "5000000000:"; Rnd
Randomize Using l + 1: Print "an expression:"; Rnd
Randomize Using 5
Print "RND(1):"; Rnd(1); "RND(0) twice:"; Rnd(0); Rnd(0); "RND:"; Rnd
Print "RND(-1) twice:"; Rnd(-1); Rnd(-1); "then RND:"; Rnd
Print "RND(-2):"; Rnd(-2); "RND(-1.5):"; Rnd(-1.5); "RND(2.5):"; Rnd(2.5)
d = -1: Print "RND(d) with -1:"; Rnd(d): l = -1: Print "RND(l) with -1:"; Rnd(l)
Randomize Using 5
x = Rnd: Print "RND in a DOUBLE:"; x
x = Rnd * 1000000000: Print "RND * 1000000000:"; x
Print "RND + 0#:"; Rnd + 0#
Print "INT(RND * 6) + 1:"; Int(Rnd * 6) + 1
Print "RND in a comparison:"; Rnd < 1; Rnd >= 0; (Rnd) < 1; -Rnd <= 0
t = Timer: Print "TIMER range:"; t >= 0 And t < 86400
Print "TIMER twice in order:"; Timer <= Timer + 1
t = Timer(.001): Print "TIMER(.001) range:"; t >= 0 And t < 86400
Print "TIMER(0)": t = Timer(0): Print t
t = Timer(1): Print "TIMER(1) is whole:"; t = Int(t)
t = Timer(.5): Print "TIMER(.5) is a multiple of a half:"; t * 2 = Int(t * 2)
l = 1: t = Timer(l): Print "TIMER(l) is whole:"; t = Int(t)
Randomize Timer: x = Rnd: Print "RANDOMIZE TIMER:"; x >= 0 And x < 1
Print "raising seed": Randomize Asc(""): Print "after"
Print "in a FUNCTION:"; dice&
System
h:
Print "  error"; Err
Resume Next

Function dice&
    Randomize Using 7
    dice& = Int(Rnd * 6) + 1
End Function
