' TEST: check-fail
$CONSOLE:ONLY
' The $ name beside a STRING * n array (m4-qb64pe-sources task 5.1): DIM of an array of its own is the old
' compiler's "Type mismatch" (verification\v52_x45), and so is a REDIM (v66_c12); COMMON SHARED s$ beside a
' STRING * n variable s is not measured
DIM a(3) AS STRING * 4
DIM a$(3)
REDIM c(2) AS STRING * 2
REDIM c$(4)
DIM s AS STRING * 4
COMMON SHARED s$
