$Console:Only
' Runtime errors of the file statements under a handler (spec language/file-io, language/builtin-statements "Runtime
' errors"): 52 for a number that is not open, 53 for a missing file, 54 for the wrong mode, 55 for a number that is
' open, 62 past the end, 6 for a value outside the target's range, 63 for a bad position, 75 and 76 from the file
' system, 5 for a raising argument. A statement with several items stops at the first that raises; what was written
' or read before it stays. Files are s37_*.tmp and removed again.
On Error GoTo handler
Dim f As String, a As String, c As String, n As Long, b As _Byte, ub As _Unsigned _Byte
Dim arr(3) As Long, sa(3) As String, retry As Long
f = "s37_errors.tmp"
Print "not open"
Print #3, "x"
Write #3, "x"
Input #3, a
Line Input #3, a
Seek 3, 1
Print EOF(3); LOF(3); Loc(3); Seek(3)
Print "file numbers 0 and -1"
Open f For Output As #0
Open f For Output As #-1
Print #0, "x"
Print EOF(0)
Print "CLOSE of numbers that are not open is no error"
Close #5: Close 0, -1, 300
Close
Print "a missing file"
Open "s37_none.tmp" For Input As #1
Open "" For Input As #1
Print "a number that is open"
Open f For Output As #1
Open f For Output As #1
Print "the wrong mode"
Input #1, a
Line Input #1, a
Print #1, "one"
Print #1, "200,7"
Print #1, "-1,8"
Print #1, "31,32,33"
Print #1, "s1,s2,s3"
Print #1, "ra"; Chr$(-1); "rb"
Print #1, "after the raising item"
Write #1, "wa", Chr$(-1), "wb"
Write #1, "after the raising item"
Close #1
Open f For Input As #1
Print #1, "x"
Write #1, "x"
Print "the old form with a bad mode letter"
Open "X", #2, f
Open "", #2, f
Print "raising arguments"
Open Chr$(-1) For Input As #2
Open f For Input As Asc("")
Close Asc("")
Print #Asc(""), "x"
Input #Asc(""), a
Seek 1, Asc("")
Print EOF(Asc(""))
Print "a bad position"
Seek 1, 0
Seek 1, -5
Print "reading"
Line Input #1, a: Print "["; a; "]"
Print "200 into a _BYTE: 0 is stored and the next target is not read"
b = 1: n = 1
Input #1, b, n
Print b; n
Input #1, a: Print "the next field is ["; a; "]"
Print "-1 into an unsigned type"
ub = 1
Input #1, ub, n
Print ub; n
Line Input #1, a: Print "the rest of the line is ["; a; "]"
Print "a target with a bad index reads nothing and ends the statement"
n = 0: b = 0
Input #1, n, arr(9), b
Print n; b
Line Input #1, a: Print "the rest of the line is ["; a; "]"
a = "": c = ""
Input #1, a, sa(9), c
Print "["; a; "]["; c; "]["; sa(0); "]"
Line Input #1, a: Print "the rest of the line is ["; a; "]"
Print "what the raising items left in the file"
Line Input #1, a: Print "["; a; "]"
Line Input #1, a: Print "["; a; "]"
Print "past the end"; EOF(1)
a = "kept": n = 5
Line Input #1, a: Print "["; a; "]"
Input #1, a, c
Input #1, n: Print n
Close
Print "RESUME runs the OPEN again, after the handler made the file"
retry = 1
Open "s37_made.tmp" For Input As #1
Print "opened; the handler ran"; retry - 1; "time"
retry = 0
Close
Kill "s37_made.tmp"
Print "the file system"
Open f For Input As #1
Kill f
Name f As "s37_other.tmp"
Close
MkDir "s37_dir"
Open "s37_dir/a.tmp" For Output As #1: Close
RmDir "s37_dir"
Kill "s37_dir"
Kill "s37_dir/*.tmp"
Kill "s37_dir/*.tmp"
RmDir "s37_dir"
Kill f
Print "in a SUB"
insub
System
handler:
Print "  error"; Err
If Err = 53 And retry = 1 Then
    retry = 2
    Open "s37_made.tmp" For Output As #9: Close #9
    Resume
End If
Resume Next

Sub insub
    Dim t As String
    Print #7, "x"
    Print "  after PRINT # in the SUB"
    Line Input #7, t
    Close #7
    Print "  end of the SUB"
End Sub
