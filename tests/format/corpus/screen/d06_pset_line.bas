' PSET, PRESET and LINE in each form: STEP, arguments left out, B and BF, a style, a line without its first point.
Screen 7
PSet (10, 10)
PSet (12, 10), 4
PSet Step(2, 0)
PSet Step(2, 0), 2
PReset (12, 10)
PReset (30, 10), 5
PReset Step(2, 2), 6
PReset Step(-20, 0)
Line (20, 20)-(60, 40)
Line (20, 50)-(60, 70), 3
Line (70, 20)-(110, 40), 4, B
Line (70, 50)-(110, 70), 5, BF
Line (120, 20)-(160, 40), , B
Line (120, 50)-(160, 70), , BF
Line (170, 20)-(210, 40), 6, B , &H5555
Line (170, 50)-(210, 70), 7, , &HF0F0
Line (220, 20)-(260, 40), , , &HFF00
Line (220, 50)-(260, 70), , B , &H3333
Line -(300, 100)
Line -(280, 120), 9
Line -Step(-20, 10)
Line -Step(-20, 10), , B
Line Step(5, 5)-(100, 150)
Line Step(5, 5)-Step(30, 10)
Line Step(5, 5)-Step(30, 10), 10, BF , &HAAAA
Line (10, 160)-Step(40, 20), 11
Line Step(10, 0)-Step(20, 10), 13, B
Line (300, 190)-(310, 199), 12, BF
Line (290, 150)-(400, 250), 14
Line (-20, 100)-(20, 120), 15, B
System
