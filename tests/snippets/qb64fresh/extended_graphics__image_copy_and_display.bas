DIM src AS LONG, dst AS LONG
src = _NEWIMAGE(100, 100, 32)
dst = 0
_PUTIMAGE (0, 0)-(100, 100), src, dst
