$CONSOLE:ONLY
' Verification (m2-numeric-types, D1): does the old compiler accept this? (bit_param_uncalled)
PRINT 1
SUB s (x AS _BIT * 5)
PRINT x
END SUB
