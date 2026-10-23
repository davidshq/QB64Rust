$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x83_lbound_sub_name)
PRINT "ok"
SYSTEM
PRINT LBOUND(s)
SUB s
END SUB
