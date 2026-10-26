Dim src As Long, dst As Long
src = _NewImage(100, 100, 32)
dst = 0
_PutImage (0, 0)-(100, 100), src, dst
