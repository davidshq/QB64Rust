$Console:Only
$Embed:'./test-soundfont.sf2','sf2'

ChDir _StartDir$

_MIDISoundBank _Embedded$("sf2"), "memory, sf2"

handle = _SndOpen("./midi.mid")

Print handle;

System
