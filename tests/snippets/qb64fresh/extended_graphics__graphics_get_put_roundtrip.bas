DIM sprite(200) AS INTEGER
' Capture a region
GET (100, 100)-(120, 120), sprite
' Draw it elsewhere with different actions
PUT (200, 200), sprite, PSET
PUT (300, 300), sprite, XOR
