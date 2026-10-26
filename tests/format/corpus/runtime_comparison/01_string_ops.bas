$Console:Only
' Runtime comparison 1: String operations
' Concat, LEN, MID$, LEFT$, RIGHT$, comparison
Dim a As String
Dim b As String
a = "Hello"
b = "World"
Print "concat: "; a + " " + b
Print "len a: "; Len(a)
Print "mid 2,3: "; Mid$(a, 2, 3)
Print "left 2: "; Left$(a, 2)
Print "right 2: "; Right$(a, 2)
Print "a<b: "; (a < b)
Print "a=a: "; (a = a)
End
