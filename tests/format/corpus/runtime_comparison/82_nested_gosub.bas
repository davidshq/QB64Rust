$Console:Only
' Runtime comparison: nested GOSUB
Print "main"
GoSub a
Print "back"
End

a:
Print " in a"
GoSub b
Print " after b"
Return

b:
Print "  in b"
Return
