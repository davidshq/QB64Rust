$CONSOLE
' Verification (m4-statements-metacommands, task 1.1): RUN after SCREEN 13 and COLOR: the screen, colours and palette after.
$SCREENHIDE
c = 0
IF _FILEEXISTS("v57_r10.tmp") THEN OPEN "v57_r10.tmp" FOR INPUT AS #1: INPUT #1, c: CLOSE #1
c = c + 1
OPEN "v57_r10.tmp" FOR OUTPUT AS #1: PRINT #1, c: CLOSE #1
_DEST _CONSOLE
PRINT "run"; c; "width"; _WIDTH(0); "height"; _HEIGHT(0); "pixel"; _PIXELSIZE(0)
_DEST _CONSOLE: PRINT "color"; _DEFAULTCOLOR(0); "bg"; _BACKGROUNDCOLOR(0); "pal 1"; HEX$(_PALETTECOLOR(1, 0))
IF c < 2 THEN SCREEN 13: COLOR 4, 2: PALETTE 1, 63: RUN
KILL "v57_r10.tmp"
SYSTEM
