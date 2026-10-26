Option _Explicit
$Console:Only

expect_bool (5 / 2) = 2, 0, "(5 / 2) = 2"
expect_bool (5 \ 2) = 2, -1, "(5 \ 2) = 2"
expect_bool (7 Mod 4) = 3, -1, "(7 MOD 4) = 3"
expect_bool (2 ^ 3) = 8, -1, "(2 ^ 3) = 8"
expect_bool (3 + 4 * 2) = 11, -1, "(3 + 4 * 2) = 11"
expect_bool ((3 + 4) * 2) = 14, -1, "((3 + 4) * 2) = 14"
expect_bool ("AB" + "CD") = "ABCD", -1, "string expression comparison"

System

Sub expect_bool (actual&, expected&, msg$)
    If actual& = expected& Then
        Print "passed: "; msg$
    Else
        Print "failed: "; msg$
    End If
End Sub
