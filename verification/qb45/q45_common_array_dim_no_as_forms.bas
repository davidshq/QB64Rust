' QB 4.5 (beside q45_common_array_dim_no_as, D-063): the same DIM without AS of an array a COMMON ... AS lists,
' as DIM SHARED, and after a COMMON without SHARED. The compile messages show which lines are errors.
COMMON SHARED g() AS DOUBLE
COMMON h() AS LONG
OPEN "OUT.TXT" FOR OUTPUT AS #1
DIM SHARED g(3)
DIM h(2)
PRINT #1, "built"
CLOSE #1
SYSTEM
