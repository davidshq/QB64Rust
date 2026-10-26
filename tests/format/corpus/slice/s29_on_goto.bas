$Console:Only
' Slice program (m2-core-builtins, D8, D9): ON n GOTO and ON n GOSUB as measured in verification\v20_i_on_goto:
' n = 0, 1, the count, count + 1, 255, 256, 258 (DIVERGENCES-QB45.md Q-001: no error above 255), negative values
' (error 5; the handler prints ERR and resumes next), floats (rounded half to even), an _INTEGER64 beyond LONG (its
' low 32 bits), an error in n (the jump uses the placeholder); ON GOSUB in the main module and in a SUB. No PRINT
' comma.
On Error GoTo h
Dim v(13) As Single
v(0) = 0: v(1) = 1: v(2) = 3: v(3) = 4: v(4) = 255: v(5) = 256: v(6) = 258: v(7) = -1
v(8) = 1.5: v(9) = 2.5: v(10) = 2.4: v(11) = -0.4: v(12) = 65537: v(13) = -65535
For k = 0 To 13
    Print "n ="; v(k); ":";
    On v(k) GOTO t1, t2, t3
    Print " fell through"
    GoTo nxt
    t1: Print " t1": GoTo nxt
    t2: Print " t2": GoTo nxt
    t3: Print " t3"
    nxt:
Next
Dim q As _Integer64
q = 4294967298
Print "_INTEGER64 4294967298:";
On q GOTO u1, u2, u3
Print " fell through"
GoTo un
u1: Print " u1": GoTo un
u2: Print " u2": GoTo un
u3: Print " u3"
un:
Dim i As Integer
i = 2
Print "INTEGER 2:";
On i GOTO w1, w2
Print " fell through"
GoTo wn
w1: Print " w1": GoTo wn
w2: Print " w2"
wn:
Print "DOUBLE 2.5:";
On 2.5# GOTO x1, x2
Print " fell through"
GoTo xn
x1: Print " x1": GoTo xn
x2: Print " x2"
xn:
Print "error in n:";
On Asc("") GOTO e1, e2
Print " after ON ASC(empty)"
GoTo en
e1: Print " e1": GoTo en
e2: Print " e2"
en:
Print "error in n, then + 1:";
On Asc("") + 1 GOTO f1, f2
Print " after ON ASC(empty) + 1"
GoTo fn
f1: Print " f1": GoTo fn
f2: Print " f2"
fn:
On 258 GOTO z1, z2
Print "ON 258 GOTO: next"
GoTo zn
z1: Print "z1": GoTo zn
z2: Print "z2"
zn:
On 1 GOSUB g: Print "back"
On 2 GOSUB g, g2
Print "back again"
On 3 GOSUB g, g2
Print "back from nothing"
On -1 GOSUB g, g2
Print "after ON -1 GOSUB"
For k = 1 To 2
    On k GOSUB g, g2
Next
Print "two GOSUBs from a loop"
sg 1
sg 2
sg 0
System

g:
Print "in g"
Return
g2:
Print "in g2"
Return

h:
Print " [handler"; Err; "]"
Resume Next

Sub sg (n)
    On n GOSUB a1, a2
    Print "back in sg"; n
    Exit Sub
    a1: Print "a1": Return
    a2: Print "a2": Return
End Sub
