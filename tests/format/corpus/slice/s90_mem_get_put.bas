$Console:Only
' Slice (m3-mem task 4.1): verification\v32_b_get_put as a slice program. Reading and writing through a block
' by each type (_MEMGET as a function and as a statement, _MEMPUT, _MEMFILL, _MEMCOPY), the pixels of an image
' through _MEMIMAGE, and when _MEMEXISTS says a block is no longer valid.
Type t
    a As Integer
    d As Double
End Type
Dim Shared gm As _MEM
Dim m As _MEM, n As _MEM, u As t, w As t, f As String * 4, s As String
Dim a As Long, i As Integer, d As Double, o As _Offset, fl As _Float, by As _Byte, b(10) As Integer
Dim i64 As _Integer64, sg As Single, ub As _Unsigned _Byte, ul As _Unsigned Long
Print "put and get a LONG"
m = _Mem(a)
_MemPut m, m.OFFSET, 7 As Long
Print a; _MemGet(m, m.OFFSET, Long)
Dim e As _MEM: e = m: Print e.OFFSET = m.OFFSET; e.SIZE
Print "each type AS, read back by the function"
m = _MemNew(64)
_MemPut m, m.OFFSET, -5 As _Byte: Print _MemGet(m, m.OFFSET, _Byte); _MemGet(m, m.OFFSET, _Unsigned _Byte)
_MemPut m, m.OFFSET, -300 As Integer: Print _MemGet(m, m.OFFSET, Integer); _MemGet(m, m.OFFSET, _Unsigned Integer)
_MemPut m, m.OFFSET, -70000 As Long: Print _MemGet(m, m.OFFSET, Long); _MemGet(m, m.OFFSET, _Unsigned Long)
_MemPut m, m.OFFSET, -5000000000 As _Integer64: Print _MemGet(m, m.OFFSET, _Integer64); _MemGet(m, m.OFFSET, _Unsigned _Integer64)
_MemPut m, m.OFFSET, 1.5 As Single: Print _MemGet(m, m.OFFSET, Single); _MemGet(m, m.OFFSET, Long)
_MemPut m, m.OFFSET, 2.25 As Double: Print _MemGet(m, m.OFFSET, Double); _MemGet(m, m.OFFSET, _Integer64)
_MemPut m, m.OFFSET, 3.5 As _Float: Print _MemGet(m, m.OFFSET, _Float)
_MemPut m, m.OFFSET, 99 As _Offset: Print _MemGet(m, m.OFFSET, _Offset); _MemGet(m, m.OFFSET, _Unsigned _Offset)
_MemPut m, m.OFFSET, 2.7 As Long: Print _MemGet(m, m.OFFSET, Long)
_MemPut m, m.OFFSET, 300 As _Unsigned _Byte: Print _MemGet(m, m.OFFSET, _Unsigned _Byte)
a = 1000: _MemPut m, m.OFFSET, a As _Byte: Print _MemGet(m, m.OFFSET, _Byte)
Print "variables and strings"
_MemPut m, m.OFFSET, "abcdefgh": Print _MemGet(m, m.OFFSET, String * 8); _MemGet(m, m.OFFSET + 2, String * 3)
f = "wxyz": _MemPut m, m.OFFSET, f: Print _MemGet(m, m.OFFSET, String * 6)
s = "12": _MemPut m, m.OFFSET, s: Print _MemGet(m, m.OFFSET, String * 6)
s = "--": _MemPut m, m.OFFSET, s + "x": Print _MemGet(m, m.OFFSET, String * 6)
u.a = 12: u.d = 3.25: _MemPut m, m.OFFSET, u: w = _MemGet(m, m.OFFSET, t): Print w.a; w.d
i = 4660: _MemPut m, m.OFFSET, i: Print Hex$(_MemGet(m, m.OFFSET, Integer)); _MemGet(m, m.OFFSET, _Unsigned _Byte)
b(1) = -2: _MemPut m, m.OFFSET, b(1): Print _MemGet(m, m.OFFSET, Integer)
Print "the statement into each kind of variable"
_MemPut m, m.OFFSET, -12345 As Long
_MemGet m, m.OFFSET, a: Print a
_MemGet m, m.OFFSET, i: Print i
_MemGet m, m.OFFSET, by: Print by
_MemGet m, m.OFFSET, b(3): Print b(3)
_MemPut m, m.OFFSET, 6.5 As Double
_MemGet m, m.OFFSET, d: Print d
_MemGet m, m.OFFSET, sg: Print sg
_MemPut m, m.OFFSET, 1.25 As _Float
_MemGet m, m.OFFSET, fl: Print fl
_MemPut m, m.OFFSET, "pqrstuvw"
_MemGet m, m.OFFSET, f: Print f
s = "abc": _MemGet m, m.OFFSET, s: Print s
_MemPut m, m.OFFSET, u
_MemGet m, m.OFFSET, w: Print w.a; w.d
_MemGet m, m.OFFSET, w.a: Print w.a
Print "fill and copy"
Dim bb(10) As Integer, la As Long
la = 7
n = _Mem(bb())
_MemFill n, n.OFFSET, n.SIZE, 3 As Integer
e = _Mem(la)
_MemCopy e, e.OFFSET, 2 To n, n.OFFSET + 4
Print bb(0); bb(10); bb(2); bb(3)
_MemFill n, n.OFFSET, 5, 258 As Integer: Print bb(0); bb(1); bb(2); bb(3)
_MemFill n, n.OFFSET, n.SIZE, "ab": Print Hex$(bb(0)); Hex$(bb(10))
i = -1: _MemFill n, n.OFFSET, 4, i: Print bb(0); bb(1); bb(2)
_MemFill n, n.OFFSET, n.SIZE, 1.5 As Single: Print Hex$(bb(0)); Hex$(bb(1)); bb(10)
_MemFill n, n.OFFSET, 0, 9 As _Byte: Print bb(0)
_MemCopy n, n.OFFSET, 4 To n, n.OFFSET + 2: Print Hex$(bb(1)); Hex$(bb(2))
Print "new memory"
Dim nm As _MEM
nm = _MemNew(16): Print _MemExists(nm); nm.SIZE; nm.TYPE; nm.ELEMENTSIZE; nm.IMAGE: _MemFree nm: Print _MemExists(nm)
nm = _MemNew(0): Print _MemExists(nm); nm.SIZE; nm.OFFSET
Dim un As _MEM: Print _MemExists(un); un.SIZE; un.OFFSET
Print "an image"
Dim img As Long, px As _MEM
img = _NewImage(4, 2, 32)
_Dest img: PSet (1, 1), _RGB32(10, 20, 30): _Dest _Console
px = _MemImage(img)
Print Hex$(_MemGet(px, px.OFFSET + (1 * 4 + 1) * 4, _Unsigned Long)); px.SIZE; px.TYPE; px.ELEMENTSIZE; px.IMAGE = img
Print _MemExists(px)
_MemFree px: Print _MemExists(px)
px = _MemImage(img): _FreeImage img: Print _MemExists(px)
img = _NewImage(3, 3, 256): px = _MemImage(img): Print px.SIZE; px.TYPE; px.ELEMENTSIZE
Print "when a block stops being valid"
ReDim dd(5) As Long
m = _Mem(dd()): Print _MemExists(m)
Erase dd: Print _MemExists(m)
ReDim dd(5) As Long: m = _Mem(dd()): ReDim dd(6) As Long: Print _MemExists(m)
ReDim dd(5) As Long: m = _Mem(dd(2)): ReDim _Preserve dd(7) As Long: Print _MemExists(m)
Dim sd(5) As Long: m = _Mem(sd()): Erase sd: Print _MemExists(m)
keep: Print _MemExists(gm)
Print "the address"
Print m.OFFSET = _Offset(a); _Offset(a) = _Offset(a)
m = _Mem(a): Print m.OFFSET = _Offset(a)
a = 9: m = _Mem(_Offset(a), 4): Print _MemGet(m, m.OFFSET, Long); m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE
m = _Mem(b()): Print m.OFFSET = _Offset(b()); m.OFFSET = _Offset(b(0)); _Offset(b(1)) - _Offset(b(0))
System

Sub keep
    Dim l As Long
    gm = _Mem(l)
    Print _MemExists(gm);
End Sub
