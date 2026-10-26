' Cut from Civil War Strategy (tools/screen/cut_game.py): DRAW strings, POINT(0) and POINT(1), CIRCLE with arcs and an aspect, PAINT with a border, VIEW and CLS 1.
DefInt A-Z
Screen 12
Call cannon
System

Sub clrrite
  View (528, 1)-(639, 450)
  Cls 1
  View
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
