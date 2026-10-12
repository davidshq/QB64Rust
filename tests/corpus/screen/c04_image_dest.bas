' The destination and the source are an image that is no screen page when the program ends.
SCREEN 13
i& = _NEWIMAGE(20, 10, 13)
_DEST i&
_SOURCE i&
PSET (1, 1), 5
SYSTEM
