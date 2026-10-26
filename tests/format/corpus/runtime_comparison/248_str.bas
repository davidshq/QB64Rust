$Console:Only
' Runtime comparison: STR$ leading space for positive (QB4.5 behavior)
' STR$(x) returns " 42" for positive (leading space), "-42" for negative
Dim n As Long
n = 99
Print "["; Str$(n); "]"
Print "["; Str$(-n); "]"
Print "["; Str$(0); "]"
End
