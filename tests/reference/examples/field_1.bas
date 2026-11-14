$CONSOLE:ONLY
OPEN "field_1.tmp" FOR RANDOM AS #1 LEN = 12
FIELD #1, 8 AS nm$, 4 AS men$
LSET nm$ = "Soult"
LSET men$ = MKL$(30000)
PUT #1, 1
LSET nm$ = "": LSET men$ = ""
GET #1, 1
PRINT "["; nm$; "]"; CVL(men$)
CLOSE #1
KILL "field_1.tmp"
END
