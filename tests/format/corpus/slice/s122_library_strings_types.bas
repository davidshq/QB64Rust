$Console:Only
' DECLARE LIBRARY with a header (v37h.h in this program's .files folder): STRING parameters (char*),
' STRING results (char* made a string, NULL as ""), and a TYPE passed as void* (verification\v37_d,
' v37_e, x39).
Type pt
    x As Long
    y As Long
End Type
Declare Library "v37h"
    Function h_len& (s As String)
    Sub h_upper (s As String)
    Function h_hello$
    Function h_empty$ ()
    Function h_nullstr$
    Function h_ptsum& (p As pt)
    Sub h_ptset (p As pt)
End Declare
Type t
    f As String * 4
    g As String * 4
End Type

Dim a As String, fx As String * 6, v As t
a = "hello"
Print "len:"; h_len&(a); h_len&("abc"); h_len&(""); h_len&(a + "xy")
Print "len left$:"; h_len&(Left$("abcdef", 3))
a = "ab" + Chr$(0) + "cd": Print "len nul:"; h_len&(a)
fx = "ab": Print "len fixed:"; h_len&(fx)
a = "mixed Case"
h_upper a: Print "upper var: "; a
fx = "abc": h_upper fx: Print "upper fixed: "; fx
b$ = "xyz": c$ = b$: h_upper b$: Print "upper copy: "; b$; " "; c$
Print "result: "; h_hello$; "|"; h_empty$; "|"; h_nullstr$; "|"; Len(h_hello$); UCase$(h_hello$)
r$ = h_hello$ + "!": Print r$

Dim p As pt, q(2) As pt
p.x = 3: p.y = 4
Print "udt:"; h_ptsum&(p)
h_ptset p: Print "udt set:"; p.x; p.y
q(1).x = 5: q(1).y = 6: Print "udt element:"; h_ptsum&(q(1))
System
