' 600 characters and Enter: more than the 15 keys the INKEY$ buffer holds; none may be lost.
Do
    k$ = InKey$
    If Len(k$) = 1 And k$ <> Chr$(13) Then
        n = n + 1
        sum& = sum& + Asc(k$) * (n Mod 7 + 1)
    End If
    _Limit 100
Loop Until k$ = Chr$(13)
Print n; sum&
System
