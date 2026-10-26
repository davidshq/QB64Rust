$Console:Only
' TYPE with PRINT fields
Type T
    a As Long
    b As Double
End Type
Dim u As T
u.a = 7
u.b = 2.5
Print u.a; u.b
End
