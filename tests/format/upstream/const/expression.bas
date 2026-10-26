$Console:Only

Const const__OR = 1 Or 2
Const const__AND = 3 And 1
Const const__NOT = Not 2
Const const__XOR = 2 Xor 3
Const const__EQV = 10 Eqv 5
Const const__IMP = 20 Imp 50

Const const__mod = 20 Mod 3
Const const__add = 2 + 3
Const const__sub = 2 - 3
Const const__div = 6 / 3
Const const__idiv = 7 \ 3
Const const__mult = 7 * 20
Const const_pow = 3 ^ 10

Const const__negate = -20

Const const__eq = 2 = 2
Const const__neq = 2 <> 3
Const const__neq2 = 2 <> 3
Const const__leq = 2 <= 3
Const const__leq2 = 2 <= 3
Const const__geq = 2 >= 3
Const const__geq2 = 2 >= 3
Const const__gt = 2 > 3
Const const__lt = 2 < 3

' The left side which has no parens to indicate order should still equal the
' right side which has parens to enforce order.
Const const__oporder1 = (2 ^ 2 * 2) = ((2 ^ 2) * 2)
Const const__oporder2 = (2 ^ 2 + 2) = ((2 ^ 2) + 2)
Const const__oporder3 = (Not 2 + 3) = (Not (2 + 3))
Const const__oporder4 = (-2 ^ 2) = (-(2 ^ 2))
Const const__oporder5 = (Not 2 ^ 3) = (Not (2 ^ 3))
Const const__oporder6 = (3 * 6 / 2) = ((3 * 6) / 2)
Const const__oporder7 = (3 * 10 \ 3) = ((3 * 10) \ 3)

' Many levels of parens
Const const__parens = (2 ^ (3 * (4 - (2 - (10 / (20 / 2))))))

Const const__str = "foobar"
Const const__str2 = "foobar" + "foobar2"
Const const__str3 = const__str + const__str2
Const const__str4 = (const__str + (const__str2))

Const const__unsignedint = 2~&& * 5~&&

Const const__division_floating = 1& / 5&

Const const__negate_floating = -.25!

Print const__OR
Print const__AND
Print const__NOT
Print const__XOR
Print const__EQV
Print const__IMP

Print const__mod
Print const__add
Print const__sub
Print const__div
Print const__idiv
Print const__mult
Print const_pow

Print const__negate

Print const__eq
Print const__neq
Print const__neq2
Print const__leq
Print const__leq2
Print const__geq
Print const__geq2
Print const__gt
Print const__lt

Print const__oporder1
Print const__oporder2
Print const__oporder3
Print const__oporder4
Print const__oporder5
Print const__oporder6
Print const__oporder7

Print const__parens

Print const__str
Print const__str2
Print const__str3
Print const__str4

Print const__unsignedint

Print const__division_floating

Print const__negate_floating

System
