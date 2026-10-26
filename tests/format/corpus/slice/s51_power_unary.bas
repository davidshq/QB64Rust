$Console:Only
' A minus right of the power operator (spec compiler/parser; found by upstream operators_test/op_exp): it takes
' what follows up to the next operator that is not ^, a negation or NOT, so a NOT inside it reaches no further:
' 2 ^ -NOT x + 3 is (2 ^ (-(NOT x))) + 3, though -NOT x + 3 alone is -(NOT (x + 3)) and 2 ^ NOT x + 3 is
' 2 ^ (NOT (x + 3)). The same lines as verification/v24_j_power_unary.
Dim x As Long, y As Long
x = 0: y = 3
Print "1. 2 ^ -NOT x + 3:"; 2 ^ -Not x + 3
Print "2. 2 ^ (-NOT x) + 3:"; 2 ^ (-Not x) + 3
Print "3. 2 ^ -(NOT x + 3):"; 2 ^ -(Not x + 3)
Print "4. 2 ^ -y + 3:"; 2 ^ -y + 3
Print "5. 2 ^ -y * 3:"; 2 ^ -y * 3
Print "6. 2 ^ NOT x + 3:"; 2 ^ Not x + 3
Print "7. 2 ^ NOT x * 3:"; 2 ^ Not x * 3
Print "8. -NOT x + 3:"; -Not x + 3
Print "9. NOT x + 3:"; Not x + 3
Print "10. - NOT x * 3:"; -Not x * 3
Print "11. 2 ^ -NOT x * 3:"; 2 ^ -Not x * 3
Print "12. 2 ^ -NOT x ^ 2:"; 2 ^ -Not x ^ 2
Print "13. 2 ^ --y:"; 2 ^ y
Print "14. 2 ^ -NOT x = 2:"; 2 ^ -Not x = 2
Print "15. 2 ^ -NOT x AND 7:"; 2 ^ -Not x And 7
Print "17. 2 * -NOT x + 3:"; 2 * -Not x + 3
Print "18. 2 + -NOT x + 3:"; 2 + -Not x + 3
Print "19. 2 ^ -NOT -x + 3:"; 2 ^ -Not -x + 3
Print "20. y ^ -NOT x + 3:"; y ^ -Not x + 3
Print "21. 2 ^ -NOT 1 ^ 3:"; 2 ^ -Not 1 ^ 3
Print "22. 2 ^ -3 ^ 2:"; 2 ^ -3 ^ 2
Print "23. 2 ^ - -y + 1:"; 2 ^ y + 1
Print "24. 2 ^ -NOT NOT x + 3:"; 2 ^ -Not Not x + 3
Print "25. 2 ^ -y MOD 2:"; 2 ^ -y Mod 2
Print "26. 2 ^ -y ^ 2 + 1:"; 2 ^ -y ^ 2 + 1
Print "27. 2 ^ -NOT x OR 8:"; 2 ^ -Not x Or 8
Print "28. -2 ^ -NOT x + 3:"; -2 ^ -Not x + 3
System
