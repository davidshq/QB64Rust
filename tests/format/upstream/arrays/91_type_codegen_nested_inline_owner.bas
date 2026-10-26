$Console:Only
Option _Explicit

Type OwnerLeaf
    Name As String
    Tags(0 TO 7) As String
    Value As Long
End Type

Type OwnerTree
    Leaves(0 TO 127) As OwnerLeaf
    Labels(0 TO 31) As String
    Marker As Long
End Type

Dim src(0 To 1) As OwnerTree
Dim dst(0 To 1) As OwnerTree

src(0).Leaves(0).Name = "leaf-0"
src(0).Leaves(63).Name = "leaf-63"
src(0).Leaves(127).Name = "leaf-127"
src(0).Leaves(63).Tags(7) = "tag-63-7"
src(0).Leaves(127).Value = 12700
src(0).Labels(31) = "label-31"
src(0).Marker = 9876

' This traverses an inline fixed array of nested owner UDT elements.
dst() = src()

If dst(0).Leaves(0).Name <> "leaf-0" Then GoTo failed
If dst(0).Leaves(63).Name <> "leaf-63" Then GoTo failed
If dst(0).Leaves(127).Name <> "leaf-127" Then GoTo failed
If dst(0).Leaves(63).Tags(7) <> "tag-63-7" Then GoTo failed
If dst(0).Leaves(127).Value <> 12700 Then GoTo failed
If dst(0).Labels(31) <> "label-31" Then GoTo failed
If dst(0).Marker <> 9876 Then GoTo failed

src(0).Leaves(63).Name = "changed"
src(0).Leaves(63).Tags(7) = "changed-tag"
src(0).Labels(31) = "changed-label"
If dst(0).Leaves(63).Name <> "leaf-63" Then GoTo failed
If dst(0).Leaves(63).Tags(7) <> "tag-63-7" Then GoTo failed
If dst(0).Labels(31) <> "label-31" Then GoTo failed

Erase src
If dst(0).Leaves(127).Name <> "leaf-127" Then GoTo failed
If dst(0).Leaves(63).Tags(7) <> "tag-63-7" Then GoTo failed

Print "PASS 91_type_codegen_nested_inline_owner.bas"
System

failed:
Print "FAIL 91_type_codegen_nested_inline_owner.bas"
System
