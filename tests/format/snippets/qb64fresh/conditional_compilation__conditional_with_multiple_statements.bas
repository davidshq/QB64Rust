Dim x As Integer
$If _LINUX Then
    x = 1
    PRINT "Linux"
    x = x + 1
$Else
    x = 9999
    Print "Other"
$End If
