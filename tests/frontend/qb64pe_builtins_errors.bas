' TEST: check-fail
$CONSOLE:ONLY
' The built-ins of the upstream programs qb64pe/* rejected (m4-qb64pe-sources task 2.1; verification\v52_x50-x64):
' _INSTRREV with one or four arguments, a number as the string, a string as the start; _READFILE$ of a number;
' _WRITEFILE with one argument, a number as the data, in parentheses, as a function; _READFILE$ as a statement;
' _FULLPATH$ with two arguments; _ARCSEC of a string; _SECH with two arguments
PRINT _INSTRREV("a")
PRINT _INSTRREV(1, "a", "b", 2)
PRINT _INSTRREV(1, 2, "a")
PRINT _INSTRREV("1", "a", "b")
PRINT _READFILE$(1)
_WRITEFILE "f"
_WRITEFILE "f", 5
_WRITEFILE ("f", "a")
x = _WRITEFILE("f", "a")
_READFILE$ "a"
PRINT _FULLPATH$(".", ".")
PRINT _ARCSEC("a")
PRINT _SECH(1, 2)
