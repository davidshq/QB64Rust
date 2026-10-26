$Console:Only
' Slice program (m2-numeric-types, tasks 4.3 and 8.1-8.5): the new numeric types and fixed-length strings
' everywhere else: static arrays and TYPE members of them, parameters and FUNCTION results (an unsigned FUNCTION, a
' FUNCTION f$n, a STRING * n parameter), passing by reference across signedness and between _INTEGER64 and _OFFSET
' (variables, elements, members; a copy in parentheses), a _BIT variable passed as a copy, FOR with each new type as
' the variable (the hidden type by width), SELECT CASE with each new type as the selector (read at each test or
' copied), CONST with the new suffixes and constants used with another suffix, and the special-cased built-ins with
' arguments of the new types. The scenarios of the spec deltas arrays-and-types, procedures, control-flow, constants
' and builtin-functions that the old compiler can run; the measurements are verification\v21_b_passing,
' v21_c_fixed_args, v21_d_*, v21_f_fixed_param and v21_g_*. No PRINT comma.
On Error GoTo h

' Arrays of the new types (scenario "Unsigned elements").
Dim u(2) As _Unsigned Integer: u(1) = -1: Print u(1); u(2)
Dim sb(-1 To 1) As _Byte, ub(3) As _Unsigned _Byte, ul(1) As _Unsigned Long
Dim uq(1) As _Unsigned _Integer64, so(1) As _Offset, uo(1) As _Unsigned _Offset
sb(-1) = 127: sb(-1) = sb(-1) + 1: ub(3) = ub(3) - 1: ul(0) = -2: uq(1) = -3: so(0) = -4: uo(1) = -5
Print "elements:"; sb(-1); sb(0); ub(3); ul(0); uq(1); so(0); uo(1)
Print "element arithmetic:"; ub(3) + ul(0); uq(1) * 2; so(0) / 3
Dim b2%%(2), ul2~&(1 To 2)
b2%%(1) = 255: ul2~&(2) = -1
Print "suffixed arrays:"; b2%%(1); ul2~&(2); LBound(ul2~&); UBound(b2%%)
Print "bad index:"; ub(4)
Dim fa(2) As String * 4
fa(1) = "abcdefg": fa(2) = "x"
Print "fixed elements: ["; fa(1); "]["; fa(2); "]"; Len(fa(1)); Asc(fa(0), 1)

' TYPE members of the new types (scenario "Members of the new types").
Type nt
    b As _Byte
    ub As _Unsigned _Byte
    ui As _Unsigned Integer
    ul As _Unsigned Long
    uq As _Unsigned _Integer64
    o As _Offset
End Type
Type wrap
    f As String * 3
    n As nt
    uo As _Unsigned _Offset
End Type
Dim v As nt
v.b = -1: v.ub = -1: v.ui = -1: v.ul = -1: v.uq = -1: v.o = -1
Print "members:"; v.b; v.ub; v.ui; v.ul; v.uq; v.o; Len(v)
v.b = 200: v.ub = 300: v.ui = 70000: v.ul = 2.5: v.uq = 1E+19: v.o = 3.5
Print "member stores:"; v.b; v.ub; v.ui; v.ul; v.uq; v.o
Dim w(1) As wrap
w(1).f = "hello": w(1).n.ul = 4294967295: w(1).n.b = -128: w(1).uo = -1
Print "members of elements: "; w(1).f; w(1).n.ul; w(1).n.b; w(1).uo; Len(w(1)); w(0).n.uq

' Parameters and FUNCTION results.
Print "big~&:"; big~&(255)
Print "fs$5: ["; fs$5("ab"); "]"; Len(fs$5("ab"))
Print "fs$ and fs: ["; fs$("abcdefgh"); "]["; fs("q"); "]"
Print "fz$3:"; Len(fz$3); Asc(fz$3, 1)
s$ = "hello world"
sp s$
Print "after sp: "; s$
Dim fx As String * 8
fx = "fixed"
sp fx
Print "after sp of a STRING * 8: ["; fx; "]"

' By reference across signedness (scenarios "Same width, other signedness", "_INTEGER64 to an _OFFSET parameter").
n& = -1: showul n&: Print " after:"; n&
q&& = -1: showuo q&&: Print " after:"; q&&
Dim sbv As _Byte: sbv = -1: showub sbv: Print " after:"; sbv
Dim uiv As _Unsigned Integer: uiv = 65535: showsi uiv: Print " after:"; uiv
Dim uov As _Unsigned _Offset: uov = 5: showsq uov: Print " after:"; uov
u(1) = 65535: showsi u(1): Print " after:"; u(1)
u(1) = 65535: showsi (u(1)): Print " after:"; u(1)
v.ul = 9: showsl v.ul: Print " after:"; v.ul
w(1).n.ub = 200: showsb w(1).n.ub: Print " after:"; w(1).n.ub
uq(0) = 3: showso uq(0): Print " after:"; uq(0)
' A _BIT variable is a copy (scenario "Bit variable passes a copy").
Dim b5 As _Bit * 5
b5 = -16: showsl b5: Print " after:"; b5

' FOR with each new type (scenarios "Unsigned byte passes its range", "Unsigned variable counts down through 0").
For b~%% = 250 To 255 Step 3: Print b~%%;: Next: Print b~%%
For ui~% = 2 To 0 Step -1: Print ui~%;: Next: Print ui~%
For y%% = 125 To 127: Print y%%;: Next: Print y%%
For l~& = 4294967294 To 4294967295: Print l~&;: Next: Print l~&
For o%& = -1 To 1: Print o%&;: Next: Print o%&
For uo~%& = 1 To 2: Print uo~%&;: Next: Print uo~%&
c = 0
For qq~&& = 1 To 18446744073709551615~&&: c = c + 1: Next
Print "~&& to 2^64-1 passes:"; c; qq~&&
For k~%% = 1 To 2.6: Print k~%%;: Next: Print k~%%

' SELECT CASE with each new type (scenario "Negative item against unsigned selectors").
Dim sui As _Unsigned Integer, sul As _Unsigned Long, suq As _Unsigned _Integer64, ssb As _Byte
sui = 65535: sul = 4294967295: suq = 18446744073709551615~&&: ssb = -1
Print "~% 65535:";
Select Case sui
    Case -1: Print " -1"
    Case Else: Print " else"
End Select
Print "~& max:";
Select Case sul
    Case -1: Print " -1"
    Case Else: Print " else"
End Select
Print "~&& max, 0 TO -1:";
Select Case suq
    Case 0 To -1: Print " 0 TO -1"
    Case Else: Print " else"
End Select
Print "%% -1:";
Select Case ssb
    Case 255: Print " 255"
    Case -1: Print " -1"
End Select
Print "element ~% 65535:";
Select Case u(1)
    Case Is > 65534: Print " > 65534"
    Case Else: Print " else"
End Select
Print "member ~%% + 0, 2.5:";
v.ub = 2
Select Case v.ub + 0
    Case 2.5: Print " 2.5"
    Case Else: Print " else"
End Select
Print "~& + 1~&&:";
Select Case sul + 1~&&
    Case 4294967296: Print " 4294967296"
    Case Else: Print " else"
End Select

' CONST with the new suffixes (scenarios "Unsigned suffix", "Suffix with a value out of range", "Bit suffix").
Const cu~% = 65535
Print "CONST ~%:"; cu~%; cu~% + 1
Const du~% = -1, dc~%% = 300
Print "out of range:"; du~%; du~% + 0; du~% < 0; dc~%%; dc~%% + 0
Const i`3 = 5, j~`3 = -1
Print "bit:"; i`3; j~`3; i`3 + 0
' Constants used with another suffix (v21_g_const_suffix).
Const bigc = 4294967295, negc = -1, flc = 2.5, smallc = 300
Print "plain as new:"; bigc~&; negc~&; negc~%%; smallc%%; flc~%; negc~&&; negc%&; negc~%&
Print "+ 0:"; negc~& + 0; negc~%% + 0; smallc%% + 0; negc~&& + 0; negc~& < 0
Const nb~%% = -1, nu~& = 4294967295
Print "new as other:"; nb%; nb&; nb!; nu&; nu%; nu% + 0; nb~&; nu~%%; nu~%% + 0

' Built-ins (scenarios "Unsigned argument", "VAL with an unsigned type").
ulv~& = 4294967295: Print Hex$(ulv~&); Str$(ulv~&); Abs(ulv~&)
Print Val("-1", _Unsigned Long); Val("300", _Unsigned _Byte); Val("-1", _Byte); Val("-1", _Offset)
Dim b3 As _Bit * 3: b3 = -4
Print "HEX$: "; Hex$(sbv); " "; Hex$(uiv); " "; Hex$(uq(1)); " "; Hex$(uq(1) + 0); " "; Hex$(b3); " "; Hex$(-1`5)
Print "OCT$ _BIN$: "; Oct$(sbv); " "; _Bin$(b3); " "; Oct$(v.ub)
Print "ABS SGN:"; Abs(sbv); Abs(uiv); Abs(b3); Sgn(uiv); Sgn(so(0)); Sgn(b3)
Print "INT FIX:"; Int(uiv); Fix(uq(1)); Int(b3); Int(uiv) + 0
Print "CINT:"; CInt(sbv); CInt(b3); CInt(uiv)
Print "CLNG:"; CLng(uiv); CLng(sul)
Print "CLNG ~&&:"; CLng(uq(1))
Print "_ROUND:"; _Round(ub(3)); _Round(uo(1)); _Round(b3)
Print "CSNG CDBL:"; CSng(sul); CDbl(uq(1))
v.ub = 2: uiv = 2: sul = 2
Print "SQR:"; Sqr(v.ub); Sqr(uiv); Sqr(sul); Sqr(b3 + 6)
Print "EXP:"; Exp(v.ub); Exp(sul); Exp(b3)
Print "slots: ["; Left$("abcdef", v.ub); "] ["; Chr$(65 + v.ub); "]"
System

h:
Print "[handler"; Err; "]"
Resume Next

Function big~& (x As _Unsigned _Byte)
    big~& = x * 16843009
End Function

Function fs$5 (x As String)
    fs$5 = x
End Function

Function fz$3
End Function

Sub sp (t As String * 5)
    Print "["; t; "]"; Len(t); Len(t + "!")
    t = "much longer text"
End Sub

Sub showsb (x As _Byte)
    Print "_BYTE param:"; x;: x = -2
End Sub
Sub showub (x As _Unsigned _Byte)
    Print "_UNSIGNED _BYTE param:"; x;: x = 254
End Sub
Sub showsi (x As Integer)
    Print "INTEGER param:"; x;: x = -3
End Sub
Sub showsl (x As Long)
    Print "LONG param:"; x;: x = 5
End Sub
Sub showul (x As _Unsigned Long)
    Print "_UNSIGNED LONG param:"; x;: x = 5
End Sub
Sub showsq (x As _Integer64)
    Print "_INTEGER64 param:"; x;: x = -6
End Sub
Sub showso (x As _Offset)
    Print "_OFFSET param:"; x;: x = -8
End Sub
Sub showuo (x As _Unsigned _Offset)
    Print "_UNSIGNED _OFFSET param:"; x;: x = 7
End Sub
