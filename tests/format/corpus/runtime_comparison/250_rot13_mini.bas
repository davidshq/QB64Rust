$Console:Only
' Minimal ROT13 on "ab" (inspired by qbasic_testcases/misc/rot13.bas)
z$ = "ab"
For n = 1 To Len(z$)
    a$ = Right$(Left$(z$, n), 1)
    c = Asc(UCase$(a$))
    If c > 64 And c < 91 Then b$ = Chr$((c - 65 + 13) Mod 26 + 65) Else b$ = a$
    Print b$;
Next n
Print
End
