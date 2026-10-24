$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): the statements DATE$ = and TIME$ = (libqb's sub_date and sub_time, which change
' nothing), read back with the functions; a variable and an empty string as the value.
d$ = DATE$
DATE$ = "01-02-2003"
PRINT DATE$ = d$; LEN(DATE$)
t$ = LEFT$(TIME$, 5)
TIME$ = "12:34:56"
PRINT LEFT$(TIME$, 5) >= t$ OR t$ = "23:59"; LEN(TIME$)
x$ = "01-01-2001": DATE$ = x$: TIME$ = x$
DATE$ = "": TIME$ = ""
DATE$ = DATE$: TIME$ = TIME$
PRINT DATE$ = d$
PRINT "ok"
SYSTEM
