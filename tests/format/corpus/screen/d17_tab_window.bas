' TAB, SPC and POS in a window: TAB moves to a column, on the next line when the cursor is past it; SPC prints
' blanks; a zone after TAB; a column past the width; TAB and SPC as functions.
Dim t As String
Print "a"; Tab(5); "b"; Tab(10); "c"
Print "abcdefgh"; Tab(5); "below"
Print "a"; Spc(3); "b"; Spc(0); "c"
Print Tab(3); 7; Tab(12); -7
Print Tab(5), "zone"
Print "x"; Tab(8); "y"; Spc(2); "z"
Print Pos(0); Tab(20); Pos(0); Spc(5); Pos(0)
Print "wide"; Tab(85); "past the width"
Print Tab(0); "zero"; Tab(-3); "negative"
Print "p"; Spc(-2); "q"; Spc(90); "after ninety"
Print Spc(78); "wrap"
t = Tab(6)
Print "["; t; "]"; Len(t)
Print "abcdef";: t = Tab(3): Print Len(t); Asc(t)
Print "abcdef";: t = Spc(3): Print Len(t); Asc(t)
Color 14
Print Tab(30); "in yellow"; Tab(60); CsrLin
System
