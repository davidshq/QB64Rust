$Console:Only

_Define A-Z As LONG 
Option _Explicit

Const FILE_NAME = "mem_checking_test.bmp"

ChDir _StartDir$

Dim img As Long: img = _NewImage(1280, 720, 32)
_Dest img
_Source img
Cls , _RGB32(255, 255, 0)

Dim i As Long
For i = 1 To 20
    Line (Rnd * 1280, Rnd * 720)-(Rnd * 1280, Rnd * 720), _RGB32(Rnd * 256, Rnd * 256, Rnd * 256), B
Next

Save32 0, 0, 1279, 719, 0, FILE_NAME

_Dest _Console
_Source _Console

Print "Saved "; FILE_NAME

Kill FILE_NAME

System

Sub Save32 (x1%, y1%, x2%, y2%, image&, Filename$)
    'Super special STEVE-Approved BMP Export routine for use with 32-bit color images.

    Type BMPFormat ' Description                          Bytes    QB64 Function
        ID As String * 2 ' File ID("BM" text or 19778 AS Integer) 2      CVI("BM")
        Size As Long ' Total Size of the file                4      LOF
        Blank As Long ' Reserved                              4
        Offset As Long ' Start offset of image pixel data      4      (add one for GET)
        Hsize As Long ' Info header size (always 40)          4
        PWidth As Long ' Image width                            4      _WIDTH(handle&)
        PDepth As Long ' Image height (doubled in icons)        4      _HEIGHT(handle&)
        Planes As Integer ' Number of planes (normally 1)          2
        BPP As Integer ' Bits per pixel(palette 1, 4, 8, 24)    2      _PIXELSIZE(handle&)
        Compression As Long ' Compression type(normally 0)          4
        ImageBytes As Long ' (Width + padder) * Height              4
        Xres As Long ' Width in PELS per metre(normally 0)    4
        Yres As Long ' Depth in PELS per metre(normally 0)    4
        NumColors As Long ' Number of Colors(normally 0)          4      2 ^ BPP
        SigColors As Long ' Significant Colors(normally 0)        4
    End Type '                Total Header bytes =  54

    Dim BMP As BMPFormat
    Dim x As Long, y As Long
    Dim temp As String * 3
    Dim m As _MEM, n As _MEM
    Dim o As _Offset
    m = _MemImage(image&)

    If x1% > x2% Then Swap x1%, x2%
    If y1% > y2% Then Swap y1%, y2%
    _Source image&
    Dim pixelbytes&: pixelbytes& = 4
    Dim OffsetBITS&: OffsetBITS& = 54 'no palette in 24/32 bit
    Dim BPP%: BPP% = 24
    Dim NumColors&: NumColors& = 0 '24/32 bit say zero
    BMP.PWidth = (x2% - x1%) + 1
    BMP.PDepth = (y2% - y1%) + 1

    Dim ImageSize&: ImageSize& = BMP.PWidth * BMP.PDepth

    BMP.ID = "BM"
    BMP.Size = ImageSize& * 3 + 54
    BMP.Blank = 0
    BMP.Offset = 54
    BMP.Hsize = 40
    BMP.Planes = 1
    BMP.BPP = 24
    BMP.Compression = 0
    BMP.ImageBytes = ImageSize&
    BMP.Xres = 3780
    BMP.Yres = 3780
    BMP.NumColors = 0
    BMP.SigColors = 0

    Dim Compression&: Compression& = 0
    Dim WidthPELS&: WidthPELS& = 3780
    Dim DepthPELS&: DepthPELS& = 3780
    Dim SigColors&: SigColors& = 0
    Dim f: f = FreeFile
    n = _MemNew(BMP.Size)
    _MemPut n, n.OFFSET, BMP
    o = n.OFFSET + 54

    $Checking:Off
    y = y2% + 1
    Dim w&: w& = _Width(image&)
    Do
        y = y - 1: x = x1% - 1
        Do
            x = x + 1
            _MemGet m, m.OFFSET + (w& * y + x) * 4, temp
            _MemPut n, o, temp
            o = o + 3
        Loop Until x = x2%
    Loop Until y = y1%
    $Checking:On

    _MemFree m
    Open Filename$ For Binary As #f
    Dim t$: t$ = Space$(BMP.Size)
    _MemGet n, n.OFFSET, t$
    Put #f, , t$
    _MemFree n
    Close #f
End Sub
