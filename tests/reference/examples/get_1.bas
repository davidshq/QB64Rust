$CONSOLE:ONLY
TYPE Rec
    nm AS STRING * 8
    men AS LONG
END TYPE
DIM r AS Rec, back AS Rec
OPEN "get_1.tmp" FOR RANDOM AS #1 LEN = LEN(r)
r.nm = "Davout": r.men = 26000: PUT #1, 1, r
r.nm = "Lannes": r.men = 20000: PUT #1, 2, r
GET #1, 2, back
PRINT back.nm; back.men
GET #1, 1, back
PRINT back.nm; back.men
CLOSE #1
KILL "get_1.tmp"
END
