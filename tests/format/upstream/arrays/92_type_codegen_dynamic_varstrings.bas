$Console:Only
$Unstable:TypeFields
Option _Explicit

Type DynamicStrings
    Names(0 TO 1) _Dynamic As String
    Id As Long
End Type

ReDim src(0 To 1) As DynamicStrings
ReDim dst(0 To 1) As DynamicStrings

ReDim src(0).Names(-4 TO 300)
src(0).Names(-4) = "low"
src(0).Names(123) = "middle"
src(0).Names(300) = "high"
src(0).Id = 42

' Assignment must clone the descriptor payload and every owned qbs string.
dst() = src()

If LBound(dst(0).Names) <> -4 Then GoTo failed
If UBound(dst(0).Names) <> 300 Then GoTo failed
If dst(0).Names(-4) <> "low" Then GoTo failed
If dst(0).Names(123) <> "middle" Then GoTo failed
If dst(0).Names(300) <> "high" Then GoTo failed
If dst(0).Id <> 42 Then GoTo failed

src(0).Names(123) = "changed"
If dst(0).Names(123) <> "middle" Then GoTo failed

' _RETAIN exercises descriptor replacement while preserving existing payload.
ReDim _Retain src(0).Names(-8 TO 400)
If src(0).Names(-4) <> "low" Then GoTo failed
If src(0).Names(123) <> "changed" Then GoTo failed
If src(0).Names(300) <> "high" Then GoTo failed
src(0).Names(400) = "new-high"
If src(0).Names(400) <> "new-high" Then GoTo failed

Erase src(0).Names
If dst(0).Names(300) <> "high" Then GoTo failed
Erase src
If dst(0).Names(-4) <> "low" Then GoTo failed

Print "PASS 92_type_codegen_dynamic_varstrings.bas"
System

failed:
Print "FAIL 92_type_codegen_dynamic_varstrings.bas"
System
