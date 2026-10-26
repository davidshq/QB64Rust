' Graphics GET and PUT in SCREEN 12: each action word over a background, no action, a colour, _CLIP at two edges,
' STEP; the array whole and from an element, of two dimensions, of LONG, dynamic, and in a SUB as a parameter, a
' local and a STATIC array; a PUT that leaves the screen and a GET into an array that is too small raise error 5.
defint A-Z
dim shared errs
dim a(150), m(1 to 4, 1 to 40), l(80) as long, small(3)
dim n
screen 12
on error goto handler
' The picture: a filled box with two diagonals and a dot.
line (10, 10)-(30, 26), 4, bf
line (10, 10)-(30, 26), 14
line (10, 26)-(30, 10), 10
pset (20, 18), 15
get (10, 10)-(30, 26), a()
' A background for the actions.
line (0, 60)-(639, 100), 3, bf
put (50, 10), a()
put (50, 70), a()
put (100, 70), a(), pset
put (150, 70), a(), preset
put (200, 70), a(), and
put (250, 70), a(), or
put (300, 70), a(), xor
put (350, 70), a(), , 4
put (400, 70), a(), pset , 14
put (450, 70), a(), _clip, 10
put (630, 70), a(), _clip pset
put (-10, 200), a(), _clip
pset (100, 150), 0
put step(10, 10), a(), pset
' From an element; two dimensions; LONG; dynamic arrays.
get (10, 10)-(20, 18), a(60)
put (150, 150), a(60), pset
get (10, 10)-(30, 26), m()
put (200, 150), m(), pset
get (10, 10)-(30, 26), m(1, 2)
put (230, 150), m(1, 2), pset
get (10, 10)-(30, 26), l()
put (260, 150), l(), pset
n = 150
dim dy(n)
get (10, 10)-(30, 26), dy()
put (300, 150), dy(), pset
redim rd(1 to 150)
pset (10, 10), 0
get step(0, 0)-step(20, 16), rd(n - 149)
put (350, 150), rd(1), or
' In a SUB.
call stamp(a(), 400, 150)
' Errors.
put (630, 300), a(), pset
get (10, 10)-(30, 26), small()
put (10, 300), small(), pset
get (10, 10)-(700, 26), a()
locate 25, 1: print "errors"; errs
system
handler:
errs = errs + 1
locate 20 + errs, 1: print "error"; err
resume next

sub stamp (p(), x, y)
    dim lc(150)
    static st(150)
    put (x, y), p(), pset
    get (x, y)-(x + 20, y + 16), lc()
    put (x + 50, y), lc(), pset
    get (10, 10)-(30, 26), st(2)
    put (x + 100, y), st(2), xor
    put (x + 150, y), p(), preset
end sub
