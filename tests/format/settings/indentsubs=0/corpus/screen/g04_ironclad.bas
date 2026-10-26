' Cut from Civil War Strategy (tools/screen/cut_game.py): LINE -(x, y) chains, PAINT with and without a border colour.
DefInt A-Z
Call ironclad
System

Sub ironclad
Screen 12
Line (1, 240)-(639, 309), 3, BF
Line (639, 309)-(1, 479), 1, BF

Color 7
Line (478, 368)-(174, 389): Line -(86, 363): Line -(78, 353)
Line -(161, 337): Line -(162, 337): Line -(406, 326): Line -(478, 368)
Paint (300, 350), 7

Color 15
Line (427, 358)-(203, 373): Line -(247, 322): Line -(205, 312)
Line -(167, 342): Line -(203, 373): Line -(246, 323): Line -(401, 318)
Line -(427, 358): Line -(400, 318): Line -(363, 306): Line -(204, 312)
Paint (300, 315), 7, 15
Paint (200, 325), 8, 15
Paint (300, 325), 8, 15

Color 7
Line (77, 353)-(76, 363): Line -(84, 370): Line -(85, 363)
Line -(84, 369): Line -(169, 396): Line -(170, 389): Line -(168, 396)
Line -(477, 375): Line -(478, 369)
Paint (80, 363), 7
Paint (150, 385), 8, 7
Paint (190, 390), 0, 7

Color 0
Line (299, 261)-(294, 254): Line -(303, 250): Line -(299, 243)
Line -(316, 243): Line -(321, 247): Line -(326, 241): Line -(335, 240)
Line -(345, 240): Line -(357, 246): Line -(366, 241): Line -(381, 245)
Line -(384, 253): Line -(398, 248): Line -(410, 248): Line -(413, 259)
Line -(424, 264): Line -(425, 254): Line -(444, 258): Line -(445, 270)
Line -(436, 277): Line -(426, 281): Line -(418, 270): Line -(411, 272)
Line -(400, 263): Line -(392, 275): Line -(374, 270): Line -(370, 262)
Line -(357, 257): Line -(345, 256): Line -(335, 257): Line -(335, 262)
Line -(324, 257): Line -(317, 258): Line -(309, 257): Line -(305, 262)
Line -(300, 261)
Paint (320, 250), 0

Line (211, 341)-(196, 357): Line -(179, 346)
Line -(194, 335): Line -(211, 341)
Paint (190, 350), 0
Line (211, 341)-(225, 330), 7: Line -(209, 324), 7: Line -(188, 340), 0

For k = 0 To 3
    x = 260 + 40 * k: y = 344 - 2 * k
    Line (x, y)-(x + 2, y + 20): Line -(x + 17, y + 19): Line -(x + 15, y)
    Line -(x, y)
    Paint (x + 10, 350), 0
    Line (x, y)-(x + 15, y), 7: Line -(x + 13, y - 14), 7: Line -(x - 2, y - 14), 7: Line -(x, y)
Next k

Color 8
Line (168, 340)-(85, 355): Line -(91, 361): Line -(173, 386): Line -(466, 366)
Line -(414, 334)

Line (358, 307)-(382, 315): Line -(249, 320): Line -(220, 314)
Line -(358, 307)

Line (314, 255)-(330, 314), 8, BF
Line (297, 310)-(309, 262), 8, BF
Line (323, 257)-(329, 314), 0, BF
Line (303, 310)-(308, 262), 0, BF
Line (319, 257)-(320, 315), 15, BF
Line (300, 262)-(301, 312), 15, BF

Line (96, 297)-(94, 357), 8, BF
Line (98, 299)-(106, 317), 4, BF
Line (107, 304)-(110, 319), 9, BF
Line (111, 300)-(114, 312), 15, BF

y = 387: For x = 180 To 470 Step 15: y = y - 1: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
y = 360: For x = 90 To 170 Step 15: y = y + 4: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
y = 373: For x = 216 To 250 Step 7: y = y - 8: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
y = 373: For x = 205 To 243 Step 7: y = y - 8: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
y = 350: For x = 172 To 200 Step 9: y = y - 8: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
y = 310: For x = 397 To 425 Step 8: y = y + 11: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
Color 11
Line (70, 361)-(68, 369): Line -(106, 384): Line -(166, 394)
Line -(210, 385): Line -(255, 391): Line -(219, 394)
Line -(242, 402): Line -(202, 405): Line -(162, 402)
Line -(121, 387)
Paint (190, 400), 3, 11

Line (331, 391)-(365, 387)
Line -(403, 375): Line -(446, 382): Line -(471, 384)
Line -(446, 391): Line -(478, 395): Line -(449, 407)
Line -(445, 406): Line -(414, 395): Line -(412, 395)
Line -(369, 396): Line -(323, 391)
Paint (390, 390), 3, 11

Line (476, 379)-(531, 372)
Line (420, 329)-(497, 327), 3
Line -(464, 340): Line -(535, 343), 3
Line (141, 337)-(67, 349): Line -(58, 372)

Line (525, 405)-(587, 392): Line -(628, 402): Line -(588, 415)
Line -(563, 409): Line -(562, 409): Line -(527, 423)
Line -(540, 410): Line -(523, 406)
Paint (560, 405), 9, 11

Line (106, 320)-(101, 318), 0, BF
Line (110, 313)-(114, 315), 0, BF
End Sub
