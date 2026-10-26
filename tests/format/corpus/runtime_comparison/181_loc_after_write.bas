$Console:Only
' Runtime comparison: LOC after PRINT # / WRITE #
Open "rt_181_loc.txt" For Output As #1
Print "loc after open:"; Loc(1)
Print #1, "abc"
Print "loc after print:"; Loc(1)
Write #1, "x"
Print "loc after write:"; Loc(1)
Close #1
Open "rt_181_loc.txt" For Input As #2
Print "loc input:"; Loc(2)
Close #2
Kill "rt_181_loc.txt"
Print "done"
End
