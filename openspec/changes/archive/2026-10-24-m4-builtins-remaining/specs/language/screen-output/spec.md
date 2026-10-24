# Spec Delta

## ADDED Requirements

### Requirement: _BLINK and the function _DISPLAY
`_BLINK ON` and `_BLINK OFF` SHALL set, and the function `_BLINK` SHALL give, whether colours 16 to 31 of the
text screen blink, as libqb does, and the function `_DISPLAY` SHALL give the handle of the image on display, as
the old compiler's program gives it. The forms the old compiler rejects SHALL be its errors (measured,
`verification\v58`).

#### Scenario: Blinking set and read
- **WHEN** a program prints `_BLINK`, runs `_BLINK OFF`, prints `_BLINK`, runs `_BLINK ON` and prints `_BLINK`
- **THEN** it prints what the old compiler's program prints

#### Scenario: The screen with blinking off
- **WHEN** a screen program runs `_BLINK OFF: COLOR 20, 1: PRINT "x"`
- **THEN** its screen state equals the old compiler's

#### Scenario: The image on display
- **WHEN** a program prints `_DISPLAY = 0` in screen 0 and `_DISPLAY = _DEST` after `SCREEN _NEWIMAGE(64, 48, 32)`
- **THEN** it prints what the old compiler's program prints
