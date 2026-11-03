' QB 4.5 (DIVERGENCES-QB45.md Q-002): SELECT CASE in a recursive FUNCTION, as in
' tests\corpus\slice\s28_select_case. A CASE item calls the FUNCTION again between the test
' and the later CASEs. QB64pe keeps one copy of the test value per SELECT, so rec(1) is 2;
' a copy per call makes it 1.
DECLARE FUNCTION rec! (n!)
DECLARE FUNCTION probe! (n!)
OPEN "OUT.TXT" FOR OUTPUT AS #1
PRINT #1, "rec(1) ="; rec(1)
PRINT #1, "rec(3) ="; rec(3)
CLOSE #1
SYSTEM

FUNCTION probe (n)
IF n = 1 THEN dummy = rec(2)
probe = -5
END FUNCTION

FUNCTION rec (n)
SELECT CASE n * 10
CASE probe(n): rec = -1
CASE 10: rec = 1
CASE 20: rec = 2
CASE 30: rec = 3
CASE ELSE: rec = 99
END SELECT
END FUNCTION
