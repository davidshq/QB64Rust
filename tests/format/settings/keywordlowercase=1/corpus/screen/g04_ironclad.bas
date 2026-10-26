' Cut from Civil War Strategy (tools/screen/cut_game.py): LINE -(x, y) chains, PAINT with and without a border colour.
defint A-Z
call ironclad
system

sub ironclad
    screen 12
    line (1, 240)-(639, 309), 3, bf
    line (639, 309)-(1, 479), 1, bf

    color 7
    line (478, 368)-(174, 389): line -(86, 363): line -(78, 353)
    line -(161, 337): line -(162, 337): line -(406, 326): line -(478, 368)
    paint (300, 350), 7

    color 15
    line (427, 358)-(203, 373): line -(247, 322): line -(205, 312)
    line -(167, 342): line -(203, 373): line -(246, 323): line -(401, 318)
    line -(427, 358): line -(400, 318): line -(363, 306): line -(204, 312)
    paint (300, 315), 7, 15
    paint (200, 325), 8, 15
    paint (300, 325), 8, 15

    color 7
    line (77, 353)-(76, 363): line -(84, 370): line -(85, 363)
    line -(84, 369): line -(169, 396): line -(170, 389): line -(168, 396)
    line -(477, 375): line -(478, 369)
    paint (80, 363), 7
    paint (150, 385), 8, 7
    paint (190, 390), 0, 7

    color 0
    line (299, 261)-(294, 254): line -(303, 250): line -(299, 243)
    line -(316, 243): line -(321, 247): line -(326, 241): line -(335, 240)
    line -(345, 240): line -(357, 246): line -(366, 241): line -(381, 245)
    line -(384, 253): line -(398, 248): line -(410, 248): line -(413, 259)
    line -(424, 264): line -(425, 254): line -(444, 258): line -(445, 270)
    line -(436, 277): line -(426, 281): line -(418, 270): line -(411, 272)
    line -(400, 263): line -(392, 275): line -(374, 270): line -(370, 262)
    line -(357, 257): line -(345, 256): line -(335, 257): line -(335, 262)
    line -(324, 257): line -(317, 258): line -(309, 257): line -(305, 262)
    line -(300, 261)
    paint (320, 250), 0

    line (211, 341)-(196, 357): line -(179, 346)
    line -(194, 335): line -(211, 341)
    paint (190, 350), 0
    line (211, 341)-(225, 330), 7: line -(209, 324), 7: line -(188, 340), 0

    for k = 0 to 3
        x = 260 + 40 * k: y = 344 - 2 * k
        line (x, y)-(x + 2, y + 20): line -(x + 17, y + 19): line -(x + 15, y)
        line -(x, y)
        paint (x + 10, 350), 0
        line (x, y)-(x + 15, y), 7: line -(x + 13, y - 14), 7: line -(x - 2, y - 14), 7: line -(x, y)
    next k

    color 8
    line (168, 340)-(85, 355): line -(91, 361): line -(173, 386): line -(466, 366)
    line -(414, 334)

    line (358, 307)-(382, 315): line -(249, 320): line -(220, 314)
    line -(358, 307)

    line (314, 255)-(330, 314), 8, bf
    line (297, 310)-(309, 262), 8, bf
    line (323, 257)-(329, 314), 0, bf
    line (303, 310)-(308, 262), 0, bf
    line (319, 257)-(320, 315), 15, bf
    line (300, 262)-(301, 312), 15, bf

    line (96, 297)-(94, 357), 8, bf
    line (98, 299)-(106, 317), 4, bf
    line (107, 304)-(110, 319), 9, bf
    line (111, 300)-(114, 312), 15, bf

    y = 387: for x = 180 to 470 step 15: y = y - 1: line (x, y)-(x + 1, y + 1), 0, bf: next x
    y = 360: for x = 90 to 170 step 15: y = y + 4: line (x, y)-(x + 1, y + 1), 0, bf: next x
    y = 373: for x = 216 to 250 step 7: y = y - 8: line (x, y)-(x + 1, y + 1), 0, bf: next x
    y = 373: for x = 205 to 243 step 7: y = y - 8: line (x, y)-(x + 1, y + 1), 0, bf: next x
    y = 350: for x = 172 to 200 step 9: y = y - 8: line (x, y)-(x + 1, y + 1), 0, bf: next x
    y = 310: for x = 397 to 425 step 8: y = y + 11: line (x, y)-(x + 1, y + 1), 0, bf: next x
    color 11
    line (70, 361)-(68, 369): line -(106, 384): line -(166, 394)
    line -(210, 385): line -(255, 391): line -(219, 394)
    line -(242, 402): line -(202, 405): line -(162, 402)
    line -(121, 387)
    paint (190, 400), 3, 11

    line (331, 391)-(365, 387)
    line -(403, 375): line -(446, 382): line -(471, 384)
    line -(446, 391): line -(478, 395): line -(449, 407)
    line -(445, 406): line -(414, 395): line -(412, 395)
    line -(369, 396): line -(323, 391)
    paint (390, 390), 3, 11

    line (476, 379)-(531, 372)
    line (420, 329)-(497, 327), 3
    line -(464, 340): line -(535, 343), 3
    line (141, 337)-(67, 349): line -(58, 372)

    line (525, 405)-(587, 392): line -(628, 402): line -(588, 415)
    line -(563, 409): line -(562, 409): line -(527, 423)
    line -(540, 410): line -(523, 406)
    paint (560, 405), 9, 11

    line (106, 320)-(101, 318), 0, bf
    line (110, 313)-(114, 315), 0, bf
end sub
