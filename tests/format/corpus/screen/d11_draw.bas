' DRAW with a literal, a variable and an expression.
Dim a As String
Screen 7
Draw "BM50,50 C4 R20 D20 L20 U20"
a = "C2 BM100,50 E10 F10 G10 H10"
Draw a
Draw "BM150,100 C5 M+20,+10 M-10,+10" + " NU5 NR5"
PSet (200, 100), 3
Draw "S8 TA45 R10 U10"
Draw "BM250,150 C14 S4 TA0 R30 P6,14"
System
