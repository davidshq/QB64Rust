$CONSOLE:ONLY
' Verification (found in m2-defxxx-common-arrays task 3.3, upstream operators_test/op_exp): what a unary minus, NOT
' and _NEGATE take as their operand when they stand right of the power operator, and what follows them.
DIM x AS LONG, y AS LONG
x = 0: y = 3
PRINT "1. 2 ^ -NOT x + 3:"; 2 ^ -NOT x + 3
PRINT "2. 2 ^ (-NOT x) + 3:"; 2 ^ (-NOT x) + 3
PRINT "3. 2 ^ -(NOT x + 3):"; 2 ^ -(NOT x + 3)
PRINT "4. 2 ^ -y + 3:"; 2 ^ -y + 3
PRINT "5. 2 ^ -y * 3:"; 2 ^ -y * 3
PRINT "6. 2 ^ NOT x + 3:"; 2 ^ NOT x + 3
PRINT "7. 2 ^ NOT x * 3:"; 2 ^ NOT x * 3
PRINT "8. -NOT x + 3:"; -NOT x + 3
PRINT "9. NOT x + 3:"; NOT x + 3
PRINT "10. - NOT x * 3:"; -NOT x * 3
PRINT "11. 2 ^ -NOT x * 3:"; 2 ^ -NOT x * 3
PRINT "12. 2 ^ -NOT x ^ 2:"; 2 ^ -NOT x ^ 2
PRINT "13. 2 ^ --y:"; 2 ^ --y
PRINT "14. 2 ^ -NOT x = 2:"; 2 ^ -NOT x = 2
PRINT "15. 2 ^ -NOT x AND 7:"; 2 ^ -NOT x AND 7
PRINT "17. 2 * -NOT x + 3:"; 2 * -NOT x + 3
PRINT "18. 2 + -NOT x + 3:"; 2 + -NOT x + 3
PRINT "19. 2 ^ -NOT -x + 3:"; 2 ^ -NOT -x + 3
PRINT "20. y ^ -NOT x + 3:"; y ^ -NOT x + 3
PRINT "21. 2 ^ -NOT 1 ^ 3:"; 2 ^ -NOT 1 ^ 3
PRINT "22. 2 ^ -3 ^ 2:"; 2 ^ -3 ^ 2
PRINT "23. 2 ^ - -y + 1:"; 2 ^ - -y + 1
PRINT "24. 2 ^ -NOT NOT x + 3:"; 2 ^ -NOT NOT x + 3
PRINT "25. 2 ^ -y MOD 2:"; 2 ^ -y MOD 2
PRINT "26. 2 ^ -y ^ 2 + 1:"; 2 ^ -y ^ 2 + 1
PRINT "27. 2 ^ -NOT x OR 8:"; 2 ^ -NOT x OR 8
PRINT "28. -2 ^ -NOT x + 3:"; -2 ^ -NOT x + 3
SYSTEM
