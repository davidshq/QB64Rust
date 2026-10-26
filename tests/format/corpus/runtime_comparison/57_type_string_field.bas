$Console:Only
' Runtime comparison: TYPE with string field
Type Person
    name As String
    age As Long
End Type
Dim p As Person
p.name = "Alice"
p.age = 30
Print "name:"; p.name; "age:"; p.age
End
