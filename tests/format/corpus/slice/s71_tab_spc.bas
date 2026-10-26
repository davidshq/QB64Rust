$Console:Only
' Slice (m3-get-put-bload, task 3.1): TAB and SPC as items of PRINT and PRINT #, and as the functions they are
' (verification\v28_a_tab_spc). On the console POS(0) is always 1, so TAB(n) is n - 1 blanks wherever the line
' stands; in a file the column is the file's. No comma follows an item of a console PRINT: with redirected output
' that never ends in the old compiler's program (study\00 §5). Then _RGB32, _RGBA32, DATE$ and TIME$ by what
' does not change from run to run, and _FULLSCREEN as code that is compiled and not reached.
Dim i As Integer, s As Single, t As String, k As String, u As _Unsigned Long, q As _Integer64
Print "a"; Tab(5); "b"; Tab(10); "c"
Print "abcdefgh"; Tab(5); "below"
Print "a"; Spc(3); "b"; Spc(0); "c"
Print Tab(3); 7; Tab(12); -7
Print Tab(5)
Print Tab(5);: Print "x"
Print Tab(5); "string after"
Print "x"; Tab(8); "y"; Spc(2); "z"
i = 4: s = 6.5: q = 3
Print Tab(i); "i"; Tab(s); "s"; Tab(2.5); "r"; Spc(q); "q"
Print Tab(0); "zero"; Tab(-3); "negative"
Print "123456789"; Tab(10); "at ten"; Tab(10); "again"
Print "wide"; Tab(85); "past the width"
Print Spc(85); "spc past the width"
Print "p"; Spc(-2); "q"
Print Pos(0); Tab(20); Pos(0); Spc(5); Pos(0)
' As functions.
t = Tab(6)
Print "["; t; "]"; Len(t)
t = Spc(4)
Print "["; t; "]"; Len(t)
Print Len(Spc(3) + Tab(9)); Len(Tab(2) + Tab(2)); Len(Spc(i) + Spc(s))
Print "abcdef";: t = Tab(3): Print Len(t); Asc(t)
Write Tab(3), Spc(2)
' To a file.
Open "s71_tab_spc.tmp" For Output As #1
Print #1, "a"; Tab(5); "b"; Tab(10); "c"
Print #1, "abcdefgh"; Tab(5); "below"
Print #1, "a"; Spc(3); "b"
Print #1, Tab(5)
Print #1, Tab(5);
Print #1, "x"
Print #1, 7; Tab(12); -7, Tab(30); "z"
Print "screen";: Print #1, "f"; Tab(4); "g"
Print
Close #1
Open "s71_tab_spc.tmp" For Input As #1
Do Until EOF(1)
    Line Input #1, k
    Print "["; k; "]"
Loop
Close #1
Kill "s71_tab_spc.tmp"
' Colours of 32 bits: red, green and blue with full alpha; one value as a grey; with an alpha.
u = _RGB32(1, 2, 3)
Print Hex$(u); " "; Hex$(_RGB32(255, 128, 0)); " "; Hex$(_RGB32(5)); " "; Hex$(_RGB32(5, 16)); " "; Hex$(_RGB32(1, 2, 3, 4))
Print Hex$(_RGBA32(1, 2, 3, 4)); " "; Hex$(_RGBA32(300, -1, s, i)); _RGB32(0, 0, 0); _RGB32(255, 255, 255) + 1
i = 200: s = 100.5
Print Hex$(_RGB32(i, s, q)); " "; Hex$(_RGB32(i * 2, s, q))
' The date and the time have their length and their separators.
t = Date$: k = Time$
Print Len(t); Mid$(t, 3, 1); Mid$(t, 6, 1); Len(k); Mid$(k, 3, 1); Mid$(k, 6, 1); Len(Date$ + Time$)
System
_FullScreen
_FullScreen _Stretch , _Smooth
_FullScreen _Off
i = _FullScreen
