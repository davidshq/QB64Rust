$Console:Only
' Runtime comparison: multiple GOSUB to same label
Print "1"
GoSub helper
Print "2"
GoSub helper
Print "3"
End

helper:
Print " helper"
Return
