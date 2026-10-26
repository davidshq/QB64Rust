$Console:Only
Option _Explicit

ChDir _StartDir$

Dim fileName As String

Read fileName
While Len(fileName) > 0
    Dim img As Long: img = _LoadImage("./" + fileName)

    If img < -1 Then
        Print fileName; ": ("; _Width(img); "x"; _Height(img); ") pixels"
        _FreeImage img
    Else
        Print fileName; " is not a valid image file!"
    End If

    Read fileName
Wend

System

Data good1.svg
Data good2.svg
Data bogus1.svg
Data bogus2.svg
Data
