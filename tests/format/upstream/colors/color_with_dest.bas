$Console:Only

'COLOR works screen mode sensitive, so we need to check all
'to make sure the detination is properly reset by each branch.
Dim img&(0 To 13)
img&(0) = _NewImage(25, 40, 0)
img&(1) = _NewImage(25, 40, 1)
img&(2) = _NewImage(25, 40, 2)

'Modes 3-6 not supported (we use it for 256/32 modes)
img&(4) = _NewImage(25, 40, 256)
img&(5) = _NewImage(25, 40, 32)

img&(7) = _NewImage(25, 40, 7)
img&(8) = _NewImage(25, 40, 8)
img&(9) = _NewImage(25, 40, 9)
img&(10) = _NewImage(25, 40, 10)
img&(11) = _NewImage(25, 40, 11)
img&(12) = _NewImage(25, 40, 12)
img&(13) = _NewImage(25, 40, 13)

For i% = 0 To 13
    Select Case i%
        Case 3, 6: _Continue 'skip
        Case Else
            Print
            Print "SCREEN Mode"; _IIf(i% = 4, 256, _IIf(i% = 5, 32, i%))
            Print "---------------"
            Print "_DEST before = "; _Dest
            Color 1, 0, , img&(i%)
            Print "_DEST after  = "; _Dest
            _FreeImage img&(i%)
    End Select
Next i%

System

