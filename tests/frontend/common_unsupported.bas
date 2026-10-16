' TEST: check-fail
$CONSOLE:ONLY
' COMMON and array parameters (m2-defxxx-common-arrays tasks 7.1, 7.2): what is marked "not supported yet", each
' form not measured with the old compiler: COMMON a() of an array that exists already; an array parameter named
' with $n; an _UNSIGNED STRING array parameter; two array parameters of one name and two types
DIM a(3) AS LONG
COMMON a() AS LONG
SUB s1 (t$3())
END SUB
SUB s2 (t() AS _UNSIGNED STRING)
END SUB
SUB s3 (t() AS LONG, t() AS INTEGER)
END SUB
