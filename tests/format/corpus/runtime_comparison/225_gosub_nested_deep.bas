$Console:Only
' Runtime comparison: GOSUB 3+ levels
Print "main"
GoSub level1
Print "back main"
End

level1:
Print "L1"
GoSub level2
Return

level2:
Print "L2"
GoSub level3
Return

level3:
Print "L3"
Return
