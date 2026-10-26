' Keys from the keys file: characters, special keys between them, Enter last.
Do
    k$ = InKey$
    If Len(k$) = 1 Then Print Asc(k$);
    If Len(k$) = 2 Then Print "x"; Asc(k$, 2);
    _Limit 200
Loop Until k$ = Chr$(13)
System
