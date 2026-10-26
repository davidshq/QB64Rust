$Console:Only
Option _Explicit

Type InlineStrings
    Names(0 TO 1023) As String
    Numbers(0 TO 63) As Long
    Tail As String
End Type

ReDim src(0 To 1) As InlineStrings
ReDim dst(0 To 1) As InlineStrings
Dim i As Long

For i = 0 To 1023 Step 127
    src(0).Names(i) = "S" + LTrim$(Str$(i))
Next
src(0).Names(1023) = "LAST"
src(0).Numbers(63) = 6300
src(0).Tail = "source-tail"

' Whole-array assignment exercises deep copy of the inline variable-STRING members.
dst() = src()

If dst(0).Names(0) <> "S0" Then GoTo failed
If dst(0).Names(254) <> "S254" Then GoTo failed
If dst(0).Names(1023) <> "LAST" Then GoTo failed
If dst(0).Numbers(63) <> 6300 Then GoTo failed
If dst(0).Tail <> "source-tail" Then GoTo failed

' The destination must own independent qbs objects.
src(0).Names(254) = "changed"
src(0).Tail = "changed-tail"
If dst(0).Names(254) <> "S254" Then GoTo failed
If dst(0).Tail <> "source-tail" Then GoTo failed

' Exercise member clear and parent-array free paths.
Erase src(0).Names
If src(0).Names(254) <> "" Then GoTo failed
If dst(0).Names(254) <> "S254" Then GoTo failed

Erase src
If dst(0).Names(1023) <> "LAST" Then GoTo failed

Print "PASS 90_type_codegen_inline_varstrings.bas"
System

failed:
Print "FAIL 90_type_codegen_inline_varstrings.bas"
System
