$Console:Only
' Runtime comparison 189: WIDTH (columns) effect on PRINT
' Set narrow width, print long string without newline; output should wrap at WIDTH columns.
Width 10
Print "123456789012345";
Print "X"
Print "done"
End
