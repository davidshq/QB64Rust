Dim sprite(200) As Integer
' Capture a region
Get (100, 100)-(120, 120), sprite()
' Draw it elsewhere with different actions
Put (200, 200), sprite(), PSet
Put (300, 300), sprite(), Xor
