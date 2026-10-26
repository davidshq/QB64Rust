' Cut from Civil War Strategy (tools/screen/cut_game.py): LINE with B and BF, PSET, SELECT CASE, text in SCREEN 12: both flags, and the row of flags of the game's end screen.
defint A-Z
screen 12
call flags(1, 0, 0)
call flags(2, -100, 300)
for i = -565 to 50 step 50: call flags(1, i, 100): next i
system

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
