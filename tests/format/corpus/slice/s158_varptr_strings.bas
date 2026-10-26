$Console:Only
' Slice (m4-memory-records, task 2.4): variables in conventional memory: VARPTR, VARSEG, SADD and VARPTR$ of
' numbers, strings, a fixed-length string, a member and elements, with PEEK and POKE through them
' (verification\v56_a1, a4, b1, t01).
Type t
    a As Long
    v As _Byte
End Type
Dim i As Integer, s As String, fx As String * 4, r As t, sa(1 To 2) As String, k As Integer
Print VarPtr(i); VarSeg(i); VarPtr(s); VarPtr(fx); VarPtr(r.v); VarPtr(sa(2))
s = "hello"
Print SAdd(s); Len(s); SAdd(sa(1))
Def Seg = VarSeg(s)
Print Peek(VarPtr(s)); Peek(SAdd(s)); Peek(SAdd(s) + 4)
i = 258: Poke VarPtr(i), 3
fx = "abcd": Poke VarPtr(fx) + 1, 66
Def Seg
Print i; fx
v$ = VarPtr$(i) + VarPtr$(s) + VarPtr$(fx) + VarPtr$(r.v) + VarPtr$(sa(1))
For k = 1 To Len(v$): Print Asc(v$, k);: Next: Print
Swap s, sa(1)
Print VarPtr(s); SAdd(s); "["; s; "]["; sa(1); "]"
Print Hex$(VarPtr(i)); " "; Oct$(VarSeg(fx)); " "; _Bin$(SAdd(sa(1)))
System
