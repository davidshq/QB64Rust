DECLARE SUB armyxy (x%, y%, z%)
DECLARE SUB barnacle (who%)
DECLARE SUB flags (who%, w%, a%)
DECLARE SUB integrity ()
DECLARE SUB ironclad ()
DECLARE SUB normal (xbar%, vary%, result%)
DECLARE SUB relieve (who%)
DECLARE SUB retreat (active%, x%)
DECLARE SUB schooner ()
DECLARE SUB shiptype (who%, i%)
DECLARE SUB stax (who%)
DECLARE SUB tinytrain (who%, flag%)
DECLARE SUB traincapacity (who%, limit%)
DECLARE SUB topbar ()
DECLARE SUB mxw (wide%)
DECLARE SUB choices (b1%, wide%)
DECLARE SUB scrnsavr ()
DECLARE SUB topbar ()
DECLARE SUB armyxy (x%, y%, z%)
DECLARE SUB center (y%, a$)
DECLARE SUB flags (who%, w%, a%)
DECLARE SUB image2 (a$, s%)
DECLARE SUB maptext ()
DECLARE SUB mountain (xx%, yy%)
DECLARE SUB mxw (wide%)
DECLARE SUB roman (target%, a$)
DECLARE SUB stax (who%)
DECLARE SUB animate (index%, flag%)
DECLARE SUB armies ()
DECLARE SUB armystat (index%)
DECLARE SUB armyxy (x%, y%, z%)
DECLARE SUB banner (a$)
DECLARE SUB battle (attack%, defend%, win%, lose%)
DECLARE SUB blastem ()
DECLARE SUB bub2 (limit%)
DECLARE SUB bubble (size%)
DECLARE SUB cancel (side%)
DECLARE SUB cannon ()
DECLARE SUB capitol ()
DECLARE SUB capture (active%, c%, s%, flag%)
DECLARE SUB chessie ()
DECLARE SUB clrbot ()
DECLARE SUB combine (who%)
DECLARE SUB clrrite ()
DECLARE SUB commander (who%, empty%)
DECLARE SUB cutoff (who%, target%, a%)
DECLARE SUB endit ()
DECLARE SUB engine ()
DECLARE SUB evaluate (index%, x%)
DECLARE SUB events ()
DECLARE SUB filer (switch%)
DECLARE SUB flags (who%, w%, a%)
DECLARE SUB flashcity (which%)
DECLARE SUB fortify (target%)
DECLARE SUB icon (from%, dest%, kind%)
DECLARE SUB iterate ()
DECLARE SUB maxx ()
DECLARE SUB menu (switch%)
DECLARE SUB movefrom (index%, target%)
DECLARE SUB navy (who%, x%)
DECLARE SUB newarmy (who%, empty%, target%)
DECLARE SUB newcity (index%)
DECLARE SUB occupy (x%)
DECLARE SUB placearmy (which%)
DECLARE SUB railroad (who%)
DECLARE SUB recruit (who%)
DECLARE SUB report (who%)
DECLARE SUB resupply (index%)
DECLARE SUB rwin ()
DECLARE SUB scribe (a$, flag%)
DECLARE SUB shen ()
DECLARE SUB ships ()
DECLARE SUB shipicon (who%, flag%)
DECLARE SUB smarts ()
DECLARE SUB showcity ()
DECLARE SUB snapshot (x%, y%, flag%)
DECLARE SUB sndblst (file$, flag%)
DECLARE SUB sndfx (flag%)
DECLARE SUB starfin (star%, fin%, who%)
DECLARE SUB strong (index%)
DECLARE SUB surrender (who%)
DECLARE SUB TICK (sec!)
DECLARE SUB touchup ()
DECLARE SUB tupdate ()
DECLARE SUB usa ()
DECLARE SUB usamap ()
DECLARE SUB victor ()
DECLARE SUB void (a%, y%)
DECLARE SUB war ()
DefInt A-Z
'==========================================================================
Common Shared choose%, tlx%, tly%, size%, mtx$(), wtype%, colour%, hilite%
Common Shared cityx(), cityy(), cityv(), cityp(), city$(), lname$(), filel
Common Shared cash(), control(), matrix(), income(), anima(), rating(), rflag
Common Shared nflag, bold, turbo!, rr(), starx(), stary(), mtn(), rrfrom()
Common Shared image(), force$(), armyloc(), armyname$(), player, aggress!
Common Shared armysize(), armylead(), armyexper(), supply(), month$()
Common Shared rcity(), price(), vicflag(), tracks(), batwon(), cityo()
Common Shared armymove(), month, year, array(), mflag, side, ATKFAC, DEFAC, TCR
Common Shared navyloc(), navysize(), navymove(), occupied(), vptotal, fort()
Common Shared victory(), graf, noise, capcity(), difficult, usadv, bw
Common Shared train(), font$(), brray(), jancam, randbal, realism, casualty&()
Common Shared pcode, history, thrill, fleet$(), commerce, raider, grudge
'==========================================================================
Dim Shared cityx(40), cityy(40), cityv(40), cityp(40), city$(40), lname$(40), rcity(5)
Dim Shared cash(2), control(2), matrix(40, 7), income(2), anima(300), rating(40)
Dim Shared mtx$(21), image(300), force$(2), armyloc(40), armyname$(40), vicflag(6)
Dim Shared armysize(40), armylead(40), armyexper(40), supply(40), month$(12)
Dim Shared armymove(40), array(40), starx(8), stary(8), mtn(1 To 1564), rrfrom(2)
Dim Shared navyloc(2), navysize(2), navymove(2), occupied(40), fort(40), rr(2)
Dim Shared victory(2), capcity(2), emancipate, price(3), tracks(2), fleet$(2)
Dim Shared train(2), font$(26), brray(40), casualty&(2), batwon(2), cityo(40)
Dim Shared graft(1 To 1564)
Dim Shared graphic(1 To 1564), Ncap(60)

$ExeIcon:'./cws.ico'
_Icon
_Title "Civil War Strategy Game"

replay = 0
newgame:
pcode = 0: rflag = 0
For k = 1 To 40
    armysize(k) = 0
    armyloc(k) = 0
    armymove(k) = 0
    armylead(k) = 0
    armyname$(k) = ""
Next k
usadv = 0: emancipate = 0
For k = 1 To 2
    navysize(k) = 0: navyloc(k) = 0: navymove(k) = 0
    rr(k) = 0: victory(k) = 0: tracks(k) = 0
Next k

filel = 1: vicflag(1) = 1
Call filer(1)
If _FileExists("mtn.vga") Then
    Def Seg = VarSeg(mtn(1))
    BLoad "mtn.vga", VarPtr(mtn(1))
    Def Seg
Else
    Color 15: Beep: Print "ERROR: Missing File  'MTN.VGA'": TICK 9
End If

Restore
For k = 1 To 26: Read font$(k): Next k

If realism > 0 And year < 1862 Then
    For i = 1 To 2
        a$ = String$(Len(fleet$(i)), "W")
        fleet$(i) = a$
    Next i
End If
iron:
If player < 1 Or player > 2 Then player = 1
If player = 2 Or side = 0 Then side = 1
If side = 1 Then randbal = 7
If side = 2 Then randbal = 3
If turbo! < 1 Then turbo! = 2
If side = 1 And difficult < 3 Then cash(2) = cash(2) + 600 - 100 * difficult
If side = 2 And difficult > 3 Then cash(1) = cash(1) + 100 * difficult


For i = 1 To 2: income(i) = cash(i): cash(i) = cash(i) + 50 * Rnd: Next i
choose = 0

' _FULLSCREEN _SQUAREPIXELS, _SMOOTH
Screen 12
Def Seg = VarSeg(graphic(1))
BLoad "cwsicon.vga", VarPtr(graphic(1))
Def Seg

Put (100, 100), graphic(), PSet
Get (101, 101)-(113, 113), Ncap()
Cls
Color 11: Locate 14, 27: Print "VGA CIVIL WAR STRATEGY GAME"
Color 4: Locate 15, 32: Print "Registered Edition"
Color 14: Locate 28, 1: Print Tab(8); "(c) 1998, 2017, 2018, 2024 by W. R. Hutsell and Dave Mackey"
Locate 28, 60: Print "v1.61"
Line (190, 170)-(440, 260), 1, B
Line (180, 180)-(450, 250), 7, B
Call flags(1, -440, 0): Call flags(2, -100, 0)
If replay = 0 And noise = 2 And choose = 0 Then
    If side = 1 Then
        If InKey$ <> "" GoTo notitle
        Play "MST170o1e8o0b8o1e8"
        If InKey$ <> "" GoTo notitle
        Play "e8e4f#8g4f#8"
        If InKey$ <> "" GoTo notitle
        Play "g4e8d2o0b8o1d2 "
        If InKey$ <> "" GoTo notitle
        Play "o1e8o0b8o1e8e8e4f#8g4f#8g4a8b2g8b2MLg16a16"
        If InKey$ <> "" GoTo notitle
        Play "MSb4b8b8a8g8a4a8a4f#8g4g8MLg8f#8"
        If InKey$ <> "" GoTo notitle
        Play "MSe8f#4f#8f#8g8a8b4.a4.g4.f#4.o0b8o1e8e8e4d8e2."
    Else
        Call shen
    End If
End If
notitle:
GoSub unionplus

city$(0) = "NONE"
Open "I", 1, "cities.grd"
vptotal = usadv + 200
For i = 1 To 40: Input #1, a, cityx(i), cityy(i), city$(i), cityp(i), cityv(i)
    cityo(i) = cityp(i)
    Call occupy(i)
    For j = 1 To 7: Input #1, matrix(i, j): Next j: Input #1, fort(i)
    If cityp(i) > 0 Then x = cityp(i): control(x) = control(x) + 1: victory(x) = victory(x) + cityv(i)
    If cityp(i) > 0 Then cash(cityp(i)) = cash(cityp(i)) + cityv(i)
    vptotal = vptotal + cityv(i)
Next i
Close #1: Call clrbot

Randomize 1861: Play "V0"
TICK .3
wtype = 2
hilite = 15
replay = 1
If player = 2 Then GoSub blanken
mtx$(0) = "CIVIL WAR STRATEGY"
mtx$(1) = "Resume Saved Game"
mtx$(2) = "Start NEW Game"
mtx$(3) = "Quit"
tlx = 33: tly = 20: colour = 5: size = 3
choose = 23
Call menu(0)
If choose = 1 Then
    Cls
    GoTo loader
    GoTo menu0
End If
If choose = 3 Then Cls: End
If history = 1 Then
    Cls
    If _FileExists("cws.his") Then Shell "copy cws.his oldhist.ory"
    Open "O", 2, "cws.his"
    Print #2, Tab(20); "[ HISTORY OF GAME BEGUN "; "10-19-2026"; " ]"
    Close #2
Else
    Screen 12: Cls
End If
Call usa
GoTo menu0
'                              New Month
newmonth:
If side > 2 Then side = 1
a$ = "--------> EVENTS FOR " + month$(month) + " " + Str$(year) + " --------"
Call scribe(a$, 0)
Call tupdate
control(1) = 0: control(2) = 0

For i = 1 To 2
    income(i) = 0
    If cash(i) > 19999 Then cash(i) = 19999
    If cash(i) < 0 Then cash(i) = 0
Next i
If player = 1 And side = 2 Then income(1) = income(1) + usadv

For i = 1 To 40
    If cityp(i) > 0 Then x = cityp(i): control(x) = control(x) + 1: income(x) = income(x) + cityv(i)
    armymove(i) = 0
Next i

For i = 1 To 2: navymove(i) = 0: If capcity(i) > 0 Then income(i) = income(i) + 100
    cash(i) = cash(i) + income(i)
    If commerce > 0 And i <> commerce Then cash(i) = cash(i) - raider
Next i
vptotal = income(1) + income(2)

chosit = 22
If player = 2 Then GoSub blanken: Call usa
Call victor
On Error GoTo 0
If pcode > 0 Then
    For k = 1 To 40: armyloc(k) = 0: Next k
    GoTo newgame
End If

'                              Main Menu
menu0:
Call topbar
If player = 2 And side = 0 Then side = 1
If cash(side) < 100 And navyloc(side) = 0 Then nflag = 1

hilite = 11
colour = 4
tlx = 67: tly = 13
mtx$(0) = "Main"
mtx$(1) = "Troops": If rflag < 0 Or cash(side) < 100 Then mtx$(1) = "-": chosit = 23
mtx$(2) = "Moves": If mflag > 0 Then mtx$(2) = "-": If chosit = 23 Then chosit = 24
mtx$(3) = "Ships": If nflag > 0 Then mtx$(3) = "-": If chosit = 24 Then chosit = 25
mtx$(4) = "Railroad": If rr(side) > 0 Then mtx$(4) = "-": If chosit = 25 Then chosit = 26
mtx$(5) = "END TURN"
mtx$(6) = "Inform"
mtx$(7) = "COMMANDS"
mtx$(8) = "UTILITY"
mtx$(9) = "Files"

size = 9
choose = chosit
Call menu(0): Call clrrite
'==========================================================================
'                            Main Menu
'==========================================================================
Select Case choose
    Case 1 ' Recruit
        If cash(side) < 100 Or rflag < 0 Then rflag = -1: GoTo menu0
        Call recruit((side))
        chosit = 23
        GoTo menu0
        '==========================================================================
    Case 2 ' Armies
        Call armies
        chosit = 23
        GoTo menu0
        '==========================================================================
    Case 3 ' Navies
        If nflag = 0 Then Call navy((side), 0)
        chosit = 25
    Case 4 ' Railroad
        If rr(side) = 0 Then
            Color 15: Locate 4, 68: Print "RAILROAD MOVE"
            PSet (550, 20), 0
            Draw "C15S6R9D4R6U3R3D3R7U5H3U2R9D3G2D6F1D3F5L10D1G1L4H2L7G2L3H2L3U9L2U5R1BF4"
            z = 9: If side = 2 Then z = 7
            x = Point(0): y = Point(1)
            Paint (x, y), z, 15
            Color 15: If side = 1 Then Color 11
            Call traincapacity(side, limit)
            Call clrbot: Print "Railroad capacity ="; RTrim$(Str$(limit)); "00";
            Call railroad(side)
        Else
            Call clrbot: Color 11: Print "Railroad is already carrying "; armyname$(rr(side)); " to "; city$(armymove(rr(side)));: GoTo menu0
        End If
        '==========================================================================
    Case 5 ' End Turn
        tlx = 67: tly = 15
        hilite = 15: colour = 3: choose = 23
        mtx$(0) = "End Turn"
        mtx$(1) = "Yes"
        mtx$(2) = "NOT YET"
        size = 2: Call menu(0): Call clrrite: If choose <> 1 Then chosit = 24: GoTo menu0
        endrnd:
        rflag = 0: mflag = 0: nflag = 0
        If player = 2 Then side = side + 1: If side = 2 Then GoSub blanken: Call usa: GoTo menu0
        GoTo newmonth
        '==========================================================================
    Case 6
        Call report((side)): chosit = 27
        Call starfin(star, fin, side)
        For i = star To fin
            If armymove(i) > 0 Then Call icon(armyloc(i), armymove(i), 1)
        Next i
    Case 7 ' Commands
        optmen:
        tlx = 67: tly = 13
        Color 11: Locate tly - 2, tlx: Print "esc=Main Menu"
        hilite = 15: colour = 3: chosit = 24
        mtx$(0) = "Commands"
        mtx$(1) = "Cancel"
        mtx$(2) = "Fortify": If cash(side) < 200 Then mtx$(2) = "-"
        mtx$(3) = "Join"
        mtx$(4) = "Supply"
        mtx$(5) = "Capital": If capcity(side) = 0 Or cash(side) < 500 Then mtx$(5) = "-"
        mtx$(6) = "Detach": If side = 1 Then mtx$(6) = "-"
        mtx$(7) = "Army Drill"
        mtx$(8) = "Relieve"
        mtx$(9) = "MAIN MENU"
        size = 9
        Call menu(0): Call clrrite
        chosit = 28
        Select Case choose
            '---------------------------------------------------------------------------
            '                               Cancel
            '---------------------------------------------------------------------------
            Case 1
                Call cancel(side): mflag = 0
                choose = 22
                '---------------------------------------------------------------------------
                '                               Fortify
                '---------------------------------------------------------------------------
            Case 2
                If cash(side) < 200 Then Color 11: Call clrbot: Print "Not enough money for fort";: GoTo menu0
                Call fortify(target)
                If cash(side) < 200 GoTo menu0
                choose = 23
                GoTo optmen
                '---------------------------------------------------------------------------
                '                               Combine
                '---------------------------------------------------------------------------
            Case 3
                x = side
                Call combine(x)
                If x < 0 Then
                    Call clrbot
                    Color 11
                    Print "No eligible armies in same city to combine";
                    Call stax(side)
                    GoTo menu0
                End If
                choose = 24
                GoTo optmen
                '---------------------------------------------------------------------------
                '                               Supply
                '---------------------------------------------------------------------------
            Case 4
                Call starfin(star, fin, (side))
                mtx$(0) = "Supply"
                tlx = 67: tly = 5: colour = 5
                size = 0
                For i = star To fin
                    If armyloc(i) = 0 Or supply(i) > 1 GoTo alone
                    If realism > 0 Then
                        Call cutoff(side, armyloc(i), a)
                        If a < 1 Then Call clrbot: Color 15: Print force$(side); " army in "; city$(armyloc(i)); " is CUT OFF !";: TICK turbo!: GoTo alone
                    End If
                    size = size + 1
                    max = 11: If Len(armyname$(i)) < 11 Then max = Len(armyname$(i))
                    mtx$(size) = Left$(armyname$(i), max)
                    array(size) = i
                    alone:
                Next i
                If size = 0 Then Color 11: Call clrbot: Print "All eligible "; force$(side); " armies have supplies";: GoTo menu0
                Call menu(6): Call clrrite
                If choose < 0 GoTo menu0
                index = array(choose)
                If supply(index) < 2 Then
                    Call resupply(index)
                    Call placearmy(index)
                End If
                choose = 25
                GoTo optmen
                '-------------------------------------------------------------------------
                '                               Move Capital
                '-------------------------------------------------------------------------
            Case 5
                If capcity(side) = 0 Or cash(side) < 500 Then Call clrbot: Color 11: Print "Cannot move capital";: GoTo menu0
                cash(side) = cash(side) - 500
                victory(3 - side) = victory(3 - side) + 50
                Call clrrite
                mtx$(0) = "Capital"
                a$ = city$(capcity(side))
                index = capcity(side): Call newcity(index)
                If index = 0 GoTo menu0
                capcity(side) = index
                Call clrbot: Print force$(side); " capital moved from "; a$; " to "; city$(capcity(side));
                Call clrrite
                Call showcity
                Call TICK(9): Call clrbot
                '-------------------------------------------------------------------------
                '                               Detach
                '-------------------------------------------------------------------------
            Case 6
                If side = 1 Then Call clrbot: Color 11: Print "Option not available to Union";: GoTo menu0
                Color 14: Locate 4, 68: Print "DETACH UNIT"
                Call movefrom(index, target): If target < 1 Or index < 1 GoTo menu0
                If armysize(index) < 65 Then Call clrbot: Print "Too small to detach";: Call TICK(turbo!): GoTo menu0
                Call commander(2, empty): If empty = 0 GoTo menu0
                supply(empty) = .3 * supply(index): supply(index) = supply(index) - supply(empty): If supply(index) < 0 Then supply(index) = 0
                armysize(empty) = .3 * armysize(index): armysize(index) = armysize(index) - armysize(empty)
                armyloc(empty) = target: armyexper(empty) = armyexper(index): armymove(empty) = 0
                armylead(empty) = rating(empty)
                armyname$(empty) = lname$(empty): lname$(empty) = ""
                armyexper(empty) = armyexper(index)
                Color 11: Call clrbot: Print "Unit #"; empty; " with ";: Call strong(empty): Print " men detached under "; armyname$(empty);: Call TICK(turbo!): If noise > 0 Then Sound 2222, 1
                Call stax(side)
                choose = 27
                GoTo optmen
                '-------------------------------------------------------------------------
                '                               Drill
                '-------------------------------------------------------------------------
            Case 7
                Color 14: Locate 4, 68: Print "DRILL ARMY"
                Call movefrom(index, target)
                If target < 1 Or index < 1 Then Color 11: Call clrbot: Print "No armies remain eligible for drills in "; month$(month);: GoTo menu0
                If armyexper(index) > 5 Or armyexper(index) >= armylead(index) Then Color 12: Call clrbot: Print armyname$(index); ": Army has reached maximum improvement through drilling";: GoTo optmen
                armyexper(index) = armyexper(index) + 1
                Call clrbot: Print armyname$(index); " has drilled to reach experience level "; armyexper(index);: If noise > 0 Then Sound 2222, 1
                Call TICK(turbo!): clrbot
                armymove(index) = -1
                choose = 28
                GoTo optmen
                '-------------------------------------------------------------------------
                '                               Relieve
                '-------------------------------------------------------------------------
            Case 8
                Call relieve(side)
                choose = 29
                GoTo optmen
                '-------------------------------------------------------------------------
            Case Else
                chosit = 28
                GoTo menu0
        End Select
        choose = 21 + choose: GoTo optmen
        '==========================================================================
        '                               Utility Menu
        '==========================================================================
    Case 8
        utile:
        chosit = 29
        mtx$(0) = "Utility"
        mtx$(1) = "Side": If player = 2 Then mtx$(2) = ""
        mtx$(2) = "1 Player": If player = 2 Then mtx$(2) = "2 Player"
        mtx$(3) = "Graphics" + Str$(graf)
        mtx$(4) = "Noise": If noise > 0 Then mtx$(4) = mtx$(4) + String$(noise, "*")
        mtx$(5) = "Display" + Str$(turbo!)
        a$ = Str$(difficult): If side = 1 Then a$ = Str$(6 - difficult)
        mtx$(6) = "Balance" + a$
        mtx$(7) = "End Cond"
        a$ = "+": If randbal = 0 Then a$ = ""
        mtx$(8) = "Rndom Evt " + a$
        mtx$(9) = "Vary Start"
        a$ = "": If jancam = 1 Then a$ = "+"
        mtx$(10) = "Jan Campgn" + a$
        a$ = "": If realism = 1 Then a$ = "+"
        mtx$(11) = "Realism " + a$
        mtx$(12) = "Chk Links"
        a$ = "": If history = 1 Then a$ = "+"
        mtx$(13) = "History" + a$
        size = 13: tlx = 67: tly = 11
        Color 11: Locate tly - 2, tlx: Print "esc=Main Menu"
        If player = 1 Then size = 14: mtx$(14) = "Aggress" + Str$(bold)
        Call menu(-1): Call clrrite
        Select Case choose
            '-------------------------------------------------------------------------
            '                               Swap Sides
            '-------------------------------------------------------------------------
            Case 1
                If player = 2 GoTo menu0
                side = 3 - side: Color 9: If side = 2 Then Color 7
                Call clrbot: Print "Now playing "; force$(side); " side";: If noise > 0 Then Sound 999, 1
                If side = 1 Then randbal = 7
                If side = 2 Then randbal = 3
                Call topbar
                GoTo menu0
                '-------------------------------------------------------------------------
                '                               Solo or 2 Player
                '-------------------------------------------------------------------------
            Case 2
                player = 3 - player: Call clrbot: Color 12
                If noise > 0 Then Sound 999, 1
                a$ = "Solo": If player = 2 Then a$ = "2 Player"
                Print a$; " Game";
                choose = 23: GoTo utile
                '-------------------------------------------------------------------------
                '                               Graphics
                '-------------------------------------------------------------------------
            Case 3
                graf = graf + 1
                If graf > 3 Then graf = 0
                a$ = "ROADS"
                If graf = 0 Then a$ = "DISABLED"
                If graf = 2 Then a$ = "CITY NAMES"
                If graf = 3 Then a$ = "FULL"
                Cls
                Call usa
                Call clrbot: Color 11: Print "Graphics : "; a$;: If noise > 0 Then Sound 2700, 1
                choose = 24: GoTo utile
                '-------------------------------------------------------------------------
                '                               Sounds
                '-------------------------------------------------------------------------
            Case 4
                Call clrrite: choose = noise + 22
                mtx$(0) = "SOUNDS"
                mtx$(1) = "Quiet"
                mtx$(2) = "Sound"
                mtx$(3) = " & Sound"
                size = 3: tlx = 67: tly = 12
                Call menu(0): Call clrrite
                If choose < 1 GoTo menu0
                Color 11: Call clrbot: Print "Sound Option : "; mtx$(choose);
                noise = choose - 1
                If noise > 0 Then Sound 999, 1
                choose = 25: GoTo utile
                '-------------------------------------------------------------------------
                '                               Display Speed
                '-------------------------------------------------------------------------
            Case 5
                choose = turbo! + 21
                mtx$(0) = "Display"
                mtx$(1) = "Fast"
                mtx$(2) = "Normal"
                mtx$(3) = "Slow"
                mtx$(4) = "Very Slow"
                mtx$(5) = "Reg Color": If bw > 0 Then mtx$(5) = "Alt Color"
                tlx = 67: tly = 15: size = 5
                Call menu(0): Call clrrite
                Select Case choose
                    Case Is < 1
                    Case Is < 5
                        turbo! = choose
                        If turbo! = 4 Then turbo! = 8
                        Call clrbot: Color 11
                        Print "Display Speed : "; mtx$(choose);
                    Case 5
                        bw = 1 - bw
                        Cls
                        Call usa
                        Call topbar
                End Select
                choose = 26: GoTo utile
                '-------------------------------------------------------------------------
                '                               Play Balance
                '-------------------------------------------------------------------------
            Case 6
                choose = difficult + 21
                mtx$(0) = "Balance"
                mtx$(1) = "Rebel ++"
                mtx$(2) = "Rebel +"
                mtx$(3) = "Balanced"
                mtx$(4) = "Union +"
                mtx$(5) = "Union ++"
                tlx = 67: tly = 15: size = 5
                Call menu(8): Call clrrite
                If choose < 1 GoTo menu0
                Call clrbot
                Color 11: Call clrbot: Print "Play Balance : "; mtx$(choose);
                difficult = choose
                GoSub unionplus
                choose = 27: GoTo utile
                '-------------------------------------------------------------------------
                '                               Ending Conditions
                '-------------------------------------------------------------------------
            Case 7
                Call endit
                choose = 28: GoTo utile
                '-------------------------------------------------------------------------
                '                           Random Events Options
                '-------------------------------------------------------------------------
            Case 8
                mtx$(0) = "Random Events"
                size = 4: tlx = 30: tly = 8
                mtx$(1) = "OFF"
                mtx$(2) = "Favor Union ": If randbal = 3 Then mtx$(2) = mtx$(2) + " "
                mtx$(3) = "Neutral     ": If randbal = 5 Then mtx$(3) = mtx$(3) + " "
                mtx$(4) = "Favor Rebels": If randbal = 7 Then mtx$(4) = mtx$(4) + " "
                colour = 5
                Call menu(0)
                colour = 4

                Select Case choose
                    Case 1
                        randbal = 0: t$ = ""
                    Case 2
                        randbal = 3
                    Case 3
                        randbal = 5
                    Case 4
                        randbal = 7
                    Case Else
                End Select
                If choose > 1 And choose < 5 Then t$ = mtx$(choose)
                Call clrbot
                a$ = "": If randbal = 0 Then a$ = "OFF"
                Color 11: Print "Random Events : "; a$; " "; t$;
                Color 14: Print "			press a key";
                Def Seg = 0: Poke 1265, 1: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop
                Cls
                Call usa
                choose = 29
                GoTo utile
                '-------------------------------------------------------------------------
                '                              Vary Start Conditions
                '-------------------------------------------------------------------------
            Case 9
                Call filer(1)
                cash(1) = cash(1) - 100 + 200 * Rnd
                cash(2) = cash(2) + 100 + 200 * Rnd
                bold = 5 * Rnd
                For k = 1 To 6
                    If Rnd > .6 Then
                        armyloc(k) = 0: armysize(k) = 0: armylead(k) = 0
                        armyexper(k) = 0: armymove(k) = 0: supply(k) = 0
                    End If
                Next k
                For k = 21 To 6
                    If Rnd > .6 Then
                        armyloc(k) = 0: armysize(k) = 0: armylead(k) = 0
                        armyexper(k) = 0: armymove(k) = 0: supply(k) = 0
                    End If
                Next k
                For k = 1 To 40: Call occupy(k): Next k
                navysize(1) = 10 * Rnd: If navysize(1) = 0 Then navyloc(1) = 0
                navysize(2) = 10 * Rnd: If navysize(2) > 0 Then navyloc(2) = 27
                For i = 1 To 2
                    fleet$(i) = ""
                    For j = 1 To navysize(i)
                        a$ = "W": If Rnd > .45 * side Then a$ = "I"
                        fleet$(i) = fleet$(i) + a$
                    Next j
                Next i
                If Rnd > .7 Then capcity(2) = 25

                For k = 1 To 40
                    If Rnd > .8 Then rating(k) = rating(k) - 3 + 6 * Rnd: If rating(k) > 9 Then rating(k) = 9
                    If rating(k) < 1 Then rating(k) = 1
                Next k
                Cls
                Call usa
                choose = 30: GoTo utile
                '-------------------------------------------------------------------------
                '                               January Campaigns
                '-------------------------------------------------------------------------
            Case 10
                jancam = 1 - jancam
                a$ = "PROHIBITED": If jancam = 1 Then a$ = "ALLOWED"
                Color 11
                Call clrbot
                Print "January Campaigns : "; a$;
                choose = 31: GoTo utile
                '-------------------------------------------------------------------------
                '                               Realism Switch
                '-------------------------------------------------------------------------
            Case 11
                realism = 1 - realism
                Call clrbot: Color 11
                If realism = 0 Then
                    Print "Recruiting FIXED :  7000 for NEW Armies   4500 for Additions";
                Else
                    Print "REALISM ON: Recruiting based on CITY SIZE";
                    If side = 2 And randbal = 1 And randbal < 5 Then randbal = randbal + 2
                    GoSub unionplus
                End If
                choose = 32: GoTo utile
                '-------------------------------------------------------------------------
                '                               Check Map Linakges
                '-------------------------------------------------------------------------
            Case 12
                Call integrity
                Call TICK(99)
                Call usa
                '-------------------------------------------------------------------------
                '                               History
                '-------------------------------------------------------------------------
            Case 13
                history = 1 - history
                a$ = "OFF": If history = 1 Then a$ = "ON"
                Call clrbot: Print "History is now "; a$;
                choose = 34: GoTo utile
                '-------------------------------------------------------------------------
                '                               Aggression
                '-------------------------------------------------------------------------
            Case 14
                bold = bold + 1: If bold > 5 Then bold = 0
                Select Case bold
                    Case 0: a$ = "PASSIVE"
                    Case 1: a$ = "TIMID"
                    Case 2: a$ = "CAUTIOUS"
                    Case 3: a$ = "NORMAL"
                    Case 4: a$ = "BOLD"
                    Case 5: a$ = "RECKLESS"
                End Select
                Call clrbot
                Color 11
                Print "Enemy Aggression : "; a$; " ("; bold; ")";
                choose = 35: GoTo utile
            Case Else
        End Select
        '==========================================================================
        '                               Files
        '==========================================================================
    Case 9
        If Not _FileExists("*.sav") Then filel = 0
        filex:
        choose = 23: chosit = 30: If year = 1861 Then choose = 22
        mtx$(0) = "Options"
        mtx$(1) = "Load": If filel = 0 Then mtx$(1) = "-": choose = 23
        mtx$(2) = "Save"
        mtx$(3) = "New Game"
        mtx$(4) = "Quit"
        size = 4: tlx = 67: tly = 15
        Call menu(0): Call clrrite
        Select Case choose
            Case Is < 1: GoTo menu0
            Case 1, 2
                loader:
                If choose = 1 And filel = 0 GoTo filex
                Call filer(choose + 1)
                If choose = 1 Then rflag = 0: mflag = 0: nflag = 0
                GoTo menu0
            Case 3
                Open "O", 1, "cws.cfg"
                Write #1, side, graf, noise, difficult, player, turbo!, randbal, train(1), train(2), jancam, realism, batwon(1), batwon(2), casualty&(1), casualty&(2), history, bold
                Close #1
                Cls
                GoTo newgame
            Case 4: GoTo rusure
        End Select
        '==========================================================================
    Case 99
        GoTo endrnd
    Case Else
        chosit = 22
        GoTo menu0
End Select
GoTo menu0
'............................................................................
'                               Subs
'............................................................................
topbar:
Locate 1, 1: Print Space$(80);
Color 11
Locate 1, 10: Print "Input your decisions now for "; force$(side); " side ";: Color 14: Print month$(month); ","; year; "  "
Return
'............................................................................
rusure:
choose = 23
mtx$(0) = "Quit"
mtx$(1) = "Yes"
mtx$(2) = "No"
size = 2: colour = 5
tlx = 67: tly = 15
Call menu(0): Call clrrite: If choose = 1 Then End Else GoTo menu0
'............................................................................
unionplus: usadv = 120 * difficult: If player = 2 Then usadv = 50 * difficult
If realism > 0 Then usadv = usadv * .7
Return
'............................................................................
blanken: c = 1: If side = 2 Then c = 7
Cls: Line (100, 200)-(500, 300), c, BF
Line (100, 200)-(500, 300), 8 - c, B
Color 7: Locate 14, 31: Print " "; month$(month); year
Color 11: Locate 17, 30: Print force$(side); " PLAYER TURN"
Def Seg = 0: Poke 1265, 2: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop
Return
'............................................................................
Data "U2E2F2D2BU1L3","U4R3F1G1L2BR2BF1G1L2","H1U2E1R2F1BD2G1L1"
Data "U4R3F1D1D1G1L2","U4R3BD2BL1L2D2R3","U4R3BD2BL1L2","H1U2E1R3BD2L2BD2R2U1"
Data "U4BD2BR1R3U2BD3D1","R1U4L1R2BL1BD4R1","R1E1U3BG3F1","U4BR3G2F2"
Data "U4BD4BR1R1","U4F2E2D4","U4F4U4","H1U2E1R2F1D2G1L1","U4R3F1G1L2"
Data "H1U2E1R2F1D2G1L1BE1F1R1","U4R3F1G1L2BR1F2","R3E1H1L2H1E1R3"
Data "U4L2BR3R1","H1U3BR4D3G1L1","H2U2BR4D2G2","H2U2BF3BU1D1F1E2U2"
Data "E4BD4H4","U2H2BR4G2","E4L4BD4R4"

Sub animate (index, flag)
    from = armyloc(index): to2 = armymove(index): armyloc(index) = 0
    x = cityx(from) - 12
    y = cityy(from) - 11
    x1 = .5 * (cityx(to2) + cityx(from))
    y1 = .5 * (cityy(to2) + cityy(from))

    Call occupy(from): If occupied(from) > 0 Then Call placearmy(occupied(from))
       
    If flag > 0 GoTo already
    Get (x - 9, y - 7)-(x + 9, y + 6), anima()
    If occupied(from) = 0 Then Line (x - 9, y - 8)-(x + 10, y + 8), 2, BF
    already:

    For i = 2 To 8
        x1 = .1 * (i * cityx(to2) + (10 - i) * cityx(from))
        y1 = .1 * (i * cityy(to2) + (10 - i) * cityy(from))
        Get (x1 - 10, y1 - 10)-(x1 + 9, y1 + 9), image()
        Put (x1 - 10, y1 - 10), anima(), PSet
        If turbo! > 1 Then Call TICK(.1) Else Call TICK(.02)
        If noise > 0 Then Sound 200, .1: Sound 50, .1
        Put (x1 - 10, y1 - 10), image(), PSet
    Next i
    armyloc(index) = from
End Sub

Sub armies
    Call movefrom(index, target): If target > 0 And index > 0 GoTo tent
    If index = -1 Then mflag = 1
    GoTo dudd
    tent:
    tlx = 67: tly = 12

    Call armystat(index)
    Color 11: Locate 11, 68: Print city$(target)
    size = 0
    mtx$(0) = "To"
    For i = 1 To 6: If matrix(target, i) > 0 Then size = size + 1: mtx$(size) = city$(matrix(target, i)): array(size) = matrix(target, i)
    Next i
    Call bubble(size)

    hilite = 10: colour = 3: Call menu(2): Call clrrite
    If choose < 0 GoTo dudd
    If month < 3 And jancam = 0 And cityp(array(choose)) <> side Then Color 11: Call clrbot: Print "No campaigns in January";: Call TICK(9): Call clrbot: GoTo dudd

    Call clrbot: Print "Army "; index; " "; armyname$(index); " is moving from "; city$(target); " to "; city$(array(choose));
    If noise > 0 Then Sound 2200, .5: Sound 2900, .7
    Call icon(target, array(choose), 1)
    armymove(index) = array(choose): Call TICK(turbo!): Call clrbot
    Call clrrite
    dudd:
End Sub

Sub armystat (index)
    Line (530, 60)-(639, 150), 0, BF
    Color 15
    Locate 5, 68: Print armyname$(index)
    Color 9: If index > 20 Then Color 7
    Locate 6, 68: Print "Size:";: Call strong(index)
    Locate 7, 68: Print "Leader:"; Tab(75); armylead(index)
    Locate 8, 68: Print "Exper:"; Tab(75); armyexper(index)
    If supply(index) < 2 Then Color 12
    Locate 9, 68: Print "Supply:"; Tab(75); supply(index)
End Sub

Sub barnacle (who)
    navysize(who) = navysize(who) - 1
    If navysize(who) > 0 Then
        fleet$(who) = Left$(fleet$(who), navysize(who))
    Else
        fleet$(who) = "": navyloc(who) = 0: navysize(who) = 0
    End If
End Sub

Sub battle (attack, defend, win, lose)
    If armysize(defend) < 1 Then armysize(defend) = 1
    If armysize(attack) < 1 Then armysize(attack) = 1

    Call icon(armyloc(defend), 0, 9)
    Locate 1, 1: Print Space$(80)
    Call clrrite: y = 68: Color 11: Locate 1, y: Print "Attacker"
    c = 9: If attack > 20 Then c = 7
    Color c: Locate 2, y: Print armyname$(attack)
    Locate 3, y: Call strong(attack)
    x = .01 * armysize(attack): If x > TCR Then x = TCR
    Locate 4, y: Print "Base    "; x
    If armysize(attack) / armysize(defend) > .2 Then x = x + .3 * armylead(attack) + .3 * armyexper(attack): If x > TCR Then x = TCR
    If armyexper(attack) > 8 Then x = x + 1: If x > TCR Then x = TCR
    Locate 5, y: Print "Ldr/Exp "; x
    If armysize(attack) < 15 Then x = .5 * x: If x > TCR Then x = TCR
    Locate 6, y: Print "Small   "; x
    If armysize(attack) / armysize(defend) <= .5 Then x = x - 2: If x < 1 Then x = 1
    If armysize(attack) / armysize(defend) > 3 Then x = x + 2: If x > TCR Then x = TCR
    If armysize(attack) / armysize(defend) > 10 Then x = TCR
    If armysize(attack) / armysize(defend) <= .2 Then x = 1
    Locate 7, y: Print "Outman  "; x
    If supply(attack) < 1 Then x = .5 * x: Color 13
    Locate 8, y: Print "Supply  "; x: Color c
    If x < 1 Then x = 1
    '                       Adjust attack advantage
    If attack < 21 And side = 2 And difficult > 3 Then x = x + 2 * difficult - 6
    If attack > 20 And side = 1 And difficult < 3 Then x = x + 6 - 2 * difficult
    Locate 9, y: Print "Difclt  "; x
    If x > TCR Then x = TCR
    Color 11: Locate 11, y: Print "Attack  "; x
    Line (530, 155)-(635, 175), 11, B

    x1 = .013 * armysize(defend) + 1: If realism = 1 Then x1 = .02 * armysize(defend) + 2: If x1 > 20 Then x1 = 20
    Locate 13, y: Print "Defender"
    Color 16 - c: Locate 14, y: Print armyname$(defend)
    Locate 15, y: Call strong(defend)
    Locate 16, y: Print "Base    "; Int(.01 * armysize(defend))
    Locate 17, y: Print "Defense "; x1
    If armysize(defend) / armysize(attack) > .2 Then x1 = x1 + .3 * armylead(defend) + .3 * armyexper(defend): If x1 > TCR Then x1 = TCR
    If armyexper(defend) > 8 Then x1 = x1 + 1: If x1 > TCR Then x1 = TCR
    Locate 18, y: Print "Ldr/Exp "; x1
    If armysize(defend) < 15 Then x1 = .5 * x1
    Locate 19, y: Print "Small   "; x1

    R! = armysize(defend) / armysize(attack)
    Select Case R!
        Case Is > 10
            x1 = TCR
        Case Is > 1.5
            x1 = x1 + 2
        Case Is < .5
            x1 = .8 * x1
        Case Is < .2
            x1 = .5 * x1
    End Select
    If x1 < 1 Then x1 = 1
    If x1 > TCR Then x1 = TCR

    Locate 20, y: Print "Outman  "; x1
    If supply(defend) < 1 Then x1 = .5 * x1: Color 13
    Locate 21, y: Print "Supply  "; x1: Color 16 - c
    If x1 < 1 Then x1 = 1
    '                       Adjust defend advantage
    If defend < 21 And side = 2 And difficult > 3 Then x1 = x1 + 2 * difficult - 6
    If defend > 20 And side = 1 And difficult < 3 Then x1 = x1 + 6 - 2 * difficult
    Locate 22, y: Print "Difclt  "; x1

    a$ = "Fort": If armymove(defend) = 0 Then x1 = x1 * (1 + fort(armyloc(defend)))
    If x1 > TCR Then x1 = TCR
    If fort(armyloc(defend)) > 0 And armymove(defend) = 0 Then
        Color 13: a$ = "Fort+"
        If fort(armyloc(defend)) = 2 Then a$ = "Fort++"
    End If
    Locate 23, y: Print a$; Tab(76); x1
    If x1 > TCR Then x1 = TCR
    Color 11: Locate 25, y: Print "Defend  "; x1
    Line (530, 380)-(635, 400), 11, B

    spin = 0
    scale = x: If x1 > scale Then scale = x1
    scale = scale + 1

    odds! = x / (x + x1): a = Int(100 * odds!): If a < 1 Then a = 1
    a$ = LTrim$(RTrim$(Str$(a))) + "%"
    Color 14: Locate 27, y: Print "Odds:  "; a$;
    Line (530, 412)-(635, 435), 14, B
    Line (528, 410)-(637, 437), 14, B
    Def Seg = 0: Poke 1265, 3: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop
    If graf > 2 Then
        Call cannon
        k = fort(armyloc(defend))
        t$ = "fort" + LTrim$(Str$(k)) + ".vga"
        If _FileExists(t$) Then
            Def Seg = VarSeg(graphic(1))
            BLoad t$, VarPtr(graphic(1))
            Def Seg
            Put (550, 270), graphic(), PSet
        End If
    Else
        Call clrrite
    End If
    For i = 1 To 2: If supply(i) > 0 Then supply(i) = supply(i) - 1
    Next i
    '---------------------------------------------------------------------------
    grapple:
    hit = 0: hit1 = 0
    star = scale * Rnd: fin = scale * Rnd
    If noise > 0 Then Sound 77, .5: Sound 59, .5: Call TICK(.1)
    If star <= x Then hit = 1
    If fin <= x1 Then hit1 = 1

    If hit = 0 And hit1 = 0 GoTo grapple
    If hit = 1 And hit1 = 1 GoTo grapple
    win = defend: lose = attack: If hit = 1 Then win = attack: lose = defend
    '---------------------------------------------------------------------------
    a$ = "UNION": If win > 20 Then a$ = "REBEL"
    Color 14: Locate 3, 68: Print a$; " VICTORY"
    Locate 4, 71: Print "in"
    Locate 5, 69: Print city$(armyloc(defend))
    a = 1: If win > 20 Then a = 2
    Call flags(a, 0, 0)
    Call clrbot: Color 11: Print armyname$(win); " defeats "; armyname$(lose); " in "; city$(armyloc(defend));: Call TICK(9)
    '===========================================================================
    pct! = .01 * DEFAC - .03 * fort(armyloc(defend)): If win = attack Then pct! = 1.3 * pct!
    Select Case armysize(defend)
        Case Is > 300
            pct! = .9 * pct!
        Case Is > 800
            pct! = .8 * pct!
    End Select
    xbar = armysize(attack) * pct!
    vary = xbar * (1 - pct!)
    Call normal(xbar, vary, killd)
    '...........................................................................
    pct! = .01 * ATKFAC + .02 * fort(armyloc(defend)): If win = defend Then pct! = 1.5 * pct!
    Select Case armysize(attack)
        Case Is > 300
            pct! = .9 * pct!
        Case Is > 800
            pct! = .8 * pct!
    End Select
    xbar = armysize(defend) * pct!
    vary = xbar * (1 - pct!)
    Call normal(xbar, vary, killa)
    '===========================================================================

    killa = .8 * killa + .2 * killd: If killa < 1 Then killa = 1
    killd = .8 * killd + .2 * killa: If killd < 1 Then killd = 1
    If killa > 9 * killd Then killa = 9 * killd
    If killd > 5 * killa Then killd = 5 * killa

    If killa >= armysize(attack) Then killa = armysize(attack) - 1
    If killd >= armysize(defend) Then killd = armysize(defend) - 1
    '---------------------------------------------------------------------------
    x = Int(100 * odds!): If x < 1 Then x = 1
    a$ = LTrim$(RTrim$(Str$(x))) + "%"

    Color c
    Locate 1, 1: Print "Attack Loss: ";
    Print LTrim$(RTrim$(Str$(killa)) + "00"); "/";
    Call strong(attack)
    Print " ("; LTrim$(RTrim$(Str$(Int(100 * (killa / armysize(attack)))))); "%) |";
    Color 16 - c: Print "| Defend Loss: ";: Print LTrim$(RTrim$(Str$(killd)) + "00"); "/";
    Call strong(defend)
    Print " ("; LTrim$(RTrim$(Str$(Int(100 * (killd / armysize(defend)))))); "%)"
    '---------------------------------------------------------------------------
    mtx$(1) = " (" + LTrim$(RTrim$(Str$(killa) + "00")) + "/" + LTrim$(Str$(armysize(attack)) + "00") + ") "
    t$ = " (" + LTrim$(RTrim$(Str$(killd) + "00")) + "/" + LTrim$(Str$(armysize(defend)) + "00") + ")"
    mtx$(2) = t$
    If win = defend Then mtx$(2) = mtx$(1): mtx$(1) = t$
    mtx$(3) = "*" + armyname$(win): mtx$(4) = armyname$(lose)
    If win = defend Then mtx$(3) = armyname$(win): mtx$(4) = "*" + armyname$(lose)
    a$ = city$(armyloc(defend)) + ": " + mtx$(3) + mtx$(1) + " defeats " + mtx$(4) + mtx$(2)
    Call scribe(a$, 0)

    If player = 1 And (side = 1 And attack > 20) Or (side = 2 And attack < 21) Then
        Color 14: Call clrbot: Print "hit any key to continue";: Def Seg = 0: Poke 1265, 4: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop
    End If

    armysize(attack) = armysize(attack) - killa
    armysize(defend) = armysize(defend) - killd
    If armysize(defend) < 1 Then armysize(defend) = 1
    '---------------------------------------------------------------------------
    s = 1: If attack > 2 Then s = 2
    casualty&(s) = casualty&(s) + killa
    casualty&(3 - s) = casualty&(3 - s) + killd
    '---------------------------------------------------------------------------
    s = 1: If win > 20 Then s = 2
    batwon(s) = batwon(s) + 1
    victory(s) = victory(s) + 1
    Call icon(armyloc(defend), 0, 8)
End Sub

Sub combine (who)
    colour = 3: target = 0: hilite = 3
    tlx = 67: tly = 2
    Call starfin(star, fin, who)
    size = 0
    For i = 1 To 40
        If cityp(i) = who And occupied(i) > 0 Then
            For j = star To fin
                If armyloc(j) = i And occupied(i) <> j And armymove(j) = 0 Then size = size + 1: mtx$(size) = city$(i): array(size) = i: Exit For
            Next j
        End If
    Next i
	
    If size = 0 Then who = -1: Exit Sub
    Call bubble(size)

    If who <> side Then choose = 1 + Int(size * Rnd): GoTo join
    mtx$(0) = "Join"
    choose = 31: hilite = 11
    Call menu(9): Call clrrite
    join:
    If choose < 1 GoTo nocity2
    target = array(choose)
    Call clrbot: Print "Combining "; force$(who); " armies in "; city$(target);: Call TICK(turbo!)

    armysize(0) = 0: armylead(0) = 0: armyexper(0) = 0: armyloc(0) = target
    armyname$(0) = "": supply(0) = 0: armymove(0) = 0
    '...........................................................................
    best = 0: x = 0: spin = 0
    For j = star To fin
        If armymove(j) <> 0 Or armyloc(j) <> target Or armysize(j) = 0 GoTo dork1
        If armysize(0) + armysize(j) > 1250 Then Call clrbot: Print "Cannot combine "; armyname$(j); "... TOO LARGE";: Call TICK(turbo!): GoTo dork1

        If armylead(j) > x Then
            x = armylead(j)
            armyname$(0) = armyname$(j)
            armylead(0) = armylead(j)
            best = j
        End If

        armysize(0) = armysize(0) + armysize(j)
        If armysize(0) < 1 GoTo dork1
        pct! = (armysize(j) / armysize(0))
        spin = spin + 1
        armyexper(0) = (1 - pct!) * armyexper(0) + pct! * armyexper(j)
        Call clrbot: Print armyname$(j); " is combining his ";: Call strong(j): Print " forces";: Call TICK(turbo!)
        supply(0) = supply(0) + supply(j)
        If supply(0) > 10 Then supply(0) = 10
        armysize(j) = 0: armyloc(j) = 0: armyexper(j) = 0: armymove(j) = 0
        lname$(j) = armyname$(j)
        armylead(j) = 0: supply(j) = 0: armyname$(j) = ""
        dork1:
    Next j

    Call clrbot
    If who <> side And spin = 0 Then Exit Sub
    If spin = 0 Then
        Print "No valid armies to combine at this time in "; city$(target);
        TICK turbo!
        Exit Sub
    End If
    armysize(best) = armysize(0): armylead(best) = armylead(0)
    armyexper(best) = armyexper(0): supply(best) = supply(0)
    armyloc(best) = target
    armyname$(best) = armyname$(0)
    lname$(best) = ""
    If spin > 1 Then
        Print "New army "; best; " of ";: Call strong(best): Print " is commanded by "; armyname$(best);
        armymove(best) = -1
    Else
        Print "Only 1 eligible army -- cannot combine at this time";
    End If
    If who = side Then Call TICK(turbo!)
    If noise > 0 Then Sound 4000, .7
	       
    Call showcity
    Call placearmy(best)
    Call icon(target, 0, 6)
    occupied(target) = best
    Call stax(who)
    nocity2:
End Sub

Sub cutoff (who, target, a)
    a = 0
    For j = 1 To 6
        y = matrix(target, j)
        If y > 0 And cityp(y) = who Then a = a + 1
    Next j
End Sub

Sub filer (switch)
    Select Case switch
        Case 1
            Open "I", 1, "cwslead.dat"
            For i = 1 To 40: Input #1, lname$(i), rating(i): Next i
            Close #1
            Open "I", 1, "cws.ini"
            For i = 0 To 2: Input #1, force$(i): Next i
            For i = 1 To 12: Input #1, month$(i): Next i
            Input #1, month, year
            '               set end game conditions
            For i = 1 To 6: Input #1, vicflag(i): Next i
            vicflag(3) = .4 * vicflag(3)
            Input #1, a ' number of Yankee armies
            For i = 1 To a
                Input #1, armyloc(i), armysize(i), armyexper(i), supply(i): occupied(armyloc(i)) = i: armyname$(i) = lname$(i): armylead(i) = rating(i): lname$(i) = ""
            Next i
            Input #1, a ' number of Rebel armies
            For i = 21 To 20 + a
                Input #1, armyloc(i), armysize(i), armyexper(i), supply(i)
                occupied(armyloc(i)) = i: armyname$(i) = lname$(i): armylead(i) = rating(i): lname$(i) = ""
            Next i
            For i = 1 To 2: Input #1, cash(i): Next i
            Input #1, ATKFAC, DEFAC, TCR
            Input #1, fleet$(1), navyloc(1), fleet$(2), navyloc(2)
            For k = 1 To 2: navysize(k) = Len(fleet$(k)): Next k
            Input #1, capcity(1), capcity(2)

            For i = 1 To 8: Input #1, starx(i), stary(i): Next i ' star locations on flag
            Close #1
            Open "I", 1, "cws.cfg"
            Input #1, side, graf, noise, difficult, player, turbo!, randbal, train(1), train(2), jancam, realism, batwon(1), batwon(2), casualty&(1), casualty&(2), history, bold
            Close #1

        Case 2
            If Not _FileExists("*.sav") Then choose = -1: Exit Sub
            mtx$(0) = "Load"
            size = 0
            For k = 1 To 9
                t$ = "cws" + LTrim$(Str$(k)) + ".sav"
                If _FileExists(t$) Then
                    size = size + 1: mtx$(size) = t$: array(size) = k
                End If
            Next k
            Call bubble((size))
            tlx = 67: tly = 14 - .5 * size
            Call menu(0)
            Call clrrite
            If choose < 1 Then Exit Sub
            Open "I", 1, "cws" + LTrim$(Str$(array(choose))) + ".sav"
            Color 11: Call clrbot: Print "Loading";
            Input #1, month, year, usadv, a
            For i = 1 To 40: Input #1, armyname$(i), armysize(i), armylead(i), armyloc(i), armyexper(i), supply(i), armymove(i)
                If armyloc(i) > 0 Then
                    If armyname$(i) = lname$(i) Then
                        lname$(i) = ""
                    Else
                        who = 1: If i > 20 Then who = 2
                        Call starfin(star, fin, who)
                        For k = star To fin
                            If armyname$(i) = lname$(k) Then lname$(k) = "": Exit For
                        Next k
                    End If
                End If
            Next i
            For i = 1 To 40: Input #1, occupied(i), cityp(i), fort(i): Print ".";: Next i
            For i = 1 To 2: Input #1, cash(i), control(i), income(i), victory(i), capcity(i)
                Input #1, fleet$(i), navyloc(i), navymove(i), rr(i), tracks(i)
                navysize(i) = Len(fleet$(i))
            Next i
       
            Close #1
            Open "I", 1, "cws.cfg"
            Input #1, myside, graf, noise, difficult, player, turbo!, randbal, train(1), train(2), jancam, realism, batwon(1), batwon(2), casualty&(1), casualty&(2), history, bold
            Close #1
            Screen 12: Cls
            Close #1: Call usa: Call clrbot
            For k = 1 To 2
                If rr(k) > 0 Then Call tinytrain(k, 1)
            Next k
            side = a

        Case 3
            mtx$(0) = "Save Game"
            For k = 1 To 9
                mtx$(k) = "cws" + LTrim$(Str$(k)) + ".sav"
                If _FileExists(mtx$(k)) Then mtx$(k) = mtx$(k) + " +"
            Next k
            tlx = 67: size = 9
            Call menu(0)
            If choose < 1 Then clrrite: Exit Sub
            Color 11: Call clrbot: Print "Saving";

            Open "O", 1, "cws" + LTrim$(Str$(choose)) + ".sav"
            Write #1, month, year, usadv, side
            For i = 1 To 40: Write #1, armyname$(i), armysize(i), armylead(i), armyloc(i), armyexper(i), supply(i), armymove(i)
            Next i
            For i = 1 To 40: Write #1, occupied(i), cityp(i), fort(i): Print ".";: Next i
            For i = 1 To 2: Write #1, cash(i), control(i), income(i), victory(i), capcity(i)
                Write #1, fleet$(i), navyloc(i), navymove(i), rr(i), tracks(i)
            Next i
            Close #1
       
            myside = side
            If myside < 1 Or myside > 2 Then myside = 1
            Open "O", 1, "cws.cfg"
            Write #1, myside, graf, noise, difficult, player, turbo!, randbal, train(1), train(2), jancam, realism, batwon(1), batwon(2), casualty&(1), casualty&(2), history, bold
            Close #1
            Cls: Call usa
    End Select
End Sub

Sub flashcity (which)
    For c = 1 To 15 Step 2
        Circle (cityx(which), cityy(which)), 4, 0
        Circle (cityx(which), cityy(which)), 3, c
        Paint (cityx(which), cityy(which)), c, c
        Call TICK(.1)
    Next c
    c = 9: If cityp(which) = 2 Then c = 7
    If cityp(which) = 0 Then c = 12
    Circle (cityx(which), cityy(which)), 4, 0
    Circle (cityx(which), cityy(which)), 3, c
    Paint (cityx(which), cityy(which)), c, c
End Sub

Sub fortify (target)
    target = 0: hilite = 11
    tlx = 67: tly = 2
    who = side
    Call starfin(star, fin, who)
    size = 0
    For i = star To fin
        If armyloc(i) > 0 And fort(armyloc(i)) < 2 Then
            size = size + 1
            mtx$(size) = city$(armyloc(i))
            array(size) = armyloc(i)
        End If
    Next i
    If size = 0 Then Call clrbot: Color 11: Print "No cities eligible to fortify";: Exit Sub
    Call bubble(size)
    mtx$(0) = "Fortify"
    choose = 31
    Call menu(9): Call clrrite
    If choose < 0 GoTo nocity
    target = array(choose): Call occupy(target): x = occupied(target): If x < 0 GoTo nocity
    If fort(target) > 1 Then Call clrbot: Print city$(target); " at MAXIMUM fortification level of 2";: Call TICK(4): GoTo nocity
    cost = 200
    If cash(side) < cost Then Call clrbot: Print "Fortifications for "; city$(target); " cost "; cost; " and you only have "; cash(side);: Call TICK(4): GoTo nocity
    Color 3
    fort(target) = fort(target) + 1
    cash(side) = cash(side) - cost
    Call clrbot: Print city$(target); " fortifications increased to "; fort(target);
    Call icon(target, 0, 6)
    Call showcity
    If armymove(x) > 0 Then Call icon(armyloc(x), armymove(x), 5)
    armymove(x) = -1
    Call placearmy(x)
    Call stax(who)
    nocity:
End Sub

Sub icon (from, dest, kind)
    If from < 1 Or from > 40 GoTo noshow
    If dest < 0 GoTo noshow
    If from = 999 GoTo noshow
    x = cityx(from) - 12: y = cityy(from) - 11
    x1 = cityx(dest): y1 = cityy(dest)
    Select Case kind
        Case 1
            Line (x, y)-(x1, y1), 15, , &HF0F0
       
        Case 2
            For i = 6 To 8
                Line (x - i, y - i + 3)-(x + i, y + i - 3), 14, B
            Next i
            Call TICK(.1)
       
        Case 3
            x = cityx(from): y = cityy(from)
            Call snapshot(x, y, 0)
            For j = 1 To 3
                For i = 4 To 10 Step 1
                    Circle (x, y), i, 4
                    Paint (x, y), 14, 4
                    If noise > 0 Then Sound 37 + 50 * Rnd, .03
                Next i
                Call TICK(.1)
                Call snapshot(x, y, 1)
            Next j
       
        Case 4
            If occupied(from) = 0 Then Line (x - 8, y - 6)-(x + 10, y + 8), 2, BF
            If x1 + y1 > 0 Then Line (x, y)-(x1, y1), 2, , &HF0F0
            If from = 27 Or from = 28 Then Call touchup

        Case 5
            If x1 + y1 > 0 Then Line (x, y)-(x1, y1), 2, , &HF0F0

        Case 6
            x = cityx(from): y = cityy(from)
            Call snapshot(x, y, 0)
            Line (x - 9, y - 9)-(x + 9, y + 9), 15, B
            Line (x - 10, y - 10)-(x + 10, y + 10), 15, B
            If noise > 0 Then Sound 3999, .3
            Call TICK(turbo! - .5)
            Call snapshot(x, y, 1)
        Case 7 ' draw white box
            x = cityx(from) - 12: y = cityy(from) - 11
            Get (x - 8, y - 8)-(x + 8, y + 8), image()
            Line (x - 7, y - 7)-(x + 7, y + 7), 15, B
            Line (x - 8, y - 6)-(x + 8, y + 6), 15, B
        Case 8 ' replace old image
            x = cityx(from) - 12: y = cityy(from) - 11
            If x > 0 Then Put (x - 8, y - 8), image(), PSet
       
        Case 9 ' draw arrow pointer
            x = cityx(from) - 12: y = cityy(from) - 11
            Get (x - 8, y - 8)-(x + 10, y + 7), image()
            x = x + 7: y = y + 5
            Line (x + 2, y)-(x + 2, y - 8), 12
            Line -(x, y - 6), 12
            Line -(x - 5, y - 11), 12
            Line -(x - 10, y - 6), 12
            Line -(x - 6, y - 2), 12
            Line -(x - 10, y), 12
            Line -(x + 1, y), 12
            Paint (x - 2, y - 1), 15, 12

        Case 11
            Line (x + 12, y + 11)-(x1, y1), 0, , &H1111

        Case Else
    End Select
    noshow:
End Sub

Sub iterate
    '---------------------------------------------------------------------------
    For i = 1 To 40: If armyloc(i) > 0 Then Call placearmy(i)
    Next i
    '---------------------------------------------------------------------------
    month = month + 2
    If month > 12 Then month = 1: year = year + 1
    '---------------------------------------------------------------------------
    s = 1
    For i = 1 To 40: If armyloc(i) < 1 GoTo rip
        If i > 20 Then s = 2
        If cash(s) >= .2 * armysize(i) Then
            a = 1: If realism > 0 Then Call cutoff(s, armyloc(i), a)
            If a > 0 Then
                supply(i) = supply(i) + 1
                cash(s) = cash(s) - .2 * armysize(i)
            End If
        End If
        If (month < 7 Or month > 10) And matrix(armyloc(i), 7) < 99 Then supply(i) = supply(i) - 1
        If matrix(armyloc(i), 7) = 99 And navyloc(3 - s) = armyloc(i) Then supply(i) = supply(i) - 1: Call clrbot: Print armyname$(i); " is blockaded";: Call TICK(turbo!)
        Color 13
        If supply(i) < 1 Then
            supply(i) = 0
            Call clrbot
            Print armyname$(i); " is out of supplies";
            Call placearmy(i)
            Call TICK(turbo!)
            If Rnd > .8 And armysize(i) > 50 Then armysize(i) = .9 * armysize(i)
        End If
        rip:
    Next i
    For k = 1 To 2: Call stax(k): Next k
    y = 0: For i = 1 To 20: y = y + .1 * armysize(i): Next i
    x = 0: For i = 21 To 40: x = x + .1 * armysize(i): Next i
    If side = 2 And x > 0 Then aggress! = y / x Else aggress! = 1
    If side = 1 And y > 0 Then aggress! = x / y Else If side = 1 Then aggress! = 1
End Sub

Sub movefrom (index, target)
    colour = 3: tlx = 67: tly = 5: index = 0: size = 0: target = 0
    who = side: Call starfin(star, fin, who)
    For i = star To fin
        If armyloc(i) > 0 And armymove(i) = 0 Then size = size + 1: mtx$(size) = city$(armyloc(i)): array(size) = armyloc(i)
    Next i
    If size = 0 Then index = -1: GoTo notarg
    mtx$(0) = "From"
    Call bubble(size)
    movopt:
    tlx = 67: tly = 5
    hilite = 15: Call menu(1): Call clrrite
    If choose < 0 Then target = 0: GoTo notarg
    target = array(choose)

    size = 0
    For i = star To fin
        If armyloc(i) = target And armymove(i) = 0 Then index = i: size = size + 1: mtx$(size) = "Army" + Str$(i): array(size) = i
    Next i
    If size = 1 GoTo notarg
    mtx$(0) = "Which"
    Call bubble(size)
    tlx = 67: tly = 15: Call menu(4): Call clrrite
    If choose < 0 Then index = 0: GoTo notarg
    index = array(choose)
    notarg:
End Sub

Sub navy (who, chx)
    Dim hit(2)
    If chx > 0 Then
        If navyloc(who) < 1 Then
            chx = 1
        Else
            If commerce = 3 - who And raider > 0 Then chx = 4
            If grudge > 0 Then chx = 3
        End If
    End If
    Color 11
    ahoy:
    cost = 100: cityp(0) = who
    If chx = 0 Then Call clrbot: Print "Funds available "; cash(who);
    If chx = 1 Then
        If cash(who) < cost Then Exit Sub
        If navysize(who) > 9 Then Exit Sub
    End If
    mtx$(0) = "Ships"
    mtx$(1) = "Build"
    If navyloc(who) <> 99 Then
        If cash(who) < cost Or navysize(who) > 9 Or cityp(navyloc(who)) <> who Then mtx$(1) = "-": choose = 23
    End If
    mtx$(2) = "Attack"
    If navyloc(who) <> 99 Then
        If cityp(navyloc(who)) <> 3 - who Then mtx$(2) = "-": choose = 22
    Else
        mtx$(1) = "-": mtx$(2) = "-": choose = 24
    End If
    mtx$(3) = "Sail": If navyloc(who) < 1 Then mtx$(3) = "-": If chx = 3 Then chx = 1
    mtx$(4) = "Raid": If navysize(who) < 1 Or commerce = who Then mtx$(4) = "-"
    tlx = 67: tly = 12: size = 4
    If navyloc(who) <> 99 Then
        If who = 1 And navysize(who) > 1 And cityp(navyloc(who)) = 0 Then size = 5: mtx$(5) = "Invasion": Call void(navyloc(1), defend): If chx > 0 And defend < 100 Then chx = 5
        If realism = 0 And who = 2 And navysize(who) > 1 And cityp(navyloc(who)) = 0 Then size = 5: mtx$(5) = "Invasion": Call void(navyloc(1), defend): If chx > 0 And defend < 100 Then chx = 5
    End If

    If chx > 0 Then choose = chx: GoTo anchor
    If nflag > 0 And choose <> 1 Then Exit Sub
    Call menu(0): Call clrrite
    anchor:
    '---------------------------------------------------------------------------
    Select Case choose
        '---------------------------------------------------------------------------
        '                               Build Ship
        '---------------------------------------------------------------------------
        Case 1
            If cash(who) < cost Or navyloc(who) = 99 GoTo ahoy
            If chx = 1 And navysize(who) > 9 Then Exit Sub
            If chx = 0 And navysize(who) > 9 Then GoTo ahoy

            If chx = 0 And navyloc(who) > 0 Then
                If cityp(navyloc(who)) <> who GoTo ahoy
                If matrix(navyloc(who), 7) < 99 GoTo ahoy
            End If
            If chx = 1 And Rnd < .07 * navysize(who) Then Exit Sub
            a$ = "W"

            If realism > 0 And ((year < 1862) Or (year = 1862 And month < 3)) GoTo noiron
            If cash(who) >= 2 * cost Then
                tlx = 67: tly = 12: colour = 9
                If who = side Then
                    mtx$(0) = "Type"
                    mtx$(1) = "Wooden"
                    mtx$(2) = "Ironclad"
                    size = 2
                    Call menu(0)
                    Call clrrite
                    If choose < 1 GoTo ahoy
                    If choose = 2 Then a$ = "I": cost = 2 * cost
                Else
                    a$ = "I"
                End If
            End If

            noiron:
            navysize(who) = navysize(who) + 1
            If a$ = "W" Then
                fleet$(who) = fleet$(who) + a$: a = 1
            Else
                fleet$(who) = a$ + fleet$(who): a = 2
            End If
            If navysize(who) > 1 GoTo add2ship
            ships:
            mtx$(0) = "Port": size = 0: tlx = 67: tly = 12
            For i = 1 To 40: If matrix(i, 7) = 99 And cityp(i) = who And navyloc(3 - who) <> i Then size = size + 1: mtx$(size) = city$(i): array(size) = i
            Next i
            If chx = 1 Then choose = 1 + Int(size * Rnd)
            If size < 1 Then
                navysize(who) = 0: fleet$(who) = ""
                If chx = 0 Then Call clrbot: Print "NO SHIPYARDS AVAILABLE !";: Exit Sub
                If chx = 1 Then Exit Sub
            End If
            If chx = 0 Then Call menu(9): Call clrrite

            If choose < 1 GoTo ships
            navyloc(who) = array(choose)
            cash(who) = cash(who) - cost: navysize(who) = 1
            x = cityx(array(choose)) + 25: y = cityy(array(choose)) + 25: Call ships
            Call clrbot: Color 11: Print force$(who); " is building NEW fleet in "; city$(array(choose));
            PSet (400, 465), 0: Call shipicon(who, a)
            If noise > 0 Then Sound 3000, 1
            Call TICK(turbo!)
            GoTo ahoy
            add2ship:
            cash(who) = cash(who) - cost
            Call clrbot: Print force$(who); " navy increased to "; navysize(who);
            PSet (400, 465), 0: Call shipicon(who, a)
            Call ships
            If noise > 0 Then Sound 3000, 1
            Call TICK(turbo!)
            '---------------------------------------------------------------------------
            '                               Attack City
            '---------------------------------------------------------------------------
        Case 2
            If navyloc(who) = 99 GoTo ahoy
            If chx = 2 And cityp(navyloc(who)) = who Then Exit Sub
            If chx = 2 And occupied(navyloc(who)) > 0 And Rnd > .5 Then chx = 3: GoTo anchor
            If chx = 2 And cityp(navyloc(who)) = 0 Then chx = 3: GoTo ahoy
            If navyloc(who) < 1 Then Exit Sub
            If who = side Then nflag = 1
            If cityp(navyloc(who)) <> 3 - who GoTo ahoy
            Call clrbot: Color 12: Print force$(who); " fleet bombards "; city$(navyloc(who));
            Call icon(navyloc(who), 0, 3)
	      
            target = navyloc(who): index = occupied(target): If index = 0 GoTo deserted
            pct! = .005 * navysize(who) + .02 * Rnd: killd = armysize(index) * pct!: If killd < 1 Then killd = 1
            Call clrbot: Print armyname$(index); " suffered "; 100 * killd; " casualties";
            x = .5 * navysize(who) + 1: If x > supply(index) Then x = supply(index)
            supply(index) = supply(index) - x
            Call TICK(turbo!)
            armysize(index) = armysize(index) - killd: If armysize(index) < 1 Then armysize(index) = 1
            Exit Sub
            deserted:
            If fort(target) = 0 GoTo blast
            If Rnd < .7 + .03 * (navysize(who) - fort(target)) GoTo hurt1
            Call barnacle(who)
            Call clrbot
            Print force$(cityp(target)); " shore battery sunk an attacking ship!"; navysize(who); "ship(s) left! ";
            If noise > 0 Then Sound 77, .5: Sound 59, .5
            Call TICK(turbo!)
            If navysize(who) < 1 Then
                navyloc(who) = 0: fleet$(who) = "": GoSub box2
                Locate 12, 27: Print force$(who); " fleet eliminated"
                Call TICK(9)
                victory(3 - who) = victory(3 - who) + 5
                If who = side Then grudge = 0
                GoTo sail3
            End If
            GoTo deserted
            hurt1:
            Call clrbot: Print city$(target); " fortifications damaged";
            fort(target) = fort(target) - 1: x = cityx(target): y = cityy(target): Line (x - 5, y - 5)-(x + 5, y + 5), 2, BF
            Call showcity
            Call TICK(turbo!)
            Exit Sub
            blast:
            If Rnd > .25 + .07 * navysize(who) Then Call clrbot: Print "Citizens of "; city$(target); " stand firm against the attack";: Call TICK(turbo!): Exit Sub
            If navyloc(who) = capcity(3 - who) Then Call clrbot: Print "The CAPITAL steadfastly stands loyal";: Call TICK(turbo!): Exit Sub
            cityp(navyloc(who)) = 0
            Call clrbot: Print city$(navyloc(who)); " is now  NEUTRAL";
            Call showcity
            victory(who) = victory(who) + cityv(navyloc(who))
            Exit Sub
            '---------------------------------------------------------------------------
            '                               Sail
            '---------------------------------------------------------------------------
        Case 3
            navysize(who) = Len(fleet$(who))
            If navysize(who) < 1 And chx > 0 Then Exit Sub
            If navyloc(who) < 1 Or navysize(who) < 1 Then Call clrbot: Print "No ships remain";: TICK 1: GoTo ahoy

            size = 0
            For i = 1 To 40: If matrix(i, 7) > 90 And navyloc(who) <> i Then size = size + 1: mtx$(size) = city$(i): array(size) = i
            Next i
            If chx = 3 Then
                If size = 0 Then Exit Sub
                If grudge > 0 Then choose = 1: array(1) = navyloc(3 - who): GoTo admiral
                GoSub nest: If choose > 0 GoTo admiral Else Exit Sub
            End If
            Color 11: Locate 11, 68
            If navyloc(who) < 41 Then
                Print city$(navyloc(who))
            Else
                Print "Raiding"
            End If
            mtx$(0) = "To"
            colour = 3: tlx = 67: tly = 12
      
            Call menu(9): Call clrrite: If choose < 1 Then Exit Sub
            admiral:
            If array(choose) = navyloc(who) Then Exit Sub
            If array(choose) < 1 Then Exit Sub
            If navysize(who) < 1 GoTo ships
            If who = side Then nflag = 1
            GoSub box2
            Locate 10, 25: Print force$(who); " fleet of"; navysize(who); "ship(s) is sailing"
            Locate 11, 25: Print "From ";
            If navyloc(who) < 41 Then
                Print city$(navyloc(who));
            Else
                Print "Raiding";
            End If
            Print " to ";
            If array(choose) <> 99 Then
                Print city$(array(choose));
            Else
                Print "Raid Commerce";
            End If

            For i = 1 To navysize(who)
                PSet (120 + 41 * i, 210), 0
                Call shiptype(who, i)
            Next i
            If graf > 2 Then
                a = Asc(Left$(fleet$(who), 1))
                Select Case a
                    Case 73
                        Call ironclad
                    Case 87
                        Call schooner
                End Select
                TICK 2 * turbo!
            Else
                TICK turbo!
            End If
            navyloc(who) = array(choose)
            If navyloc(1) = navyloc(2) GoTo pirate
            GoTo sail3
            '---------------------------------------------------------------------------
            '                               Raid Commerce
            '---------------------------------------------------------------------------
        Case 4
            navysize(who) = Len(fleet$(who))
            If navysize(who) < 1 And chx > 0 Then Exit Sub
            If navysize(who) < 1 Or commerce = who GoTo ahoy

            If who = side Then nflag = 1
            GoSub box2
            Locate 10, 25: Print force$(who); " fleet of"; navysize(who); "ship(s) is sailing"
            Locate 11, 25: Print "to RAID "; force$(3 - who); " COMMERCE !"

            For i = 1 To navysize(who)
                PSet (120 + 41 * i, 210), 0
                Call shiptype(who, i)
            Next i
            If graf > 2 Then
                a = Asc(Left$(fleet$(who), 1))
                Select Case a
                    Case 73
                        Call ironclad
                    Case 87
                        Call schooner
                End Select
                TICK 2 * turbo!
            Else
                TICK turbo!
            End If
            navyloc(who) = 99: commerce = who
            If navyloc(1) = navyloc(2) GoTo pirate
            Cls: Call usa
            Exit Sub
            '---------------------------------------------------------------------------
            '                               Invasion
            '---------------------------------------------------------------------------
        Case 5
            Call commander((who), empty)
            If chx > 0 And empty = 0 Then chx = 3: GoTo anchor
            x = 35
            Call barnacle(who)
            c = navyloc(who): Call newarmy((who), empty, c)
            cash(who) = cash(who) + 100 'compensate for cost of recruiting new army
            Call capture(empty, c, who, 0)
            armysize(empty) = x
            If who = side Then nflag = 1
            Exit Sub
            '---------------------------------------------------------------------------
            '                               Exit
            '---------------------------------------------------------------------------
        Case Else
            Exit Sub
    End Select
    GoTo ahoy
    '---------------------------------------------------------------------------
    '                               Ship vs. Ship
    '---------------------------------------------------------------------------
    pirate:
    Cls: Locate 1, 30: Color 11: Print "NAVAL COMBAT"
    For k = 1 To 2
        hit(k) = 10
        If Right$(fleet$(k), 1) = "I" Then hit(k) = 20
    Next k
    cannon:
    Locate 9, 1: Print Space$(79)
    Locate 15, 1: Print Space$(79)
    x = 10: y = 100
    Color 9: Locate 5, 25: Print "UNION"; navysize(1); "ship(s)"
    GoSub boxes
    Color 1
    For i = 1 To navysize(1)
        x = x + 50: PSet (x, y)
        Call shiptype(1, i)
    Next i
    Color 11: Locate 11, 22: Print "CONFEDERATES "; navysize(2); "ship(s)"
    Line (10, 180)-(530, 210), 1, BF
    Line (10, 180)-(530, 210), 11, B
    x = 10: y = 200
    Color 1
    For i = 1 To navysize(2)
        x = x + 50: PSet (x, y)
        Call shiptype(2, i)
    Next i
    Color 11
    wave:
    mtx$(0) = "Options"
    mtx$(1) = "Attack"
    mtx$(2) = "Retreat"
    If chx > 0 Then choose = 1: GoTo powder
      
    size = 2: colour = 3: hilite = 14: tlx = 50: tly = 18: Call menu(0)
    powder:
    Select Case choose
        Case 1
            firemore:
            If noise > 0 Then Sound 77, .5: Sound 59, .5: Call TICK(.1)
            pct! = 0: a$ = Right$(fleet$(who), 1)
            If a$ <> Right$(fleet$(3 - who), 1) Then
                If a$ = "I" Then pct! = .1 Else pct! = -.1
            End If
            If Rnd <= .5 + pct! GoTo hitem Else GoTo hitme1
            GoTo firemore
            hitem:
            hit(3 - who) = hit(3 - who) - 1: GoSub showhit: If hit(3 - who) > 0 GoTo firemore
            x = 10 + 50 * navysize(3 - who)
            y = 90: If who = 1 Then y = 190
            GoSub xout
            Locate 17, 5: Print force$(3 - who); " ship SUNK!": Call TICK(turbo! + 1): GoSub clr1
            Call barnacle(3 - who)
            If navysize(3 - who) < 1 Then
                Locate 19, 5: Color 12
                Print force$(3 - who); " fleet DEFEATED"
                Call TICK(turbo!)
                navyloc(3 - who) = 0: fleet$(3 - who) = ""
                victory(3 - who) = victory(3 - who) + 10
                If who = side Then grudge = 1 Else grudge = 0
                GoTo sail3
            End If
            hit(3 - who) = 10
            If Right$(fleet$(3 - who), 1) = "I" Then hit(3 - who) = 20
            GoTo cannon
            hitme1:
            hit(who) = hit(who) - 1: GoSub showhit: If hit(who) > 0 GoTo firemore
            x = 10 + 50 * navysize(who)
            y = 190: If who = 1 Then y = 90
            GoSub xout
            Locate 17, 5: Print "One of the "; force$(who); " ships was SUNK!": Call TICK(turbo!): GoSub clr1
            Call barnacle(who)
            If navysize(who) < 1 Then
                Locate 19, 5: Color 12
                Print "Attacking "; force$(who); " fleet has been ELIMINATED !"
                Call TICK(turbo!)
                navyloc(who) = 0: fleet$(who) = ""
                victory(who) = victory(who) + 10
                If who <> side Then grudge = 1 Else grudge = 0
                GoTo sail3
            End If
            hit(who) = 10
            If Right$(fleet$(who), 1) = "I" Then hit(who) = 20
            GoTo cannon

        Case 2
            target = 0
            For i = 1 To 40
                If cityp(i) = who Then If i <> navyloc(who) And matrix(i, 7) = 99 Then target = i: If Rnd > .3 GoTo found1
            Next i
            found1:
            Call clrbot: Color 11: Print force$(who); " is retreating to "; city$(target);: Call TICK(turbo!)
            navyloc(who) = target
            GoTo sail3
        Case Else
            GoTo wave
    End Select
    GoTo pirate
    '---------------------------------------------------------------------------
    '                               Subs
    '---------------------------------------------------------------------------
    showhit:
    Locate 9, 6 * navysize(1): Print hit(1); " ";
    Locate 15, 6 * navysize(2): Print hit(2); " ";
    If noise = 0 Then TICK .1 * turbo!
    Return
    clr1:
    Locate 17, 1: Print Space$(60): Return
    nest:
    best = 0: x = 0
    For i = 1 To size
        target = array(i)
        If cityp(target) = 0 GoTo neutral

        If target = navyloc(3 - who) Then If navysize(who) >= navysize(3 - who) And Rnd > .1 Then choose = i: Return
        If cityp(target) <> who Then
            best = i: x = x + 1
            If Rnd > .8 Then choose = best: Return
        End If
        neutral:
    Next i
    If best = 0 Then
        choose = 1 + Int(Rnd * size)
    Else
        choose = best
    End If

    If who = 2 And navyloc(2) = 30 Then If Rnd > .5 Then choose = 30: Return
    Return
    boxes:
    Line (10, 80)-(530, 110), 1, BF
    Line (10, 80)-(530, 110), 11, B
    Return
    box2:
    Cls
    Line (100, 110)-(550, 240), 1, B
    Line (95, 115)-(555, 235), 7, B
    Return
    xout:
    PSet (x, y)
    Draw "S5C15G5F5G3H5G5H3E5H5E3F5E5F3"
    Paint (x - 3, y + 1), 12, 15
    Return
    sail3:
    commerce = 0
    For k = 1 To 2
        If navyloc(k) = 99 Then commerce = k
    Next k
    Cls: Call usa
End Sub

Sub newarmy (who, empty, target)
    supply(empty) = 3 + 5 * Rnd: If who = 1 Then supply(empty) = supply(empty) + 2
    armyexper(empty) = 1: If who = 2 Then armyexper(empty) = 2
    armylead(empty) = rating(empty)
    armyname$(empty) = lname$(empty): lname$(empty) = ""
    Color 12: Call clrbot: Print "Placing NEW army "; empty; armyname$(empty); " in "; city$(target);

    x = 70: If realism > 0 Then x = 3 * cityv(target) + 33
    Call cutoff(who, target, a): If a < 1 Then x = x \ 3
    occupied(target) = empty: armysize(empty) = x
    Print " Size = ";: Call strong(empty)
    cash(who) = cash(who) - 100: If noise > 0 Then Sound 2222, .5
    armyloc(empty) = target: Call placearmy(empty)
    Call TICK(turbo! - .5)
    armymove(empty) = -1
End Sub

Sub newcity (index)
    mtx$(1) = "  "
    mtx$(3) = "  "
    size = 3: a$ = city$(index)
    morecap:
    tlx = 68: tly = 15: colour = 3: choose = 23
    mtx$(2) = city$(index)
    Call menu(3): Call clrrite
    Select Case choose
        Case Is = -27
            index = 0: Exit Sub
        Case Is < 1
            a$ = Chr$(Abs(choose))
            x = 0
            For k = 1 To 40
                If Left$(city$(k), 1) = a$ Then
                    y = index: index = k
                    spin2:
                    If cityp(index) = side GoTo morecap
                    index = index + 1
                    If index > 40 Then index = 1
                    x = x + 1: If x < 39 GoTo spin2
                    index = y: GoTo morecap
                End If
            Next k
            GoTo morecap
        Case 1
            minus1:
            index = index - 1: If index < 1 Then index = 40
            If cityp(index) <> side GoTo minus1 Else GoTo morecap
        Case 2
            GoTo fnew
        Case 3
            plus1:
            index = index + 1: If index > 40 Then index = 1
            If cityp(index) <> side GoTo plus1 Else GoTo morecap
        Case Else
            index = 0
    End Select
    fnew:
End Sub

Sub normal (xbar, vary, result)
    ' NOTE : vary is VARIANCE
    pct! = 0
    For k = 1 To 12: pct! = pct! + Rnd: Next k
    pct! = pct! - 5.5
    result = xbar + pct! * Sqr(vary)
End Sub

Sub placearmy (which)
    who = 1: If which > 20 Then who = 2
    x = cityx(armyloc(which)) - 12
    y = cityy(armyloc(which)) - 11
    Call armyxy(x, y, who)
    If supply(which) < 1 Then
        x = x - 3: y = y + 4
        PSet (x, y), 13: Draw "C11S8"
        Draw font$(19)
        Draw "S4"
    End If
End Sub

Sub railroad (who)
    Select Case rr(who)
        Case 0
            If who = side GoTo human
            Call starfin(star, fin, who): max = 32767
            index = 0
            For i = star To fin: If armyloc(i) = 0 Or armymove(i) <> 0 GoTo puter
                x = 0: For j = 1 To 6: from = matrix(armyloc(i), j): If from > 0 And cityp(from) <> who Then x = x + 1
                Next j
                If occupied(capcity(who)) > 0 And x > 1 GoTo puter
                Call void(armyloc(i), defend)
                If vicflag(5) > 0 And occupied(capcity(who)) = 0 Then If defend > max Then defend = max - 1
                If defend < max Then target = armyloc(i): GoSub route: If x > 1 Then max = defend: index = i
            puter: Next i
            If index = 0 Or max > .3 * armysize(index) GoTo toot
            Call traincapacity(who, limit)
            If armysize(index) > limit Then GoTo toot
            from = armyloc(index)
            x1 = 0: For i = 1 To 40: If cityp(i) <> who GoTo puted
                Call void(i, defend): If defend = 0 Or defend > 3 * armysize(index) GoTo puted
                If occupied(i) > 0 GoTo puted
                target = i
                GoSub route: If x = 0 GoTo puted
                If side = 1 And cityy(i) < cityy(from) Then x1 = i
                If side = 2 And cityy(i) > cityy(from) Then x1 = i
                If fort(i) > 0 And occupied(i) = 0 Then x1 = i: Exit For
            puted: Next i
            If vicflag(5) > 0 And occupied(capcity(who)) = 0 Then
                GoSub route
                If x > 0 Then
                    Call void(capcity(who), defend)
                    If defend > 0 Then x1 = capcity(who)
                End If
            End If
            If x1 > 0 GoTo haul Else GoTo toot
            human:
            Call movefrom(index, a): If a < 1 Or index < 1 Then Color 11: Call clrbot: Print "Railroad move not allowed";: GoTo toot
            x = 1: If index > 20 Then x = 2
            Call traincapacity(who, limit)
            If armysize(index) > limit Then Call clrbot: Print "Too many troops for railroad capacity";: GoTo toot
	
            target = armyloc(index): GoSub route: If x > 1 GoTo aboard
            Call clrbot: Print "Track from "; city$(target); " is blocked";: GoTo toot
            aboard:
            Call clrbot: Print "Select destination for "; armyname$(index); " from "; city$(target);
            mtx$(0) = "To": from = target: x1 = from: If from = 0 GoTo toot
            Call newcity(x1): If x1 = from Or x1 = 0 GoTo toot

            target = x1: GoSub route: If x > 1 GoTo haul
            Call clrbot: Print armyname$(index); "'s train cannot go to "; city$(x1); " - select another city";: Call TICK(9)
            GoTo aboard
            haul:
            Color 11: Call clrbot: Print armyname$(index); " is taking the train from "; city$(from); " to "; city$(x1);
            tracks(who) = armyloc(index)
            from = armyloc(index)
            rrfrom(who) = from
            rr(who) = index: armyloc(index) = 0: armymove(index) = x1
            Call tinytrain(who, 1)
            If noise > 0 Then Sound 2222, 1
            Call engine

            Call occupy(from)
            If occupied(from) > 0 Then Call placearmy(occupied(from))
            GoTo toot

        Case Is > 0
            Call tinytrain(who, 0)
            Call occupy(rrfrom(who))
            If occupied(rrfrom(who)) > 0 Then Call placearmy(occupied(rrfrom(who)))

            index = rr(who)
            tracks(who) = armymove(index)
            Call tinytrain(who, 1)

            rr(who) = 0: armyloc(index) = armymove(index): armymove(index) = -1
            Color 11: Call clrbot: Print "Train with "; armyname$(index); " has arrived in "; city$(armyloc(index));: If noise > 0 Then Sound 1200, 2
            Call TICK(turbo!)
            x = cityx(armyloc(index)) - 12: y = cityy(armyloc(index)) - 11
            Line (x - 9, y - 8)-(x + 10, y + 8), 2, BF
            If cityp(armyloc(index)) <> who Then Call capture(index, armyloc(index), who, 0)
            Call placearmy(index)
            If occupied(armyloc(index)) > 0 Then Call stax(who): GoTo toot
            Call occupy(armyloc(index)): Call placearmy(index)
            GoTo toot
    End Select

    route:
    x = 0: For j = 1 To 6
        y = matrix(target, j)
        If y > 0 And cityp(y) = who Then
            x = x + 1
            For m = 1 To 6
                z = matrix(y, m)
                If z > 0 And z <> target And cityp(z) = who Then x = x + 1
            Next m
        End If
    Next j
    Return

    toot:
    If who = side Then Call TICK(9)
    Call clrrite
    clrbot
End Sub

Sub recruit (who)
    If cash(who) < 100 GoTo menu1
    size = 0: Color 12
    mtx$(0) = "Recruit"
    If who = side And rflag > 0 Then For i = 1 To rflag: x = rcity(i): mtx$(i) = city$(x): array(i) = x: Next i: size = rflag: GoTo alldone
    max = 4: If Rnd > .8 Then max = max - 1
    If difficult < 3 Then max = max + 1

    For i = 1 To 40
        If control(who) > 0 Then pct! = .6 * max / control(who) Else pct! = .3
        If size = 0 And i > 20 Then pct! = .3
        If size < 2 And i > 30 Then pct! = .3
        array(i) = 0
        If occupied(i) > 0 And cityp(i) = who Then pct! = .4
        If Rnd < pct! And cityp(i) = who Then
            If realism > 0 And cityo(i) = 3 - who GoTo foe
            size = size + 1
            mtx$(size) = city$(i)
            array(size) = i
            If size > max - 1 GoTo alldone
        End If
        foe:
    Next i
    alldone:
    If size = 0 GoTo menu1
    If who = side And rflag = 0 Then For i = 1 To size: rcity(i) = array(i): Next i: rflag = size
    Call bubble(size)
    '............................................................................
    levy:
    If cash(who) < 100 GoTo menu1
    If (who = side) Or (player = 2) Then Call clrbot: Print force$(who); " funds available "; cash(who);
    tlx = 67: tly = 12: colour = 3
    If who <> side Then GoSub randsel: GoTo auto1
      
    If choose > 0 Then choose = 21 + choose
    Call menu(2): Call clrrite
    If choose < 1 GoTo menu1
    auto1:
    empty = 0
    index = array(choose)
    Call occupy(index)
    If occupied(index) > 0 Then i = occupied(index): GoTo add2
    Call commander((who), empty)
       
    If who <> side And empty = 0 GoTo menu1
    If empty = 0 GoTo levy
    armyloc(empty) = index
    GoSub playb
    Call newarmy(who, empty, index)
    GoTo levy
    '............................................................................
    add2:
    target = array(choose)
    x = 45: a$ = ""
    If realism > 0 Then
        x = 2 * cityv(armyloc(i)) + 20
        Call cutoff(who, target, a)
        If a < 1 Then x = x \ 3: a$ = " ISOLATED !"
    End If
    armysize(i) = armysize(i) + x

    GoSub playb
    cash(who) = cash(who) - 100
    from = armyloc(i): x = cityx(from) - 12: y = cityy(from) - 11
    Get (x - 9, y - 7)-(x + 9, y + 6), anima()
    Call icon(armyloc(i), 0, 2)
    If noise > 0 Then Sound 2222, 1
    Color 11: Call clrbot: Print "Army "; i; armyname$(i); " in "; city$(armyloc(i)); " strength increased to ";: Call strong(i)
    If a$ <> "" Then Print a$;
    Put (x - 9, y - 7), anima(), PSet
    Call TICK(turbo! - .5)
    GoTo levy
    randsel:
    choose = 0: x = 0
    For i = 1 To size
        target = array(i)
        Locate 14 + i, 69
        x = occupied(target)
        If x = 0 Xor armysize(x) < 155 Then choose = i
    Next i

    If choose = 0 Then choose = 1 + Int(Rnd * size)
    If realism > 0 Then
        x = 0
        For i = 1 To size
            target = array(i)
            If occupied(target) = 0 Then
                y = 3 * cityv(target) + 33
                Call cutoff(who, target, a)
                If a < 1 Then y = y \ 3
            Else
                y = 2 * cityv(target) + 20
                Call cutoff(who, target, a)
                If a < 1 Then y = y \ 3
            End If
            If y > x And Rnd > .5 Then x = y: choose = i
        Next i
    End If
    If cash(who) < 100 Then choose = size
    Return
    playb:
    If side = 1 And difficult < 3 Then armysize(empty) = armysize(empty) + 15 - 5 * difficult
    If side = 2 And difficult > 3 Then armysize(empty) = armysize(empty) + 5 * difficult - 15
    Return
    menu1:
End Sub

Sub shiptype (who, i)
    a = 1: If Mid$(fleet$(who), i, 1) = "I" Then a = 2
    Call shipicon(who, a)
End Sub

Sub showcity
    For i = 1 To 40
        c = 9: If cityp(i) = 2 Then c = 7
        If cityp(i) = 0 Then c = 12
        x = cityx(i): y = cityy(i)
        If i = capcity(1) Or i = capcity(2) Then
            Put (x - 6, y - 6), Ncap(), PSet
            Line (x + 9, y - 4)-(x + 15, y + 4), 0, BF
            Line (x + 8, y - 5)-(x + 13, y + 2), 3, BF
            PSet (x + 8, y - 4)
            If fort(i) = 1 Then Draw "BR2C0e1D6BL1R2"
            If fort(i) = 2 Then Draw "C0e1R1F1D1G3R3"
        Else
            If fort(i) = 1 Then Line (x - 5, y - 5)-(x + 5, y + 5), 0, B
            If fort(i) > 1 Then Line (x - 5, y - 5)-(x + 5, y + 5), 0, BF
            Circle (cityx(i), cityy(i)), 4, 0
            Circle (cityx(i), cityy(i)), 3, c
            Paint (cityx(i), cityy(i)), c, c
        End If
        If graf = 0 GoTo nocon
        For j = 1 To 6: If matrix(i, j) > 0 Then Call icon(i, matrix(i, j), 11)
        Next j
        nocon:
    Next i
End Sub

Sub smarts
    Call events
    slush = 0: who = 3 - side
    c = 9: If side = 1 Then c = 15
    Locate 1, 1: Print Space$(80);
    Color c: Locate 1, 10: Print force$(who); " Side is making decisions";
    Call starfin(star, fin, who)
    '==========================================================================
    '                       Check Supply
    '==========================================================================
    For i = star To fin: If armyloc(i) > 0 And supply(i) < 1 Then Call resupply(i)
    Next i
    '==========================================================================
    '                       Fortify Capital or Other City
    '==========================================================================
    cost = 200
    If cost > cash(who) GoTo signup

    target = 0: If occupied(capcity(who)) = 0 Or fort(capcity(who)) > 1 GoTo city2
    target = capcity(who)
    If cityp(target) = who GoTo capital Else GoTo city2
    If target > 0 GoTo capital

    city2:
    If 1 + Rnd < bold + aggress! GoTo signup
    target = 0: For i = star To fin
        x = armyloc(i): If x = 0 Or armymove(occupied(x)) < 0 Or armymove(i) < 0 GoTo endlook
        If fort(x) > 1 GoTo endlook
        Call void(x, defend): If defend > armysize(i) Then target = armyloc(i): GoTo capital
        endlook:
    Next i

    capital:
    If target = 0 GoTo signup
    If fort(target) > 1 GoTo signup
    fort(target) = fort(target) + 1
    x = occupied(target): armymove(x) = -1
    cash(who) = cash(who) - cost
    Call clrbot: Print city$(target); " fortifications increased to "; fort(target);
    Call icon(target, 0, 6)
    Call showcity
    If who = 1 And navysize(1) < 1 And Rnd > .5 And cash < 222 GoTo signup
    If 3 * Rnd > 1 + aggress! And cash(who) >= cost GoTo city2
    '==========================================================================
    '                       Recruit New Armies
    '==========================================================================
    signup:
    If who = 1 And navysize(1) < 1 And cash(1) > 100 Then Call navy(1, 1): If cash(who) < 100 GoTo isok
    GoSub enufarmies
    If x > 2 + 3 * side + 3 * Rnd GoTo isok
    If navysize(who) < 5 And cash(who) > 100 And navyloc(who) <> 99 Then If cityp(navyloc(who)) = who Then slush = slush + 100: cash(who) = cash(who) - 100: If cash(who) < 100 GoTo isok
    Call recruit(who)
    '==========================================================================
    '                         Naval Commands
    '==========================================================================
    isok:
    cash(who) = cash(who) + slush
    dock:
    If side = 2 And navyloc(1) = 0 And navysize(1) < 1 And cityp(30) = 1 And cash(1) > 100 Then navyloc(1) = 30: Call navy(1, 1): GoTo movenavy
    If navyloc(who) = 99 Then
        If Rnd < .9 Then Call navy(who, 3)
        GoTo movearmy
    End If
    If cash(who) < 100 Or navysize(who) > 9 Or cityp(navyloc(who)) <> who GoTo movenavy
    Call navy(who, 1)
    '- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    movenavy:
    If navysize(who) = 0 Then navyloc(who) = 0
    If navyloc(who) = 0 GoTo movearmy
    '- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    '                     move ships & prohibit further action this turn
    '- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    If navyloc(who) <> 99 Then
        If cityp(navyloc(who)) = who Then Call navy(who, 3): GoTo movearmy 'move ships & prohibit further action this turn
        If cityp(navyloc(who)) = 0 Then Call navy(who, 3): GoTo movearmy
    Else
        If Rnd < .5 Then Call navy(who, 3)
        GoTo movearmy
    End If

    If cityp(navyloc(who)) <> who Then
        If occupied(navyloc(who)) = 0 Or Rnd > .3 Then
            Call navy(who, 2) 'attack
        Else
            Call navy(who, 3) 'move
        End If
    End If
    '==========================================================================
    '                       Give Move Orders to Each Army
    '==========================================================================
    movearmy:
    Call ships
    '---------------------------------------------------------------------------
    '                               Check to Combine
    '---------------------------------------------------------------------------
    a = who: Call combine(a)
    Call railroad(who)
    '---------------------------------------------------------------------------
    '                               Top Priority
    '---------------------------------------------------------------------------
    For i = star To fin
        target = armyloc(i)
        If armyloc(i) < 1 Or armymove(i) < 0 GoTo deadman
        x = armysize(i): If supply(i) < 1 And Rnd > .1 GoTo deadman

        choose = 0

        Call void(target, defend)

        If defend > 0 And target = capcity(who) And occupied(target) = i GoTo deadman 'hold capital
        If target <> capcity(who) GoTo nocap
        If vicflag(5) = 0 GoTo nocap
        For j = 1 To 6
            k = matrix(target, j): If j = 0 GoTo nocap
            Call void(k, flag): If flag > 0 GoTo deadman
        Next j
        nocap:
        Call evaluate(i, best): If best = 0 GoTo deadman
        move9:
        If best = 0 Then choose = 1 + Int((j - 1) * Rnd)
        If choose = 0 Then choose = best
        armymove(i) = matrix(target, choose)

        If cityp(armymove(i)) = who Then y = occupied(armymove(i)): If y > 0 Then If armymove(y) = x Then armymove(i) = 0: GoTo deadman ' eliminate crossing moves
        If month = 1 And jancam = 0 And cityp(armymove(i)) <> who Then armymove(i) = 0: GoTo deadman

        If turbo! > 1 Then
            Color c: Call clrbot
            Print armyname$(i); " plans to move from "; city$(armyloc(i)); " to "; city$(armymove(i));
        End If
        Call icon(armyloc(i), armymove(i), 1)
        Call icon(armyloc(i), armymove(i), 9)
        Call TICK(turbo! - 1)
        Put (cityx(armyloc(i)) - 20, cityy(armyloc(i)) - 19), image(), PSet
        deadman:
    Next i
    For i = star To fin
        If armymove(i) = 0 And armyexper(i) < 6 And armyexper(i) < armylead(i) Then
            armyexper(i) = armyexper(i) + 1

            If turbo! > 1 Then
                Call clrbot
                Print armyname$(i); " has drilled to reach experience level "; armyexper(i);
            End If
            armymove(i) = -1
            Call TICK(turbo! - 1)
        End If
    Next i
    Exit Sub
    '---------------------------------------------------------------------------
    enufarmies:
    Call starfin(star, fin, 3 - who)
    For i = star To fin
        If armyloc(i) > 0 Then y = y + .1 * armysize(i)
    Next i
    Call starfin(star, fin, who)
    x = 0: z = 0: For i = star To fin: If armyloc(i) > 0 Then x = x + 1: z = z + .1 * armysize(i)
    Next i
    If y > z Then x = x - 2
    If y > 2 * z Then x = 0
    Return
    '---------------------------------------------------------------------------
End Sub

Sub snapshot (x, y, flag)
    If flag = 0 Then Get (x - 10, y - 10)-(x + 10, y + 10), image()
    If flag = 1 Then Put (x - 10, y - 10), image(), PSet
End Sub

Sub strong (index)
    a$ = LTrim$(RTrim$(Str$(armysize(index)))) + "00"
    Print a$;
End Sub

Sub tinytrain (who, flag)
    If tracks(who) = 0 Then Exit Sub
    from = tracks(who)
    x = cityx(from) - 12: y = cityy(from) - 11
    Select Case flag
        Case Is > 0
            Line (x - 8, y - 6)-(x + 10, y + 7), 10, BF
            PSet (x - 6, y - 2), 3: c = 9: If who = 2 Then c = 15
            Draw "C0S2R9D4R6U3R3D3R7U5H3U2R9D3G2D6F1D3F5L10D1G1L4H2L7G2L3H2L3U8L2U5BF4S4"
            x = Point(0): y = Point(1): Paint (x, y), c, 0
        Case Else
            Line (x - 9, y - 8)-(x + 10, y + 8), 2, BF
    End Select
End Sub

Sub tupdate
    flag = 0
    If player = 1 Then
        Call smarts
    Else
        For i = 1 To 40
            If armyloc(i) > 0 And armymove(i) > 0 Then Call icon(armyloc(i), armymove(i), 1)
        Next i
    End If
    Color 14: Locate 1, 1: Print Space$(80);
    Locate 1, 20: Print "Update for "; month$(month); ","; year
    Call clrbot: Print "press any key for "; month$(month); ","; year; " events";
    Def Seg = 0: Poke 1265, 5: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop
    GoSub upbox
    '==========================================================================
    '                       Assign Time of Action
    '==========================================================================
    For i = 1 To 2: If rr(i) > 0 Then Call railroad(i): Line (5, 17)-(100, 63), 3, BF: Line (5, 17)-(100, 63), 0, B: Call engine
    Next i

    For i = 1 To 40
        brray(i) = 999
        If armymove(i) < 0 Then armymove(i) = 0
        If armyloc(i) > 0 And armymove(i) > 0 Then brray(i) = Int(4 + 4 * Rnd) * 100 + i
        If supply(i) < 1 And armymove(i) > 0 Then brray(i) = 900 + i
        Select Case armysize(i) 'bigger armies move slower
            Case Is > 400
                If brray(i) < 900 Then brray(i) = brray(i) + 100: x = 2
            Case Is > 800
                If brray(i) < 800 Then brray(i) = brray(i) + 200: x = 3
            Case Is > 1000
                If brray(i) < 700 Then brray(i) = brray(i) + 300: x = 4
            Case Else
                x = 1
        End Select
        If brray(i) <> 999 And armylead(i) > 10 * Rnd Then
            brray(i) = brray(i) - 100 * (armylead(i) \ 2)
            If brray(i) < 100 Then brray(i) = 100 + i
        End If
    Next i

    Call bub2(40)
    '==========================================================================
    '                       Begin Main Loop
    '==========================================================================
    For j = 1 To 40
        flag = 0
        If brray(j) = 999 GoTo allthru
        active = Int(brray(j) / 100): active = brray(j) - 100 * active: s = 1: If active > 20 Then s = 2
        If armymove(active) < 1 GoTo digin
        Color 11: Call clrbot: Print armyname$(active); " is moving to "; city$(armymove(active));

        supply(active) = supply(active) - 1: If supply(active) < 0 Then supply(active) = 0: Call TICK(turbo!): Call clrbot: Print armyname$(active); " is out of supplies !";

        Call placearmy(active)
        Call icon(armyloc(active), armymove(active), 5)

        Call animate(active, 0)
        '==========================================================================
        target = armymove(active)
        If occupied(target) = 0 GoTo easy
        If (s = 1 And occupied(target) < 21) Or (s = 2 And occupied(target) > 20) GoTo friend Else GoTo enemy
        '==========================================================================
        '                         Join Forces
        '==========================================================================
        friend:
        Color 11: Call clrbot: Print armyname$(active); " and "; armyname$(occupied(target)); " meet in "; city$(target);: Call TICK(turbo!)
        Call icon(armymove(active), 0, 6)
        Call clrbot
        GoTo easy
        '==========================================================================
        '                       Attack
        '==========================================================================
        enemy:
        Call icon(target, 0, 3)
        defend = occupied(target)

        Color 11: Call clrbot: Print armyname$(active); " attacks "; armyname$(defend); " in "; city$(armyloc(defend));: Call TICK(turbo! - 1)
        If armysize(defend) > 0 Then Call battle(active, defend, win, lose): flag = 1: If graf > 0 Then GoSub upbox
        If armyexper(win) < 10 Then armyexper(win) = armyexper(win) + 1

        If armysize(lose) < 2 Then
            Call clrbot
            a$ = armyname$(lose) + "'s army is crushed in " + city$(armyloc(defend))
            Call scribe(a$, 2)
            index = lose: GoTo crushed
        End If

        Call clrrite
        '==========================================================================
        '                       Attacker Loses
        '==========================================================================
        If win = active GoTo kickbutt
        armymove(active) = armyloc(active)
        armyloc(active) = target
        Color 11: Call clrbot: Print armyname$(active); " withdrew to "; city$(armymove(active));

        Call placearmy(armyloc(active))
        Call animate(active, 1)

        armyloc(active) = armymove(active)
        Call placearmy(armyloc(active))
        Call placearmy(active)
        occupied(armyloc(active)) = active
        armymove(active) = -2
	      
        If 11 * Rnd > armylead(defend) Then Call icon(armyloc(defend), armymove(defend), 4): If armymove(defend) > 0 Then armymove(defend) = -2

        Call TICK(turbo!): GoTo digin
        '==========================================================================
        '                       Defender Retreats
        '==========================================================================
        kickbutt:
        Call icon(armyloc(active), target, 4)
        If armymove(defend) > 0 Then Call icon(target, armymove(defend), 4)

        Call retreat(defend, flee): If flee > 0 Then move2 = flee: GoTo outta

        best = 0: flee = 0
        For i = 1 To 6
            x = matrix(target, i)
            If x > 0 And (cityp(x) = 3 - s) And cityv(x) > best Then
                If best = 0 Then
                    flee = i: best = cityv(x)
                Else
                    If occupied(x) = 0 Then flee = i: best = cityv(x)
                End If
            End If
        Next i
        If flee = 0 GoTo deadmeat
        move2 = matrix(target, flee)
        outta:
        Call placearmy(defend) ' in old city

        armymove(defend) = move2
        Call animate(defend, 0) ' animate retreat
       
        armyloc(defend) = move2
        occupied(move2) = defend
        Call clrbot
        Print armyname$(defend) + " is withdrawing to " + city$(move2);

        Call placearmy(defend) ' in new city
       
        Call icon(target, 0, 6)
        armymove(defend) = -2
        GoTo holdon

        deadmeat:
        index = defend
        a$ = armyname$(index) + " surrenders to " + armyname$(active) + " in " + city$(armyloc(index))
        Call scribe(a$, 2)
        crushed:
        If graf > 2 Then Call surrender(index): Color 14: Locate 3, 68: Print armyname$(index): Locate 4, 68: Print "surrenders !"
        x = 1: If index > 20 Then x = 2
        If noise > 1 And x <> side Then Play "MFMST220o3e4g8g2.g8g8g8o4c2"
        If armymove(index) > 0 Then Call icon(armyloc(index), armymove(index), 4)
        victory(3 - s) = victory(3 - s) + 25
        armyloc(index) = 0: lname$(index) = "": armyname$(index) = ""
        armysize(index) = 0: armylead(index) = 0
        armyexper(index) = 0: armymove(index) = 0: supply(index) = 0
        supply(active) = supply(active) + supply(index): supply(index) = 0
        If supply(active) > 10 Then supply(active) = 10
        Call TICK(9): Call clrrite
        If armyloc(active) = 0 GoTo digin
        '---------------------------------------------------------------------------
        '                       check for more defenders !
        '---------------------------------------------------------------------------
        holdon:
        Call occupy(target): If occupied(target) = 0 GoTo easy
        Color 11: Call clrbot: Print "There are still defenders in "; city$(target);: Call TICK(99)
        GoTo enemy
        '==========================================================================
        '                       Move Into New City
        '==========================================================================
        easy:
        Call icon(armyloc(active), target, 4)
        armyloc(active) = target: armymove(active) = -2
        Call occupy(armyloc(active))
        Call placearmy(active)
        '---------------------------------------------------------------------------
        '                            City Capture
        '---------------------------------------------------------------------------
        If cityp(armyloc(active)) = s GoTo horde
        c = armyloc(active)
        If player = 1 And s = side And fort(c) > 0 And flag = 0 Then
            tlx = 67: tly = 15
            hilite = 15: colour = 3
            mtx$(0) = "Raze ?"
            mtx$(1) = "No"
            mtx$(2) = "Yes"
            size = 2
            Call menu(0)
            Call clrrite
            If choose = 2 GoTo smoke
        End If
        If player = 1 And s <> side And fort(c) > 0 And flag = 0 Then
            If realism > 0 And cityy(c) > 150 Then
                smoke:
                fort(c) = 0
                x = cityx(c)
                y = cityy(c)
                Line (x - 5, y - 5)-(x + 5, y + 5), 2, BF
                Call showcity
                Call clrbot
                Print armyname$(active); " has destroyed the forts at "; city$(c);
                TICK 3
            End If
        End If
        Call capture(active, c, s, flag)
        horde:
        If supply(active) < 10 Then supply(active) = supply(active) + 1: If Rnd > .8 GoTo horde
        digin:
    Next j
    GoTo allthru

    upbox:
    Line (450, 318)-(526, 420), 1, BF
    Line (527, 315)-(527, 439), 10
    Color 14: Locate 23, 50: Print "+-----------+"
    Locate 24, 50: Print "| U P D A T E |"
    Locate 25, 50: Print "+-----------+"
    Return

    allthru:
    If navysize(commerce) < 1 Then commerce = 0
    If commerce > 0 Then
        Call clrbot
        If Rnd < .8 + .02 * navysize(commerce) Then
            raider = .05 * navysize(commerce) * (1 + Rnd) * (income(3 - commerce))
            If raider < 1 Then raider = 1
            If raider / income(3 - commerce) > .3 Then raider = .3 * income(3 - commerce)
            Color 15: Print force$(commerce); " raiders have sunk $"; raider; " of "; force$(3 - commerce); " commerce";
            a = 1: If Left$(fleet$(commerce), 1) = "I" Then a = 2
            PSet (500, 465), 0: Call shipicon(commerce, a)
            If noise > 0 Then Play "t210l8o4co3bo4l4co3ccL8gfego4co3bo4c"
            If commerce = side Then grudge = 1
            TICK 9
        Else
            raider = 0
            Call barnacle(commerce)
            Color 15: Print force$(commerce); " raiders have lost a ship ("; navysize(commerce); " remain)";
            If noise > 0 Then Sound 590, .5: Sound 999, .5: Sound 1999, .5
            If navyloc(commerce) > 0 Then
                TICK 9
                GoTo allthru
            Else
                commerce = 0
                Line (447, 291)-(525, 335), 1, BF
                For k = 1 To 5: Circle (480, 315), 4 * k, 11: Next k
                TICK 9
                Line (447, 291)-(525, 335), 1, BF
            End If
        End If
    End If

    Call touchup: Call ships: Call iterate

    Call clrbot
    Line (390, 350)-(520, 400), 1, BF
    Color 13: Locate 24, 51: Print " "; UCase$(Left$(month$(month), 3)); ","; year
    victory(1) = .8 * victory(1) + .3 * (income(1) + control(1))
    If control(1) > 29 Then victory(1) = victory(1) + 50: If control(1) > 34 Then victory(1) = victory(1) + 100
    If side = 2 And control(1) < 11 Then aggress! = aggress! + .7

    If victory(1) < 1 Then victory(1) = 0
    victory(2) = .8 * victory(2) + .3 * (income(2) + control(2))
    If control(2) > 29 Then victory(2) = victory(2) + 50: If control(2) > 34 Then victory(2) = victory(1) + 100
    If side = 1 And control(2) < 11 Then aggress! = aggress! + .7
    If player = 2 Then Call clrbot: Color 14: Print "press a key";: Def Seg = 0: Poke 1265, 6: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop
End Sub

Sub armyxy (x, y, z)
    Line (x - 5, y - 3)-(x + 10, y + 8), 0, BF
    Select Case z
        Case 1
            Line (x - 7, y - 5)-(x + 7, y + 5), 4, BF
            Line (x - 7, y - 5)-(x - 1, y), 1, BF
            Line (x, y - 2)-(x + 7, y - 1), 7, B
            Line (x - 7, y + 2)-(x + 7, y + 3), 7, B
            Line (x - 8, y - 6)-(x + 8, y + 6), 0, B
        Case 2
            Line (x - 7, y - 5)-(x + 7, y + 5), 4, BF
            Line (x - 8, y - 6)-(x + 8, y + 6), 0, B
            Line (x - 7, y - 4)-(x + 6, y + 5), 9
            Line (x - 7, y + 4)-(x + 6, y - 5), 9
            Line (x - 7, y - 5)-(x + 7, y + 5), 9
            Line (x - 7, y + 5)-(x + 7, y - 5), 9
        Case Else
    End Select
End Sub

Sub bub2 (limit)
    Do
        swaps% = FALSE
        For i = 1 To limit - 1
            If brray(i) > brray(i + 1) Then
                Swap brray(i), brray(i + 1)
                swaps% = i
            End If
        Next i
    Loop While swaps%
End Sub

Sub bubble (limit)
    Do
        swaps% = FALSE
        For i = 1 To limit - 1
            If mtx$(i) > mtx$(i + 1) Then
                Swap mtx$(i), mtx$(i + 1)
                Swap array(i), array(i + 1)
                swaps% = i
            End If
        Next i
    Loop While swaps%
End Sub

Sub cancel (side)
    Call starfin(star, fin, (side))
    counter:
    Call clrrite
    size = 0
    For j = star To fin
        a$ = armyname$(j): If Len(a$) > 10 Then x = InStr(a$, " "): a$ = Right$(a$, Len(a$) - x)
        If armyloc(j) > 0 And armymove(j) > 0 Then size = size + 1: mtx$(size) = a$: array(size) = armyloc(j)
    Next j
    Call bubble(size)
    tlx = 67: tly = 2: wtype = 2
    mtx$(0) = "Cancel"
    If size < 1 Then Call clrbot: Color 11: Print "No units have orders to cancel";: Exit Sub
    Call menu(1)
    Select Case choose
        Case 1 To size
            target = array(choose)
	       
            For i = star To fin
                If armyloc(i) = target Then x = InStr(armyname$(i), mtx$(choose)): If x > 0 And armymove(i) > 0 Then index = i
            Next i

            Call clrbot: Color 11: Print armyname$(index); " has cancelled move to "; city$(armymove(index));: Call TICK(turbo + 1)
            Call icon(armyloc(index), armymove(index), 4)
            armymove(index) = 0: If noise > 0 Then Sound 2999, .5
            Call stax(side)
        Case Else
            GoTo finix
    End Select
    finix:
    Call clrrite
End Sub

Sub cannon
    Call clrrite
    Line (528, 80)-(639, 310), 3, BF
    Line (528, 310)-(639, 435), 2, BF
    Line (528, 405)-(639, 420), 0, BF
    PSet (580, 380), 0

    Draw "BR15"
    x = Point(0): y = Point(1): Circle (x, y), 25, 1, , , 1.4
    Circle (x + 5, y), 27, 1, 4, 1.5, 1.4
    Circle (x, y), 29, 1, , , 1.4
    Draw "BR20"
    x = Point(0): y = Point(1): Paint (x, y), 3, 1
    Draw "BR3BU7"
    x = Point(0): y = Point(1): Paint (x, y), 5, 1
    Draw "BL15C7G4L50H1L12H1L7H3U1H1U4E5R6E1R13E1R52F3D2R1E1R1F2D5G3H2G2BL2"
    Draw "BH2C6"
    x = Point(0): y = Point(1): Paint (x, y), 7
    Draw "BD2R4C0G4L50H1L12H1L7H3U1H1U4E5R6E1R13E1R52F3D2R1E1R1F2D5G3H2G2BR5U3"
    Draw "C8G2H2G6L15G1L32H1L1H1L19H3E2R2F1R6"
    Draw "E2R4F1R6E1R58F1G2BL9BH2BU5BR3C15H1L48G1L13G1L4"
    Draw "BD3BF3BR9"
    x = Point(0): y = Point(1): Paint (x, y), 8
    Draw "BR34BE1BR7BL5BD8C6R16F5D1F4D6F7R17D7L30H18L8H4L18H3U5R33BR17BD5"
    x = Point(0): y = Point(1): Paint (x, y), 6
    Draw "BU5C8F8D7F6R1F1R16D7E3U7G3E2BE2C8L16H6U8C8G3BE3C8H8G6BR5"
    x = Point(0): y = Point(1): Paint (x, y), 8
    Draw "BL38"
    x = Point(0): y = Point(1): Circle (x, y), 30, 4, , , 1.4
    Circle (x + 5, y), 35, 4, 4, 1.5, 1.4
    Circle (x, y), 35, 4, , , 1.4
    Circle (x, y), 8, 4, , , 1.4: Paint (x, y), 1, 4: Paint (x, y), 0, 4
    Draw "BR23"
    x = Point(0): y = Point(1): Paint (x, y), 1, 4: Paint (x, y), 4, 4
    Draw "BR5BU7"
    x = Point(0): y = Point(1): Paint (x, y), 1, 4: Paint (x, y), 0, 4
    Draw "BU22BL28"
    Draw "C4D20R1U19BD37C4D20L1U21BH2BL1BH1C4G12F1E11BU3BH2"
    Draw "BU3BL1C4L13U1R15BR1BU3C4H12E1F11BF5BE4C4E11F1G11BD3C4R13D1L10D3BL3BD1C4"
    Draw "F12G1H12BL33BH13BU3BH2BU1BE1BR2BU1BE1BR1"
    Draw "BL5BU5C8E4U1"
    Draw "H4E3H3U3E3H6L3G7F1H7E2U2E2H7L7G9D6F8G3L1H3L5G7D4F4R2D3F3R2G5D4L4G4D5F8D2"
    Draw "R5F9E3F5R6E6H4E2F6R3E8F1E4H5U2H3U1H1U7E4H7BL5"
    x = Point(0): y = Point(1): Paint (x, y), 8, 8
    Draw "BR9BD6C15H8L1H2L12G1F5L8G4F4R3G3D1G2D3F2E5G1D8R4E3D6F3E12H1U1H1U6E5BH6BU4"
    Draw "BD15"
    x = Point(0): y = Point(1): Paint (x, y), 14, 15
End Sub

Sub capitol
    Screen 12
    Cls
    Line (1, 1)-(639, 150), 4, BF
    Line (1, 150)-(639, 300), 15, BF
    Line (1, 300)-(639, 450), 1, BF
    x = 0
    Line (150, 190)-(500, 254), 7, B
    Line (270, 190)-(370, 254), 7, B
    Line (270, 185)-(370, 175), 7, B
    Line (280, 173)-(360, 168), 7, B
    Line (270, 235)-(370, 254), 7, B
    Circle (320, 150), 74, 7, .2, 2.95, 2.1
    Line (285, 138)-(355, 168), 7, B
    lwing:
    PSet (180 + 120 * x, 200), 0

    Draw "C7ER1E1R1E1R1E1R1E1R1E1R1E1R1E1R1E1R1E1R1E1R1F1R1F1R1F1"
    Draw "R1F1R1F1R1F1R1F1R1F1R1F1R1F1R1F1R1F1L47D4R48U3D51L4U17L41D17L3U47"
    For i = 1 To 7: Draw "BR6R2D25L2U25": Next i
    Draw "BD33R2"
    For i = 1 To 5: Draw "L40BR40D3": Next i
    x = x + 1: If x < 3 GoTo lwing
    PSet (270, 190), 0
    For i = 1 To 3: Draw "C7BR6R2D45L2U45": Next i
    PSet (344, 190), 0
    For i = 1 To 3: Draw "C7BR6R2D45L2U45": Next i
    PSet (283, 140), 0
    For i = 1 To 11: Draw "C7BR6R2D25L2U25": Next i
    PSet (283, 120), 0
    For i = 1 To 11: Draw "C7BR6R2D15L2U15": Next i
    Line (315, 55)-(325, 77), 7, B
    Line (318, 57)-(322, 75), 7, B
    PSet (315, 53), 0
    Draw "S3C7R13U6L13D5BU5C7U2E1U2E2U3H2U3H1E3U2H1U2E2U2E2F2D2F4D3R2E2F1G4L1D7F1D2F1D1G2"
    Line (140, 270)-(510, 430), 14, BF
    Color 15
    If noise = 2 Then Play "T130MFMSO2f16f8.f16f8.e-16d8.f16b-8.o3c16d8.d16d8.c16o2b-4"
    Locate 20, 20: Print Chr$(34); "... and that the government of the people,"
    If noise = 2 Then Play "o2b-8.a16g8.g16g8.a16b-8.a16b-8.g16f8.g16f8.d16f4"
    Locate 21, 20: Print "by the people, for the people,"
    If noise = 2 Then Play "f8.f16f8.f16f8.e-16d8.f16b-8.o3c16d8.d16d8.c16o2b-4b-4MNo3c4c4o2b-4a4b-2"
    Locate 22, 20: Print "shall not perish from the earth."; Chr$(34)
    Locate 25, 40: Print "- Abraham Lincoln"
    For i = -565 To 50 Step 50: Call flags(1, i, 0): Next i
    If noise = 2 Then Play "P2f4..e-16d8.f16b-8.o3c16d2o2b-2g4..a16b-8.a16b-8.g16f2d2": Play "f4..e-16d8.f16b-8.o3c16d2o2b-4b-4o3c4c4o2b-4a4b-2.."
End Sub

Sub capture (active, c, s, flag)
    cityp(c) = s: Call clrbot
    a$ = armyname$(active) + " has captured " + city$(c)
    Color 11: Print armyname$(active) + " has captured " + city$(c);
    If active < 21 And noise > 1 Then Play "MNMFL16o2T120dd.dd.co1b.o2do3g.ab.bb.ag"
    If active > 20 And noise > 1 Then Play "MNMFT160o2L16geL8ccL16cdefL8ggge"
    If c <> capcity(3 - s) Then Call flashcity(c)
    victory(s) = victory(s) + cityv(c)
    If c = capcity(3 - s) Then
        victory(s) = victory(s) + 100
        victory(3 - s) = victory(3 - s) - 100
        a$ = force$(3 - s) + " CAPITAL captured !"
        Call scribe(a$, 1)
        Call image2(city$(capcity(3 - s)) + " has fallen!", 4)
        capcity(3 - s) = 0
        Call flashcity(c)
    End If
    If fort(c) > 0 And flag > 0 Then fort(c) = fort(c) - 1: x = cityx(c): y = cityy(c): Line (x - 5, y - 5)-(x + 5, y + 5), 2, BF: Call showcity
End Sub

Sub center (y, a$)
    x = Len(a$)
    x = 26 - x \ 2
    Locate y, 7 + x: Print a$
End Sub

Sub chessie
    Color 10
    Line (500, 185)-(505, 180): Line -(505, 175)
    Line -(500, 170): Line -(490, 165)
    Line -(495, 160): Line -(490, 155)
    Line -(485, 150): Line -(495, 155)
    Line -(495, 145): Line -(490, 140)
    Line -(485, 130): Line -(470, 120)
    Line -(470, 110): Line -(475, 120)
    Line -(485, 120): Line -(485, 115)
    Line -(480, 100): Line -(485, 90)
    Line -(495, 80): Line -(500, 80)
    Line -(500, 85): Line -(495, 90)
    Line -(495, 100): Line -(495, 115)
    Line -(500, 120): Line -(500, 130)
    Line -(515, 135): Line -(515, 140)
    Line -(510, 160): Line -(520, 145)
    Line -(525, 120): Line -(525, 115)
    Line -(515, 85): Line -(527, 95)
    Paint (500, 400), 1, 10
    Paint (510, 110), 2
End Sub

Sub clrbot
    Locate 29, 1: Print Space$(79);
    Locate 29, 1
End Sub

Sub clrrite
    View (528, 1)-(639, 450)
    Cls 1
    View
End Sub

Sub commander (who, empty)
    empty = 0
    Call starfin(star, fin, who)
    For i = star To fin
        If armyloc(i) = 0 And lname$(i) <> "" Then empty = i: Exit Sub
    Next i

    '                 generic leaders if set list is exhausted
    For i = star To fin
        If armyloc(i) = 0 Then
            x = i
            Call roman(x, a$)
            empty = i
            lname$(i) = a$
            rating(i) = 1 + 8 * Rnd
            Exit Sub
        End If
    Next i
    Call clrbot: Print "No more "; force$(who); " commanders available";: Call TICK(turbo!)
End Sub

Sub endit
    a$ = Str$(Abs(vicflag(3)) * 2.5) + "% Cities"
    t$ = Str$(Abs(vicflag(3)) * 2.5) + "% Total Income"
    mtx$(8) = "Time: " + month$(vicflag(1)) + Str$(Abs(vicflag(2)))
    mtx$(9) = RTrim$(Str$(Abs(vicflag(6)))) + ":1 Force Ratio"
    Color 15: Locate 1, 1: Print "Press ENTER to toggle ending conditions";
    Color 14: Print "				ESC when done"
    what4:
    For i = 2 To 6: If vicflag(i) > 0 Then mtx$(i - 1) = "+ " Else mtx$(i - 1) = "  "
    Next i
    choose = chosit
    mtx$(0) = "Victory Conditions"
    mtx$(1) = mtx$(1) + mtx$(8)
    mtx$(2) = mtx$(2) + a$
    mtx$(3) = mtx$(3) + t$
    mtx$(4) = mtx$(4) + "Capital Captured"
    mtx$(5) = mtx$(5) + mtx$(9)
    mtx$(6) = "DONE"
    wtype = 2: colour = 3: size = 6: hilite = 14
    Call menu(0)
    chosit = 21 + choose
    Select Case choose
        Case 1
            vicflag(2) = -vicflag(2)
        Case 2 To 5
            vicflag(choose + 1) = -vicflag(choose + 1)
        Case Else
            GoTo cheer
    End Select
    GoTo what4
    cheer:
    Cls
    Call usa
End Sub

Sub engine
    If rr(1) + rr(2) = 0 GoTo notrain
    Line (5, 17)-(100, 63), 3, BF
    Line (5, 17)-(100, 47), 0, B
    Line (5, 47)-(100, 63), 0, B

    For i = 1 To 2
        If rr(i) = 0 GoTo notrane
        PSet (15, 25), 3: c = 9: If i = 2 Then c = 15: PSet (60, 25), 3
        Draw "C0S4R9D4R6U3R3D3R7U5H3U2R9D3G2D6F1D3F5L10D1G1L4H2L7G2L3H2L3U8L2U5BF4"
        x = Point(0): y = Point(1): Paint (x, y), c, 0
        Locate 4, 6 * (i - 1) + 2: Print Left$(city$(armymove(rr(i))), 5)
        notrane:
    Next i

    notrain:
End Sub

Sub evaluate (index, x)
    from = armyloc(index)
    x = 200: If aggress! > 1.5 Then x = 80: If aggress! > 2 Then x = 20: If aggress! > 3 Then x = 5
    Call void(from, defend): If defend = 0 Then x = 0
    If bold > 1 Then x = .5 * x

    best = armysize(index)

    For j = 1 To 6: array(j) = -1
        a = matrix(from, j): If a > 0 Then If occupied(a) > 0 Then best = best - armysize(occupied(a)): y = 1
    Next j: best = .01 * best
    If bold > 0 Then best = best + 20 * bold

    Call starfin(star, fin, 3 - side)

    For j = 1 To 6
        y = -1: a = matrix(from, j): If a = 0 Then max = j - 1: GoTo alleval
      
        c = occupied(a)
        For k = star To fin: If armyloc(k) > 0 Then If armymove(k) = a And c = 0 GoTo alle
        Next k
	
        y = best - x * fort(from) - cityv(from) + 30 * Rnd
        If fort(from) > 0 Then If index = occupied(from) Then y = y - 25: If realism > 0 Then y = y - 50
        If a = capcity(side) Then If vicflag(5) > 0 Then y = y + 200
        If cityp(a) <> 3 - side Then y = y + 5 * cityv(a) + 10 * fort(a)
        If cityp(a) <> side GoTo ourn
        If c = 0 Then y = y + 10 * cityv(a): GoTo ourn
        x1 = 1: If fort(a) = 1 Then x1 = 2: If fort(a) = 3 Then x1 = 3

        If armysize(c) > 0 Then
            odds! = armysize(index) / (x1 * armysize(c))
            If realism > 1 Then odds! = .8 * odds!
        Else
            odds! = 5
        End If

        If armysize(index) < 15 Then y = y - 300: odds! = .1
        If bold < 3 And armysize(index) < 40 And odds! < .8 Then y = 0
        If realism > 0 Then y = y - 15
        If odds! < .5 Then y = y - 200
        If bold = 0 And Rnd > .9 Then y = y + 10 * (armylead(index) - armylead(c) + armyexper(index) - armylead(index))
        If supply(index) < 1 Then y = y - 100
        Select Case odds! + .5 * bold
            Case Is < .3
                y = -9999
            Case Is < .5
                y = y - 300
            Case Is <= .8
                y = y - 20
            Case .8 To 1.2
                y = y - 5
            Case 1.2 To 1.5
                y = y + 10
            Case Else
                y = y + .5 * (armysize(index) - armysize(c)) + 50 * Rnd
        End Select
        ourn:
        If cityp(a) = 3 - side And c > 0 And armymove(c) = 0 Then y = y - armysize(c)
        If side = 1 And cityy(a) < cityy(from) Then y = y + 2
        If side = 2 And cityy(a) > cityy(from) Then y = y + 2
        '---------------------------------------------------------------------------
        For k = 1 To 6: m = matrix(a, k): If m = 0 GoTo tally3
            If cityp(m) = 0 Then y = y + 3 * cityv(m) + 4 * fort(m)
            If cityp(m) = side Then y = y + 3 * cityv(m) + 4 * fort(m)
            If cityp(m) = side And c > 0 Then
                odds! = 5 - 4 * Rnd
                If armysize(c) > 0 Then odds! = armysize(index) / armysize(c) Else odds! = 5
                If odds! < 1 Then y = y - 2
            End If
            If cityp(m) = capcity(side) Then y = y + 50
            tally3:
        Next k
        '---------------------------------------------------------------------------
        array(j) = y
        alle:
    Next j: max = 6
    alleval:
    x = 0: best = 0
    For j = 1 To max
        If array(j) < 0 GoTo allof
        If array(j) > x Then x = array(j): best = j
        allof:
    Next j
    If defend > armysize(index) And array(best) < 50 Then best = 0
    x = best
End Sub

Sub events
    '==========================================================================
    '                              Special Events
    '==========================================================================
    If realism > 0 And year = 1862 And month < 3 Then
        Color 14: Call clrbot
        Print "SPECIAL DEVELOPMENT : IRONCLAD ships now available";
        If noise > 0 Then For k = 1 To 5: Sound 140, 1: Sound 37, 1: Next k
        Def Seg = 0: Poke 1265, 7: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop
        Exit Sub
    End If
    If randbal = 0 Then Exit Sub
    plus = difficult: If side = 1 Then plus = 6 - difficult
    pct! = .005 * (year - 1860) * plus * plus: If pct! > .9 Then pct! = .9
    If Rnd > pct! Then Exit Sub

    Call clrbot: Color 14: Print "SPECIAL EVENT...";
    who = 1: If Rnd > .1 * randbal Then who = 2

    Select Case who
        '----------------------------------------------------------------------------
        '                               good for South
        '----------------------------------------------------------------------------
        Case 1
            If year = 1864 And month = 1 Then victory(2) = victory(2) + 50
            If year = 1865 And month = 1 Then victory(2) = victory(2) + 100
            If Rnd > .9 Then GoSub riot: Exit Sub

            Call clrbot
            If Rnd > .2 Or navysize(2) > 9 GoTo mercen
            If navysize(2) > 0 And navyloc(2) <> 99 Then empty = navyloc(2): GoTo float1
            empty = 0
            For i = 1 To 40
                If cityp(i) = 2 And matrix(i, 7) = 99 And navyloc(1) <> i Then empty = i: Exit For
            Next i
            If empty = 0 GoTo mercen
            float1:
            Call scribe("England has given ships to the South", 2)
            navysize(2) = navysize(2) + 2 * plus
            If navysize(2) > 10 Then navysize(2) = 10
            x = navysize(2) - Len(fleet$(2))
            fleet$(2) = fleet$(2) + String$(x, "W")
            navyloc(2) = empty: Call ships
            GoTo dull1
            mercen:
            If Rnd > .1 Or control(2) < 30 GoTo money
            a$ = "French": If Rnd > .5 Then a$ = "British"
            Call scribe(a$ + " mercenaries join " + armyname$(index) + "'s army", 2)
            armysize(index) = armysize(index) + 100 * plus
            armyexper(index) = 10: supply(index) = 10
            GoTo dull1
            money:
            If Rnd > .3 Or control(2) < 12 GoTo cotton
            Call scribe("The South has obtained a loan from Europe", 2)
            GoTo purse
            cotton:
            If Rnd > .5 Or control(2) < 12 GoTo uprising
            Call scribe("Cash from cotton sales fill the Rebel Treasury", 2)
            purse:
            cash(2) = cash(2) + 100 * plus
            GoTo dull1
            uprising:
            pct! = .9
            a$ = "Union troops diverted to fight Western Indian uprisings"
            If Rnd > .5 Then a$ = "Union 90-day enlistees return home"
            If year = 1864 And month > 5 Then a$ = "20% of Union forces take furloughs to vote in 1864 elections": pct! = .8
            Call scribe(a$, 2)
            For k = 1 To 20
                armysize(k) = pct! * armysize(k)
            Next k
            GoTo dull1
            '----------------------------------------------------------------------------
            riot:
            If realism = 0 Then Return
            If control(who) = 1 Then Exit Sub
            For k = 1 To 99
                x = 1 + Int(40 * Rnd)
                If cityo(x) <> who And cityp(x) = who And occupied(x) = 0 Then
                    Call clrbot: Color 15
                    Print " Riots have broken out in "; city$(x);
                    cityp(x) = 0
                    Call showcity
                    TICK turbo!
                    Call clrbot
                    Call image2(city$(x) + " is now NEUTRAL !", 4)
                    TICK turbo!
                    Return
                End If
            Next k
            Return
            '----------------------------------------------------------------------------
            '                               good for Union
            '----------------------------------------------------------------------------
        Case 2
            If Rnd > .1 Or navyloc(1) = 0 Or navysize(1) > 9 GoTo event2
            If Rnd > .95 Then GoSub riot: Exit Sub
            Call scribe("Union shipworks have produced extra ships", 2)
            navysize(1) = navysize(1) + plus
            If navysize(1) > 10 Then navysize(1) = 10
            x = navysize(1) - Len(fleet$(1))
            fleet$(1) = fleet$(1) + String$(x, "W")
            GoTo dull2
            event2:
            If Rnd < .7 GoTo abe
            Call scribe("Volunteer troops swell the Union ranks", 2)
            x = 0: For i = 1 To 20: x = x + 1: If x > 5 GoTo dull2
                If armysize(i) > 0 And Rnd > .5 Then armysize(i) = armysize(i) * 1.1 + plus
            Next i: GoTo dull2
            abe:
            If emancipate = 0 And year > 1862 Then

                emancipate = 1
                Call scribe("Abraham Lincoln announces the Emancipation Proclamation", 2)
                victory(1) = victory(1) + 100: victory(2) = victory(2) - 100
                Call usa
                GoTo dull2
            End If
            If year = 1864 And month = 11 Then
                Call scribe("Lincoln has been reelected", 2)
                victory(2) = .5 * victory(2)
                GoTo dull2
            End If
            If Rnd > .5 Then
                Call scribe("Wealthy Unionists give generously to the Federal Cause", 2)
                cash(1) = cash(1) + 100 * plus
                GoTo dull2
            End If
            If Rnd > .5 And year > 1861 Then
                Call scribe("Rebel deserters leave the battlefield to go home", 2)
                For i = 21 To 40: armysize(i) = .92 * armysize(i): Next i: GoTo dull2
            End If
            If Rnd > .5 And year > 1861 Then
                Call scribe("Union soldiers now have repeating rifles", 2)
                For i = 1 To 20
                    If armyexper(i) < 9 Then armyexper(i) = armyexper(i) + 2
                Next i
            End If
            Call scribe("Secretary of War Stanton predicts the end of the Rebellion", 2)
            victory(1) = victory(1) + 10: GoTo dull2
        Case Else
    End Select

    dull2:
    If noise = 2 Then Play "MNMFL16o2T120dd.dd.co1b.o2do3g.ab.bb.ag"
    Def Seg = 0: Poke 1265, 8: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop: Exit Sub
    dull1:
    If noise = 2 Then Play "MNMFT160o2L16geL8ccL16cdefL8ggge"
    Def Seg = 0: Poke 1265, 9: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop
End Sub

Sub flags (who, w, a)
    x = 585 + w: y = 200: If w = 0 Then y = 180
    If a <> 0 Then y = a
    Select Case who
        Case 1
            Line (x - 17, y - 15)-(x + 17, y + 7), 4, BF
            For i = -13 To 9 Step 5
                Line (x - 17, y + i)-(x + 17, y + i - 1), 7, B
            Next i
            Line (x - 17, y - 15)-(x, y), 1, BF
            If w = 0 Then Color 9: Locate 10, 70: Print "U N I O N"
            For i = -16 To -1 Step 3
                For j = -14 To -1 Step 4: PSet (x + i, y + j), 7: Next j
            Next i
        Case 2
            Line (x - 17, y - 15)-(x + 17, y + 7), 4, BF
            Line (x - 17, y - 13)-(x + 15, y + 7), 7
            Line (x - 15, y - 15)-(x + 17, y + 5), 7

            Line (x - 17, y + 7)-(x + 17, y - 15), 1
            Line (x - 17, y + 6)-(x + 16, y - 15), 1
            Line (x - 16, y + 7)-(x + 17, y - 14), 1
            Line (x - 17, y + 5)-(x + 15, y - 15), 7
            Line (x - 15, y + 7)-(x + 17, y - 13), 7

            Line (x - 17, y - 15)-(x + 17, y + 7), 1
            Line (x - 17, y - 14)-(x + 16, y + 7), 1
            Line (x - 16, y - 15)-(x + 17, y + 6), 1
            Line (x - 17, y - 15)-(x + 17, y + 7), 4, B

            If w = 0 Then Color 4: Locate 10, 70: Print "R E B E L"
        Case Else
    End Select
End Sub

Sub image2 (a$, s)
    Dim g2(1 To 8000)
    mtx$(1) = a$
    tlx = 32 - Len(a$) \ 2
    tly = 10: size = 1
    Call mxw(wide)
    If wide > 59 Then wide = 59: a$ = Left$(a$, 59)
    x = 8 * (tlx + wide)
    y = 16 * (tly + 1)
    Get (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 15, 16 * (tly + 3) + 7), g2()
    View (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3) + 4)
    Cls 1
    View
    Line (8 * tlx, 16 * tly - 1)-(8 * (tlx + wide + 1) + 15, 16 * (tly + 3) + 7), 0, BF
    Line (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3)), s, BF
    Line (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3)), 15, B
    Color 14
    Locate tly + 2, tlx + 2: Print a$
    Call TICK(9)
    Put (8 * tlx - 4, 16 * tly - 3), g2(), PSet
End Sub

Sub integrity
    Cls: Color 15
    x = 0: y = 0
    For i = 1 To 40
        For j = 1 To 6: If matrix(i, j) = 0 GoTo done
            index = matrix(i, j)
            For k = 1 To 6
                If matrix(index, k) = i GoTo ret
                If matrix(index, k) = 0 Then matrix(index, k) = i: Print "+ Adding return route from "; city$(index); " to "; city$(i): y = y + 1: GoTo ret
            Next k
            x = x + 1: Print "Error in CITIES.GRD entry for city #"; i; city$(index); ": no return route to "; city$(i): TICK 1
            ret:
        Next j
        done:
    Next i
    If x + y = 0 Then Print "ALL MAP LINKS ARE OK": Exit Sub
    If y > 0 Then Print "* "; y; " fixes made to provide RETURN ROUTES"
    If x > 0 Then Print "** "; x; " UNRESOLVED RETURN ROUTES"
End Sub

Sub maptext
    For k = 1 To 40
        For j = 1 To Len(city$(k))
            a = cityx(k) + 6 * (j - 4) - 3
            x = Asc(Mid$(UCase$(city$(k)), j, 1)) - 64
            If a > 527 GoTo nextc
            PSet (a, cityy(k) + 12), 10
	
            If matrix(k, 7) < 90 Then
                If bw = 0 Then Draw "S4C0" Else Draw "S4C7"
            Else
                Draw "C10"
            End If
            If x > 0 And x < 27 Then Draw font$(x)
        Next j
        nextc:
    Next k
End Sub

Sub maxx
    Color 14: Locate 28, 1: Print "press a key";: Def Seg = 0: Poke 1265, 10: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop
    Screen 0: Color 14, 5: Cls: Color 11, 0

    Locate 2, 7: Print String$(57, "|")
    Locate 9, 7: Print String$(57, "|")
    For i = 1 To 6: Locate 2 + i, 7: Print "|"; Space$(55); "|": Next i

    Open "I", 1, "hiscore.cws"
    For s = 1 To 2
        Color 14, 4: Locate 3, 30 * (s - 1) + 10: Print force$(s); " HALL of FAME"
        Color 15, 0: For i = 1 To 5
            Input #1, city$(5 * (s - 1) + i), matrix(s, i)
            Locate 3 + i, 30 * (s - 1) + 8: Print i, city$(5 * (s - 1) + i), matrix(s, i)
    Next i: Next s: Close #1
		      
    For s = 1 To 2: For i = 1 To 5
            If victory(s) < matrix(s, i) GoTo oldskor
            For j = 5 To i + 1 Step -1
                matrix(s, j) = matrix(s, j - 1)
                city$(5 * (s - 1) + j) = city$(5 * (s - 1) + j - 1)
            Next j

            Color 15, 1: If s = 2 Then Color 4, 7
            For k = 12 To 14: Locate k, 1: Print Space$(80);: Next k
            Locate 12, 1: Print "Congratulations ! Score of"; victory(s)
            Print "New Entry into "; force$(s); " HALL of FAME in place "; i
            If player = 1 And s <> side Then a$ = "COMPUTER": Print a$; " was "; force$(s); " commander": GoTo automate
            who4:
            Print "What is name of "; force$(s); " commander";: Def Seg = 0: Poke 1265, 11: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Input a$: If a$ = "" GoTo who4
            automate:
            city$(5 * (s - 1) + i) = a$
            matrix(s, i) = victory(s): GoTo foun
            oldskor:
        Next i
        foun:
    Next s
	       
    Open "O", 1, "hiscore.cws"
    For s = 1 To 2
        For i = 1 To 5
            Write #1, city$(5 * (s - 1) + i), matrix(s, i)
        Next i
    Next s: Close #1: Print "Game Over": Def Seg = 0: Poke 1265, 12: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop: pcode = 1
End Sub

Sub occupy (x)
    occupied(x) = 0
    For i = 1 To 40
        If armyloc(i) = x Then occupied(x) = i: GoTo holdup
    Next i
    holdup:
End Sub

Sub relieve (who)
    colour = 3: target = 0: hilite = 3
    tlx = 67: tly = 2
    Call starfin(star, fin, who)
    size = 0
    For i = star To fin
        If armysize(i) > 0 And armyloc(i) > 0 And armymove(i) = 0 Then
            size = size + 1
            mtx$(size) = armyname$(i)
            If Len(mtx$(size)) > 11 Then mtx$(size) = Left$(mtx$(size), 11)
            array(size) = i
        End If
    Next i

    If size = 0 Then Exit Sub
    Call bubble(size)
    mtx$(0) = "Relieve"
    choose = 31: hilite = 11
    Call menu(6): Call clrrite

    If choose < 1 Then Exit Sub
    index = array(choose)
    Call icon(armyloc(index), 0, 9)

    t$ = armyname$(index)
    lname$(index) = t$

    size = 0
    For i = star To fin
        If lname$(i) <> "" Then
            size = size + 1
            mtx$(size) = lname$(i)
            If Len(mtx$(size)) > 11 Then mtx$(size) = Left$(mtx$(size), 11)
            array(size) = i
        End If
    Next i
    Call bubble(size)

    have2:
    mtx$(0) = "New Leader"
    tlx = 67: tly = 2
    Call menu(0): Call clrrite
    If choose < 1 GoTo have2
    armymove(index) = -1
    armylead(index) = rating(array(choose))
    If armylead(index) > 0 Then armylead(index) = armylead(index) - 1
    If armyexper(index) > 0 Then armyexper(index) = armyexper(index) - 1
    Color 15: Call clrbot: Print lname$(array(choose)); " has replaced "; t$; " in "; city$(armyloc(index));: TICK 9
    armyname$(index) = lname$(array(choose))
    lname$(array(choose)) = ""
    Call icon(armyloc(index), 0, 8)
    Call clrbot
End Sub

Sub report (who)
    Dim summ(40)
    If who = -1 GoTo frep
    If who > 100 GoTo batrep
    mtx$(0) = "Information"
    mtx$(1) = force$(side) + " Armies"
    mtx$(2) = force$(3 - side) + " Armies"
    mtx$(3) = "Cities"
    mtx$(4) = "Force Summary"
    mtx$(5) = "Intelligence"
    mtx$(6) = "Battles": size = 6
    If _FileExists("cws.his") Then mtx$(7) = "Recap": size = 7
    Call menu(0)
    Select Case choose
        Case Is < 0
            Cls
            Call usa
            Exit Sub
        Case 2: who = 3 - side
        Case 3: GoTo cityrep
        Case 4: GoTo frep
        Case 5: GoTo srep
        Case 6: GoTo batrep
        Case 7: GoTo recap
        Case Else
    End Select
    '..........................................................................
    '                               Army Report
    '..........................................................................
    If choose < 0 GoTo endrep
    If choose = 4 GoTo frep
    Call starfin(star, fin, who)
    Cls: Locate 1, 1: c = 9: If who = 2 Then c = 15
    Color c
    Print " Report for "; force$(who); " Forces"; Tab(30); month$(month); ","; year; Tab(49);: Color 14: Print " Victory Points "; victory(who);
    If c = 15 Then c = 7
    x = victory(who) + victory(3 - who)
    If x = 0 Then
        Print
    Else
        y = (victory(who) / x) * 100
        Print "("; y; " %)";
    End If
    Call armyxy(215, 8, who)
    Color c: Print control(who); "/ 40"; Tab(15); "CITIES CONTROLLED"; Tab(40);: Print "(";: x = Int(control(who) * 2.5 + .5): Print x; "%)";
    Print Tab(50);: Color 12: Print "Capital: "; city$(capcity(who))
    y = vptotal: If capcity(1) > 0 Then y = y + 100
    If capcity(2) > 0 Then y = y + 100
    Print income(who); "/"; y; Tab(15); "INCOME"; Tab(40); "(";: x = (income(who) / vptotal) * 100: Print x; "%)   ";: Color 14: Print "Cash:"; cash(who)
    Color 11: Print " #      Name             Size  Location   Ldr Exp Suply Move To"
    x = 0: For i = star To fin
        Color c
        If armyloc(i) < 1 GoTo deadeye
        x = x + armysize(i)
        Print i; Tab(5); armyname$(i); Tab(25);: y = CsrLin
        Call strong(i): Print Tab(32); city$(armyloc(i));
        If who <> side Then Print: GoTo deadeye
        Print Tab(43); armylead(i); Tab(47); armyexper(i); Tab(52); supply(i);
        Print Tab(57);: If armymove(i) > -1 Then Print city$(armymove(i)) Else Print "Resting"
        If fort(armyloc(i)) > 0 Then Locate y, 68: Print "FORT +"; fort(armyloc(i))
        deadeye:
    Next i
    Print " "; String$(70, "-")
    Print " Total Forces "; Tab(25); LTrim$(Str$(x)); "00"; Tab(40); "[ Enemy Forces "; LTrim$(Str$(Int(aggress! * x))); "00 ]"
    Print " "; String$(70, "-")
    Print " "; force$(who); " NAVY has "; navysize(who); " ship(s) ";
    x = 0
    For k = 1 To navysize(who)
        If Mid$(fleet$(who), k, 1) = "I" Then x = x + 1
    Next k
    If x > 0 Then Print "("; x; "Ironclads) ";
    If navyloc(who) > 0 And navyloc(who) < 99 Then Print "in "; city$(navyloc(who)) Else Print
    If rr(who) > 0 Then Color 14: Print " Army #"; rr(who); armyname$(rr(who)); " on train to "; city$(armymove(rr(who)));
    Line (0, 0)-(639, 449), 15, B
    Line (0, 62)-(639, 62), 15
    GoTo endrep
    '..........................................................................
    '                               City Report
    '..........................................................................
    cityrep:
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
    GoTo endrep
    '..........................................................................
    '                               Intelligence
    '..........................................................................
    srep:
    c = 9: If side = 2 Then c = 7
    Call usa
    Call starfin(star, fin, side)
    For k = star To fin
        If armysize(k) > 0 Then
            Color c
            x = cityx(armyloc(k))
            y = cityy(armyloc(k))
            z = 10: If armysize(k) < 1000 Then z = 9
            x = x / 8 - 2: y = y / 16
            If y > 26 Then y = 26
            For j = 0 To 3
                Locate y + j, x: Print Space$(z);
            Next j
            Locate y, x: Print "Siz:";: Call strong(k)
            Locate y + 1, x: Print "Ldr:"; armylead(k)
            Locate y + 2, x: Print "Xpr:"; armyexper(k)
            Locate y + 3, x: Print "Sup:"; supply(k);
            Line (8 * (x - 1) - 1, 16 * (y - 1) - 1)-(8 * (x + z - 1) + 1, 16 * (y + 3) + 1), 15, B
            TICK 1
        End If
    Next k
    GoTo endrep
    '..........................................................................
    '                               Force Summary
    '..........................................................................
    frep:
    For k = 1 To 40
        If occupied(k) > 0 Then
            For j = 1 To 40
                If armyloc(j) = k Then summ(k) = summ(k) + armysize(j)
            Next j
        End If
    Next k

    Cls: Call usa
    For k = 1 To 40
        If summ(k) > 0 Then
            c = 9: If cityp(k) = 2 Then c = 7
            Color c
            Locate Int(cityy(k) / 16 + 1), cityx(k) / 8: Print summ(k)
        End If
    Next k
    Color 14: Locate 1, 20: Print "Total Forces in Cities (100's)"
    GoTo endrep
    '..........................................................................
    '                               Battle Report
    '..........................................................................
    batrep:
    View (200, 123)-(400, 284)
    Cls 1
    View
    Line (200, 123)-(400, 284), 15, B
    Color 14
    Locate 9, 32: Print "BATTLE SUMMARY"
    Call armyxy(320, 160, 1)
    Call armyxy(370, 160, 2)
    Line (200, 176)-(400, 230), 15, B
    Line (200, 230)-(400, 284), 15, B
    Line (290, 176)-(345, 284), 15, B
    Locate 13, 27: Print "BATTLES"
    Locate 14, 27: Print "WON"
    Locate 16, 27: Print "MEN LOST"
    Locate 17, 27: Print "(1000's)"
    Locate 14, 38: Print batwon(1)
    Locate 14, 45: Print batwon(2)
    Locate 17, 38: Print Int(.1 * casualty&(1))
    Locate 17, 45: Print Int(.1 * casualty&(2))
    If history > 0 And who > 2 Then
        Open "O", #1, "battsumm"
        Print #1, " SIDE      BATTLES WON       LOSSES"
        For k = 1 To 2
            a$ = " ": If thrill = k Then a$ = "*"
            Print #1, a$; force$(k), batwon(k), 100 * casualty&(k)
        Next k
        Close #1
    End If
    GoTo endrep
    '..........................................................................
    '                               Recap (History)
    '..........................................................................
    recap:
    Cls: x = 0
    Open "I", 2, "cws.his"
    Do While Not EOF(2)
        Input #2, a$
        Color 7: If InStr(a$, "[") Then Color 15
        If InStr(a$, ">") Then Color 14
        If InStr(a$, "..") Then Color 11
        If InStr(a$, "!") Then Color 12
        Locate 29, 1: Print a$
        x = x + 1
        If x > 27 Then TICK .4
    Loop
    Close #2

    endrep:
    Color 14: Locate 29, 29: Print "press a key";
    Def Seg = 0: Poke 1265, 13: Poke 1266, 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do While InKey$ = "": Loop
    If who < 3 Then
        Cls
        Call usa
    End If
End Sub

Sub resupply (index)
    who = 1: If index > 20 Then who = 2
    If realism > 0 Then
        Call cutoff(who, armyloc(index), a)
        If a < 1 Then Exit Sub
    End If
    If armysize(index) > 0 Then x = cash(who) / armysize(index) * 10
    y = x + supply(index): If y > 5 Then x = 5 - supply(index)
    If x < 1 Then Call clrbot: Print "Not enough funds in the Treasury to supply "; armyname$(index);: GoTo nocash
    supply(index) = supply(index) + x: cash(who) = cash(who) - .1 * x * armysize(index)
    Call clrbot: Print armyname$(index); " has received supplies ";: If noise > 0 Then Sound 4500, .5
    nocash:
    Call TICK(turbo!): If cash(who) < 0 Then cash(who) = 0
End Sub

Sub retreat (defend, x)
    x = 0
    If player = 1 Then
        If (side = 1 And defend > 20) Or (side = 2 And defend < 21) Then Exit Sub
    End If

    hilite = 13: colour = 3: tlx = 67: tly = 5: size = 0
    y = armyloc(defend)
    who = 1: If defend > 20 Then who = 2
    For k = 1 To 6
        If matrix(y, k) > 0 And cityp(matrix(y, k)) = who Then size = size + 1: mtx$(size) = city$(matrix(y, k)): array(size) = matrix(y, k)
    Next k
    If size = 0 Then Exit Sub
    If size = 1 Then x = array(1): Exit Sub
    mtx$(0) = "Retreat"
    Call bubble(size)
    Call menu(9)
    Call clrrite
    If choose < 0 Or choose > size Then x = 0: Exit Sub
    x = array(choose)
End Sub

Sub roman (target, a$)
    a$ = "Union ": If target > 20 Then a$ = "Rebel "
    If target > 20 Then target = target - 20
    If target > 10 Then a$ = a$ + "X": target = target - 10
    Select Case target
        Case Is < 4
            x = target: GoSub add1s
        Case 4
            a$ = a$ + "IV"
        Case 5 To 8
            a$ = a$ + "V"
            x = target - 5
            GoSub add1s
        Case 9
            a$ = a$ + "IX"
        Case 10
            a$ = a$ + "X"
    End Select
    Exit Sub

    add1s:
    If x > 0 Then For k = 1 To x: a$ = a$ + "I": Next k
    Return

End Sub

Sub rwin
    Line (2, 2)-(637, 239), 4, BF
    Color 15: Line (2, 40)-(597, 239): Line -(637, 239): Line -(637, 199): Line -(40, 2): Line -(2, 2): Line -(2, 40)
    Color 15: Line (2, 199)-(2, 239): Line -(40, 239): Line -(637, 40): Line -(637, 2): Line -(597, 2): Line -(2, 199)
    Line (242, 95)-(395, 145), 4, BF
    Paint (4, 20), 1, 15

    Line (2, 239)-(637, 438), 2, BF
    GoSub stars

    '                               landscape
    PSet (1, 240)
    Draw "s14BR68C0E6U1E3R2E4R10F2R7F2R5E3R12F7R4F2E3R5E3R9F4"
    Draw "C0R6F2R5F1R3F3L44F2L42E1H1L29E2R1BH5BL3BR5"
    Paint (300, 230), 5, 0
    PSet (2, 330)
    Draw "C0D18U1R32E4R26E2R27E5R20E2R1E2U1E2U2E4H4L5H2L9H1L5H3L4H2L5H1L3H2L12H4"
    Draw "C0D1F4R5F2R3F2R4F5R5F3L13G1L8G2L24G1L30G1L18D21"
    Draw "BE5": x = Point(0): y = Point(1) + 5: Paint (x, y), 9, 0
    Draw "BU12C11R21F1R2BR2BD6C11R9E1R9E1R6BH7C11R9E1R9BF5C11R9E1R1E1R10"
    Line (1, 1)-(638, 440), 14, B
    Line (2, 2)-(637, 439), 14, B

    '                               mansion
    x = 100: y = 240
    Circle (x + 50, y + 40), 80, 0, , , .2
    Paint (x + 50, y + 40), 6, 0
    Line (x + 95, y - 14)-(x + 102, y + 8), 8, BF
    Line (x + 100, y - 14)-(x + 102, y + 8), 7, BF
    Line (x, y)-(x + 100, y + 36), 7, BF
    For i = 1 To 6: Line (x + 17 * i, y + 6)-(x + 17 * i + 4, y + 32), 0, BF: Next i
    Line (x + 12, y + 18)-(x + 98, y + 22), 7, BF
    Line (x + 50, y + 20)-(x + 57, y + 36), 8, BF
    Line (x + 100, y + 36)-(x + 105, y + 39), 7
    Line -(x + 5, y + 39), 7: Line -(x, y + 38), 7

    Line (x, y)-(x - 7, y - 7), 8: Line -(x - 14, y + 5), 8
    Line -(x - 14, y + 33), 8: Line -(x, y + 36), 8: Line -(x, y), 8
    Color 10: Line (x - 5, y - 7)-(x + 95, y - 7): Line -(x + 107, y + 7)
    Line -(x + 7, y + 7): Line -(x - 5, y - 7)
    Paint (x, y - 3), 10
    Paint (x - 5, y + 15), 8

    For i = 1 To 6: Line (x + 19 * i - 12, y + 7)-(x + 19 * i - 12 + 2, y + 40), 15, BF: Next i

    If noise < 2 GoTo relee
    Play "MBMS T120"
    Play "O3 L16 g e L8 c c L16 c d e f L8 g g g e a a a. L16 g a8. g a b"
    Play "O4 L16 c d e4. c O3 g O4 c4. O3 g e g4. d e c4 P8"
    Play "L16 g e L8 c c L16 c d e f L8 g g g e a a a. L16 g a8. g a b"
    Play "O4 L16 c d e4. c O3 g O4 c4. O3 g e g4. d e c4."
    Play "L16 T150 g a b T120 O4 L8 c e d. c16 O3 a O4 c4 O3 a O4 d4."
    Play "O3 a O4 d4. O3 T150 L16 g a b T120 L8 O4 c e d. c16"
    Play "L8 O3 a b O4 c. O3 a16 g e O4 c. O3 e16 e d4 e c4. e d4. a"
    Play "L8 g e O4 c. e16 d c4 O3 e c4. e d4. a g e O4 e4. c16 d c4"

    GoTo relee

    stars:
    For i = 1 To 8
        x = starx(i): y = stary(i)
        PSet (x, y), 0
        Line (x + 2, y - 1)-(x + 8, y + 16), 15
        Line -(x - 6, y + 2): Line -(x - 20, y + 16)
        Line -(x - 14, y - 1): Line -(x - 30, y - 9)
        Line -(x - 12, y - 9): Line -(x - 6, y - 25)
        Line -(x, y - 9): Line -(x + 16, y - 9)
        Line -(x + 2, y - 1): Paint (x, y), 15
    Next i
    Return
    relee:
End Sub

Sub scribe (a$, flag)
    Select Case flag
        Case 1: Call clrbot: Print a$;
        Case 2: Call image2(a$, 4)
    End Select
    If history > 0 Then
        Open "A", 2, "cws.his"
        a$ = LTrim$(RTrim$(a$))
        Print #2, a$
        Close #2
    End If
End Sub

Sub shen
    '                       Shenandoah
    If noise = 2 Then
        If InKey$ <> "" Then choose = 1: Exit Sub
        Play "T90MFMNo1c4f8f8f4."
        If InKey$ <> "" Then choose = 1: Exit Sub
        Play "g8a8b-8o2d8c4."
        If InKey$ <> "" Then choose = 1: Exit Sub
        Play "MLf8e8MNd4.c8d8c8o1a8o2c4.c4d8d8d4."
        If InKey$ <> "" Then choose = 1: Exit Sub
        Play "o1a8o2c8o1a8g8f4.g4a4.f8"
        If InKey$ <> "" Then choose = 1: Exit Sub
        Play "a8o2d8c4.o1f8g8a4.f8g4f2."
    End If
End Sub

Sub shipicon (who, flag)
    x = Point(0) - 10: y = Point(1) - 10
    Circle (x, y), 18, 9, , , .4
    Paint (x + 3, y + 4), 9, 9
    Circle (x, y), 18, 10, , , .4
    Select Case flag
        Case 1 'wooden ship
            Draw "BF5R5D1C4L20C0H1R22E1L9": Draw "BL6": Draw "C0L10H1L1H2": Draw "BR9D2"
            Draw "C0U9": Draw "BR13": Draw "C0D9BU7R3"
            Draw "C0L7BL5C0L7BD3BL2BD1BL1BD1C0R11BR3": Draw "C0R10BL12"
            PSet (x + 8, y - 1)
            GoSub jack

        Case 2 'ironclad
            Draw "BL15BD4E1"
            Draw "C0R30H1L2C8L24E1R1"
            Draw "C8E1R18F1L19BR4C6C5C4C3C2C1": Draw "C0R2BR3C0R2BR3C0R2"
            Draw "BU3BL10D1": Draw "C0U4R1D4BR9"
            PSet (x + 8, y - 1)
            GoSub jack
        Case Else
    End Select
    Exit Sub
    jack:
    Select Case who
        Case 1
            Draw "C4R7BU1C7L6BU1C1R3C4R3BU1C7L2BL1C1L3"
        Case 2
            Draw "C4R4U1L4U1R4U1L4C3F4BU4C3G4BD2BR2BU1"
    End Select
    Return
End Sub

Sub ships
    For s = 1 To 2
        If navysize(s) = 0 Or navyloc(s) = 0 Then navyloc(s) = 0: GoTo sink
        If navyloc(s) = 30 Then x = 515: y = 268: GoTo float
        If navyloc(s) = 28 Then x = 517: y = 172: GoTo float
        If navyloc(s) = 17 Then x = 380: y = 425: GoTo float
        If navyloc(s) = 99 Then x = 495: y = 310: GoTo float
        x = cityx(navyloc(s)) + 25: y = cityy(navyloc(s)) + 25
        If navyloc(s) = 24 Then y = y + 5: x = x - 5
        float:
        PSet (x, y), 1
        a = 1: If Left$(fleet$(s), 1) = "I" Then a = 2
        Call shipicon(s, a)
        sink:
    Next s
End Sub

Sub starfin (star, fin, who)
    star = 1: fin = 20: If who = 2 Then star = 21: fin = 40
End Sub

Sub stax (who)
    Call starfin(star, fin, who)
    For i = star To fin
        If armysize(i) > 0 And occupied(armyloc(i)) <> i Then
            target = armyloc(i)
            x = cityx(target) - 12: y = cityy(target) - 12
            Circle (x, y), 3, 14
        End If
    Next i
End Sub

Sub surrender (who)
    Screen 12
    s = 1: c = 1: Color 0: w = 514
    If who > 20 Then c = 7: s = 2
    '               background
    Line (w + 15, 440)-(w + 125, 16), 2, BF
    Line (w + 15, 16)-(w + 125, 290), 3, BF
    Line (w + 14, 16)-(w + 125, 440), c, B
    x = 77: y = 280
    Circle (w + x + 10, y + 30), 30, 4, , , .1
    Paint (w + x + 10, y + 30), 6, 4
    Line (w + x, y + 28)-(w + x + 6, y + 28): Line -(w + x + 6, y + 8)
    Line -(w + x + 8, y + 8): Line -(w + x + 8, y + 28)
    Line -(w + x + 12, y + 28): Line -(w + x + 12, y + 12)
    Line -(w + x + 15, y + 12): Line -(w + x + 15, y + 30)
    Line -(w + x + 18, y + 30): Line -(w + x + 18, y + 20)
    Line -(w + x + 21, y + 21): Line -(w + x + 21, y + 32)
    Line -(w + x + 25, y + 32): Line -(w + x + 25, y + 17)
    Line -(w + x + 27, y + 17): Line -(w + x + 25, y - 10)
    Line -(w + x + 19, y - 14): Line -(w + x + 15, y - 5)
    Line -(w + x + 12, y - 12): Line -(w + x + 8, y - 15)
    Line -(w + x + 6, y - 15): Line -(w + x + 6, y - 11)
    Line -(w + x + 3, y - 14): Line -(w + x + 2, y - 14)
    Line -(w + x + 1, y - 10): Line -(w + x - 3, y - 11)
    Line -(w + x - 3, y + 15): Line -(w + x, y + 15)
    Line -(w + x, y - 3): Line -(w + x + 2, y - 3)
    Line -(w + x, y + 28)
    Paint (w + x + 10, y), 8 - c, 0

    Circle (w + x + 7, y - 10), 3, 0: Paint (w + x + 7, y - 10), 0, 0
    Circle (w + x + 20, y - 8), 3, 0: Paint (w + x + 20, y - 8), 0, 0

    Line (w + x + 5, y - 30)-(w + x + 6, y - 9)
    Line (w + x + 20, y)-(w + x + 35, y + 30)
    Line (w + x + 20, y + 1)-(w + x + 35, y + 31)
    Line (w + x + 22, y - 31)-(w + x + 21, y - 8)
    Line (w + x - 1, y - 28)-(w + x, y - 8)
    '                       boot
    Circle (w + 37, 425), 22, 8, , , .3
    Paint (w + 37, 430), 8, 8
    Line (w + 26, 426)-(w + 46, 426)
    Line -(w + 46, 423): Line -(w + 36, 419)
    Line -(w + 36, 397): Line -(w + 26, 397)
    Line -(w + 26, 426)
    Paint (w + 27, 399), 0, 0
    '                       arm
    Line (w + 26, 397)-(w + 25, 396): Line -(w + 24, 374)
    Line -(w + 25, 346): Line (w + 25, 346)-(w + 22, 341)
    Line -(w + 46, 341): Line -(w + 46, 349)
    Line -(w + 41, 365): Line -(w + 41, 380)
    Line -(w + 36, 398)
    Paint (w + 30, 350), c, 0
    '                       saber
    Line (w + 55, 288)-(w + 58, 360), 14, BF
    Line (w + 55, 288)-(w + 55, 360), 14
    Line (w + 56, 361)-(w + 55, 360), 14
    Line (w + 57, 361)-(w + 58, 350), 14
    Line (w + 58, 350)-(w + 58, 288), 14
    Line (w + 55, 272)-(w + 62, 288), 6, B
    Line (w + 56, 273)-(w + 61, 287), 6, B
    Line (w + 57, 274)-(w + 60, 286), 6, B
		  
    Line (w + 54, 295)-(w + 60, 305), 12, BF
    Line (w + 53, 294)-(w + 61, 306), 0, B
    Line (w + 57, 303)-(w + 61, 303)
    Line (w + 57, 298)-(w + 61, 298)
    Line (w + 34, 310)-(w + 46, 312), c, BF
    Line (w + 25, 380)-(w + 35, 378)
    Line (w + 25, 365)-(w + 34, 366)
    '                       shoulder
    Color 0
    Line (w + 53, 297)-(w + 31, 305): Line -(w + 25, 270)
    Line -(w + 18, 270): Line -(w + 16, 275)
    Line -(w + 21, 309): Line -(w + 28, 316)
    Line -(w + 52, 306): Line -(w + 53, 298)
    Paint (w + 20, 280), c, 0
    Line (w + 30, 310)-(w + 32, 305): Line -(w + 25, 307)
    Line (w + 46, 298)-(w + 47, 275)
    Line (w + 47, 275)-(w + 41, 269)
    Line (w + 41, 269)-(w + 25, 269)
    Paint (w + 45, 275), c, 0
    Circle (w + 40, 275), 1, 14
    Circle (w + 40, 285), 1, 14
    Circle (w + 40, 295), 1, 14

    Line (w + 21, 306)-(w + 18, 340)
    Line (w + 18, 340)-(w + 47, 340)
    Line (w + 46, 310)-(w + 48, 339)
    Paint (w + 30, 330), c, 0
    Line (w + 46, 312)-(w + 21, 316), 0, BF
    Line (w + 25, 312)-(w + 35, 313), c, BF

    Line (w + 30, 268)-(w + 41, 264), 12, BF
    Line (w + 30, 263)-(w + 41, 263), 0
    Line (w + 30, 264)-(w + 26, 260): Line -(w + 25, 248)
    Line -(w + 28, 243): Line -(w + 46, 243)
    Line -(w + 48, 245): Line -(w + 48, 248)
    Line -(w + 51, 254): Line -(w + 51, 255)
    Line -(w + 48, 256): Line -(w + 48, 261)
    Line -(w + 46, 263): Line -(w + 39, 263)
    Paint (w + 40, 250), 12, 0

    Line (w + 16, 242)-(w + 57, 241), 0, BF
    Line (w + 45, 240)-(w + 28, 240)
    Line -(w + 31, 232): Line -(w + 45, 232)
    Line -(w + 48, 240)
    Paint (w + 40, 235), c, 0

    Line (w + 80, 303)-(w + 81, 200), 6, BF

    Line (w + 48, 251)-(w + 44, 250), 0, B
    Line (w + 29, 263)-(w + 43, 266)
    Line -(w + 46, 266): Line -(w + 48, 263)
    Line -(w + 48, 260): Line -(w + 37, 258)
    Line -(w + 36, 255): Line -(w + 37, 250)
    Line -(w + 39, 246): Line -(w + 40, 243)
    c = 0: If s = 2 Then c = 15: Line (547, 263)-(558, 263), 15
    Paint (w + 33, 250), c, 0
    Paint (w + 42, 264), c, 0

    Call flags(3 - s, 26, 0)
    If who < 99 Then Call TICK(turbo!)
End Sub

Sub TICK (sec!)
    ' start! = TIMER
    ' DO WHILE TIMER - start! < sec! AND INKEY$ = "": LOOP
End Sub

Sub topbar
    Locate 1, 1: Print Space$(80);
    Color 11
    Locate 1, 10: Print "Input your decisions now for "; force$(side); " side ";: Color 14: Print month$(month); ","; year; "  "

    Call flags((side), 0, 0)
    Color 4: If bw > 0 Then Color 7
    Locate 6, 68: Print "Difficulty "; LTrim$(RTrim$(Str$(difficult)))
    Locate 7, 68: Print "Funds:"; cash(side);

    For i = 1 To 2: If victory(i) < 0 Then victory(i) = 0
    Next i
    x = victory(1) + victory(2): y = 0
    c = 9: If side = 2 Then c = 7
    Line (580, 15)-(580, 35), 15
    Line (530, 20)-(630, 30), 8 - c, BF
    If x > 0 Then y = 100 * (victory(side) / x)

    Line (530, 20)-(530 + y, 30), c, BF
    Color c: Locate 4, 68: Print "VP :"; victory(side)

    Locate 5, 68: Print "("; y; "%)"

    a$ = "  Snd": If noise < 2 Then a$ = "   Snd": If noise = 0 Then a$ = "      "
    If graf > 0 Then a$ = a$ + " G" + LTrim$(Str$(graf))
    a$ = a$ + Str$(player)
    Color c
    Locate 26, 68: Print "F3 Redrw Scrn"
    Locate 27, 68: Print "F7 End Turn"
    Locate 28, 68: Print a$
End Sub

Sub touchup

    Line (500, 170)-(490, 165), 10 'Norfolk Coast
    Line -(495, 160), 10
    Line -(490, 155), 10
    Line -(485, 150), 10

    ' PAINT (502, 160), 1, 10

    Line (145, 375)-(145, 405), 10 'Mobile border

    Line (291, 111)-(301, 101), 1
    Line -(316, 96), 1
    Line -(331, 76), 1
    Line -(345, 51), 1
    Line -(351, 30), 1
    Line -(370, 35), 1

    Line (381, 81)-(431, 66), 1 'Potomac River
    Line -(456, 81), 1
    Line -(471, 111), 1

    Line (115, 165)-(105, 170), 1 'Paducah
    Line -(105, 190), 1
    Line (106, 170)-(91, 140), 1 'Missouri R

    Line (105, 190)-(150, 190), 10
    Line (120, 160)-(130, 200), 1 'Tenn River

    Line (60, 295)-(65, 325), 1 'Vicksburg
    Line -(65, 335), 1


    Line (195, 125)-(170, 130), 1
    Line -(165, 135), 1

    Line (50, 375)-(60, 395), 1

    Line (60, 395)-(105, 405), 1
    Line -(110, 420), 1

    Line (291, 265)-(350, 340), 1 'Savannah R.

    Line (1, 80)-(15, 77), 1 'Missouri River
    Line -(25, 95), 1
    Line -(50, 98), 1
    Line -(75, 96), 1

    Line (71, 90)-(61, 95), 1
    Line -(56, 85), 1

    Line (110, 398)-(105, 398), 10
    Line -(90, 396), 10
    Line -(90, 400), 10
    Line -(105, 402), 10
    Line -(120, 415), 10
    Line -(115, 420), 10
    ' PAINT (108, 400), 1, 10

    Line (66, 375)-(52, 375), 10
End Sub

Sub traincapacity (who, limit)
    If realism = 0 Then limit = train(who): Exit Sub
    x = 11: If side = 2 Then x = 23
    limit = train(who) + 5 * (control(who) - x)
    x = 2 * train(who)
    If limit > x Then limit = x
    If limit < x \ 4 Then limit = x \ 4
End Sub

Sub usa
    Screen 12
    Line (1, 16)-(527, 440), 10, B
    Paint (200, 200), 2, 10
    Color 10
    '==========================  Mountains =================================
    If _FileExists("mtn.vga") Then
        Put (380, 30), mtn(), PSet
        Put (270, 200), mtn(), PSet
        Put (310, 160), mtn(), PSet
        Put (350, 120), mtn(), PSet
        Put (200, 185), mtn(), PSet
        Put (250, 130), mtn(), PSet
        Put (320, 80), mtn(), PSet
        Put (30, 150), mtn(), PSet
        Line (30, 150)-(70, 190), 2, BF
    End If
    '===========================  Kentucky =================================
    Line (105, 190)-(150, 190)
    Line -(150, 185)
    Line -(290, 185)
    Line (276, 185)-(301, 175)
    Line -(305, 160)
    Line -(310, 155)
    Line -(305, 145)
    Line -(300, 125)
    Line -(290, 110)

    Line -(275, 95), 1
    Line -(270, 95), 1
    Line -(260, 100), 1
    Line -(250, 100), 1
    Line -(240, 90), 1
    Line -(235, 85), 1
    Line -(230, 85), 1
    Line -(220, 90), 1
    Line -(220, 100), 1
    Line -(210, 105), 1
    Line -(205, 115), 1
    Line -(195, 125), 1
    Line -(170, 130), 1
    Line -(165, 135), 1
    Line -(130, 140), 1
    Line -(120, 150), 1
    Line -(120, 160), 1
    Line -(115, 165), 1
    Line -(105, 170), 1
    Line -(105, 190), 1
    '===========================  Tennessee =================================
    Line (290, 185)-(320, 185)
    Line -(320, 185)
    Line -(315, 195)
    Line -(302, 210)
    Line -(290, 215)
    Line -(275, 230)
    Line -(260, 241)
    Line -(260, 241)
    Line -(70, 241)

    Line (105, 190)-(95, 200), 1
    Line -(80, 220), 1
    Line -(70, 241), 1
    '===========================  Mississippi =================================
    Line (143, 241)-(145, 405)
    Line -(135, 400)
    Line -(125, 400)
    Line -(115, 405)
    Line -(110, 400)
    Line -(110, 390)
    Line -(115, 380)
    Line -(115, 375)
    Line -(50, 375)
    Line (115, 375)-(60, 375)

    Line (70, 241)-(65, 280), 1
    Line -(60, 295), 1
    Line -(65, 325), 1
    Line -(65, 335), 1
    Line -(50, 375), 1
    '===========================  Alabama =================================
    Line (215, 241)-(232, 375)
    Line (176, 395)-(181, 410)
    Line -(166, 415)
    Line -(161, 400)
    Line -(156, 405)
    Line -(146, 405)
    Line (176, 395)-(176, 385)
    Line -(231, 385)
    '===========================  Georgia & Florida ============================
    Line (261, 241)-(296, 241)
    Line -(291, 265)
    Line -(350, 340)
    Line -(336, 390)
    Line -(366, 440)
    Line (336, 385)-(325, 385) 'Fla /Ga Border
    Line -(320, 388)
    Line -(245, 388)
    Line -(230, 385)
    Line (180, 410)-(195, 410)
    Line -(215, 415)
    Line -(225, 425)
    Line -(255, 420)
    Line -(265, 420)
    Line -(270, 425)
    Line -(275, 440)
    Line (347, 409)-(343, 409)
    Line -(343, 431)
    Line -(346, 431)
    Line -(346, 412)
    Line -(349, 412)
    Line (347, 411)-(350, 411), 1

    Line (353, 333)-(358, 336), 10, B
    '===========================  South Carolina ================================
    Line (290, 241)-(305, 240)
    Line -(345, 240)
    Line -(350, 250)
    Line -(380, 250)
    Line -(415, 280)
    Line -(385, 315)
    Line -(380, 320)
    Line -(375, 325)
    Line -(350, 330)
    Line -(350, 340)
    '===========================  North Carolina ================================
    Line (320, 185)-(500, 185)
    Line -(505, 190)
    Line -(490, 195)
    Line -(490, 205)
    Line -(505, 205)
    Line -(500, 215)
    Line -(485, 220)
    Line -(490, 225)
    Line -(500, 225)
    Line -(500, 230)
    Line -(490, 235)
    Line -(488, 240)
    Line -(480, 241)
    Line -(460, 250)
    Line -(455, 255)
    Line -(440, 265)
    Line -(439, 270)
    Line -(425, 278)
    Line -(415, 280)
    Line (510, 190)-(513, 200), , B
    Line (510, 206)-(510, 226)
    Line -(500, 236)
    Line -(502, 241)
    Line -(512, 228)
    Line -(512, 208)
    Line -(510, 206)

    '=========================== Chesapeake ====================================
    Call chessie
    '=============== Ohio, Pennslyvania, Maryland, Virginia =====================
    Line (291, 111)-(301, 101), 1
    Line -(316, 96), 1
    Line -(331, 76), 1
    Line -(345, 51), 1
    Line -(351, 30), 1
    Line -(370, 35), 1

    Line (351, 16)-(351, 54)
    Line -(527, 54)
    Line (381, 54)-(381, 81)
    Line -(431, 66)
    Line -(456, 81)
    Line -(471, 111)
    Line (226, 85)-(226, 16)
    '=============== Louisiana, Arkansas, Missouri =============================
    Line (50, 375)-(60, 395), 1
    Line -(105, 405), 1
    Line -(110, 420), 1
    Line -(125, 438), 1
    Line (110, 398)-(105, 398)

    Line -(90, 396)
    Line -(90, 400)


    Line -(105, 402)
    Line -(120, 415)
    Line -(115, 420)
    Line -(125, 435)
    Line -(120, 440)
    Line -(110, 425)
    Line -(100, 435)
    Line -(90, 440)
    Line -(50, 430)
    Line -(25, 435)
    Line -(1, 435)
    '----------------------------------
    Paint (250, 430), 1, 10
    Paint (110, 439), 1, 10
    Paint (50, 439), 1, 10
    '----------------------------------
    Line (1, 300)-(61, 300) 'Ark
    Line (91, 205)-(71, 205)
    Line -(76, 190)
    Line -(71, 185)
    Line -(1, 185)

    Line (106, 170)-(91, 140), 1 'Missouri
    Line -(71, 120), 1
    Line -(76, 110), 1
    Line -(76, 95), 1
    Line -(71, 90), 1
    Line -(61, 95), 1
    Line -(56, 85), 1
    Line -(50, 35), 1
    Line -(46, 20), 1

    Line -(38, 16)

    Line (46, 20)-(61, 17), 1 'Illinois

    Line (150, 17)-(150, 110)
    Line -(147, 125), 1
    Line -(140, 138), 1
    '========================== Rivers =========================================

    Line (1, 80)-(15, 77), 1 'Missouri River
    Line -(25, 95), 1
    Line -(50, 98), 1
    Line -(75, 96), 1

    Line (120, 160)-(130, 200), 1 'Tenn River
    Line -(135, 240), 1
    Line -(130, 247), 1
    Line -(160, 250), 1
    Line -(200, 260), 1
    Line -(240, 244), 1
    Line -(245, 240), 1
    Line -(270, 200), 1

    Line (120, 160)-(140, 200), 1 'Cumberland River
    Line -(160, 210), 1
    Line -(200, 208), 1
    Line -(240, 170), 1

    Line (161, 400)-(155, 320), 1 'Tombigbee River
    Line -(145, 300), 1

    Line (161, 400)-(170, 370), 1 'Alabama River
    Line -(200, 350), 1

    Line (230, 425)-(231, 385), 1 'Chattahoochee River
    Line -(233, 350), 1
    Line -(240, 330), 1
    Line -(270, 290), 1

    Line (381, 81)-(431, 66), 1 'Potomac River
    Line -(456, 81), 1
    Line -(471, 111), 1

    Line (485, 150)-(455, 145), 1 'James River
    Line -(400, 150), 1

    Line (489, 200)-(415, 185), 1 'Roanoke River
    Line -(400, 160), 1

    Line (296, 241)-(291, 265), 1 'Savannah River
    Line -(350, 340), 1

    Line (438, 271)-(430, 235), 1 'Cape Fear River
    Line -(420, 205), 1

    Line (500, 80)-(498, 50), 1 'Susquehanna River
    Line -(470, 30), 1

    Line (405, 290)-(360, 240), 1 'Pee Dee River

    Line (400, 300)-(350, 280), 1 'Santee River

    Line (270, 423)-(280, 410), 1 'Suwanee River
    Line -(290, 390), 1

    Line (342, 370)-(300, 350), 1 'Altamaha River

    Line (50, 370)-(1, 330), 1 'Red River

    Line (65, 280)-(1, 240), 1 'Arkansas River

    Line (430, 66)-(400, 100), 1 ' Shenandoah River
    Line -(380, 120), 1

    Call showcity
    PSet (493, 280), 1
    If navyloc(1) = 30 Or navyloc(2) = 30 Then
        Draw "C11U7R4D3L3BR6BU3D7BU4R3U3D7BR3U7BD4BR4BU4D7R4"
        Line (485, 241)-(525, 270), 11, B
    End If

    If graf > 1 Then Call maptext
    If commerce > 0 Then
        Line (447, 291)-(525, 317), 4, BF: Line (447, 291)-(525, 317), 10, B
        y = 312
        a$ = "COMMERCE"
        For j = 1 To Len(a$): x = Asc(Mid$(UCase$(a$), j, 1)) - 64
            PSet (440 + 10 * j, y), 0: Draw font$(x)
        Next j
    Else
        Line (447, 291)-(525, 335), 1, BF
    End If

    For i = 1 To 40
        If armyloc(i) > 0 Then Call placearmy(i)
    Next i

    For k = 1 To 2
        Call stax(k)
    Next k

    If player = 2 GoTo nosee
    For i = 1 To 40: If armyloc(i) > 0 And armymove(i) > 0 Then Call icon(armyloc(i), armymove(i), 1)
    Next i
    nosee:
    Call ships
    Call engine
    Color 13: Locate 24, 51: Print " "; UCase$(Left$(month$(month), 3)); ","; year
End Sub

Sub victor
    x = 0: For j = 1 To 20: If armyloc(j) > 0 Then x = x + armysize(j)
    Next j
    y = 0: For j = 21 To 40: If armyloc(j) > 0 Then y = y + armysize(j)
    Next j

    Call clrbot: Color 14
    If vicflag(2) > 0 And year >= vicflag(2) And month < vicflag(1) Then
        a$ = "Time almost expired (" + month$(vicflag(1)) + "," + Str$(vicflag(2)) + ")"
        Call image2(a$, 4)
    End If
    For i = 1 To 2
        If (i = 1 And y = 0) Or (i = 2 And x = 0) Then j = 7: victory(i) = victory(i) + 300: GoTo finis

        If year >= vicflag(2) And month >= vicflag(1) And vicflag(2) > 0 Then j = 2: GoTo finis
        If control(i) >= vicflag(3) And vicflag(3) > 0 Then j = 3: GoTo finis
        If income(i) / (income(1) + income(2)) >= .01 * vicflag(4) And vicflag(4) > 0 Then j = 4: GoTo finis
        If vicflag(5) > 0 And capcity(3 - i) = 0 And capcity(i) > 0 Then j = 5: GoTo finis

        If vicflag(6) > 0 Then
            If i = 1 Then
                If y = 0 Then j = 7: GoTo finis
                If x / y > vicflag(6) Then j = 6: GoTo finis
            End If
            If i = 2 Then
                If x = 0 Then j = 7: GoTo finis
                If y / x > vicflag(6) Then j = 6: GoTo finis
            End If
        End If

        Call clrbot: Color 14
        If vicflag(3) > 0 And control(i) >= .9 * vicflag(3) Then
            a$ = force$(i) + " side almost controls" + Str$(vicflag(3)) + " cities"
            Call image2(a$, 4)
        End If

        If vicflag(4) > 0 And income(i) / (income(1) + income(2)) >= .009 * vicflag(4) Then
            a$ = force$(i) + " side close to" + Str$(vicflag(4)) + " % of total income"
            Call image2(a$, 4)
        End If

        If vicflag(6) > 0 And x > 0 And y > 0 Then
            If (i = 1 And x / y > .9 * vicflag(6)) Or (i = 2 And y / x > .9 * vicflag(6)) Then
                a$ = force$(i) + "side close to" + Str$(vicflag(6)) + ":1 strength ratio"
                Call image2(a$, 4)
            End If
        End If
        GoTo stale

        finis:
        Select Case j
            Case Is < 3
                a$ = "TIME EXPIRED"
            Case 3
                a$ = Str$(2.5 * vicflag(3)) + "% CITIES CONTROLLED"
            Case 4
                a$ = Str$(vicflag(4)) + " % OF TOTAL INCOME"
            Case 5
                a$ = "CAPITAL CAPTURED"
            Case 6
                a$ = Str$(vicflag(6)) + ":1 ARMY STRENGTH RATIO"
            Case 7
                a$ = "ENEMY ANNIHILATED"
        End Select
        Cls
        c = 1: If i = 2 Then c = 7
        Line (0, 0)-(639, 479), 4, BF
        Line (0, 40)-(550, 460), 0, BF
        Call usa
        Line (70, 140)-(485, 265), 0, BF
        Line (50, 120)-(465, 250), c, BF
        Line (50, 120)-(465, 250), 4, B
        Color 14
        t$ = force$(i) + " SIDE IS WINNING"
        If j = 2 Then Call center(10, "Confederates will win a technical victory") Else Call center(10, t$)

        Color 15
        t$ = "END GAME VICTORY CONDITION" + Str$(j - 1) + " REACHED"
        Call center(12, t$)
        Call center(14, a$)

        mtx$(0) = "End Game"
        mtx$(1) = "Yes"
        mtx$(2) = "No-Override"
        size = 2: colour = 4
        tlx = 27: tly = 18
        hilite = 15
        If j = 7 Then size = 1
        Call menu(0)

        If choose <> 1 And j < 7 Then
            play4ever:
            vicflag(j) = vicflag(j) + 1
            If j = 5 Then vicflag(j) = 0
            Cls
            Call usa
            Exit Sub
        End If
        mtx$(0) = "Options"
        mtx$(1) = "Quit this Game"
        mtx$(2) = "Play More"
        If player = 1 Then mtx$(2) = "No - Press Onward to " + city$(capcity(3 - side))
        size = 2: colour = 8
        tlx = 27: tly = 18
        If j <> 7 Then
            Call menu(0)
            If choose <> 1 GoTo play4ever
        Else
            victory(i) = victory(i) + 100
        End If

        thrill = i
        Call usa: Call report(100 + side)
        If j = 2 Then For k = 1 To 2: victory(k) = .7 * victory(k): Next k: Call rwin: GoTo death

        If i = 1 Then Call capitol: Color 15: Locate 2, 27: t$ = "UNION VICTORY  VP's=" + Str$(victory(1)): Print t$: GoTo death
        If i = 2 Then Call rwin: Color 15: Locate 2, 27: t$ = "REBEL VICTORY  VP's=" + Str$(victory(2)): Print t$
        death:
        Color 14: Locate 4, 40 - .5 * Len(a$)
        If history > 0 Then
            Call scribe(t$, 0)
            Call scribe(a$, 0)
        End If
        Print a$: Call maxx
        Exit Sub
        stale:
    Next i
End Sub

Sub void (a, y)
    y = 0
    For j = 1 To 6
        x = matrix(a, j): If x = 0 GoTo tally5
        If cityp(x) = side And occupied(x) > 0 Then y = y + armysize(occupied(x))
        For k = 1 To 6: m = matrix(x, k): If m = 0 Or m = a GoTo tally4
            If cityp(m) = side And occupied(m) > 0 Then y = y + .1 * armysize(occupied(m))
            tally4:
        Next k
    Next j
    tally5:
End Sub


'============================================================================
'                GENmenu 3.05 : Generic Menu Main Module Level  12/91
'                Updated to Use Graphic Screens 8, 9, 12
'============================================================================
'       tlx  = top left x  (if zero, automatically centered)
'       tly = top left y   (if zero, automatically centered)
'       size = # rows (current maximum=20)
'       mtx$() = text for menu
'                mtx$(0) = heading (blank defaults to M E N U)
'                mtx$(1-size) are options
'       choose = option selected (row #)
'            -1 indicates ESC key pressed
'            entering with choose+21 causes choose to be default entry
'       wtype = window type (0 = no line; 1=single line;
'                            2=double line
'               if >10 then 10's digit=box color  1's digit= window type

Sub choices (b1, wide)
    boxc = colour: If wtype > 10 Then boxc = Int(wtype / 10): wtype = wtype - 10 * boxc
    Color boxc
    View (8 * tlx - 6, 16 * tly - 11)-(8 * (tlx + wide + 1) + 7, 16 * (tly + size + 2) + 8)
    Cls 1
    View
    Line (8 * tlx - 6, 16 * tly - 11)-(8 * (tlx + wide + 1) + 7, 16 * (tly + size + 2) + 8), colour, B
    Line (8 * tlx - 2, 16 * (tly + 1) + 3)-(8 * (tlx + wide + 1) + 3, 16 * (tly + 1) + 6), colour, B
    If Abs(wtype) = 2 Then
        Line (8 * tlx - 2, 16 * tly - 8)-(8 * (tlx + wide + 1) + 3, 16 * (tly + size + 2) + 4), colour, B
    End If
    Color colour

    Locate tly + 1, tlx + b1: Print mtx$(0)
    For i = 1 To size
        Locate tly + 2 + i, tlx + 2: Print mtx$(i);
    Next i
End Sub

Sub ironclad
    Screen 12
    Line (1, 240)-(639, 309), 3, BF
    Line (639, 309)-(1, 479), 1, BF

    Color 7
    Line (478, 368)-(174, 389): Line -(86, 363): Line -(78, 353)
    Line -(161, 337): Line -(162, 337): Line -(406, 326): Line -(478, 368)
    Paint (300, 350), 7

    Color 15
    Line (427, 358)-(203, 373): Line -(247, 322): Line -(205, 312)
    Line -(167, 342): Line -(203, 373): Line -(246, 323): Line -(401, 318)
    Line -(427, 358): Line -(400, 318): Line -(363, 306): Line -(204, 312)
    Paint (300, 315), 7, 15
    Paint (200, 325), 8, 15
    Paint (300, 325), 8, 15

    Color 7
    Line (77, 353)-(76, 363): Line -(84, 370): Line -(85, 363)
    Line -(84, 369): Line -(169, 396): Line -(170, 389): Line -(168, 396)
    Line -(477, 375): Line -(478, 369)
    Paint (80, 363), 7
    Paint (150, 385), 8, 7
    Paint (190, 390), 0, 7

    Color 0
    Line (299, 261)-(294, 254): Line -(303, 250): Line -(299, 243)
    Line -(316, 243): Line -(321, 247): Line -(326, 241): Line -(335, 240)
    Line -(345, 240): Line -(357, 246): Line -(366, 241): Line -(381, 245)
    Line -(384, 253): Line -(398, 248): Line -(410, 248): Line -(413, 259)
    Line -(424, 264): Line -(425, 254): Line -(444, 258): Line -(445, 270)
    Line -(436, 277): Line -(426, 281): Line -(418, 270): Line -(411, 272)
    Line -(400, 263): Line -(392, 275): Line -(374, 270): Line -(370, 262)
    Line -(357, 257): Line -(345, 256): Line -(335, 257): Line -(335, 262)
    Line -(324, 257): Line -(317, 258): Line -(309, 257): Line -(305, 262)
    Line -(300, 261)
    Paint (320, 250), 0

    Line (211, 341)-(196, 357): Line -(179, 346)
    Line -(194, 335): Line -(211, 341)
    Paint (190, 350), 0
    Line (211, 341)-(225, 330), 7: Line -(209, 324), 7: Line -(188, 340), 0

    For k = 0 To 3
        x = 260 + 40 * k: y = 344 - 2 * k
        Line (x, y)-(x + 2, y + 20): Line -(x + 17, y + 19): Line -(x + 15, y)
        Line -(x, y)
        Paint (x + 10, 350), 0
        Line (x, y)-(x + 15, y), 7: Line -(x + 13, y - 14), 7: Line -(x - 2, y - 14), 7: Line -(x, y)
    Next k

    Color 8
    Line (168, 340)-(85, 355): Line -(91, 361): Line -(173, 386): Line -(466, 366)
    Line -(414, 334)

    Line (358, 307)-(382, 315): Line -(249, 320): Line -(220, 314)
    Line -(358, 307)

    Line (314, 255)-(330, 314), 8, BF
    Line (297, 310)-(309, 262), 8, BF
    Line (323, 257)-(329, 314), 0, BF
    Line (303, 310)-(308, 262), 0, BF
    Line (319, 257)-(320, 315), 15, BF
    Line (300, 262)-(301, 312), 15, BF

    Line (96, 297)-(94, 357), 8, BF
    Line (98, 299)-(106, 317), 4, BF
    Line (107, 304)-(110, 319), 9, BF
    Line (111, 300)-(114, 312), 15, BF

    y = 387: For x = 180 To 470 Step 15: y = y - 1: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
    y = 360: For x = 90 To 170 Step 15: y = y + 4: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
    y = 373: For x = 216 To 250 Step 7: y = y - 8: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
    y = 373: For x = 205 To 243 Step 7: y = y - 8: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
    y = 350: For x = 172 To 200 Step 9: y = y - 8: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
    y = 310: For x = 397 To 425 Step 8: y = y + 11: Line (x, y)-(x + 1, y + 1), 0, BF: Next x
    Color 11
    Line (70, 361)-(68, 369): Line -(106, 384): Line -(166, 394)
    Line -(210, 385): Line -(255, 391): Line -(219, 394)
    Line -(242, 402): Line -(202, 405): Line -(162, 402)
    Line -(121, 387)
    Paint (190, 400), 3, 11

    Line (331, 391)-(365, 387)
    Line -(403, 375): Line -(446, 382): Line -(471, 384)
    Line -(446, 391): Line -(478, 395): Line -(449, 407)
    Line -(445, 406): Line -(414, 395): Line -(412, 395)
    Line -(369, 396): Line -(323, 391)
    Paint (390, 390), 3, 11

    Line (476, 379)-(531, 372)
    Line (420, 329)-(497, 327), 3
    Line -(464, 340): Line -(535, 343), 3
    Line (141, 337)-(67, 349): Line -(58, 372)

    Line (525, 405)-(587, 392): Line -(628, 402): Line -(588, 415)
    Line -(563, 409): Line -(562, 409): Line -(527, 423)
    Line -(540, 410): Line -(523, 406)
    Paint (560, 405), 9, 11

    Line (106, 320)-(101, 318), 0, BF
    Line (110, 313)-(114, 315), 0, BF
End Sub

Sub menu (switch%)
    remenu:
    If colour = 0 Then colour = 7
    Locate 1, 1, 0
    If mtx$(0) = "" Then mtx$(0) = "M E N U"

    If wide = 0 Then Call mxw(wide)
    If tlx = 0 Then GoSub noadjust
    If choose < 21 Then choose = 1
    If choose > 21 Then choose = choose - 21
    If choose > 21 Then choose = 1
    row = choose: If row < 1 Then row = 1
    If row > size Then row = 1
    choose = row
    row1 = row
       
    If tly = 0 Then tly = Int(11.5 - .5 * size)
    If tly + size > 26 Then tly = 26 - size
       
    brx = tlx + wide + 1
    bry = tly + size + 3
       
    boxc = colour: If wtype > 10 Then boxc = Int(wtype / 10): wtype = wtype - 10 * boxc

    Color boxc

    View (8 * tlx - 6, 16 * tly - 11)-(8 * (tlx + wide + 1) + 7, 16 * (tly + size + 2) + 8)
    Cls 1
    View
    Line (8 * tlx - 6, 16 * tly - 11)-(8 * (tlx + wide + 1) + 7, 16 * (tly + size + 2) + 8), colour, B
    Line (8 * tlx - 2, 16 * (tly + 1) + 3)-(8 * (tlx + wide + 1) + 3, 16 * (tly + 1) + 6), colour, B
    If Abs(wtype) = 2 Then
        Line (8 * tlx - 2, 16 * tly - 8)-(8 * (tlx + wide + 1) + 3, 16 * (tly + size + 2) + 4), colour, B
    End If

    Color colour
    b1 = Int(.5 * (wide - Len(mtx$(0))) + .5) + 1

    Call choices(b1, wide)
    sel1:
    Select Case switch
        Case 1: Call icon(array(row), 0, 7)
        Case 2: Call icon(array(row), 0, 9)
            target = occupied(array(row))
            If target > 0 Then
                flag = 1
                t$ = armyname$(target)
                If Len(t$) > wide Then t$ = Left$(t$, wide)
                Color 12
                Locate tly + 2 + row, tlx + 2: Print Space$(wide);
                Locate tly + 2 + row, tlx + 2: Print t$;
                Locate tly + 4 + size, tlx + 1: Print Space$(12);
                Locate tly + 4 + size, tlx + 1: Print "Size"; armysize(target);
                x = Pos(0): y = CsrLin: Locate y, x - 1: Print "00";
            Else
                flag = 0
                Locate tly + 4 + size, tlx + 1: Print Space$(12);
            End If
        Case 4: Call armystat(array(row))
        Case 5: Color 11: Call clrbot: Print armyname$(index); "  Exp="; armyexper(index); " Cash="; cash(side);
        Case 6: Call icon(armyloc(array(row)), 0, 9)
        Case 8
            If graf > 2 And row > 0 Then
                Line (548, 148)-(592, 216), 15, B
                a = row: If side = 1 Then a = 6 - row
                t$ = "face" + LTrim$(Str$(a)) + ".vga"
                If _FileExists(t$) Then
                    Def Seg = VarSeg(graft(1))
                    BLoad t$, VarPtr(graft(1))
                    Def Seg
                    Put (550, 150), graft(), PSet
                    If side = 2 Then
                        Paint (560, 160), 8, 0
                        Paint (570, 155), 7, 0
                    End If
                End If
            End If
        Case 9: Call icon(array(row), 0, 9)
    End Select
    If flag = 0 Then
        Color hilite
        Locate tly + 2 + row, tlx + 2
        Print mtx$(row)
        If bw > 0 Then Line (8 * (tlx + 1), 16 * (tly + row + 1))-(8 * (tlx + Len(mtx$(row)) + 1) - 1, 16 * (tly + row + 2) - 1), hilite, B
    End If
    GoSub crsr
    If switch <> 3 GoTo reglr
    If a$ = "H" Then choose = 1: GoTo called
    If Asc(a$) = 13 Then choose = 2: GoTo called
    If a$ = "P" Then choose = 3: GoTo called
    reglr:
    If Asc(a$) = 13 GoTo called
    If switch = 2 Then Locate tly + 2 + row1, tlx + 2: Print Space$(wide);
    Color colour
    Locate tly + 2 + row1, tlx + 2: Print mtx$(row1)

    Select Case switch
        Case 1, 2, 9
            If mtx$(row1) <> "EXIT" Then Call icon(array(row1), 0, 8)
        Case 6
            If mtx$(row1) <> "EXIT" Then Call icon(armyloc(array(row1)), 0, 8)
    End Select

    choose = row
    GoTo sel1
    crsr: Def Seg = 0: Poke 1265, 14: Poke 1266, Asc(mtx$(0) + Chr$(0)): Poke 1264, (Peek(1264) + 1) And 255: Def Seg: Do: a$ = InKey$: Loop While a$ = ""
    If Asc(a$) = 13 Then Return
    If Len(a$) = 2 GoTo arrows
    If switch = 3 Then choose = -Asc(UCase$(a$)): GoTo called
    If Asc(a$) = 27 Then choose = -1: GoTo called
    row1 = row
    For k = 1 To size
        c1$ = UCase$(Left$(LTrim$(mtx$(k)), 1))
        If c1$ = UCase$(a$) Then choose = k: GoSub limits: GoTo called
    Next k
    GoTo crsr
    arrows:
    a$ = Right$(a$, 1)
    row1 = row
    If a$ = "H" Then row = row - 1: GoSub limits: Return
    If a$ = "I" Then row = 1: GoSub limits: Return
    If a$ = "P" Then row = row + 1: GoSub limits: Return
    If a$ = "Q" Then row = size: GoSub limits: Return
    If a$ = "=" And switch = 0 Then
        Cls
        Call usa
        Call choices(b1, wide)
        Call topbar
        Return
    End If
    If a$ = "A" And switch = 0 Then
        choose = 99
        GoTo called
        Return
    End If
    If a$ = ">" And switch = 0 Then
        Call report(-1)
        Call choices(b1, wide)
        Call topbar
        Return
    End If
    Return

    limits:
    If row > size Then row = 1
    If row < 1 Then row = size
    Return
    noadjust:
    If tlx = 0 Then tlx = Int(40.5 - .5 * wide)
    Return
    boss1:
    Cls
    Call usa
    GoTo remenu
    called:
    If noise > 0 Then Sound 700, .5
    Color colour
    tlx = 0: tly = 0
    Select Case switch
        Case 1, 2, 9
            Call icon(array(row), 0, 8)
        Case 6
            Call icon(armyloc(array(row)), 0, 8)
    End Select
    View
End Sub

Sub mxw (wide)
    wide = Len(mtx$(0)) + 1
    For i = 1 To size
        x = Len(mtx$(i))
        If x > wide Then wide = x
    Next i
End Sub

Sub schooner
    Screen 12
    Line (1, 240)-(639, 309), 3, BF
    Line (639, 309)-(1, 479), 1, BF
    Color 6
    Line (243, 241)-(247, 395), , BF
    Line (369, 395)-(373, 241), , BF
    Line (183, 395)-(435, 400), , BF
    Line (188, 395)-(107, 356): Line -(106, 360): Line -(173, 392)
    Line -(182, 419): Line -(191, 429): Line -(230, 426): Line -(275, 432)
    Line -(314, 418): Line -(354, 431): Line -(390, 421): Line -(390, 422)
    Line -(413, 427): Line -(437, 419): Line -(447, 401): Line -(447, 377)
    Line -(401, 377): Line -(399, 396)
    Paint (300, 415), 6

    Line (378, 290)-(434, 262): Line -(432, 258): Line -(377, 286)
    Line -(378, 290)
    Paint (380, 286), 6

    Line (378, 346)-(456, 336): Line -(457, 340): Line -(376, 350)
    Line -(378, 346)
    Paint (382, 347), 6


    Color 15
    Line (109, 355)-(131, 361)
    Line -(239, 259): Line -(239, 256): Line -(109, 355)
    Paint (140, 340)

    Line (238, 268)-(256, 273)
    Line -(241, 295): Line -(240, 309): Line -(250, 322)
    Line -(226, 321): Line -(220, 299): Line -(227, 284)
    Line -(239, 269)
    Paint (245, 275), 15

    Line (232, 327)-(225, 350): Line -(233, 385): Line -(251, 387)
    Line -(242, 352): Line -(243, 345): Line -(249, 333)
    Line -(232, 327)
    Paint (235, 335), 15

    Line (381, 268)-(368, 286)
    Line -(366, 303): Line -(374, 317): Line -(350, 315)
    Line -(347, 297): Line -(361, 264): Line -(381, 268)
    Paint (365, 275), 15

    Line (379, 335)-(360, 328)
    Line -(351, 347): Line -(355, 367)
    Line -(357, 382): Line -(378, 386): Line -(368, 373)
    Line -(369, 345): Line -(379, 334)
    Paint (360, 345), 15

    Line (379, 290)-(406, 342): Line -(456, 336): Line -(435, 289)
    Line -(433, 264): Line -(378, 291)
    Paint (410, 325), 15

    For k = 260 To 400 Step 25
        Circle (k, 405), 3, 0
        Paint (k, 405), 0
    Next k

    Color 11
    Line (247, 437)-(290, 422): Line -(314, 416): Line -(334, 423)
    Line -(305, 432): Line -(287, 430): Line -(286, 430)
    Line -(267, 441): Line -(225, 439): Line -(225, 438)
    Line -(246, 437)
    Paint (300, 425), 3, 11

    Line (362, 428)-(385, 419)
    Line -(394, 421): Line -(375, 437): Line -(354, 432)
    Line -(345, 429): Line -(362, 428)
    Paint (370, 430), 3, 11

    Line (16, 383)-(43, 379)
    Line -(44, 379): Line -(78, 385): Line -(16, 384)
    Paint (40, 383), 9, 11

    Line (173, 440)-(204, 431): Line -(227, 431): Line -(210, 439)
    Line -(175, 441): Line -(173, 440)
    Paint (200, 435), 11

    Line (445, 417)-(552, 409)
    Line -(599, 416): Line -(539, 415): Line -(468, 415)
    Paint (550, 411), 9, 11

    Line (509, 349)-(540, 351)
    Line -(541, 351)
    Line -(517, 343): Line -(509, 349)
    Paint (520, 347), 3, 11

    Line (577, 342)-(598, 343)
    Line (275, 340)-(287, 342), 9

    Line (149, 346)-(188, 345), 9
    Line (24, 337)-(72, 338), 3

    Color 7
    Line (227, 354)-(236, 383): Line -(246, 386): Line -(239, 356)
    Line -(233, 357): Line -(227, 354)
    Paint (238, 365), 7

    Line (223, 301)-(231, 321): Line -(244, 321): Line -(237, 308): Line -(240, 303)
    Line -(223, 301)
    Paint (236, 312), 7

    Line (349, 299)-(353, 313): Line -(370, 316)
    Line -(363, 302): Line -(364, 303): Line -(350, 300)
    Paint (360, 310), 7

    Line (356, 359)-(359, 380): Line -(375, 385): Line -(366, 372): Line -(368, 364)
    Line -(356, 360)
    Paint (360, 370), 7

    Line (393, 302)-(394, 302): Line -(409, 323)
    Line -(417, 309): Line -(434, 325): Line -(429, 304): Line -(428, 291)
    Line -(410, 296): Line -(400, 290): Line -(394, 302)
    Paint (420, 300), 7

    Color 0
    Line (247, 256)-(291, 394)
    Line (247, 256)-(284, 394)
    Line (244, 256)-(191, 394)
    Line (244, 256)-(184, 394)

    Line (371, 257)-(321, 394)
    Line (371, 257)-(314, 394)
    Line (375, 257)-(431, 378)
    Line (375, 257)-(438, 378)

    Line (375, 258)-(434, 259)

    Color 8
    Line (180, 400)-(183, 415): Line -(192, 426): Line -(212, 426)
    Line -(202, 421): Line -(210, 414): Line -(191, 408): Line -(201, 402)
    Line -(180, 400)
    Paint (185, 410), 8

    Line (445, 381)-(445, 400): Line -(435, 417)
    Line -(413, 424): Line -(422, 413): Line -(418, 404): Line -(434, 401)
    Line -(427, 393): Line -(435, 391): Line -(437, 384): Line -(445, 382)
    Paint (440, 388), 8

    Line (186, 425)-(155, 429), 11: Line -(146, 433), 11
    Line (150, 417)-(116, 425), 11
    Line (123, 452)-(153, 460), 11
    Line (297, 463)-(319, 468), 11

    For k = 1 To 4
        x = 185 + 200 * Rnd
        Line (x, 390)-(x + 2, 392), 12, BF: Line (x - 2, 392)-(x + 4, 394), 8, BF
    Next k
End Sub

' Copied from Civil War Strategy by tools/screen/cut_game.py.
' Civil War Strategy played from a keys file (tools/screen/README.md, "Prompts"). Edits:
' a fixed seed and silent sound (PLAY "V0": every PLAY and SOUND takes its time, unheard);
' no _FULLSCREEN (a test does not take the screen); SUB TICK returns at once (its pauses
' only wait, and a key that came while one waits would be eaten); a fixed date in the
' history; and before each of the 14 places the game waits for a key, a line that writes
' the place (and at the menu the first letter of its title) to bytes &H4F1 and &H4F2 and
' announces the wait in byte &H4F0 of conventional memory, which the keys file's prompt,
' rule and play until steps read.
