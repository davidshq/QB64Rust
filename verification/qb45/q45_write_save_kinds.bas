' QB 4.5, PDS 7.1 and QB64: the text WRITE # and PRINT # give for the kinds of values a game's
' saved files hold, and what INPUT # reads back. Found by the Wars of Napoleon port
' (Hutsell-Games/wars-of-napoleon, docs\port-study.md step 3). Its saved games, NWS.CFG and
' HISCORE.NWS are written by WRITE # with INTEGERs, LONGs, strings and one SINGLE (turbo!, 1 to 8);
' BATTSUMM and NWS.HIS by PRINT # with ; , and TAB.
' Each part writes a file, then copies its lines to OUT.TXT, then reads the values back.
DEFINT A-Z
OPEN "OUT.TXT" FOR OUTPUT AS #1
DIM c&(2)
c&(1) = 0: c&(2) = 123456
t! = 2: u! = 8: h! = .5: q! = 1 / 3
OPEN "T.TXT" FOR OUTPUT AS #2
WRITE #2, "1796", 11, 1796, 225, 2
WRITE #2, 0, -1, 32767, -32768, c&(1), c&(2), -c&(2)
WRITE #2, "Napoleon", "", "a b", "x,y", 1
WRITE #2, 2, 2, 3, 3, 1, t!, 3, 0, 1, u!
WRITE #2, h!, q!, -h!, 1E+20, 1E-20
WRITE #2, "COMPUTER", 634
CLOSE #2
PRINT #1, "write"
GOSUB copy
OPEN "T.TXT" FOR INPUT AS #2
INPUT #2, s$, a, b, c, d: PRINT #1, s$; a; b; c; d
INPUT #2, a, b, c, d, e&, f&, g&: PRINT #1, a; b; c; d; e&; f&; g&
INPUT #2, s$, r$, v$, w$, a: PRINT #1, "["; s$; "]["; r$; "]["; v$; "]["; w$; "]"; a
INPUT #2, a, b, c, d, e, x!, f, g, h, y!: PRINT #1, a; b; c; d; e; x!; f; g; h; y!
INPUT #2, x!, y!, z!, p!, r!: PRINT #1, x!; y!; z!; p!; r!
INPUT #2, s$, a: PRINT #1, s$; a
CLOSE #2
OPEN "T.TXT" FOR OUTPUT AS #2
f$ = "French ": g$ = "Allied "
PRINT #2, " SIDE      BATTLES WON       LOSSES"
PRINT #2, " "; f$, 3, 100 * c&(2)
PRINT #2, " "; g$, 12, 100 * c&(1)
PRINT #2, TAB(20); "[ HISTORY OF GAME BEGUN "; "06-24-1998"; " ]"
PRINT #2, t!; u!; h!; q!; -q!; c&(2)
PRINT #2, "a"; TAB(30); "b"; SPC(3); "c"
CLOSE #2
PRINT #1, "print"
GOSUB copy
CLOSE #1
SYSTEM

copy:
OPEN "T.TXT" FOR INPUT AS #2
DO WHILE NOT EOF(2)
LINE INPUT #2, l$
PRINT #1, "|"; l$; "|"
LOOP
CLOSE #2
RETURN
