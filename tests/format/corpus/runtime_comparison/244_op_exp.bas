$Console:Only
' Runtime comparison: operator precedence (NOT, ^, * before +)
Print (2 ^ 2 * 2); ((2 ^ 2) * 2)
Print (3 * 6 / 2); ((3 * 6) / 2)
Print (3 * 10 \ 3); ((3 * 10) \ 3)
Print (Not 0); (Not 1)
Print (-2 ^ 2); (-(2 ^ 2))
End
