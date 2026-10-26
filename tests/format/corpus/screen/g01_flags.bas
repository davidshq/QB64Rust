' Cut from Civil War Strategy (tools/screen/cut_game.py): LINE with B and BF, PSET, SELECT CASE, text in SCREEN 12: both flags, and the row of flags of the game's end screen.
DefInt A-Z
Screen 12
Call flags(1, 0, 0)
Call flags(2, -100, 300)
For i = -565 To 50 Step 50: Call flags(1, i, 100): Next i
System

Sub flags (who, w, a)
    x = 585 + w: y = 200: If w = 0 Then y = 180
    If a <> 0 Then y = a
    Select Case who
        Case 1
            Line (x - 17, y - 15)-(x + 17, y + 7), 4, BF
            For i = -13 To 9 Step 5
                Line (x - 17, y + i)-(x + 17, y + i - 1), 7, B
            Next i
            Line (x - 17, y - 15)-(x, y), 1, BF
            If w = 0 Then Color 9: Locate 10, 70: Print "U N I O N"
            For i = -16 To -1 Step 3
                For j = -14 To -1 Step 4: PSet (x + i, y + j), 7: Next j
            Next i
        Case 2
            Line (x - 17, y - 15)-(x + 17, y + 7), 4, BF
            Line (x - 17, y - 13)-(x + 15, y + 7), 7
            Line (x - 15, y - 15)-(x + 17, y + 5), 7

            Line (x - 17, y + 7)-(x + 17, y - 15), 1
            Line (x - 17, y + 6)-(x + 16, y - 15), 1
            Line (x - 16, y + 7)-(x + 17, y - 14), 1
            Line (x - 17, y + 5)-(x + 15, y - 15), 7
            Line (x - 15, y + 7)-(x + 17, y - 13), 7

            Line (x - 17, y - 15)-(x + 17, y + 7), 1
            Line (x - 17, y - 14)-(x + 16, y + 7), 1
            Line (x - 16, y - 15)-(x + 17, y + 6), 1
            Line (x - 17, y - 15)-(x + 17, y + 7), 4, B

            If w = 0 Then Color 4: Locate 10, 70: Print "R E B E L"
        Case Else
    End Select
End Sub
