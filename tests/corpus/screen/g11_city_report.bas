' Cut from Civil War Strategy (tools/screen/cut_game.py): TAB in PRINT in SCREEN 12, after LOCATE and with numbers: the game's city report, the lines of SUB report from its label cityrep on, over values written here.
DEFINT A-Z
DIM SHARED month$(12), city$(40), cityp(40), cityv(40), force$(2), control(2), income(2), cash(2)
SCREEN 12
month = 4: year = 1861: month$(4) = "April"
force$(0) = "NEUTRAL": force$(1) = "Union": force$(2) = "Rebel"
control(1) = 14: control(2) = 12: income(1) = 420: income(2) = 310: cash(1) = 1250: cash(2) = 980
FOR i = 1 TO 40
city$(i) = "City" + LTRIM$(STR$(i)): cityp(i) = i MOD 3: cityv(i) = 5 * i
NEXT i
CLS : LOCATE 1, 1: COLOR 14
PRINT "City Report"; TAB(20); month$(month); ","; year: COLOR 15
PRINT " #    City       Control   Value"; TAB(41); " #    City       Control   Value"
LINE (1, 30)-(630, 30), 15
FOR i = 1 TO 20: COLOR 4: a$ = "neutral": IF cityp(i) = 1 THEN COLOR 9: a$ = "UNION"
IF cityp(i) = 2 THEN COLOR 7: a$ = "REBEL"
LOCATE 2 + i, 1: PRINT i; TAB(6); city$(i); TAB(18); a$; TAB(29); cityv(i)
NEXT i

FOR i = 21 TO 40: COLOR 4: a$ = "neutral": IF cityp(i) = 1 THEN COLOR 9: a$ = "UNION"
IF cityp(i) = 2 THEN COLOR 7: a$ = "REBEL"
LOCATE i - 18, 41: PRINT i; TAB(46); city$(i); TAB(58); a$; TAB(69); cityv(i)
NEXT i
LINE (1, 360)-(630, 360), 15
COLOR 9: LOCATE 24, 1: PRINT "Side    No. Cities   Income    Cash"
COLOR 9: PRINT force$(1); TAB(12); control(1); TAB(23); income(1); TAB(32); cash(1)
COLOR 7: PRINT force$(2); TAB(12); control(2); TAB(23); income(2); TAB(32); cash(2)
COLOR 4: PRINT force$(0); TAB(12); 40 - control(1) - control(2)
LINE (1, 382)-(630, 382), 15
SYSTEM
