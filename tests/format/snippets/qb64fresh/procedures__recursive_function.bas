Print Factorial(5)
End

Function Factorial (n As Long)
    If n <= 1 Then
        Factorial = 1
    Else
        Factorial = n * Factorial(n - 1)
    End If
End Function
