' Cut from Civil War Strategy (tools/screen/cut_game.py): TAB in PRINT in SCREEN 12, after LOCATE and with numbers: the game's city report, the lines of SUB report from its label cityrep on, over values written here.
DefInt A-Z
Dim Shared month$(12), city$(40), cityp(40), cityv(40), force$(2), control(2), income(2), cash(2)
Screen 12
month = 4: year = 1861: month$(4) = "April"
force$(0) = "NEUTRAL": force$(1) = "Union": force$(2) = "Rebel"
control(1) = 14: control(2) = 12: income(1) = 420: income(2) = 310: cash(1) = 1250: cash(2) = 980
For i = 1 To 40
    city$(i) = "City" + LTrim$(Str$(i)): cityp(i) = i Mod 3: cityv(i) = 5 * i
Next i
Cls: Locate 1, 1: Color 14
Print "City Report"; Tab(20); month$(month); ","; year: Color 15
Print " #    City       Control   Value"; Tab(41); " #    City       Control   Value"
Line (1, 30)-(630, 30), 15
For i = 1 To 20: Color 4: a$ = "neutral": If cityp(i) = 1 Then Color 9: a$ = "UNION"
    If cityp(i) = 2 Then Color 7: a$ = "REBEL"
    Locate 2 + i, 1: Print i; Tab(6); city$(i); Tab(18); a$; Tab(29); cityv(i)
Next i

For i = 21 To 40: Color 4: a$ = "neutral": If cityp(i) = 1 Then Color 9: a$ = "UNION"
    If cityp(i) = 2 Then Color 7: a$ = "REBEL"
    Locate i - 18, 41: Print i; Tab(46); city$(i); Tab(58); a$; Tab(69); cityv(i)
Next i
Line (1, 360)-(630, 360), 15
Color 9: Locate 24, 1: Print "Side    No. Cities   Income    Cash"
Color 9: Print force$(1); Tab(12); control(1); Tab(23); income(1); Tab(32); cash(1)
Color 7: Print force$(2); Tab(12); control(2); Tab(23); income(2); Tab(32); cash(2)
Color 4: Print force$(0); Tab(12); 40 - control(1) - control(2)
Line (1, 382)-(630, 382), 15
System
