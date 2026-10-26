$Console:Only

ChDir _StartDir$

_MIDISoundBank "./test-soundfont.sf2"

handle = _SndOpen("./midi.mid")
Print handle;
_Delay 1

System

