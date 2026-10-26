' PAINT in each form: a fill colour, a border colour, both left out, STEP, a tile string, a background string.
Dim t As String, c As Long
Screen 12
Circle (60, 60), 40, 4
Paint (60, 60), 2, 4
Line (120, 20)-(200, 100), 5, B
c = 2
Paint (160, 60), c + 1, c + 3
Line (220, 20)-(300, 100), , B
Paint (260, 60)
Line (320, 20)-(400, 100), 6, B
Paint (360, 60), , 6
Line (420, 20)-(500, 100), 7, B
PSet (450, 60), 0
Paint Step(10, 0), 9, 7
Line (520, 20)-(600, 100), 9, B
PSet (550, 60), 0
Paint Step(10, 0), 9
Line (20, 120)-(100, 200), 8, B
Paint (60, 160), Chr$(170) + Chr$(85), 8
t = Chr$(255) + Chr$(0) + Chr$(0) + Chr$(255)
Line (120, 120)-(200, 200), 10, B
Paint (160, 160), t, 10
Line (220, 120)-(300, 200), 11, B
Paint (260, 160), 12, 11
Paint (260, 160), t, 11, Chr$(255)
Line (320, 120)-(400, 200), 13, B
PSet (350, 160), 0
Paint Step(10, 0), t + Chr$(60), 13
Line (420, 120)-(500, 200), 14, B
Paint (460, 160), 14
System
