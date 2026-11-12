$CONSOLE:ONLY
x = 2.5: PRINT "Before DEFINT, x ="; x
DEFINT A-Z
y = 2.5: PRINT "After DEFINT A-Z, y ="; y; "in"; LEN(y); "bytes"
DEFDBL D
d = 1 / 3: PRINT "After DEFDBL D, d ="; d
DEFSTR S
s = "text": PRINT "After DEFSTR S, s = "; s
w! = 2.5: PRINT "A suffix wins: w! ="; w!
END
