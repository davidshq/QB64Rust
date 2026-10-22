$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): _INSTRREV with and without a start: the last match, a start
' position, no match, an empty string, an empty search string, a search string longer than the string, the whole
' string, a start of 0, below 0, at the length and beyond it, a start between two matches, a start inside a
' match, a numeric start of another type, and the type of the result.
PRINT _INSTRREV("a.b.c", ".")
PRINT _INSTRREV(3, "a.b.c", ".")
PRINT _INSTRREV("abc", "x")
PRINT _INSTRREV("", "a")
PRINT _INSTRREV("", "")
PRINT _INSTRREV("abc", "")
PRINT _INSTRREV(2, "abc", "")
PRINT _INSTRREV(0, "abc", "")
PRINT _INSTRREV(9, "abc", "")
PRINT _INSTRREV("ab", "abc")
PRINT _INSTRREV("abc", "abc")
PRINT _INSTRREV(1, "abc", "abc")
PRINT _INSTRREV(0, "a.b.c", ".")
PRINT _INSTRREV(-5, "a.b.c", ".")
PRINT _INSTRREV(5, "a.b.c", ".")
PRINT _INSTRREV(6, "a.b.c", ".")
PRINT _INSTRREV(100, "a.b.c", ".")
PRINT _INSTRREV(1, "a.b.c", ".")
PRINT _INSTRREV(2, "a.b.c", ".")
PRINT _INSTRREV(4, "abcabc", "bc")
PRINT _INSTRREV(5, "abcabc", "bc")
PRINT _INSTRREV(6, "abcabc", "bc")
PRINT _INSTRREV("aaaa", "aa")
PRINT _INSTRREV(3, "aaaa", "aa")
s! = 3.7
PRINT _INSTRREV(s!, "a.b.c", ".")
d# = 2.5
PRINT _INSTRREV(d#, "a.b.c", ".")
x& = 2147483647
PRINT _INSTRREV(x&, "a.b.c", ".")
a$ = "x\y\z"
p = _INSTRREV(a$, "\")
PRINT p; LEN(STR$(_INSTRREV(a$, "\")))
r& = _INSTRREV(a$, "\") * 1000000
PRINT r&
SYSTEM
