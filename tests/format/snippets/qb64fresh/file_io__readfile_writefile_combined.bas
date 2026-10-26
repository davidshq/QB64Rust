Dim content As String
content = _ReadFile$("input.txt")
_WriteFile "output.txt", content + " (modified)"
