$CONSOLE:ONLY
d$ = DATE$
PRINT "Ten characters, mm-dd-yyyy:"; LEN(d$); MID$(d$, 3, 1); MID$(d$, 6, 1)
DATE$ = "01-02-2003"
PRINT "DATE$ = changes nothing:"; DATE$ = d$ OR DATE$ <> "01-02-2003"
END
