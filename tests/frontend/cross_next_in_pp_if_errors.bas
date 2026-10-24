' TEST: check-fail
$CONSOLE:ONLY
' A NEXT inside a $IF of an included file, whose FOR stands in this file before the include: the old compiler's
' "NEXT without FOR" (verification/v57_b21_next_in_pp_if_inc). One error, at the NEXT: the NEXT still closes its
' FOR, and the variable after it is part of it.
FOR i = 1 TO 2
'$INCLUDE:'inc/cross_next_in_pp_if.bi'
PRINT "after"
SYSTEM
