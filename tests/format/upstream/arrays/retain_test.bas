$Console:Only
Option _Explicit

ReDim a(55, 66, 12) As String
a(33, 33, 10) = "six"
a(11, 11, 11) = "seven"
ReDim _Retain a(11 To 1024, 5 To 66, -3 To 12) As String
a(11, 66, -3) = "nothing"
Print a(33, 33, 10)
Print a(11, 11, 11)
System
