' Cut from Civil War Strategy (tools/screen/cut_game.py): graphics GET and PUT, VIEW, CLS 1: a message box drawn and taken away again. TICK waits for a key, which the keys file gives.
defint A-Z
dim shared mtx$(21), tlx, tly, size
screen 12
line (100, 100)-(540, 300), 2, bf
circle (320, 200), 90, 14
call image2("A message over the drawing", 4)
system

sub image2 (a$, s)
    dim g2(1 to 8000)
    mtx$(1) = a$
    tlx = 32 - len(a$) \ 2
    tly = 10: size = 1
    call mxw(wide)
    if wide > 59 then wide = 59: a$ = left$(a$, 59)
    x = 8 * (tlx + wide)
    y = 16 * (tly + 1)
    get (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 15, 16 * (tly + 3) + 7), g2()
    view (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3) + 4)
    cls 1
    view
    line (8 * tlx, 16 * tly - 1)-(8 * (tlx + wide + 1) + 15, 16 * (tly + 3) + 7), 0, bf
    line (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3)), s, bf
    line (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3)), 15, b
    color 14
    locate tly + 2, tlx + 2: print a$
    call TICK(9)
    put (8 * tlx - 4, 16 * tly - 3), g2(), pset
end sub

sub mxw (wide)
    wide = len(mtx$(0)) + 1
    for i = 1 to size
        x = len(mtx$(i))
        if x > wide then wide = x
    next i
end sub

sub TICK (sec!)
    start! = timer
    do while timer - start! < sec! and inkey$ = "": loop
end sub
