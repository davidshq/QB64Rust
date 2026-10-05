DIM content AS STRING
content = _READFILE$("input.txt")
_WRITEFILE "output.txt", content + " (modified)"
