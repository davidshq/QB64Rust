' Cut from Civil War Strategy (tools/screen/cut_game.py): graphics GET and PUT, VIEW, CLS 1: a message box drawn and taken away again. TICK waits for a key, which the keys file gives.
DefInt A-Z
Dim Shared mtx$(21), tlx, tly, size
Screen 12
Line (100, 100)-(540, 300), 2, BF
Circle (320, 200), 90, 14
Call image2("A message over the drawing", 4)
System

Sub image2 (a$, s)
    Dim g2(1 To 8000)
    mtx$(1) = a$
    tlx = 32 - Len(a$) \ 2
    tly = 10: size = 1
    Call mxw(wide)
    If wide > 59 Then wide = 59: a$ = Left$(a$, 59)
    x = 8 * (tlx + wide)
    y = 16 * (tly + 1)
    	GET (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 15, 16 * (tly + 3) + 7), g2
    	VIEW (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3) + 4)
    	CLS 1
    	VIEW
    	LINE (8 * tlx, 16 * tly - 1)-(8 * (tlx + wide + 1) + 15, 16 * (tly + 3) + 7), 0, BF
    	LINE (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3)), s, BF
    	LINE (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3)), 15, B
    	COLOR 14
    	LOCATE tly + 2, tlx + 2: PRINT a$
    	CAlL TICK(9)
    	PUT (8 * tlx - 4, 16 * tly - 3), g2, PSET
End Sub

Sub mxw (wide)
    	wide = LEN(mtx$(0)) + 1
    	FOR i = 1 TO size
        	x = LEN(mtx$(i))
        	IF x > wide THEN wide = x
    	NEXT i
End Sub

Sub TICK (sec!)
    start! = Timer
    Do While Timer - start! < sec! And InKey$ = "": Loop
End Sub
