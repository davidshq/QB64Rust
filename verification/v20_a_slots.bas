$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): how an argument is converted to its slot, once per slot type.
' LONG slot (LEFT$, CHR$, MID$, STRING$): a float literal, a float variable, values beyond LONG, an _INTEGER64.
' _FLOAT slot (SQR, _ATAN2, _HYPOT): an integer argument. DOUBLE slot (_PI). any-numeric slot (STR$).
ON ERROR GOTO h
DIM s AS STRING
s = "abcdefghij"
PRINT "LEFT$ 2.5: ["; LEFT$(s, 2.5); "]"
PRINT "LEFT$ 3.5: ["; LEFT$(s, 3.5); "]"
PRINT "LEFT$ 1.5: ["; LEFT$(s, 1.5); "]"
PRINT "LEFT$ 2.6: ["; LEFT$(s, 2.6); "]"
PRINT "LEFT$ -0.5: ["; LEFT$(s, -0.5); "]"
PRINT "LEFT$ -0.6: ["; LEFT$(s, -0.6); "]"
DIM f AS SINGLE
f = 2.5
PRINT "LEFT$ SINGLE var 2.5: ["; LEFT$(s, f); "]"
DIM d AS DOUBLE
d = 3.5
PRINT "LEFT$ DOUBLE var 3.5: ["; LEFT$(s, d); "]"
DIM fl AS _FLOAT
fl = 4.5
PRINT "LEFT$ _FLOAT var 4.5: ["; LEFT$(s, fl); "]"
PRINT "LEFT$ 3E9: ["; LEFT$(s, 3E9); "]"
PRINT "LEFT$ 4294967298#: ["; LEFT$(s, 4294967298#); "]"
PRINT "LEFT$ -4294967294#: ["; LEFT$(s, -4294967294#); "]"
DIM q AS _INTEGER64
q = 4294967298
PRINT "LEFT$ _INTEGER64 4294967298: ["; LEFT$(s, q); "]"
q = -4294967294
PRINT "LEFT$ _INTEGER64 -4294967294: ["; LEFT$(s, q); "]"
PRINT "LEFT$ 4294967298&&: ["; LEFT$(s, 4294967298&&); "]"
PRINT "CHR$ 65.5: ["; CHR$(65.5); "]"
PRINT "CHR$ 66.5: ["; CHR$(66.5); "]"
PRINT "CHR$ 4294967361&&: ["; CHR$(4294967361&&); "]"
PRINT "MID$ start 1.5 len 2.5: ["; MID$(s, 1.5, 2.5); "]"
PRINT "STRING$ 3.5, 65.5: ["; STRING$(3.5, 65.5); "]"
PRINT "SQR 2%:"; SQR(2%)
PRINT "SQR 2&:"; SQR(2&)
PRINT "SQR 2!:"; SQR(2!)
PRINT "SQR 2#:"; SQR(2#)
PRINT "SQR 2##:"; SQR(2##)
PRINT "SQR 2&&:"; SQR(2&&)
q = 9007199254740993
PRINT "SQR(q) * SQR(q) - q for q = 2^53 + 1:"; SQR(q) * SQR(q) - q
PRINT "_ATAN2 1%, 2%:"; _ATAN2(1%, 2%)
PRINT "_ATAN2 1#, 2#:"; _ATAN2(1#, 2#)
PRINT "_HYPOT 1%, 1%:"; _HYPOT(1%, 1%)
PRINT "_HYPOT 1#, 1#:"; _HYPOT(1#, 1#)
PRINT "_PI(2%):"; _PI(2%)
PRINT "_PI(1.5!):"; _PI(1.5!)
PRINT "STR$ 2.5!: ["; STR$(2.5!); "]"
PRINT "STR$ 2.5#/3: ["; STR$(2.5# / 3); "]"
PRINT "STR$ 2.5!/3: ["; STR$(2.5! / 3); "]"
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
