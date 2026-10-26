Option _Explicit
$Console:Only

Dim As Long valueA, valueB

valueA = 5
valueB = 2

expect_bool (valueA / valueB) = 2, 0, "variable real division comparison"
expect_bool (valueA \ valueB) = 2, -1, "variable integer division comparison"
expect_bool ((valueA \ valueB) + 3) = 5, -1, "nested variable expression"
expect_bool ((valueA = 5) And (valueB = 2)) = -1, -1, "combined boolean expression"
expect_bool ((valueA = 0) Or (valueB = 2)) = -1, -1, "boolean OR expression"

System

Sub expect_bool (actual&, expected&, msg$)
    If actual& = expected& Then
        Print "passed: "; msg$
    Else
        Print "failed: "; msg$
    End If
End Sub
