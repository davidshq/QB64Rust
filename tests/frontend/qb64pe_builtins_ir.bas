' TEST: ir
$CONSOLE:ONLY
' _WRITEFILE in the IR (m4-qb64pe-sources task 2.1; verification\v52_c_files): two string slots, a variable-length
' and a fixed-length string
DIM f AS STRING, fx AS STRING * 4
_WRITEFILE f, "data"
_WRITEFILE fx + ".x", f + _READFILE$(f)
' m4-qb64pe-builtins task 3.1 (verification v53_d_keys): _KEYCLEAR with no argument (its slot not passed), with a
' LONG and with a DOUBLE converted to the LONG slot
DIM d AS DOUBLE
_KEYCLEAR
_KEYCLEAR 2
_KEYCLEAR d
' m4-qb64pe-builtins task 4.2 (verification v53_a_window): _RESIZE in each form of its template, _CONSOLE ON and
' OFF, _ACCEPTFILEDROP bare, ON and OFF, _FINISHDROP and CALL _FINISHDROP
_RESIZE ON
_RESIZE OFF
_RESIZE ON, _STRETCH
_RESIZE , _SMOOTH
_RESIZE
_CONSOLE ON
_CONSOLE OFF
_ACCEPTFILEDROP
_ACCEPTFILEDROP ON
_ACCEPTFILEDROP OFF
_FINISHDROP
CALL _FINISHDROP
' m4-qb64pe-builtins task 4.3 (never run; verification v53_x30, x41): _SCREENMOVE with coordinates, _MIDDLE and
' alone; _MESSAGEBOX with one, two and three arguments, with its first slots left out, and alone
_SCREENMOVE 10, d
_SCREENMOVE _MIDDLE
_SCREENMOVE
_MESSAGEBOX "t"
_MESSAGEBOX "t", "m"
_MESSAGEBOX "t", "m", "warning"
_MESSAGEBOX , , "info"
_MESSAGEBOX
' m4-qb64pe-builtins task 5.1 (screen program c16_controlchr): _CONTROLCHR OFF and ON
_CONTROLCHR OFF
_CONTROLCHR ON
' Review follow-up of m4-qb64pe-builtins (verification v53_c_system): a call of _COMPILEDATE$ marks the program's
' settings, so its build is never taken from a build cache
PRINT _COMPILEDATE$
