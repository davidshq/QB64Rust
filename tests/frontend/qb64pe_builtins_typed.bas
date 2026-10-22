' TEST: typed
$CONSOLE:ONLY
' The built-ins of the upstream programs qb64pe/* in the typed tree (m4-qb64pe-sources task 2.1; measured in
' verification\v52_a_instrrev, v52_b_math, v52_c_files): _INSTRREV with and without a start (a LONG, the start a
' float converted as the LONG slot), _READFILE$ and _FULLPATH$ (strings), _WRITEFILE (two string slots), and
' _ARCSEC, _ARCCSC, _ARCCOT, _SECH, _CSCH of an integer, a SINGLE and a DOUBLE (believed _FLOAT)
DIM a AS STRING, n AS LONG, s AS SINGLE, d AS DOUBLE, i AS INTEGER, f AS _FLOAT
DIM fx AS STRING * 4
n = _INSTRREV(a, ".")
n = _INSTRREV(3, a, fx)
n = _INSTRREV(s, "a.b", ".")
a = _READFILE$(a)
a = _FULLPATH$(fx + ".")
_WRITEFILE a, _READFILE$("x")
_WRITEFILE fx, a + "b"
f = _ARCSEC(i)
f = _ARCCSC(s)
d = _ARCCOT(d)
f = _SECH(n)
f = _CSCH(f)
' m4-qb64pe-builtins task 3.1 (verification v53_d_keys): _KEYHIT bare, _KEYDOWN of a LONG and of a SINGLE (converted
' as the LONG slot), each a LONG
n = _KEYHIT
n = _KEYDOWN(100304)
i = _KEYDOWN(s)
' m4-qb64pe-builtins task 4.2 (verification v53_a_window): _RESIZE, _RESIZEWIDTH, _RESIZEHEIGHT, _EXIT,
' _ACCEPTFILEDROP and _TOTALDROPPEDFILES bare, each a LONG; _DROPPEDFILE with and without $, bare and with an index
' (a DOUBLE converted as the LONG slot), a string
n = _RESIZE
n = _RESIZEWIDTH + _RESIZEHEIGHT
n = _EXIT
n = _ACCEPTFILEDROP
n = _TOTALDROPPEDFILES
a = _DROPPEDFILE$
a = _DROPPEDFILE
a = _DROPPEDFILE$(n)
a = _DROPPEDFILE(d)
' m4-qb64pe-builtins task 4.3 (never run; verification v53_x29-x31, x38): _MESSAGEBOX bare, with two and five
' arguments (the last a LONG slot) and with slots left out, a LONG; the file dialogs, strings, bare and with
' arguments (a fixed-length string, an INTEGER for the last slot of _OPENFILEDIALOG$)
n = _MESSAGEBOX
n = _MESSAGEBOX("t", "m")
n = _MESSAGEBOX(a, fx, "yesno", "warning", d)
n = _MESSAGEBOX(, "m", , , 0)
a = _OPENFILEDIALOG$
a = _OPENFILEDIALOG$("t", a, "*.bas", "BASIC", i)
a = _SAVEFILEDIALOG$(fx)
a = _SELECTFOLDERDIALOG$
a = _SELECTFOLDERDIALOG$("t", a)
' m4-qb64pe-builtins task 5.1 (verification v53_b_colours): _CONTROLCHR bare, a LONG; _DEFAULTCOLOR and
' _BACKGROUNDCOLOR bare and of a handle (a LONG, a SINGLE converted as the LONG slot), each an _UNSIGNED LONG
DIM c AS _UNSIGNED LONG
n = _CONTROLCHR
c = _DEFAULTCOLOR
c = _DEFAULTCOLOR(n)
c = _BACKGROUNDCOLOR
c = _BACKGROUNDCOLOR(s)
' m4-qb64pe-builtins task 6.1 (verification v53_c_system): _DIR$ of a string and of a fixed-length string;
' _SHELLHIDE, an _INTEGER64; _COMMANDCOUNT, a LONG; _COMPILEDATE$ and _COMPILETIME$, strings; _READBIT of a LONG,
' an _UNSIGNED _INTEGER64 and a SINGLE (converted as the _UNSIGNED _INTEGER64 slot), with a DOUBLE bit number,
' an _INTEGER64
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64
a = _DIR$("fonts")
a = _DIR$(fx)
q = _SHELLHIDE("cmd /c exit 3")
n = _COMMANDCOUNT
a = _COMPILEDATE$ + _COMPILETIME$
q = _READBIT(n, 0)
q = _READBIT(uq, 63)
q = _READBIT(s, d)
