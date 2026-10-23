$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): _DIR$ of each kind of name (its last character, whether the folder
' exists, whether it is the desktop's; never the path, design D5), in another case and with blanks; _SHELLHIDE's
' exit codes (0, 3, 300, a command that does not exist) and its result type, an empty command; _COMMANDCOUNT with
' no arguments; the shape of _COMPILEDATE$ and _COMPILETIME$ (each digit printed as 9, never the value; the day of
' _COMPILEDATE$, which is one digit after a blank before the 10th, only as "there"); _READBIT
' of each type, bit 0 to 2, bit 63, a negative value, a SINGLE value and a SINGLE bit number.
ON ERROR GOTO handler
desk$ = _DIR$("desktop")
FOR i = 1 TO 22
    READ n$
    d$ = _DIR$(n$)
    PRINT "["; n$; "]"; RIGHT$(d$, 1) = "\"; _DIREXISTS(d$); d$ = desk$
NEXT
DATA desktop,documents,music,pictures,videos,downloads,appdata,localappdata,programfiles,programfiles x86
DATA temp,home,fonts,userfont,programdata,bogus,"",Fonts,  fonts,my documents,text,TMP
PRINT _DIR$("fonts") = _DIR$("FONT"); _DIR$("home") = _DIR$("user profile")
PRINT _SHELLHIDE("cmd /c exit 0")
PRINT _SHELLHIDE("cmd /c exit 3")
PRINT _SHELLHIDE("cmd /c exit 300")
PRINT _SHELLHIDE("cmd /c exit 2") * 1000000000000
PRINT _SHELLHIDE("qb64rust_no_such_command_v53") <> 0
r&& = 77
r&& = _SHELLHIDE("")
PRINT r&&
PRINT _COMMANDCOUNT
cd$ = _COMPILEDATE$
PRINT shape$(LEFT$(cd$, 3)); ASC(cd$, 4); shape$(LTRIM$(MID$(cd$, 5, 2))) <> ""; ASC(cd$, 7); shape$(RIGHT$(cd$, 4)); LEN(cd$)
PRINT shape$(_COMPILETIME$); LEN(_COMPILETIME$)
PRINT _READBIT(5, 0); _READBIT(5, 1); _READBIT(5, 2)
b%% = -1: PRINT _READBIT(b%%, 7); _READBIT(b%%, 8); _READBIT(b%%, 63)
ub~%% = 255: PRINT _READBIT(ub~%%, 7); _READBIT(ub~%%, 8)
i% = -2: PRINT _READBIT(i%, 0); _READBIT(i%, 1); _READBIT(i%, 40)
l& = &H40000000: PRINT _READBIT(l&, 30); _READBIT(l&, 31)
i64&& = -9223372036854775808: PRINT _READBIT(i64&&, 63); _READBIT(i64&&, 62)
u~&& = &HFFFFFFFFFFFFFFFF~&&: PRINT _READBIT(u~&&, 63); _READBIT(u~&&, 0)
s! = 5.7: PRINT _READBIT(s!, 0); _READBIT(s!, 1); _READBIT(s!, 2)
d# = 6.5: PRINT _READBIT(d#, 0); _READBIT(d#, 1)
bit! = 1.6: PRINT _READBIT(5, bit!)
x&& = _READBIT(1, 0): PRINT x&&
SYSTEM

handler:
PRINT "error"; ERR
RESUME NEXT

FUNCTION shape$ (s$)
    t$ = ""
    FOR k = 1 TO LEN(s$)
        c$ = MID$(s$, k, 1)
        IF c$ >= "0" AND c$ <= "9" THEN
            t$ = t$ + "9"
        ELSEIF UCASE$(c$) >= "A" AND UCASE$(c$) <= "Z" THEN
            t$ = t$ + "a"
        ELSE
            t$ = t$ + c$
        END IF
    NEXT
    shape$ = t$
END FUNCTION
