' PRINT and WRITE in a window: each separator, zones of 14 columns, a line that wraps, enough lines to scroll.
Dim i As Integer, s As String
For i = 1 To 27
    Print "line"; i
Next i
Print "plain"
Print 1; -2; 3.5; "text"; 4
Print "a", "b", 12, "d"
Print "no line end";
Print " goes on"
Print
Print "comma at the end",
Print "next zone"
Print "1234567890123456789012345678", "past two zones"
Write 1, "two", 3.5
s = String$(100, "w")
Print s
Print "0123456789012345678901234567890123456789012345678901234567890123456789012345"; "wraps whole"
Print "last";
System
