Option _Explicit
$Console:Only

Call expect_bool((5 / 2) = 2, 0, "CALL with false comparison")
Call expect_bool((5 \ 2) = 2, -1, "CALL with true comparison")
Call expect_bool(((2 + 3) * 4) = 20, -1, "CALL with nested expression")
Call expect_bool(("QB" + "64") = "QB64", -1, "CALL with string comparison")

System

Sub expect_bool (actual&, expected&, msg$)
    If actual& = expected& Then
        Print "passed: "; msg$
    Else
        Print "failed: "; msg$
    End If
End Sub
