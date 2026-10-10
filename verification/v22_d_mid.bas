$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "The rest"): the MID$ statement with every start and length (0, negative,
' past the end, longer than the value or the target), without a length, a float start, fixed-length, element and
' member targets, the target used in its own value, a raising value.
ON ERROR GOTO h
TYPE rec
    s AS STRING * 5
END TYPE
DIM s AS STRING, fx AS STRING * 5, sa(2) AS STRING, r AS rec, n AS LONG, d AS DOUBLE
s = "abcdef": MID$(s, 3) = "XY": PRINT "no length ["; s; "]"
s = "abcdef": MID$(s, 3, 1) = "XY": PRINT "length 1 ["; s; "]"
s = "abcdef": MID$(s, 3, 5) = "XY": PRINT "length beyond the value ["; s; "]"
s = "abcdef": MID$(s, 5) = "VWXYZ": PRINT "value beyond the end ["; s; "]"
s = "abcdef": MID$(s, 5, 9) = "VWXYZ": PRINT "length and value beyond the end ["; s; "]"
s = "abcdef": MID$(s, 1) = "": PRINT "empty value ["; s; "]"
s = "abcdef": MID$(s, 6) = "Z": PRINT "last position ["; s; "]"
s = "abcdef": MID$(s, 3, 0) = "XY": PRINT "length 0 ["; s; "]"
s = "q": MID$(s, 1, 1) = "ZZ": PRINT "one byte ["; s; "]"
s = "abcdef": PRINT "start 7 (one past the end)": MID$(s, 7) = "Z": PRINT "["; s; "]"
s = "abcdef": PRINT "start 8": MID$(s, 8) = "Z": PRINT "["; s; "]"
s = "abcdef": PRINT "start 0": MID$(s, 0) = "Z": PRINT "["; s; "]"
s = "abcdef": PRINT "start -1": MID$(s, -1) = "Z": PRINT "["; s; "]"
s = "abcdef": PRINT "length -1": MID$(s, 2, -1) = "XYZ": PRINT "["; s; "]"
s = "": PRINT "empty target, start 1": MID$(s, 1) = "Z": PRINT "["; s; "]"
s = "abcdef": d = 2.5: MID$(s, d, d) = "XYZ": PRINT "start and length 2.5 ["; s; "]"
s = "abcdef": d = 3.5: MID$(s, d) = "XYZ": PRINT "start 3.5 ["; s; "]"
s = "abcdef": MID$(s, 2, 3) = s: PRINT "its own value ["; s; "]"
s = "abcdef": MID$(s, 1, 2) = MID$(s, 5, 2): PRINT "its own MID$ ["; s; "]"
fx = "abcde": MID$(fx, 4) = "XYZ": PRINT "fixed-length ["; fx; "]"
sa(1) = "abcdef": n = 1: MID$(sa(n), 2, 2) = "XY": PRINT "element ["; sa(1); "]"
r.s = "abcde": MID$(r.s, 2, 2) = "XY": PRINT "member ["; r.s; "]"
s = "abcdef": PRINT "raising value": MID$(s, 2) = CHR$(-1): PRINT "["; s; "]"
s = "abcdef": PRINT "raising start": MID$(s, ASC("")) = "Z": PRINT "["; s; "]"
n = 9: PRINT "element with a bad index": MID$(sa(n), 1) = "Z": PRINT "["; sa(1); "]"
s = "abcdef": MID$(s, 2, 2) = "X" + "Y" + "Z": PRINT "an expression ["; s; "]"
s = "abcdef": MID$ (s, 2) = "sp": PRINT "a blank before ( ["; s; "]"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
