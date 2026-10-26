$Console:Only
Option _Explicit

On Error GoTo test_failed

ChDir _StartDir$

Print "Creating directory temp_dir"
MkDir "temp_dir"

Print "Changing to directory temp_dir"
ChDir "temp_dir"

Print "Changing to parent directory"
ChDir ".."

Print "Renaming temp_dir to dummy_dir"
Name "temp_dir" As "dummy_dir"

Print "_DIREXISTS(dummy_dir):"; _DirExists("./dummy_dir")

Print "Creating a temporary file inside dummy_dir"
Dim fileName As String: fileName = CreateDummyFile("./dummy_dir/", ".tmp")

Print "_FILEEXISTS(fileName):"; _FileExists(fileName)

Print "Deleting fileName"
Kill fileName

Print "Creating 10 dummy files inside dummy_dir"
Dim i As Long: For i = 0 To 9
    fileName = CreateDummyFile("./dummy_dir/", ".tmp")
Next i

' Start _FILES$ test
ChDir "dummy_dir"

fileName = CreateDummyFile("./", "")

i = 0
Dim dirEntry As String: dirEntry = _Files$("") ' should count 13 entries

Do While Len(dirEntry) > 0
    i = i + 1

    dirEntry = _Files$
Loop

Print "Counted"; i; "entries."

i = 0
dirEntry = _Files$("*.*") ' should count 12 entries

Do While Len(dirEntry) > 0
    i = i + 1

    dirEntry = _Files$
Loop

Print "Counted"; i; "entries."

Kill fileName

ChDir ".."
' End _FILES$ test

Print "Deleting all 10 dummy files"
Kill "./dummy_dir/*.tmp"

Print "Deleting dummy_dir"
RmDir "dummy_dir"

System

test_failed:
Print "Test failed!"
System 1


Function CreateDummyFile$ (directory As String, extension As String)
    Do
        Dim fileName As String: fileName = directory + LTrim$(Str$(Int(100! * (Timer + Rnd)))) + extension
    Loop While _FileExists(fileName)

    Dim h As Long: h = FreeFile

    Open fileName For Output As h
    Print #h, "Delete me!"
    Close h

    CreateDummyFile = fileName
End Function
