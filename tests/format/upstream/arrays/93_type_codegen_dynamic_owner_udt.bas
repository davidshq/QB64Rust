$Console:Only
$Unstable:TypeFields
Option _Explicit

Type DynamicOwnerLeaf
    Name As String
    Samples(0 TO 1) _Dynamic As Long
End Type

Type DynamicOwnerRoot
    Items(0 TO 1) _Dynamic As DynamicOwnerLeaf
    Note As String
End Type

ReDim src(0 To 0) As DynamicOwnerRoot
ReDim dst(0 To 0) As DynamicOwnerRoot
Dim i As Long

ReDim src(0).Items(0 TO 31)
For i = 0 To 31
    src(0).Items(i).Name = "item-" + LTrim$(Str$(i))
    ReDim src(0).Items(i).Samples(-1 TO 3)
    src(0).Items(i).Samples(-1) = i * 10 + 1
    src(0).Items(i).Samples(3) = i * 10 + 3
Next
src(0).Note = "root-note"

' Clone a _DYNAMIC UDT array whose elements own both STRING and another descriptor.
dst() = src()

If LBound(dst(0).Items) <> 0 Then GoTo failed
If UBound(dst(0).Items) <> 31 Then GoTo failed
If dst(0).Items(17).Name <> "item-17" Then GoTo failed
If LBound(dst(0).Items(17).Samples) <> -1 Then GoTo failed
If UBound(dst(0).Items(17).Samples) <> 3 Then GoTo failed
If dst(0).Items(17).Samples(-1) <> 171 Then GoTo failed
If dst(0).Items(17).Samples(3) <> 173 Then GoTo failed
If dst(0).Note <> "root-note" Then GoTo failed

src(0).Items(17).Name = "changed"
src(0).Items(17).Samples(3) = -999
If dst(0).Items(17).Name <> "item-17" Then GoTo failed
If dst(0).Items(17).Samples(3) <> 173 Then GoTo failed

' Grow the owner array itself. Existing nested owners must survive _RETAIN.
ReDim _Retain src(0).Items(0 TO 47)
If src(0).Items(17).Name <> "changed" Then GoTo failed
If src(0).Items(17).Samples(3) <> -999 Then GoTo failed
src(0).Items(47).Name = "item-47"
ReDim src(0).Items(47).Samples(0 TO 2)
src(0).Items(47).Samples(2) = 4702
If src(0).Items(47).Samples(2) <> 4702 Then GoTo failed

Erase src(0).Items
If dst(0).Items(17).Name <> "item-17" Then GoTo failed
If dst(0).Items(17).Samples(3) <> 173 Then GoTo failed

Erase src
Erase dst
Print "PASS 93_type_codegen_dynamic_owner_udt.bas"
System

failed:
Print "FAIL 93_type_codegen_dynamic_owner_udt.bas"
System
