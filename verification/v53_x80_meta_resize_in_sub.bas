' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x80_meta_resize_in_sub)
' $RESIZE:ON in a SUB.
s
PRINT "ok"
SYSTEM
SUB s
$RESIZE:ON
END SUB
