$Console:Only
' Runtime comparison 4: GOSUB/RETURN
Print "main 1"
GoSub sub1
Print "main 2"
GoSub sub1
Print "main 3"
End

sub1:
Print " in sub1"
Return
