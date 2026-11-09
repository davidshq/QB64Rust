' QB 4.5 (beside SOMEDAY's row on signs in a row): a unary plus before a variable, and doubled signs. QB64pe
' rejects +b, 2 * +b and (+b), drops + + and two - after an operator, so 2 * - -i% never negates. The compile
' messages show which lines are errors.
OPEN "OUT.TXT" FOR OUTPUT AS #1
b = 5: i% = 3
x = +b
PRINT #1, "1. +b:"; x
x = 2 * +b
PRINT #1, "2. 2 * +b:"; x
x = (+b)
PRINT #1, "3. (+b):"; x
x = 2 * - -i%
PRINT #1, "4. 2 * - -i%:"; x
x = b + +b
PRINT #1, "5. b + +b:"; x
x = -+b
PRINT #1, "6. -+b:"; x
x = - -b
PRINT #1, "7. - -b:"; x
x = 2 - -b
PRINT #1, "8. 2 - -b:"; x
CONST c1 = +-3
PRINT #1, "9. CONST c1 = +-3:"; c1
CONST c2 = 2 * - -3
PRINT #1, "10. CONST c2 = 2 * - -3:"; c2
CLOSE #1
SYSTEM
