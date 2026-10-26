$Console:Only
$Unstable:TypeFields
Option _Explicit

Type DescriptorLeaf
    Values(0 TO 1) _Dynamic As Long
    Tag As Long
End Type

Type DescriptorRoot
    Leaves(0 TO 127) As DescriptorLeaf
    Guard As Long
End Type

ReDim src(0 To 0) As DescriptorRoot
ReDim dst(0 To 0) As DescriptorRoot

' The parent contains a large fixed inline member array. Each element contains
' a descriptor-backed member, so descriptor init/free/clone must traverse Leaves().
ReDim src(0).Leaves(0).Values(-2 TO 4)
ReDim src(0).Leaves(63).Values(10 TO 20)
ReDim src(0).Leaves(127).Values(0 TO 5)
src(0).Leaves(0).Values(-2) = 102
src(0).Leaves(63).Values(20) = 6320
src(0).Leaves(127).Values(5) = 12705
src(0).Leaves(63).Tag = 6300
src(0).Guard = 777

dst() = src()

If LBound(dst(0).Leaves(0).Values) <> -2 Then GoTo failed
If UBound(dst(0).Leaves(0).Values) <> 4 Then GoTo failed
If dst(0).Leaves(0).Values(-2) <> 102 Then GoTo failed
If dst(0).Leaves(63).Values(20) <> 6320 Then GoTo failed
If dst(0).Leaves(127).Values(5) <> 12705 Then GoTo failed
If dst(0).Leaves(63).Tag <> 6300 Then GoTo failed
If dst(0).Guard <> 777 Then GoTo failed

src(0).Leaves(63).Values(20) = -1
src(0).Leaves(63).Tag = -2
If dst(0).Leaves(63).Values(20) <> 6320 Then GoTo failed
If dst(0).Leaves(63).Tag <> 6300 Then GoTo failed

Erase src
If dst(0).Leaves(127).Values(5) <> 12705 Then GoTo failed
Erase dst

Print "PASS 94_type_codegen_inline_descriptor_udt.bas"
System

failed:
Print "FAIL 94_type_codegen_inline_descriptor_udt.bas"
System
