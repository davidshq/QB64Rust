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
' m4-qb64pe-builtins task 3.1 (verification v53_x10-x17): _KEYCLEAR of a string, with two arguments, as a function;
' _KEYHIT with an argument, as a statement; _KEYDOWN bare, of a string, as a statement
_KEYCLEAR "a"
_KEYCLEAR 1, 2
x = _KEYCLEAR
x = _KEYHIT(1)
_KEYHIT
x = _KEYDOWN
x = _KEYDOWN("a")
_KEYDOWN 1
' m4-qb64pe-builtins task 4.2 (verification v53_x01-x09, x18-x25): _CONSOLE as a statement bare, with a number,
' with two words, as a function with an argument; _RESIZE with two words, a number, _MIDDLE, as a function with an
' argument; _RESIZEWIDTH as a statement; _DROPPEDFILE of a string, with two arguments, as a statement;
' _ACCEPTFILEDROP with a number, as a function with an argument; _FINISHDROP with an argument, as a function;
' _TOTALDROPPEDFILES as a statement
_CONSOLE
_CONSOLE 1
_CONSOLE ON, OFF
x = _CONSOLE(1)
_RESIZE ON, ON
_RESIZE 1
_RESIZE ON, _MIDDLE
x = _RESIZE(1)
_RESIZEWIDTH
f$ = _DROPPEDFILE("a")
f$ = _DROPPEDFILE$(1, 2)
_DROPPEDFILE
_ACCEPTFILEDROP 1
x = _ACCEPTFILEDROP(1)
_FINISHDROP 1
x = _FINISHDROP
_TOTALDROPPEDFILES
' m4-qb64pe-builtins task 4.3 (verification v53_x26-x28, x32-x37, x39, x40, x42, x43): _MESSAGEBOX with six
' arguments, as a statement with four, with a number for the icon; the file dialogs without $, of a number, as a
' statement, with too many arguments; _SCREENMOVE _MIDDLE with coordinates, with one coordinate, as a function, of
' strings
r& = _MESSAGEBOX("a", "b", "ok", "info", 1, 2)
_MESSAGEBOX "a", "b", "ok", "info"
_MESSAGEBOX "a", "b", 1
f$ = _OPENFILEDIALOG("t")
f$ = _OPENFILEDIALOG$(1)
_OPENFILEDIALOG$ "t"
f$ = _OPENFILEDIALOG$("t", "", "*.bas", "BASIC", 0, 1)
f$ = _SAVEFILEDIALOG$("a", "b", "c", "d", "e")
f$ = _SELECTFOLDERDIALOG$("a", "b", "c")
_SCREENMOVE _MIDDLE, 10, 20
_SCREENMOVE 10
x = _SCREENMOVE
_SCREENMOVE "a", "b"
' m4-qb64pe-builtins task 5.1 (verification v53_x44-x49): _CONTROLCHR as a statement bare, with a number, as a
' function with an argument; _DEFAULTCOLOR of a string, with two arguments; _BACKGROUNDCOLOR as a statement
_CONTROLCHR
_CONTROLCHR 1
x = _CONTROLCHR(1)
x = _DEFAULTCOLOR("a")
x = _DEFAULTCOLOR(1, 2)
_BACKGROUNDCOLOR 1
' m4-qb64pe-builtins task 6.1 (verification v53_x50-x61, x64): _DIR without $, of a number, bare; _SHELLHIDE of a
' number, bare, as a statement; _COMMANDCOUNT with an argument; _COMPILEDATE without $; _COMPILETIME$ with an
' argument; _READBIT with one argument, of a string, with a string bit number; _COMPILEDATE$ as a statement
d$ = _DIR("fonts")
d$ = _DIR$(1)
d$ = _DIR$
x = _SHELLHIDE(1)
x = _SHELLHIDE
_SHELLHIDE "cmd /c exit 0"
x = _COMMANDCOUNT(1)
d$ = _COMPILEDATE
d$ = _COMPILETIME$(1)
x = _READBIT(1)
x = _READBIT("a", 1)
x = _READBIT(1, "a")
_COMPILEDATE$
