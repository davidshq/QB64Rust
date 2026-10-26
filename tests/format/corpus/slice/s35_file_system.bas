$Console:Only
' Built-in statements that are one call of the runtime (spec language/builtin-statements, language/file-io
' "File-system statements"): MKDIR, CHDIR, NAME, RMDIR, KILL and ENVIRON. Their effect is shown by the errors a
' second call raises under a handler (the file functions come with the files, s38): 75 for a folder that exists, 76
' for one that does not, 53 for a missing file, 5 for a raising argument (once: the first error is the one serviced)
' and for an ENVIRON text without a separator. RESUME runs a failing statement and its arguments again. Every folder
' is named s35_* and removed again.
On Error GoTo handler
Dim Shared handled As Long, calls As Long
Dim d As String, tries As Long
d = "s35_dir"
Print "MKDIR"
MkDir d
Print "MKDIR again"
MkDir d
Print "CHDIR into it and back"
ChDir d
MkDir "s35_inner"
ChDir ".."
Print "RMDIR of a folder that is not empty"
RmDir d
Print "RMDIR inner, by an expression"
RmDir d + "/s35_inner"
Print "NAME"
Name d As d + "2"
Print "MKDIR of the old name works again"
MkDir d
Print "NAME onto an existing folder"
Name d As d + "2"
Print "RMDIR both"
RmDir d
RmDir (d + "2")
Print "RMDIR again"
RmDir d
Print "CHDIR missing"
ChDir "s35_none"
Print "NAME missing"
Name "s35_none" As "s35_other"
Print "KILL missing"
Kill "s35_none.tmp"
Call Kill("s35_none.tmp")
Print "KILL empty name"
Kill ""
Print "raising arguments"
Kill Chr$(-1)
MkDir "s35_" + Chr$(-1)
Name Chr$(-1) As "s35_x"
Name "s35_x" As Chr$(-1)
Print "ENVIRON"
Environ "S35_VAR=seven"
Environ "S35_VAR2 eight"
Environ "S35_VAR3"
Environ ""
Print "in a SUB"
insub "s35_none.tmp"
Print "RESUME runs the statement again"
tries = 0
RmDir dirname$
Print "the FUNCTION ran"; calls; "times"
Print "handled"; handled
System
handler:
handled = handled + 1
Print "  error"; Err
If Err = 76 And tries = 0 And calls = 1 Then
    tries = 1
    MkDir "s35_again"
    Resume
End If
Resume Next

Sub insub (f As String)
    Kill f
    Print "  after KILL in the SUB"
    MkDir "s35_sub"
    MkDir "s35_sub"
    RmDir "s35_sub"
    Print "  end of the SUB"
End Sub

Function dirname$
    calls = calls + 1
    dirname$ = "s35_again"
End Function
