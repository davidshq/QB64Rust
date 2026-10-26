$Console:Only

num## = _Pi(10): typ$ = "_FLOAT": mi% = 1: ma% = 19

Print "Printing _PI(10) as "; typ$; ":"
Print "  default:  "; _ToStr$(num##)
For x% = 0 To 20
    Print Using "## digits:  "; x%;
    num$ = _ToStr$(num##, x%)
    Print num$; Spc(ma% + 5 - Len(num$));
    If x% < mi% Then
        Print "(forced to at least 1 digit)"
    ElseIf x% > ma% Then
        Print "(cropped to max. digits for "; typ$; ")"
    ElseIf x% > 1 _AndAlso InStr(num$, ".") > 0 _AndAlso Len(num$) < x% + 1 Then
        Print "(rounding eliminated digit(s))"
    Else
        Print
    End If
Next x%

System

