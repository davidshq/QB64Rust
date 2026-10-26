$Console:Only
Option _Explicit

Type T
    Values(0 To 2) As String * 5
End Type

Dim plain(0 To 2) As String * 5
Dim one As T
Dim many(1) As T
Dim i As Long, j As Long

plain(0) = "AB"
plain(1) = "XYZ"
plain(2) = "HELLO"

Erase plain
For i = 0 To 2
    For j = 1 To 5
        If Asc(plain(i), j) <> 0 Then
            Print "FAIL 47_zero_fill_fixed_strings_erase_redim.bas: ERASE plain mismatch at"; i; j
            System
        End If
    Next j
Next i

one.Values(0) = "AA"
one.Values(1) = "BB"
one.Values(2) = "CC"

Erase one.Values
For i = 0 To 2
    For j = 1 To 5
        If Asc(one.Values(i), j) <> 0 Then
            Print "FAIL 47_zero_fill_fixed_strings_erase_redim.bas: ERASE one.Values mismatch at"; i; j
            System
        End If
    Next j
Next i

one.Values(0) = "11"
one.Values(1) = "22"
one.Values(2) = "33"

ReDim one.Values(0 To 2)
For i = 0 To 2
    For j = 1 To 5
        If Asc(one.Values(i), j) <> 0 Then
            Print "FAIL 47_zero_fill_fixed_strings_erase_redim.bas: REDIM one.Values mismatch at"; i; j
            System
        End If
    Next j
Next i

many(0).Values(1) = "AB"
many(1).Values(2) = "CD"

Erase many(1).Values
For i = 0 To 2
    For j = 1 To 5
        If Asc(many(1).Values(i), j) <> 0 Then
            Print "FAIL 47_zero_fill_fixed_strings_erase_redim.bas: ERASE many(1).Values mismatch at"; i; j
            System
        End If
    Next j
Next i

If Asc(many(0).Values(1), 1) <> 65 Or Asc(many(0).Values(1), 2) <> 66 Then
    Print "FAIL 47_zero_fill_fixed_strings_erase_redim.bas: many(0) changed unexpectedly"
    System
End If

many(0).Values(0) = "X"
many(0).Values(2) = "Y"
many(1).Values(0) = "Z"

ReDim many(1).Values(0 To 2)
For i = 0 To 2
    For j = 1 To 5
        If Asc(many(1).Values(i), j) <> 0 Then
            Print "FAIL 47_zero_fill_fixed_strings_erase_redim.bas: REDIM many(1).Values mismatch at"; i; j
            System
        End If
    Next j
Next i

If Asc(many(0).Values(0), 1) <> 88 Or Asc(many(0).Values(2), 1) <> 89 Then
    Print "FAIL 47_zero_fill_fixed_strings_erase_redim.bas: many(0) changed unexpectedly after REDIM many(1).Values"
    System
End If

Erase many
For i = 0 To 1
    Dim k As Long
    For k = 0 To 2
        For j = 1 To 5
            If Asc(many(i).Values(k), j) <> 0 Then
                Print "FAIL 47_zero_fill_fixed_strings_erase_redim.bas: ERASE many mismatch at"; i; k; j
                System
            End If
        Next j
    Next k
Next i

Print "PASS 47_zero_fill_fixed_strings_erase_redim.bas"
System
