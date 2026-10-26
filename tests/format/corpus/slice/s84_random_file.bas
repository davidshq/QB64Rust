$Console:Only
' Slice (m3-console-files, task 5.3; from verification\v30_e_random): GET and PUT on a file open FOR RANDOM, 12
' numbered lines. Records of a TYPE with LEN = LEN(r): written at a record, at the next one, past the end (a gap
' of zeros), read back in turn and past the end (zeros, EOF); the default record length of 128, also with no mode;
' values shorter than the record (the file ends after them, not at the record's end); a variable-length string
' (two bytes of length and its characters, read back by that length); a value longer than the record (error 59,
' nothing written); record 0 and below (error 63), also for SEEK; GET and PUT with no variable (one whole record
' each). The file is s84_random_file.tmp and is removed.
On Error GoTo handler
Type rec
    n As Integer
    s As String * 4
    d As Double
End Type
Dim f As String, r As rec, r2 As rec, i As Integer, l As Long, v As String, w As String, x As String * 6, k As Long
Dim ub As _Unsigned _Byte, d As Double, big As String * 20
f = "s84_random_file.tmp"
Open f For Random As #1 Len = Len(r)
Print "1:"; Len(r); LOF(1); Seek(1); Loc(1); EOF(1)
r.n = 1: r.s = "one": r.d = 1.5: Put #1, 1, r: Print "2:"; LOF(1); Seek(1); Loc(1);
r.n = 2: r.s = "two": r.d = 2.5: Put #1, , r: Print LOF(1); Seek(1);
r.n = 9: r.s = "nine": r.d = 9.5: Put #1, 6, r: Print LOF(1); Seek(1)
Print "3:";: Get #1, 2, r2: Print r2.n; "["; r2.s; "]"; r2.d; Seek(1); EOF(1);
Get #1, 1, r2: Print r2.n; "["; r2.s; "]";: Get #1, , r2: Print r2.n;: Get #1, 6, r2: Print r2.n; r2.d; EOF(1)
Print "4:";: Get #1, 5, r2: Print r2.n; Asc(r2.s); r2.d; EOF(1);: Get #1, 7, r2: Print r2.n; EOF(1); LOF(1);
Get #1, 20, r2: Print r2.n; EOF(1); LOF(1); Seek(1)
Print "5:";: Get #1, 0, r2: Print Seek(1);: Put #1, -1, r: Print Seek(1);: Seek #1, 0: Print Seek(1);: Seek #1, 2: Get #1, , r2: Print r2.n; Seek(1)
Print "6:";: Get #1, 1: Print Seek(1);: Get #1: Print Seek(1);: Put #1, 9: Print Seek(1); LOF(1)
Close #1: Kill f
Open f For Random As #1
i = 258: Put #1, 1, i: Print "7:"; LOF(1); Seek(1);: Put #1, 2, i: Print LOF(1); Seek(1);: i = 0: Get #1, 2, i: Print i; EOF(1)
Close #1: Kill f
Open f As #1
i = 258: Put #1, 2, i: Print "8:"; LOF(1); Seek(1)
Close #1: Kill f
Open f For Random As #1 Len = 8
i = 258: Put #1, 1, i: Print "9:"; LOF(1);: l = 65537: Put #1, , l: Print LOF(1);: x = "abcdef": Put #1, , x: Print LOF(1); Seek(1);
i = 0: Get #1, 1, i: l = 0: Get #1, , l: x = "": Get #1, , x: Print i; l; "["; x; "]"; EOF(1);: Get #1, 1, l: Print l
v = "abc": Put #1, 5, v: Print "10:"; LOF(1);: w = "": Get #1, 5, w: Print "["; w; "]"; Len(w);
v = "": Put #1, 6, v: Get #1, 6, w: Print Len(w);: v = "123456": Put #1, 7, v: w = "keep": Get #1, 7, w: Print "["; w; "]"
Print "11:";: d = 1: Put #1, 9, d: Print LOF(1);: big = "too long": Put #1, 9, big: Print LOF(1);: Get #1, 1, big: Print "["; RTrim$(big); "]";
v = "1234567": Put #1, 9, v: Print LOF(1)
Close #1
Open f For Binary As #1
Print "12:";: For k = 1 To 16: Get #1, k, ub: Print ub;: Next: For k = 33 To 37: Get #1, k, ub: Print ub;: Next: Print
Close #1: Kill f
System
handler:
Print " <error"; Err; ">";
Resume Next
