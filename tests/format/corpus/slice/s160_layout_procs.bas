$Console:Only
' Slice (m5b-format-settings, task 1.1): SUBs and FUNCTIONs laid out by hand, for the format oracle under the
' other layout settings (tests\format\settings.list): bodies indented wrong on purpose, blocks inside them, a
' comment after the SUB and FUNCTION lines and their ends, and odd spacing.
DECLARE SUB show (t AS STRING)   ' a DECLARE
Dim Shared total As Long
show "start"
Print twice(4); twice(5)
Call show("call")
tally 3
Print total
Print fact&(5)
End

Sub show (t As String) ' a SUB
    Print "show: "; t
End Sub ' its end

Function twice% (x As Integer) ' a FUNCTION
    twice% = x * 2
End Function ' its end

Sub tally (n)
    Static calls
    calls = calls + 1
    For i = 1 To n
        total = total + i
        If total > 4 Then
            Exit Sub ' leaving early
        End If
    Next
End Sub

Function fact& (n As Long)
    Select Case n
        Case Is <= 1
            fact& = 1
        Case Else
            fact& = n * fact&(n - 1)
    End Select
End Function
