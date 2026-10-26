' Cut from Civil War Strategy (tools/screen/cut_game.py): the end screen without its music (noise is 0): CIRCLE arc, DRAW with a scale and in loops, GOTO, LOCATE and PRINT over a filled box.
DefInt A-Z
Call capitol
System

Sub capitol
    Screen 12
    Cls
    Line (1, 1)-(639, 150), 4, BF
    Line (1, 150)-(639, 300), 15, BF
    Line (1, 300)-(639, 450), 1, BF
    x = 0
    Line (150, 190)-(500, 254), 7, B
    Line (270, 190)-(370, 254), 7, B
    Line (270, 185)-(370, 175), 7, B
    Line (280, 173)-(360, 168), 7, B
    Line (270, 235)-(370, 254), 7, B
    Circle (320, 150), 74, 7, .2, 2.95, 2.1
    Line (285, 138)-(355, 168), 7, B
    lwing:
    PSet (180 + 120 * x, 200), 0

    Draw "C7ER1E1R1E1R1E1R1E1R1E1R1E1R1E1R1E1R1E1R1E1R1F1R1F1R1F1"
    Draw "R1F1R1F1R1F1R1F1R1F1R1F1R1F1R1F1R1F1L47D4R48U3D51L4U17L41D17L3U47"
    For i = 1 To 7: Draw "BR6R2D25L2U25": Next i
    Draw "BD33R2"
    For i = 1 To 5: Draw "L40BR40D3": Next i
    x = x + 1: If x < 3 GoTo lwing
    PSet (270, 190), 0
    For i = 1 To 3: Draw "C7BR6R2D45L2U45": Next i
    PSet (344, 190), 0
    For i = 1 To 3: Draw "C7BR6R2D45L2U45": Next i
    PSet (283, 140), 0
    For i = 1 To 11: Draw "C7BR6R2D25L2U25": Next i
    PSet (283, 120), 0
    For i = 1 To 11: Draw "C7BR6R2D15L2U15": Next i
    Line (315, 55)-(325, 77), 7, B
    Line (318, 57)-(322, 75), 7, B
    PSet (315, 53), 0
    Draw "S3C7R13U6L13D5BU5C7U2E1U2E2U3H2U3H1E3U2H1U2E2U2E2F2D2F4D3R2E2F1G4L1D7F1D2F1D1G2"
    Line (140, 270)-(510, 430), 14, BF
    Color 15
    If noise = 2 Then Play "T130MFMSO2f16f8.f16f8.e-16d8.f16b-8.o3c16d8.d16d8.c16o2b-4"
    Locate 20, 20: Print Chr$(34); "... and that the government of the people,"
    If noise = 2 Then Play "o2b-8.a16g8.g16g8.a16b-8.a16b-8.g16f8.g16f8.d16f4"
    Locate 21, 20: Print "by the people, for the people,"
    If noise = 2 Then Play "f8.f16f8.f16f8.e-16d8.f16b-8.o3c16d8.d16d8.c16o2b-4b-4MNo3c4c4o2b-4a4b-2"
    Locate 22, 20: Print "shall not perish from the earth."; Chr$(34)
    Locate 25, 40: Print "- Abraham Lincoln"
    For i = -565 To 50 Step 50: Call flags(1, i, 0): Next i
    If noise = 2 Then Play "P2f4..e-16d8.f16b-8.o3c16d2o2b-2g4..a16b-8.a16b-8.g16f2d2": Play "f4..e-16d8.f16b-8.o3c16d2o2b-4b-4o3c4c4o2b-4a4b-2.."
End Sub

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
