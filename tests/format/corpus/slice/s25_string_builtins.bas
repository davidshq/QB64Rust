$Console:Only
' Slice program (m2-core-builtins, D9): the plain string built-ins and their edge values, measured in
' verification\v20_c_string_edges. Errors (ASC, CHR$, _TOSTR$) are trapped: the handler prints ERR and resumes next.
' No PRINT comma.
On Error GoTo h
Dim s As String, r As String, n As Long
s = "abc"
Print Left$("hello", 2); Len("abc")
Print "[" + Left$(s, 0) + "][" + Left$(s, 2) + "][" + Left$(s, 5) + "][" + Left$(s, -1) + "]"
Print "[" + Right$(s, 0) + "][" + Right$(s, 2) + "][" + Right$(s, 5) + "][" + Right$(s, -1) + "]"
Print Mid$("hello", 2); Mid$("hello", 2, 3)
Print "[" + Mid$(s, 0) + "][" + Mid$(s, 0, 2) + "][" + Mid$(s, -1) + "][" + Mid$(s, 3) + "][" + Mid$(s, 4) + "][" + Mid$(s, 5) + "]"
Print "[" + Mid$(s, 2, 0) + "][" + Mid$(s, 2, -1) + "][" + Mid$(s, 2, 9) + "][" + Mid$("", 1) + "]"
' Float arguments to LONG slots: rounded half to even; beyond LONG, the low 32 bits.
Print Left$("abcdef", 2.5); Left$("abcdef", 3.5); " "; Left$("abcdef", 4294967298#); " "; Mid$("abcdef", 1.5, 2.5)
Dim big As _Integer64
big = 4294967299
Print "["; Left$("abcdef", big); "]["; Left$("abcdef", 3E9); "]"
Print Asc("a"); Asc(s, 2); Asc(Chr$(200)); Asc("xyz", 3.4)
n = 7: n = Asc(""): Print "ASC empty:"; n
n = 7: n = Asc(s, 0): Print "ASC 0:"; n
n = 7: n = Asc(s, 4): Print "ASC past end:"; n
n = 7: n = Asc(s, -1): Print "ASC -1:"; n
r = "x": r = Chr$(256): Print "CHR$ 256: ["; r; "]"
Print Chr$(65.5); Chr$(66.5); Chr$(97)
Print "[" + String$(3, "xyz") + "][" + String$(3, 65) + "][" + String$(0, 65) + "][" + String$(-1, 65) + "]"
Print Asc(String$(2, 256)); Asc(String$(2, -1)); Asc(String$(1, 321)); Len(String$(2, 256))
Print "[" + Space$(2) + "][" + Space$(0) + "][" + Space$(-1) + "]"
Dim i As Integer, l As Long, q As _Integer64, f As Single, d As Double, x As _Float
i = -5: l = 70000: q = 9007199254740993: f = 1 / 3: d = 1 / 3: x = 1 / 3
Print "[" + Str$(i) + "][" + Str$(l) + "][" + Str$(q) + "][" + Str$(f) + "][" + Str$(d) + "][" + Str$(x) + "]"
Print "[" + Str$(5) + "][" + Str$(-5) + "][" + Str$(-0) + "][" + Str$(2.5) + "][" + Str$(1D+300) + "][" + Str$(1.5E-07) + "]"
f = 1E+20
Print "[" + _ToStr$(i) + "][" + _ToStr$(f) + "][" + _ToStr$(d) + "][" + _ToStr$(1 / 3) + "]"
Print "[" + _ToStr$(d, 3) + "][" + _ToStr$(1 / 3, 5) + "][" + _ToStr$(l, 2) + "][" + _ToStr$(2.5, 0) + "]"
r = "x": r = _ToStr$(2.5, -1): Print "_TOSTR$ -1: ["; r; "]"
r = Chr$(9) + " " + Chr$(0) + "a" + Chr$(0) + " " + Chr$(9)
Print Len(LTrim$(r)); Len(RTrim$(r)); Len(_Trim$(r))
r = "  a b  "
Print "[" + LTrim$(r) + "][" + RTrim$(r) + "][" + _Trim$(r) + "][" + _Trim$("") + "][" + LTrim$("   ") + "]"
r = "aZ" + Chr$(130) + Chr$(154) + Chr$(228) + "~"
Print UCase$("Hello, World 1"); " "; LCase$("Hello, World 1")
Print Asc(UCase$(r), 1); Asc(UCase$(r), 2); Asc(UCase$(r), 3); Asc(LCase$(r), 2); Asc(LCase$(r), 4); Asc(LCase$(r), 6)
Print InStr("abcabc", "c"); InStr(4, "abcabc", "c"); InStr("abc", ""); InStr(0, "abc", "b"); InStr(2.5, "abcabc", "b")
' Nested and in arithmetic.
Print Len(Left$(s + s, 4) + Mid$(s, 2)) * 2; Asc(Right$(UCase$(s), 1)) + 1; Left$(Str$(Len(s)), 2)
System

h:
Print "[handler"; Err; "]"
Resume Next
