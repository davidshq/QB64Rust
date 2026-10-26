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
        	CASE 1
            	LINE (x - 17, y - 15)-(x + 17, y + 7), 4, BF
            	FOR i = -13 TO 9 STEP 5
                	LINE (x - 17, y + i)-(x + 17, y + i - 1), 7, B
            	NEXT i
            	LINE (x - 17, y - 15)-(x, y), 1, BF
            	IF w = 0 THEN COLOR 9: LOCATE 10, 70: PRINT "U N I O N"
            	FOR i = -16 TO -1 STEP 3
                		FOR j = -14 TO -1 STEP 4: PSET (x + i, y + j), 7: NEXT j
            	NEXT i
        	CASE 2
            	LINE (x - 17, y - 15)-(x + 17, y + 7), 4, BF
            	LINE (x - 17, y - 13)-(x + 15, y + 7), 7
            	LINE (x - 15, y - 15)-(x + 17, y + 5), 7

            	LINE (x - 17, y + 7)-(x + 17, y - 15), 1
            	LINE (x - 17, y + 6)-(x + 16, y - 15), 1
            	LINE (x - 16, y + 7)-(x + 17, y - 14), 1
            	LINE (x - 17, y + 5)-(x + 15, y - 15), 7
            	LINE (x - 15, y + 7)-(x + 17, y - 13), 7

            	LINE (x - 17, y - 15)-(x + 17, y + 7), 1
            	LINE (x - 17, y - 14)-(x + 16, y + 7), 1
            	LINE (x - 16, y - 15)-(x + 17, y + 6), 1
            	LINE (x - 17, y - 15)-(x + 17, y + 7), 4, B

            	IF w = 0 THEN COLOR 4: LOCATE 10, 70: PRINT "R E B E L"
        	CASE ELSE
    End Select
End Sub
