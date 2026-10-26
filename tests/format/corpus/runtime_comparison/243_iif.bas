$Console:Only
' Runtime comparison: _IIF (numeric and string) - minimal subset of QB64pe iif_test
Dim x As Long
x = 1
Print _IIf(x > 0, 10, 20)
Print _IIf(x < 0, 10, 20)
Print _IIf(x > 0, "yes", "no")
Print _IIf(0, 100, 200)
Print _IIf(-1, 100, 200)
End
