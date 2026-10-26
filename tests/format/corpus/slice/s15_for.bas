$Console:Only
' FOR (spec language/control-flow, "FOR loops", "NEXT variables", "EXIT from loops"): the probe rows of the
' design (limits evaluated once, the body changing the variable, a loop that does not run, a step of 0), negative
' and float steps, start, limit and step rounded to the wider type, SINGLE, DOUBLE and _FLOAT variables, the
' integer types at their limits (INTEGER and LONG pass their range and end, _INTEGER64 wraps and is capped),
' NEXT j, i, NEXT with a suffix, EXIT FOR from inside an IF and from a nested FOR, FOR in a SUB (temporaries per
' call).
Dim ml As Long, q As _Integer64
Print "plain"
For i = 1 To 3: Print i;: Next: Print i
Print "negative step"
For i = 5 To 1 Step -2: Print i;: Next: Print i
Print "float start, plain SINGLE variable"
For i = 1.5 To 3: Print i;: Next: Print i
Print "SINGLE steps"
c = 0: For s! = 0 To 1 Step 0.1: c = c + 1: Next: Print s!; c
Print "DOUBLE steps"
c = 0: For d# = 0 To 1 Step 0.1: c = c + 1: Next: Print d#; c
For d# = 1 To 0 Step -0.25: Print d#;: Next: Print d#
Print "_FLOAT steps"
For f## = 1 To 2 Step 0.5: Print f##;: Next: Print f##
Print "rounded to the wider type"
For i% = 1 To 2.6: Print i%;: Next: Print i%
For i% = 0.5 To 2: Print i%;: Next: Print i%
For i% = 1 To 4 Step 1.5: Print i%;: Next: Print i%
For ml = 0.5 To 3.5: Print ml;: Next: Print ml
Print "limits evaluated once"
e = 3: For m = 1 To e: e = 10: Print m;: Next: Print
st = 1: For m = 1 To 6 Step st: st = 3: Print m;: Next: Print
Print "the body changes the variable"
For k = 1 To 5: k = k + 1: Print k;: Next: Print
Print "loops that do not run"
For m = 3 To 1: Print "never": Next: Print m
For m = 1 To 3 Step -1: Print "never": Next: Print m
Print "step 0"
For z = 1 To 2 Step 0: z = z + 1: Print z;: Next: Print z
Print "INTEGER passes its range"
For i% = 32760 To 32767 Step 4: Print i%;: Next: Print i%
For i% = -32760 To -32768 Step -4: Print i%;: Next: Print i%
Print "LONG passes its range"
For ml = 2147483640 To 2147483647 Step 5: Print ml;: Next: Print ml
Print "_INTEGER64 wraps (capped at five passes)"
n = 0
For q = 9223372036854775800 To 9223372036854775807 Step 5
    Print q;
    n = n + 1
    If n = 5 Then Exit For
Next
Print
Print q
Print "NEXT j, i"
For i = 1 To 2
    For j = 1 To 2
        Print i; j;
Next j, i
Print
Print i; j
Print "NEXT with a suffix"
For i = 1 To 2: Next i!: Print i
For ml = 1 To 2: Next ml&: Print ml
Print "EXIT FOR from inside an IF"
For i = 1 To 9
    If i = 3 Then Exit For
Next
Print i
Print "EXIT FOR leaves the inner FOR only"
For i = 1 To 2
    For j = 1 To 9
        If j = 2 Then Exit For
    Next
    Print i; j;
Next
Print
Print "FOR in a SUB"
countdown 3
countdown 2
System

Sub countdown (n As Integer)
    For c% = n To 1 Step -1
        Print c%;
    Next
    Print "/"; c%
End Sub
