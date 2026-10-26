$Console:Only
' Slice (m3-console-files, task 6.2; from verification\v30_j_field, v30_k_field_errors, v30_l_lset_rset): FIELD,
' LSET and RSET, 20 numbered lines. Records written through fields and read back with GET and PUT with no
' variable; what a field string holds after FIELD; a second FIELD on the file (both name the buffer from its first
' byte); = to a field variable (the string changes, the buffer does not, and the string is still a field); GET and
' PUT with a variable beside fields (the buffer is not theirs); a width of 0; after CLOSE; the errors of FIELD
' (widths beyond the record 50, a negative width 5, not open 52, not random 54) and of GET and PUT with no variable
' (record 0: 63, a binary file: 54, ACCESS READ and WRITE: 75); the file number before a target's index before
' its width; elements and members as fields. LSET and RSET on any string: shorter, equal and longer values, an
' empty target, fixed-length strings, elements, members, the target in its own value, an error in the value.
' The files are s86_field_lset.tmp and s86_field_lset_2.tmp and are removed.
On Error GoTo handler
Type t
    s As String * 6
    v As String
    n As Long
End Type
Dim f As String, a As String, b As String, c As String, d As String, w As String, k As Long, l As Long
Dim ub As _Unsigned _Byte, x As String * 8, y As String * 5, e(1 To 3) As String, r As t, q(2) As t
Dim fx(2) As String * 4, s As Single
f = "s86_field_lset.tmp"
Open f For Random As #1 Len = 8
a = "before"
Field #1, 3 As a, 5 As b
Print "1:"; Len(a); Len(b); Asc(a); Asc(b)
LSet a = "xy": LSet b = "hello"
Print "2: ["; a; "]["; b; "]";: Put #1, 1: Print LOF(1); Seek(1)
LSet a = "": LSet b = ""
Print "3: ["; a; "]["; b; "]";: Get #1, 1: Print "["; a; "]["; b; "]"; Seek(1)
RSet a = "z": RSet b = "ab"
Print "4: ["; a; "]["; b; "]";: Put #1: Print LOF(1); Seek(1);: Put #1, 4: Print LOF(1); Seek(1)
Print "5:";: Get #1, 2: Print " ["; a; "]["; b; "]";: Get #1, 3: Print Asc(a); Asc(b); EOF(1);: Get #1, 9: Print Asc(a); EOF(1); LOF(1)
Get #1, 1
Field #1, 8 As w
Print "6: ["; w; "]["; a; "]["; b; "]";: LSet w = "12345678": Print "["; a; "]["; b; "]";: LSet b = "B": Print "["; w; "]"
Field #1, 2 As c, 2 As d
Print "7: ["; c; "]["; d; "]";: Field #1, 4 As a: Print "["; a; "]"; Len(a);: LSet w = "abcdefgh": Print "["; a; "]["; b; "]["; c; "]"
a = "QQ"
Print "8: ["; a; "]"; Len(a); "["; w; "]";: LSet w = "ABCDEFGH": Print "["; a; "]"; Len(a);
a = "longer than the field": Print Len(a);: Get #1, 1: Print "["; a; "]"; Len(a)
b = "": Print "9:"; Len(b);: LSet b = "k": Print Len(b); "["; w; "]"
LSet w = "ABCDEFGH": Put #1, 1
Print "10:";: l = 0: Get #1, 1, l: Print l; "["; w; "]";: x = "ijklmnop": Put #1, 1, x: Print "["; w; "]";: Get #1, 1: Print "["; w; "]"
Field #1, 0 As c, 8 As d
Print "11:"; Len(c); "["; d; "]";: LSet c = "x": Print Len(c)
Close #1
Print "12: ["; w; "]"; Len(w);: LSet w = "closed": Print "["; w; "]";: w = "free": Print "["; w; "]"
Open f For Random As #1 Len = 8
Print "13:"; LOF(1); "["; d; "]";: LSet d = "again": Put #1, 1: Field #1, 8 As w: Get #1, 1: Print Asc(w); "["; d; "]"
Print "14:";: Field #1, 20 As a: Print Len(a);: Field #1, 5 As a, 4 As b, 1 As c: Print Len(a); Len(b); Len(c);
Field #1, -1 As c: Print Len(c);: Field #1, 2.5 As c: Print Len(c);: Field #1, 3.5 As c: Print Len(c)
k = 0: s = 2.6
Print "15:";: Field 1, 4 As c: Print Len(c);: Field #k + 1, s + 2 As c: Print Len(c);: Field (1), 7 As c: Print Len(c);
Field #3, 2 As c: Print Len(c);: Field #0, 2 As c: Print Len(c);: Field #-1, 2 As c: Print Len(c)
Field #1, 2 As e(1), 3 As e(2), 3 As r.v
LSet e(1) = "ab": LSet e(2) = "cde": LSet r.v = "fgh"
Field #1, 8 As a
Print "16:"; Len(e(1)); Len(e(2)); Len(e(3)); Len(r.v); "["; a; "]";: k = 2: Field #1, 4 As e(k + 1), 4 As q(k).v: Print "["; e(3); "]["; q(2).v; "]";
Field #1, 2 As e(4): Print
Print "17:";: Field #fn&, wd& As c: Print Len(c);: Field #fn&, wd& As e(ix&): Print Len(e(1))
Print "18:";: Get #1, 0: Print Seek(1);: Put #1, -1: Print Seek(1);: Get #3: Put #3: Print
Close #1
Open f For Binary As #1: Print "19:";: Field #1, 2 As c: Get #1: Put #1: Close #1
Open "s86_field_lset_2.tmp" For Output As #1: Field #1, 2 As c: Close #1: Kill "s86_field_lset_2.tmp"
Open f For Random Access Read As #1 Len = 8: Field #1, 8 As c: Get #1, 1: Put #1, 1: Close #1
Open f For Random Access Write As #1 Len = 8: Field #1, 8 As c: Get #1, 1: Put #1, 1: Close #1
Print
Open f For Binary As #1
Print "20:";: For k = 1 To LOF(1): Get #1, k, ub: Print ub;: Next: Print
Close #1: Kill f
a = Space$(5)
LSet a = "ab": Print "21: ["; a; "]";: RSet a = "ab": Print "["; a; "]";: LSet a = "abcde": Print "["; a; "]";: RSet a = "vwxyz": Print "["; a; "]"
a = Space$(3)
LSet a = "abcdef": Print "22: ["; a; "]";: RSet a = "abcdef": Print "["; a; "]";: LSet a = "": Print "["; a; "]";
a = "": LSet a = "abc": Print "["; a; "]"; Len(a);: RSet a = "abc": Print Len(a)
LSet y = "ab": Print "23: ["; y; "]";: RSet y = "ab": Print "["; y; "]";: RSet y = "abcdefgh": Print "["; y; "]";: LSet y = "": Print "["; y; "]"
e(2) = ".....": k = 1
LSet e(2) = "ab": Print "24: ["; e(2); "]";: RSet e(k + 1) = "cd": Print "["; e(2); "]";: RSet fx(1) = "z": Print "["; fx(1); "]";: LSet e(4) = "ab": Print
r.v = "12345": q(2).v = "1234"
LSet r.s = "ab": Print "25: ["; r.s; "]";: RSet r.s = "ab": Print "["; r.s; "]";: LSet r.v = "ab": Print "["; r.v; "]";: RSet q(1).s = "ab": RSet q(2).v = "a": Print "["; q(1).s; "]["; q(2).v; "]"
a = "abcdef": LSet a = Mid$(a, 3): Print "26: ["; a; "]";: a = "abcdef": RSet a = Left$(a, 2): Print "["; a; "]";: LSet a = a + "x": Print "["; a; "]"
e(1) = "111"
Print "27:";: LSet e(ix&) = vl$: Print " ["; e(1); "]";: a = Space$(8): RSet a = r.s: Print "["; a; "]";: LSet y = a: Print "["; y; "]"
a = "keep."
Print "28:";: LSet a = Chr$(-1): Print "["; a; "]";: If Len(a) Then LSet a = "x" Else RSet a = "y"
Print "["; a; "]"
System
handler:
Print " <error"; Err; ">";
Resume Next
Function fn&
Print " fn";
fn& = 1
End Function
Function wd&
Print " wd";
wd& = 3
End Function
Function ix&
Print " ix";
ix& = 1
End Function
Function vl$
Print " vl";
vl$ = "v"
End Function
