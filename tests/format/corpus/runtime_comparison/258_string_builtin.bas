$Console:Only
' String built-ins: LEN, LEFT$, RIGHT$, MID$, INSTR
a$ = "abcdef"
Print Len(a$); Left$(a$, 2); Right$(a$, 2); Mid$(a$, 2, 3)
Print InStr(a$, "cd"); InStr(2, a$, "b")
End
