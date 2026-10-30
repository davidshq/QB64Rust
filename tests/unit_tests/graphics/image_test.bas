' Which image functions work under the entry point's $CONSOLE:ONLY (OpenSpec change basic-unit-tests, task 4.5;
' its design's "Graphics" non-goal): one test per function, each ending in an assertion that passes, and
' TestDrawOnConsole, an error: PSET with the console as the destination is "Illegal function call".
SUB TestNewImage
    DIM h AS LONG
    h = _NEWIMAGE(10, 10, 32)
    AssertTrue h < -1, "a handle"
END SUB

SUB TestDestImage
    DIM h AS LONG
    h = _NEWIMAGE(10, 10, 32)
    _DEST h
    AssertTrue _DEST = h, "dest"
END SUB

SUB TestSourceImage
    DIM h AS LONG
    h = _NEWIMAGE(10, 10, 32)
    _SOURCE h
    AssertTrue _SOURCE = h, "source"
END SUB

SUB TestPset
    DIM h AS LONG
    h = _NEWIMAGE(10, 10, 32)
    _DEST h
    PSET (1, 1)
    AssertTrue 1, "pset"
END SUB

SUB TestRgb32
    DIM h AS LONG
    h = _NEWIMAGE(10, 10, 32)
    _DEST h
    _SOURCE h
    PSET (1, 1), _RGB32(255, 0, 0)
    AssertEqualInt _RGB32(255, 0, 0), POINT(1, 1), "red"
END SUB

SUB TestFreeImage
    DIM h AS LONG
    h = _NEWIMAGE(10, 10, 32)
    _FREEIMAGE h
    AssertTrue 1, "freed"
END SUB

SUB TestDrawOnConsole
    PSET (1, 1)
    AssertTrue 1, "drawn"
END SUB
