' Cut from Civil War Strategy (tools/screen/cut_game.py): the end screen without its music (noise is 0): CIRCLE arc, DRAW with a scale and in loops, GOTO, LOCATE and PRINT over a filled box.
defint A-Z
call capitol
system

sub capitol
    screen 12
    cls
    line (1, 1)-(639, 150), 4, bf
    line (1, 150)-(639, 300), 15, bf
    line (1, 300)-(639, 450), 1, bf
    x = 0
    line (150, 190)-(500, 254), 7, b
    line (270, 190)-(370, 254), 7, b
    line (270, 185)-(370, 175), 7, b
    line (280, 173)-(360, 168), 7, b
    line (270, 235)-(370, 254), 7, b
    circle (320, 150), 74, 7, .2, 2.95, 2.1
    line (285, 138)-(355, 168), 7, b
    lwing:
    pset (180 + 120 * x, 200), 0

    draw "C7ER1E1R1E1R1E1R1E1R1E1R1E1R1E1R1E1R1E1R1E1R1F1R1F1R1F1"
    draw "R1F1R1F1R1F1R1F1R1F1R1F1R1F1R1F1R1F1L47D4R48U3D51L4U17L41D17L3U47"
    for i = 1 to 7: draw "BR6R2D25L2U25": next i
    draw "BD33R2"
    for i = 1 to 5: draw "L40BR40D3": next i
    x = x + 1: if x < 3 goto lwing
    pset (270, 190), 0
    for i = 1 to 3: draw "C7BR6R2D45L2U45": next i
    pset (344, 190), 0
    for i = 1 to 3: draw "C7BR6R2D45L2U45": next i
    pset (283, 140), 0
    for i = 1 to 11: draw "C7BR6R2D25L2U25": next i
    pset (283, 120), 0
    for i = 1 to 11: draw "C7BR6R2D15L2U15": next i
    line (315, 55)-(325, 77), 7, b
    line (318, 57)-(322, 75), 7, b
    pset (315, 53), 0
    draw "S3C7R13U6L13D5BU5C7U2E1U2E2U3H2U3H1E3U2H1U2E2U2E2F2D2F4D3R2E2F1G4L1D7F1D2F1D1G2"
    line (140, 270)-(510, 430), 14, bf
    color 15
    if noise = 2 then play "T130MFMSO2f16f8.f16f8.e-16d8.f16b-8.o3c16d8.d16d8.c16o2b-4"
    locate 20, 20: print chr$(34); "... and that the government of the people,"
    if noise = 2 then play "o2b-8.a16g8.g16g8.a16b-8.a16b-8.g16f8.g16f8.d16f4"
    locate 21, 20: print "by the people, for the people,"
    if noise = 2 then play "f8.f16f8.f16f8.e-16d8.f16b-8.o3c16d8.d16d8.c16o2b-4b-4MNo3c4c4o2b-4a4b-2"
    locate 22, 20: print "shall not perish from the earth."; chr$(34)
    locate 25, 40: print "- Abraham Lincoln"
    for i = -565 to 50 step 50: call flags(1, i, 0): next i
    if noise = 2 then play "P2f4..e-16d8.f16b-8.o3c16d2o2b-2g4..a16b-8.a16b-8.g16f2d2": play "f4..e-16d8.f16b-8.o3c16d2o2b-4b-4o3c4c4o2b-4a4b-2.."
end sub

sub flags (who, w, a)
    x = 585 + w: y = 200: if w = 0 then y = 180
    if a <> 0 then y = a
    select case who
        case 1
            line (x - 17, y - 15)-(x + 17, y + 7), 4, bf
            for i = -13 to 9 step 5
                line (x - 17, y + i)-(x + 17, y + i - 1), 7, b
            next i
            line (x - 17, y - 15)-(x, y), 1, bf
            if w = 0 then color 9: locate 10, 70: print "U N I O N"
            for i = -16 to -1 step 3
                for j = -14 to -1 step 4: pset (x + i, y + j), 7: next j
            next i
        case 2
            line (x - 17, y - 15)-(x + 17, y + 7), 4, bf
            line (x - 17, y - 13)-(x + 15, y + 7), 7
            line (x - 15, y - 15)-(x + 17, y + 5), 7

            line (x - 17, y + 7)-(x + 17, y - 15), 1
            line (x - 17, y + 6)-(x + 16, y - 15), 1
            line (x - 16, y + 7)-(x + 17, y - 14), 1
            line (x - 17, y + 5)-(x + 15, y - 15), 7
            line (x - 15, y + 7)-(x + 17, y - 13), 7

            line (x - 17, y - 15)-(x + 17, y + 7), 1
            line (x - 17, y - 14)-(x + 16, y + 7), 1
            line (x - 16, y - 15)-(x + 17, y + 6), 1
            line (x - 17, y - 15)-(x + 17, y + 7), 4, b

            if w = 0 then color 4: locate 10, 70: print "R E B E L"
        case else
    end select
end sub
