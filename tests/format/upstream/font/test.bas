Option _Explicit
$Console:Only
ChDir _StartDir$

Const FONT_FILE_1 = "LiberationSans-Regular.ttf"
Const FONT_FILE_2 = "LiberationSerif-Regular.ttf"
Const FONT_FILE_3 = "LiberationMono-Regular.ttf"
Const FONT_SIZE_1 = 12
Const FONT_SIZE_2 = 16
Const FONT_SIZE_3 = 22
Const FONT_OPTION_1 = ""
Const FONT_OPTION_2 = "monospace"
Const FONT_OPTION_3 = "automono"
Const SAMPLE_TEXT_1 = "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ 0123456789~`!@#$%^&*()_-+={[}]|\:;<,>.?/'" + Chr$(34)
Const SAMPLE_TEXT_2 = Chr$(34) + "0123456789~`!@#$%^&*()_-+={[}]|\:;<,>.?/' aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ"
Const TEST_IMAGE_FORMAT = "bmp"
Const TEST_IMAGE_01 = "test01"
Const TEST_IMAGE_02 = "test02"
Const TEST_IMAGE_03 = "test03"
Const TEST_IMAGE_04 = "test04"
Const TEST_IMAGE_05 = "test05"
Const TEST_IMAGE_06 = "test06"
Const TEST_IMAGE_07 = "test07"
Const TEST_IMAGE_08 = "test08"
Const TEST_IMAGE_09 = "test09"
Const TEST_IMAGE_10 = "test10"
Const TEST_IMAGE_11 = "test11"
Const TEST_IMAGE_12 = "test12"
Const TEST_IMAGE_13 = "test13"
Const TEST_IMAGE_14 = "test14"
Const TEST_IMAGE_15 = "test15"
Const TOLERANCE_LIMIT = 4

SetFontTest

DoFontBuiltIn TEST_IMAGE_01, TEST_IMAGE_02, TEST_IMAGE_03

DoFontMemory FONT_SIZE_1, FONT_OPTION_1, TEST_IMAGE_04
DoFontMemory FONT_SIZE_2, FONT_OPTION_2, TEST_IMAGE_05
DoFontMemory FONT_SIZE_3, FONT_OPTION_3, TEST_IMAGE_06

DoFontFile FONT_FILE_1, FONT_SIZE_1, FONT_OPTION_1, TEST_IMAGE_07
DoFontFile FONT_FILE_1, FONT_SIZE_2, FONT_OPTION_2, TEST_IMAGE_08
DoFontFile FONT_FILE_1, FONT_SIZE_3, FONT_OPTION_3, TEST_IMAGE_09

DoFontFile FONT_FILE_2, FONT_SIZE_1, FONT_OPTION_1, TEST_IMAGE_10
DoFontFile FONT_FILE_2, FONT_SIZE_2, FONT_OPTION_2, TEST_IMAGE_11
DoFontFile FONT_FILE_2, FONT_SIZE_3, FONT_OPTION_3, TEST_IMAGE_12

DoFontFile FONT_FILE_3, FONT_SIZE_1, FONT_OPTION_1, TEST_IMAGE_13
DoFontFile FONT_FILE_3, FONT_SIZE_2, FONT_OPTION_2, TEST_IMAGE_14
DoFontFile FONT_FILE_3, FONT_SIZE_3, FONT_OPTION_3, TEST_IMAGE_15

System


Sub PrintFontDetails (handle As Long, testFileName As String)
    Dim image As Long: image = _NewImage(320, 200, 32)
    _Dest image
    _Font handle

    Dim fWidth As Long: fWidth = _FontWidth(handle)
    Dim fheight As Long: fheight = _FontHeight(handle)
    Dim ufHeight As Long: ufHeight = _UFontHeight(handle)
    Dim uLineSpacing As Long: uLineSpacing = _ULineSpacing(handle)
    Dim strPxWidth As Long: strPxWidth = _PrintWidth(SAMPLE_TEXT_1, image)
    Dim uStrPxWidth As Long: uStrPxWidth = _UPrintWidth(SAMPLE_TEXT_1, , handle)

    _Font 16
    _Dest _Console

    Print "_FONTWIDTH ="; fWidth
    Print "_FONTHEIGHT ="; fheight
    Print "_UFONTHEIGHT ="; ufHeight
    Print "_ULINESPACING ="; uLineSpacing
    Print "_PRINTWIDTH ="; strPxWidth
    Print "_UPRINTWIDTH ="; uStrPxWidth
    Print "_UCHARPOS ="; _UCharPos(SAMPLE_TEXT_1, , , handle)
    Print

    _FreeImage image

    image = _NewImage(strPxWidth, fheight, 32)
    _Dest image
    _Font handle
    Color _RGB32(255)
    _PrintString (0, 0), SAMPLE_TEXT_1, image
    '_SAVEIMAGE testFileName, image, TEST_IMAGE_FORMAT

    _Font 16
    _Dest _Console

    AssertImage2 image, testFileName + "." + TEST_IMAGE_FORMAT, TOLERANCE_LIMIT

    _FreeImage image

    image = _NewImage(uStrPxWidth, uLineSpacing * 2, 32)
    _Dest image
    _Font handle
    Color _RGB32(255)
    _UPrintString (0, 0), SAMPLE_TEXT_1, , , handle
    _UPrintString (0, uLineSpacing), SAMPLE_TEXT_2, , , , image
    '_SAVEIMAGE testFileName + "u", image, TEST_IMAGE_FORMAT

    _Font 16
    _Dest _Console

    AssertImage2 image, testFileName + "u." + TEST_IMAGE_FORMAT, TOLERANCE_LIMIT

    _FreeImage image

    Print
End Sub


Sub DoFontFile (fileName As String, size As Long, options As String, testFileName As String)
    Print "Loading font "; fileName; " ("; options; ") ... ";

    Dim fh As Long: fh = _LoadFont(fileName, size, options)

    If fh > 0 Then
        Print "done."

        PrintFontDetails fh, testFileName

        _FreeFont fh
    Else
        Print "failed!"
    End If
End Sub


Sub DoFontMemory (size As Long, options As String, testFileName As String)
    Print "Loading font from memory ("; options; ") ... ";
    Dim fh As Long: fh = _LoadFont(GetFontBinary, size, "memory," + options)

    If fh > 0 Then
        Print "done."

        PrintFontDetails fh, testFileName

        _FreeFont fh
    Else
        Print "failed!"
    End If
End Sub


Sub DoFontBuiltIn (testFileName1 As String, testFileName2 As String, testFileName3 As String)
    Print "Built-in font 8."
    PrintFontDetails 8, testFileName1

    Print "Built-in font 14."
    PrintFontDetails 14, testFileName2

    Print "Built-in font 16."
    PrintFontDetails 16, testFileName3
End Sub


Sub SetFontTest
    Dim image As Long: image = _NewImage(320, 200, 32)
    _Dest image

    _Echo "Current font:" + Str$(_Font)

    _Echo "Changing font to 14"
    _Font 14

    _Echo "New font:" + Str$(_Font)
    _Echo ""

    _Dest _Console
    _FreeImage image
End Sub


Function GetFontBinary$
    Restore Data_Script_ttf_118784

    GetFontBinary = Base64_LoadResourceData

    Data_Script_ttf_118784:
    Data 118784,69760,-1
    Data eNpcj1N4NEkYRs9XzdhOKpWerG3btjUdO722bdu4WkZr27Zu17Zre/DzlHmeFwGqOAmHZKvNt9iSGrkSxAUad9ptpVXP/faYW0BeALK943ESvLRiCOoskNt6
    Data jzjMeG+2HA6SBVYcSAbHNznKXgH1R0GwzWB8aAJUA/8A4eDY0QNtcyucAN0rI3e8NtQf9+kr/1ge5CJgzaF0o2ot/2WQ9YHuofHDjrph+W1mINgf/P3HJnvj
    Data 4T/eXQHc5cFZeTw+KlFHhiFIO2Am4vH+p69/436oeAD8x5LJQw/7suro1aDtGQiHk0P6E7vZEUuDShtZBLGWKgQop4BCELkivycgV3IBQfGkm5VYHzgi2AZV
    Data OA/e+PcNKDn6v+P//Sr8HGER5EoW5iMOzbdD/Tr7m7+hW2Z/+/dtv37BHIreU/3tH179oKr1fzUlIQAv/7LTDQCvRAcMp643ws+Dd4EQRQHH3U0uwgPvGm81
    Data kM7C6LzJgAIUvhO4+Lg3wT07Y7alyG67b2bYGP5W+SxpIrWygUsBSGQFwCDbTb2w697TIhfs82BISK+ZkszmZsrJJFP1WxgztXR2wJyzx95Taol4SrbozfRM
    Data qczmsyWljlputqw619c0pv0+M+7SwRa7bp7uuWq5B8SePuWeP+2xOaAAB6jHQfKjmyaCLBvjYYASKmjhBNlaTpBL5F/VotZSz6tPnSe0o8t0p95E76VjPapP
    Data 1z+betNqtInMkmZls67Z3NzZFXUtGanIj6qi2qg10tFy0dZRNupf4qXfnv5bWQuAUEEDhptk29Twj2pODc+pD1KD0qVa6431njqrR/RJ+idTZ5pNuzF5wzqL
    Data GWoWMvTlDaQGQdlf7e/2R/uGfd0+Yv+nZRzU41rAKHomZxDbtm3btm3bttHYtu0qtlWPatvt3LzB9VqffuxthVPAMf8u/Z36qwnrhLXBamG5sGRYBAaHeYFJ
    Data Q7ujXdDOaCe0PdoMrYdWRV2gPFFuyGxkBjIBGY+MQIYiPZG6SDGk8N1NwkN41WX7/0/eX/ABRBVyBy8VlALfQvmhMdDrMClYFmwKhoEj4HLwGjgGkYxPg99A
    Data oEuwRShIWEXESeRDNEn0gViROJ74DcksqTfpD7IQshfk+RTSFDuUZZS3qSSpTqjtaIhp8mjQtK607+gK6UnosxiIGEoZNRm3mMyZjpk5mY2YG5mPWSAs4iwT
    Data rJysRWxsbCXsdOyVHFwcJRzPOaM4F7nKuQW4k7l/8hTwfOX1473L58u3zW/HvydgI3BLUEcwURAlpHJppTC18BORSlFT0W4xc7FycT3xDQkhCV8JX0kNyWjJ
    Data m1ICUiPSstKu0hiZbJlBmeey8rLlsh/lDuXnFNYVNZSIlc6Vx1XaVDPVfNUt1J9qdGg6a/FpfdP20N7S0dRZ1nXTY9ar0ifSrzQQM7huSG7YY2Rs9M6418TH
    Data lNA0y4zCbM7c3oLAYt8y10rD6rN1qg25TYOtrO1tO2O7WXtp+zkHc0chx04naqcaZ2HnaRdmlz5XO1ec25x7vAeLR6uns5eg1773qI+vr7LvE79Yv1/+zQEy
    Data ASuB+oEfg0qDdYN/htSGmoQRhzWFh0SQRWxEmkeRR/VHp8YwxqBii+Ls4hHxCwmWieSJk0ndyZ4pTqlkqU1pImn16QTp+RmEGfGZeJn+mTNZ+lm47LzsLzk3
    Data cyPySPNG82nylwrECy4KNQoxRXpF34tdij+XDJaalpGULV0pKCctr6mgrMirpKlsrpKr2q+urJGpeVEbVydXt1ff0qDccK8xoImxaa7ZvvlxS0erZiuyLald
    Data oH2pw6XjU+fNrtJu+e5HPe297n1sfZj+gYH4QZ7Bs6HZ4agRgRHUaPOY8tj98aQJxomzyfwpoWmpGdjM6KzObN7s0uzPOeU5r7m+udfzKvNFC/QLO4t2i++u
    Data tl8LuE52HXnj6GbgEnype1l8eWnFcpV79Xytd91yQ2nj92bnlvlW6tbQ1v1t4m3l7eLtezuiO1W79Luqu9V7yXtzex/22fbvH0QfvDl0OMw9nDn8eUR/5Hd0
    Data /Zj/uP34w4nESdhJ+8nWad2Z7tkThAgQDvxL8GIAALQDOsA0CBNsHwiGugLRCBBogGYCOpC7ECbwFECBpzgZKA+EDvoOKMXLwP0CW4BBPAFAAlTDuUO/AXmg
    Data GlAKlQRooMaANVQZiIZaATrQcKAI3AbuX+49/yCkGoAtSYJgTnf1w9q2bXzjjG/b9n862wicbdu2bdu2/fqqYmfWiImMzBrWFJJ1izoMGfKsGY9j+D2n0ts4
    Data jWbgIn8amuk7W0QdqKZd0WAGopBGoYDmII2eQqupsHeoC+xXenb8IvreXkKz7Ov+iJ1rrsZck4YY9dg7mc+jf+3j+hks1Y/jELKYT7cg5j8WNdRlQ3QZZpK2
    Data X+hFnPsdqNPz7JFUiJsoPX6H8xD2pmKk0mrk67uQQVEU8Tuy9Bz7Bq1FFvVbq/ZCgsqIn6//YZ2CTN9RUHQQYzL2pDZ+/y78zM6YohsxTn2AMHVjGY1Bqk9h
    Data d32ffYFakKT77d+6BW0qAqtb7EFkcYgql9ojQX+Bs02hfZFORSddg7AqxUd0LQbobOT7/sL5dCTeM1k40byB833XAOZH3K9n4niu+8dcyxP11Thbh3CuOpX/
    Data 6Wd7l/kaSepV/K46cLJOxr/6P7xhCrmvX+JVQWAY8rj2fqn71uAP2BbuRaP0YWNIHzYG9yrB68PmoNPxNPO+0otNIL1ItOfTAnuW1H1r8M1FKvciXfqwMaQP
    Data G4NWItvrwxb4CRcKSy82hvSCOm0+Ccv/liN9E5YceA71A9h5WywzKnPCnCAstZH8tsHZXizzTGu3zTLrPG9xl63UWP5zS45/ohrsc15sHrVf0fFIkB1RO8cv
    Data lT2RWRXIvvDMpq5j+4uwvh+ZpoL5G2S619Olj1LLLfh0jBKWnko9N2f/apzjS0C17J/swDq2h3mx7KTsxTb5cXTxzvhlb/XfOEWY47kyK9KvHbHZD63Csuuy
    Data bzJf0ltv52XvNmd9T/wgmXHfAmR4eajE+LnqdYzUQ1CoHkKZcz/7V4IzWB2MMr0E+/G+zTBT7SfiU+IV3gyIT4lXiCeJL4gfiSd479X9yBFP8PLbnL18fYSb
    Data A8fhbjXbmcCYr2YjqD9xZum3nPn6NtZf4hbGMfoQfOY7AMuD+/HO5/DuFyPC70iifxAKXOfk+e5AE8cJJhdjTCnOMp3YjXfhUHMd0kwYPXSNLTF1CIkHm4vR
    Data wxzj+yuVQoFvOXvef/ZlM8E+C2AYHoeGQ0EAd5oMVzsYYZa4WmGImeZqjSx61dWEGXSzqw0m0Jmu9vH5mKv9KCHv2QDPjXJ1EBP0l64eoPZyznf1QMxy9nf1
    Data IMxxilw92JmGVa4ewgquHgHJoQ0RPvqQhKV8hNGIENo5jrBewmhHF3Mvn23l67nYDVkoQhufD2MaR73oQYRVIasWVvugHiE0Y5p7RxdzI8c9HDWjic9EWTex
    Data DrGOoA3/c1LWam7FQBg9j6IqHFOZKsxMvdYMFz7rGl8+cAKrZTRr5ieNNLT7FW8JfKC2Nkd+S45wj8B3uxNTawJdWj5DpizjzdRE1kQ9RQ5YuLJhqo5AlPcR
    Data n/zc8IBwrXw2/rfQHXMTqVRUEmnRp6L4X5XsnrvasDOBfzVPqFiJPVRH7iRQmOiUFYWOKzYE+pgcSyLJBA9YsiIxYWd/Q8R8CLkjQsYfeHRRp9WLTJOcKmpw
    Data b6xe2DvSZXA1UbsDO32U/6ci6bVWTzKPMXafmsMGcyP5eShTsOsYi25eUlm1dnZOQ7xJOjdFlbrihDNi9d9doe/hf58FCffHLufciTJDsRbu+/kuEgUmiJrN
    Data ZM+QlKnRkdX22JH+nTV9bWlo8eboFHquIn0mfMnmL/AVJwATonXBBJqPzguiKMF7IFwZ9T6fGTI208jy1DPeoUPvNEXHejPu/8zyeqIv7LeqR4cuvWMrx3vF
    Data pWbnzI6ZuLu36HM7w7h3fh52XOY2uJ3fVipLprR2t/M78z2RQvyflFlXettGEF6wqbqqC3Iy5ESVS7rbKq4Ay+c8uRtIXhZq58gzTjOQcoDcLP6HAOHe2PDP
    Data 7D91Z6HC1ldybsGjEWPpWMidspjn0UncEQ8Kkv+eS23jL6n1/46a3PTTiOT586gpT2KfZFfRbhyTTPWTY9mCCERyQ8ENMGAe0SmlaUIy/Txy0JCuTSu6p+ie
    Data 810cx76YdhwXsRG8YmWCA2ngbfQdSiN8EclE2D4zngldIIsna+BVLdYIISv9I/H6h67XkVqhg0vOGrV1R/2UEzouYhtf8xPyUcM4olTXOenBtm6zer0vXgLc
    Data sOI5Qm3hn8oC4CCWaZVeQpqG1JEJDSaV9aDIK5sCzqbxId4iExwjFhi19SBNCXlIY6fJCDDGfmGP90iOEW+IKEMn/xx+qCszZo574oViHp15nodQHZm0man3
    Data X0eGHDyez8x4JkhdNlFp+y3kNmWl3v4uY9pCP+MoZJSqaeD1ImJ6AJHl6sma32yCNmNlui3TOx2ZtURD+pAWJ7us/2f8XOmr2YVxemiFzO6It3BjFPWi5vX1
    Data pUuW7iJ2Ry5bI9XcH/YRrktoFv81+ug94t3skreArK5YekDDDwmgpcluR67a7tKDjsx9ZRWNPAJj3mYVs7hOXRrqnKH6Z2k65CEnh+JxcDbneQvzCLBgERZ7
    Data iJeuwrx9knaZ6EEKL4vlKjyN1knqHIBE4nRwnryIzqtUI/+8ulFbjYOASSZDTMCIzAMn9RC77XRk88NSDd0xSy1Mjp9HEBIf2OmYgpYgLg4lD9Bmhp8B0sZl
    Data 5MtR4YrVF0Nw2rP6eoI39LBLCHmq9yo+cSB8bsYfPGIPlrQcInjaKMrhB6hyWdXImwOsDXio/rXLK6rXzCpFb8zrqEsPGJNdKZVw9qF5jXVIzz78a7qYmqKf
    Data rKOzWgQLxw11em9CAeOe+5apq5UPcBYfxN1sxptvd2StVD//WH3tU/ZXOb8h5EI+BzjL2Pe5rlzB3v/+DT1hur35ObkK3LSiY7JhZaH91by2rSy2U+Sug4Ea
    Data v+Rgq7oyA+pmOU1o/3jaGCPc5d3C2x82m8QN5denbvhrg4YUMRKIzTjQH216My4SYavzIC3Ada19XO6Wltvkol7e/bTCllbYRw9okMJbYbNjZal00FZBfCA7
    Data QmtAHTVT4RqErpXlkn0dQs6+AZSzbwLl1FtWVkrqbQg59Q5QTr0LlFPvWVktqfch5NRdoJy6B5RT9y0+Doo7nOyXtzCDR/Ed0P+n5jRHX/kWCt89eJX3LNaD
    Data dQQAFACw/Hd2bdu2bdv7b1FnikQikpGKdGQiG7nIRyGKUYpyVCQkpaRlZOXkFRSVlFVUqVajVp16DRo1adaiVZt2HTp16dajV59+AwYNGTZi1JhxEyb/FzJr
    Data zrwFi5YsW7FqzboNm7Zs27Frz74Dh44cO3HqzLkLl65cu3Hrzr0Hj548e/HqzbsPXwTBA6wYBgAA0btocWbbtm3b/rZt28ZQ27ZtpbZtpm7fCyCQIAbTi1LK
    Data mEI7+yinnhq6MIDeXks1GyihhROcpI4OKpnFFo7TlYGc5hRn6MkQFjCPoQQTQiOhLCKM+SxkGYtZwlL2E85KlrOCYURwjCbWsIrVRHKQw1QRTRQxxBFLPN1J
    Data IIlEkkkhjVTSyeAAmWSTRQ555DKeHhSQTyFFHOIIE73O673BG73Jm7nEZW/xVm/zdq6Id3ind6l3e4/3ep/3+4AP+pAP+4iP+hhnOefjPuGTPuXTPuOzPufz
    Data vuCLvuTLvuKrvsZ51vq6b/imb/m27/iu7/m+H/ihH/mxn/gpO9jpZ37uF37pV37tN37rd37vD/7oT/7McEb4i7/6m7/7h3/6l38bYCAXuMgudhtksCGGGma4
    Data EUYaZbQxxhpnvAkmmmSyKaaaZjqTzDDTLLPZw176mmOueeZbwDq2s5FNbGYb69nKfxZaZLElllpmuRVWWmW1NdZaZ70NNtpksy222sZIRjGWccxmNGOYQzEz
    Data qWAQc5nKNCbbTi3d6KQ///IPR+lDM//TjwZaaWOCHXYynRnXXKWyLnTb54EAgEsF+1vTPUuZxGUc84Se66XuUkdy4tHTf6nv/s1NkN9Z65EzckbX6/UG2kRb
    Data aBvtoF20h/bRAdjooF20h/bRATic8OFBSBsbLkA2fLPqUzHh4cM4jI364AIMxlJb6Q7vkfoOZB5Xxts4FVIqk1bkKWQTKY4lt8Akqy9SPsWGCmRTaKg8lWle
    Data SJ1CPsUxFMimUFF5gjkZKiRDzfNa4SmszmV8OAg8hOQQLEidXR6XFm/ClnbZgy1THW0V0x6+xE00yJawifbwJc6swcJyVdD7YEV67PO4uqZTffw5hFYpEwmz
    Data 1ZJthHSpYpGnuqGfi8iBbeCCIk9pk21firIHu4R8A/mXNN/Q/EvIN57i1IRFZUJ+hQvHILuChWPP+dXOmVBYd4iES89jemK30NdC31va19K+t9DXAneQlXiC
    Data O3J1Cbm6e1otpdXuoUzqKd0fX2OaPdgDvEbn4Q+4lQPZA2zlPOUHq01Ydsfn+cOfDR098Qd83Q4Mnsi0XyR+IfFPHrNX2PXXU3nNv3V/TyEfTv1XuVDe6h3d
    Data PCGH/7KcSCXJnjujG9N6E22hQ3QENqboDGzX0QbY6aI9sFtHG+gQHbGn0Irs8r6AJ1jqy3P2tNXKqkQnZ1//IvYCH/zxlKbOxgHZo/Du/7ko/C+Ggv/v2O+y
    Data czNIAWtkACVfAd7GlbU9d2Y0YzEzs4UWS7ZkWWbGOMbEiUMOo9Mwp2FsQ6WkyabM3cIWsszMzKXl/beQtttm8t87Y1lJvu4HT/1Y5MZz3vuec94D47Lnc04B
    Data hosDGk8ilQbpVCqdittjWo2KogFNUTTltGOaGHw3rdHEY+lUwumgacrh8Xi96E34P3i8Hgf6QZr2pBLwKXxGaeErSqX5fI9RDVJZv1cZrCX3DWlojZr5fC7s
    Data UUfqPnocuK0VA6Ge9urdPYuaI2qBUOdpTwcVzmWvrp2h54sVKv9Q0yz/0ha7uUwUbzDXkvTgsst1crmiZay1iQy9vLNBEahVJmpaPnr2s4eySj/YKq4Jz1h5
    Data z6muRB8lUmTn5DacuDjPJSLUZoHGWOMdX5t+6nDSZNRX9Vx69/z+pNiiR2hcuv4O8TClwLRI+RREALOYBXxMC7RkWaCgIgAGcIDhSQzHUxgAQuAkns4GgCOZ
    Data UCqg/amYBmFF0TiBMMAhitx7ahVEzwM2RReOrM1X29J+mbN8KHtsv0QpUP/aYNZ4B6sNNdHw/DzYffjzIATE982Ul5WVyWKZAyuZD5hzrqw6yLxQ2Z5V2ERi
    Data W649kxDoMRwYyb2kjToLr7YCXq1CrwOY06Gr0FcIBeiaKXjNEgIADKR5OElgWBhnr9bjRRc2fWnccVH2//A+/o5YSYulkuiwJRNUeExWTdydrGByn/QuuVer
    Data 9C2eF7lo4vPLFBadyWVpbFuy4BPfhGgvv36V9xylxJqgQpxb8Dc3A6y5rbmttaWhLp/LVibjFeGA3+N22EwGvVYpFwlpCmsCTQpoV1guFRGkmIDHQUILfXxA
    Data 00KHRIjzeBTAvIIynKIkDlyGo10wJw/azXMkoXVKJwHN4p7a4UM6pY2ni0+UqVQMHhVReoK+U04nXXxCUMq0k8Z/bzdKBt5ZZa5U18EnzAdVtkr/+y3Mv9Hj
    Data e8e1fUsln/od9/D7kLUy9oWev6GH76wKplre6BFY1LV+IGW+1mGZehrw2XP4H9jv4PNV6YChnMlNPykP2HJ/QN+Yr03is/7wBwxgk8y/yIdpL1TRMwqynu5C
    Data TcDv9ei0EhEOaIhNlgY4lcviPBAFBI+wYxiFpSFzeTjgJTGSIDOQvwgljKKEDozHI9BzghARzjJEENZ2itawJjsgKdKIFKwzq1VaDctx+LkEh46P+DLNGzUN
    Data P0bhIRHGvR6EKeSSNp0iBb5VuurmS60j6+PmiDpzjIf3Ht8QmN1qa7W21SZX311vcAW9VFmZxLHBXmZRe13d/D5jV17ti7TOdPd3Tf64YYHOFqJFRpfKOXxs
    Data /He71h5Vu9Kq5MHtXQO16fWXxjbEF6V3fmFXZEHZyfkKmTHhyM0Jr7d2tBsq+OqWzk1BqVETWj9nbH7VvjvuPPL9Jg0xuBXjYXfDXb01lAZTwGonibXCOmSo
    Data YGlv87j7Z7SNtI+0NtdUV4TdSU/SYlKrRAIciCC6MQEgMZwiccg6CCy/jOKjZzwegCgSAAC4AgXgGiN8RQOWfRQE1cVyLpXG0imWSzQgFDBwcijFUYiV4LxY
    Data GmGHQqgEhxETeBJ5fPpTdBbpW3/iVNfitKmCeDDZvyQ696Vr5/9+aMX6cVfbrv1PP1Olsma6mfcSVTPtAh4tzyeSMZUfn1RZq3qiSye5jyOLV195+eafSGSc
    Data 5k53Fbmzeo0tuP3NaxbmL/ivfHZ78+GHz124O7l99dLZoH35fZG8V2p1186MLP3iNSyxfc3SMWcn96mzY+XNn6NqtfH6VfIopYLV3cKCZniwt7umOp2qCAV8
    Data FrNaKRETQACRbW4COAB2DI09+SAJGQu/4hhOwq84RvLgVxxVtQRycR/al0S8LSuDHCYIyHkvRtMi2ilAHEaZiYIIQbJOh2AEmXqKoemUF5KXAzeO3kCU9qKo
    Data h3CeIq/HWzqBkjPAHybeOXrpxEynQim1+qLtIanJEdi0Orhg3DY2bG/G8R61Xij3x7XhAW/fmDUZdPdMEm01xz/zwpz256rTy1zCYO+axrMPPjzcOrFgVsLv
    Data kT+V1fDsi++ZtTzRn5S7+SqlxGSI40SiN772ti3lEp+1oX2OjoKLiGGBUlsm7anXJ3GpNLMxtnpR74pfPlKlCe32r96zcYmnf0Hv5OaNtwXL8/w5Y06IPMoR
    Data +FY2R5gLAoBxmYGAaAsxlBhw8D+kBdD9fwj0GMB+z5SRP6atmIv7fVoN5gIu9vfhAAMoDgHu97FegLIkTJdylXYqqjhQ5JApFOyFkI/6g31u34mnN4UXNteO
    Data KkUCXdPswS5zTc3SI53HP/UkbbWpnDUm5+Hr2Ou3M79mvvyPVx+tFpm1fFV2ycnBV2DtmgTqdxdd2YUWnK8nmTLCB6/MwV2ZWoU5gOMTkCAcSS7iKRXyhHcK
    Data lZgF12pkOM4Kmov+UK/b9/t7v9dUM1spEmpvuiimjLuoHwMfLG6/+7crDxeKlzTwKnM38y3mr9wlAXgOJHGNUmMZLIO0hr+cX4ZlQAZpDVfUgWO4TowTAMPt
    Data 6BvAMeQTAEekx3E5cJIIRg/NcdPhhZeKDoyC18txN51CsVqLqMxxmAUcvYBPie+e2BOfbFb4+NVOXyLc50r57KZU3y6rvt4htaljoahY7mrrsPCljhnNiUUT
    Data IYeZXEe3RdZUVJpo2eJszuRtc/iMyUpLLjc41hXSeWPGupDy3km7zOEUueX+JSc8bRq+x74A+f8x7F2oVf6CWbF8QQEwo16tlEuFfB6BWYGVRvZiBI5BMz0k
    Data wHkAmYtcnCBQGIXejh5dyF63IymfStd2uYcNpnI7l5ztcvgdX1rBnGvU+D2gGdeVO6JgZ0bjCzPP67zvZsDOqEPrw0GzR1feyJyrcGrLmefD2nLIjWvMb8Au
    Data 4MHUmPZ5iRhiDS9JwNICm2IFjNaIFckEihuc0JPgezuy6sgsn0Rp6J6ZUbjduQ6GsQe1sbHWmgaduyas0wR2TczuQQg8hpsIJf5DTIz5EfvKeJgYiBD7dOxi
    Data OrQcfkPWIjdB1iJlqeQsnTLw9Upzwgy+GzTH3bgpWO6swcv9zhr4s9Hr74CNwIdJMemnKR4oCzyHrhrAfyB1iwRt58SmMY/EZuYWQQlxmAv3mc5DH1FCHMTC
    Data /4ADcg/AcooTCpi6Y4mumxgb9xnr5kXa5c6P08zP4i6Lez8leLg5uNgbXWvIftiA9PV+mAeuQ8WnwfxYuiABmMNu0ImFmAZoePBXQaITOLBDegPEcpRLpzhA
    Data ghuEnCcZSSYUqSKrKRzFKi7kEzDGa2J5Ep4U+NI9F2esOTvyxe/9qlYjblp7evhz4LEdG0/uX7hERtWtWzQxb6TT0ib4wsv2TSD34c97Hvtpy+6HmHfe/3nP
    Data o+S7D734+HWMufra2LLs4qMPvDD7z0cRi6+/TzRBvWDHfC/qtGIRDtABGvjIKe0oXcGzQ7UBvGr2ahFh0yxeCvgfim5TGYU7EglB42/Y0qJLya/ZenbbglaB
    Data 0tba7GiYbE6pQqS9siP8L5neqDKBTYdw44WZOkoiVMfMtpipcpFOGZh/90s/WHHtFcQtNUS1DmbXNmykoIMauqmhLpUMBmxWqUTAp3lYG2hD+bWqFuA0vFAe
    Data iZJpkg8wmsRomGABjSP+keS0KCwDNF3UMyLAykJ3jMuNHlTglUxJpTinKKpDtWr6sxLxUNxJpeCLWB4n1sON7uqFTc/O6Y76lOVmk8jkXn2g47e3x1eNmoZH
    Data rIWIylk7K6P02KpnJtf/YY+v448bNn/2nmznghrnrJW9ccfHa1v08UqDTCc2ycQWx9A9Z9t+eaTx9MYdbmHc2z2rX8WXuywZhymzZ/H8IXtLLjLr2J//sW3e
    Data XTv7HDRBlKkNaoRZP8RsEcSsC+stKLo6W1vyOVTO6nViEfROiFY6ADCWiwCdbJIGGA9gSIBQgEfFMQCmVDM8bIoikfogSRHJKWh05HkcAfM/4KKZlnYlJUix
    Data uoWTKTIn+L43Kpt9Z25/18b57m6EzUhG5XXVzq7c/Oc95W0/72kbsAkc+ZHMpqMne5vC6aGo2u028hUCoWH5hHfGCnyGpSkqMw7mjz7XNRz39g1P4WPSJ/Yt
    Data XTjibMtFhk/+7TsdbsOka/byhYP2po7a2a3VTWp3wKlwZOxic0NyDeBBxNZdf58MQsRqsPaCrDqbjPu8ZqNKIRGSBIrg8RiO8zADhAW3YxSkGAV9ASJNoJwF
    Data 9TFG4KjQ8EGYIGQ4LsJZYQxUxFTZ7PU4kQcjMNiUlWbJ5vXAZ4obVDCNOg8o+rBvkZJ93nKX1KEyRcI5sWSyakufz+ULDn4ms6zdJ7MTmr0XNGQi42pw5bp8
    Data K6Le4T1bQ3USrUJeqEwTH6xg/h+hlZeHZSQNfjL+Hbc32zHnizKNR5+sDb4Loh+6TKn1Ow7f2fRks4Y/+XB7/2DAX1SzP4dRrA9bVNB0tDfW56urKsNBrwfW
    Data CUoUNpG3tfQAPo2hWsGO0RSfovlJqGwpPgYpROIkD3URCBIn4mUAeRyEBcU5iJgExTyACIV8j5OzJcbcRBUacQvCMe2DFM39AHxHUSRaLIU6M3FO0iEOIp0A
    Data Xl/xqXWB2f39R84+emyoO5FsiyntUmNww/y5g5XpheZCY605JVW4tH4Sd+1s0MjFpprawLpjDQml06m1pOxq76r+8MJhzVzi29Uu3+BTFx64vHh1f8ea7nyN
    Data WKcLNlRvXTB6+MCG06YyZ9ySEhkjTkMHoJ/sCSlDUr5OIOPL5HhdvNcmVIpFOs/6F1F2OAxxPQNxrcBqsUJBUqhJJ8Mhl9Og59Ool+G3AZKHPBIByENUAmAK
    Data Mxa/UhlLcamvlCIQcBxsyCVZGpXyIouKgvW5eKnAop5yFVafGXj+hZ1N+VSis9rv1h/yjS9eONc3MzDneHTlRPnASPft23+x50KlSGKt6ezdEZJIpfbtW750
    Data 71vA8u/vtd3b2fDN3h4R3+MIOUUNo7ZwcvaOtjOPrfOIA/6xyw8f29Hd0Q3ef2V31pDr2vCF/Ly8NtaH2PUTtPdKYVgE8xUEVotBr1GJRUgTavlTEYm7r62c
    Data ANMtqLQnj9fgqWKO4UzlYghnJjx61moVwuGPYvmc/T33HC64X/50piOvC9l0Nk3cOfbspsgAFAyqiKEl9timdaFxktb2FUac9//zr6t++ExPutctFIu0UWtd
    Data Bw7UYqdarBXatMmGlY1/ff5LC379ab84aNahjdB58BwXwzxpwMqxFOZ/sSKsVtI8lCuNPqOEQEUel9txVOF5S8kSsMdRqr846QoPScbzsgyn2Wiq0RLFQg6d
    Data pxf/7eqdmx98cqThZw2dR/bueOWJ+bOYTR1Xs6mZXZX5OmPTWxPO3jpjttqAP/PSsQeWLVjau+SzhwLmr3/lG4f34/jiOWeZZ15YvfvD22uq+FR9y4LZZ0LM
    Data V7q/uj8qMxkqOxtXOxHei+DW93KWmwXYdVFFI1WVkUK0EA7ZbVqNXErz2JxRDiieCuAU9PmiYqfQ3XqoWUADHq+UMwAo5Qy6qNQ4p6aoKSYqKa5FICtVZSrU
    Data W0SHDKbr2xhydNkes0ETa4worPIgKRIqTQe2bWurfeLo/PKRV9Yea7qw1dsGLK+deHns8hg+0Ne/JSgRS8xLR3ob9Mmm9rZaXUpp01b3WfjyixeBpLrN7h14
    Data FDi+vjmsrUmM38NX8BkJ88TP+nxO3pbe235cv7hOn/F3LKnZC9IIm8L1d3Av3JuWYTaoG7UafhmnG2+q9yiP9yYxit3aHd1nz2sjxV5oiDl5Y18U303oNRXL
    Data SnL12lO3dkORhseD7FV4MMuLVotSwS9DnJOiS0yj42AvhvjEi/kkdQ+OOPKaaFEwh5gX/ova/y+X9fH6T5L/AM4tZ5KHyLugro4i9S8SYkqgRBdnRcUOsOOA
    Data i2UeVIVy2hc9uMhipQwAVx57AexAgBQ5Fkxfa8G/5ZQI9QXdx7/orSY2KMm7qgo2Zhkzrgro+WoJuBJJ2cCfBcj8U+At8jwpgNWHEYsXFFIJwDRKiVFq4OoQ
    Data FF31PMhXAtWb5RgARc0tmWpxT5VYXD1y8ytyESqxPn4VVV1EQ+k5WVbh1PiiDo0fZ1ZPP0WkOI0NkefIeyEakZvRsPCQ2fb/UBGxchr9TgC4w9NCUGhAfSGY
    Data xl+8lnJKRLqCjvD2Vn98UDkEoQCnwQUIhUAlYeohFIxWAJG4CGNTP/TjAJbFagqSdCIUtFs5MY9Q8BX1HoZB88mbdR5J8tBzHk/EY1MMBv0QNfVuEHrcKXHs
    Data KvaN2F5pkWlOLrqBT5u89kR/6tefbVvcpA2ZVcrQoE+iT8XXPn9waOC72x7t1zp/yvIuUUsXKgLgvCG74baJimfO7mtoc5vSeX+12iVzmbX+zTsT62bO3P3a
    Data +bFtS6yVFzkiZiPhxCCMxAuZq7SOtxbrx+ahvc6Cbu7Y0EBnR0tzvjoR95dbLTqNREQAIbS8oRXqlfkzqwiMD6MXiQGabazDSgDKFhr1OOFRUYCHY2zqxQmC
    Data LRpwnMMHPbhQIANFQYKcyWmjSYKNUnb4ZlEDy9mAxgbOYvpFmNxYIcHDnXJPGqYtwoOUISd2lCk6oia/FK3tP5kzHtr/N+bq15m/d+lyhcNA8OgL7l+su/rR
    Data 6BfvOvmvd4AYND51edGeXzROFHQJ/tINgcFOc02gbV5m8vVsLCCyV8i8yu3WqibteHX9IreobvuDWhNIV+SWXOkPJL67acOWh5nLy37+udYLtRL7zLSlu/th
    Data EALWd988cOD+M8xH6icf/W2rFg6m+qzlD1aqnLamr9y9f6W3b/2EkJBJrG7m+uCnwMjhtrXPVVWuBovjD6P7hfvgxCML2deCzcT6Cqr2NqQdKzPJRNjvcogE
    Data JM6HJ5HqBQTeCjBCDSiMgGIaNT15yTJAQEpSBKSnjysyuZ4zzdVoiJDuhJLr2nFJFMlAyE6a4vT0tByiIKapOFc7o5+lijphuvtJoP8RfVEE/OE/df5rp9eW
    Data 3zKvvb9gF5H8dS01NWtqcq6aXODyE8e7n3g4rrBo9Pc7ZWLN6jJca0hua1BaK10Sg0AtcVuahvaHZF9/9pzJlrdZq77/rYDvtf1oONVc2PJ2Z84vt/ICQ91r
    Data v8kXWizGCqkASH9+Ir1BbW9vYjoWAw3tjshd1F+fqxHSIsoYrNZlNvrt+JCWHOi/v6NhS4r5XoHtLIxCv/42RDaFNcH8rGhsyGVTCZfTZNSopZIyGmXnyiCA
    Data WKHMjHpNSYwAkLTQw6E8BzTLaRwCjHIyW9PxAD7l62WswoLoKaYoSdGlqpcjLYTafUNZB2t7hCQ6jel2s5eiVqtJn3/8IXNNtbZKdXTktSeu7POt2ZRZNcsQ
    Data TKi6FpwExuQzL2Um1i97pLeXTM/e0nD4+Dde7CDx5q1709tJYo3DW/HIqcyF3+8Evte9JmXE8scXZDsPf22/W6hV+eqVWnld8yILryo66/KpE7fPcO06vW1h
    Data pz6B91050tCaCi/dtzkoJvEgBliF9iuIVgZLFUTIELNRLiujUA/GgQGIDyBQI6OkrgnEPU6usE0YZVqhLEW4ouNyhQjnqzQ93WrX3lB7zAPCI7u+/MXPHOlt
    Data XJZvsldGFG7Z5ax21om7jo8vpMAcx5x+f8NSm1phbU9tf/tUfCm4bAYBIHjnzKnx/s13HOtzKXQKb15027ZIePy+k7/6w6uePzTpdWrngN5mcFT6MrpMNj2J
    Data skwD9DUbtDGH4nxVpiLs9dhtahW/jIvzkAM4geI8kmnxm84egJvjfJo7+mI8IiS4WsX1mG6hQDplQxSQETiP9aMZ/Gx4yQmVNViflFpUz++Pq/j9izfu2rrS
    Data N/qlhjsXVcyplUfSmphA+Pw5kHi/88R3rzAfPXJsVc/A32rf2A38f7bxKKlIb2L+yWw4sz4gs3ubF+041ffcU/+v26iUOyoFJnU0VKjw9jfv+wPzU+avzPXb
    Data B2obke0wzpBPQtt7sa6CrLYmFHA5LWYVO2mhUYRJAZzLs0iAJ6dHLAT8isMDLw0HCaIERdn/brAS404bAcCF8lvnKBKcC/ewwa9c8Ll7Z7kVapkjBOcoErMj
    Data sXYyNKH3T64KNRRsEX2sPbHt+L+Gbe6qFmdjaI1XFO6fbDr70FMDjYtrFlZGFT7DyCVesv+ZxZuyI0m5q2xqcsKLdEZWrkoaK0K2aIen0lZXU2MsnM9uXTra
    Data akzRcFiy8eTtk4FZg00rtu7oSPGUMrfFrSpsQMjlr39IfgiR68BWFEwAa2muq63OZtKJeDDgdmo1IiGJYx2gA00CW+sdOJ/0AR6/CWACGKh5fAGfJ4CBBcJK
    Data QFhh5uSxmRMTACF8S4Ch51ybjav62T4IByxqdcBQfIMvKSX49FAG8g/lQPQ5+3EKAc2J/3iM9S1Y1BNcqUdcgWlubKV+XWbjVoXbIlBLj50RamRqvUDjNERa
    Data U3MG+7y3JeI+QFgCYalNUh+eeeWR/t9tzvrlSkuZy5x9YbfdMmtsfkFfhePHAQ/ggoGPBl8O0mKBMiycAPcTFCXmqzx2qUn3MPCupe32TqlJatVFtwybEkqR
    Data RWLij9aqA6L6FcwLUpXCuxbaPQEZ+UeIaxvWWVA0N9UWYJkd87i5uIOicxwykcQBmcRIHn7zpO/mQEQDbGrSR//vAhE1PfIL49yPotfTM7/i/zQBhEe3fx1G
    Data pp6mZflGR2VE7pFdzmlm3XHvyYWLKMFKz+qF7i4cT2nidpmmqhFO93JOhc+V9diblwgrF16pf+hIbCH4lAVFq3+d+4Ro9cBdb/39qz9q0ad8I+OtJqFIXqEv
    Data CJQEyRerZveZC3jtxEtN+mwaonX9xevv8z4F0RqHmk0zOjxzRk9XW2tlOh71uk1GfhnXYWoqRHAk1CjUYiqDVpTRSfgaNd8Qapwy44GpFuUNL7jeLp/TDXyW
    Data fA4av5l5XDxDXWvUMSgGP04xIPFQ7MlB36dLOwE4149CH5Uq+VSa+PuWv26/8EJ9i8Jt5qulF54tk6hMQoPSOX8fz+fssEIK03el8tX6BVqBTGpL/vLPgfQs
    Data tbg5bMmQloi1Ric1CtRadbx+c3mgXO8SCtQTDa2LyiXlOvx3QinT8LlmPSSnKiRcC36Hk5RC6pJbTeDCE7gqIA/6PZVuMUHp8qbuZQ2nHjZcBupj/VrfnNvu
    Data 6H8EYkSWeQPKMM587qQhJVbw1MO58lGrqfZ2FEev//n6h/i3KCW69+lFk1EmpSlUpiixYpEGii6MiiRvCm0BJRFintQ0YEQJ2LInnl5ULvQ5260T/Ky3b9VM
    Data Zw9OLnnw99V1rJvO2oFLQfz9Rje67KFqr8QsV4UHTjP3XPvWGzVa1vswDDAXr32fWACvyYuFCgK3E+59iIV8EjVXMHZQRqC269SUkHUYGcE1VxJpF1AoOb+4
    Data tRdN4MWcxVz0Ag3Q/HJrVLL3m6pohTIU7lqQntw2Flg8tuDSX5iPzx2Ia2kGs0iW4+XgafDbK/3Wb6pUMmdrviqq9EoVof4zzIvMZebTTPA7G418iOKnIJuj
    Data bL+0o6ABWG9Pocbvs1nNJrmM4mF9oA+p3nB7TAzzD7CjNESQWHIq91CAxxM66DKcwxy/Qe3GihNjKK7wEnnRIUjwWw2kaK4jXyr8UTuZ7cFD/sZS5PcUZlPj
    Data PXP7GizZieZv/Unqtgq1Mu7EmmM2t9Rpj1Wb0/DeiKa5Vu/bE1srFhnLlBJDb60mkJ/Y13JHS751vHPGonr/ncGgKhLuXJBa/mX7td/lSJFAGRBNn63HppI6
    Data Uy6DkifU6+LtF5iHVq9+ZebGwpGjCa9Yp6KFTmP6ruPb1yYWx2KVx9bt6FZDBH8Ko+cuSoPlYPRUASwe85cb9GqVTAInxjmQQ/EzCp0bAFQtoDItyTbeiTg7
    Data VsXjaBOFS+QQSxKwlGWLNQcB+TktXbmGO1Q0xZIMoZnHPzG8Jog9+w4+eP7k94G2vGPQ1liu8SedPY/c27grW22XWDQDyxe0Dk3kb79/TWfTbY093hqk9Db1
    Data j/KOMJ9hZjGrtrfqSL0+NS+lUfHVWrFeaFSHzRXiMEgD25dfuVy9d3x4/YVH54fUeoW3RtC+ZM2PIA7e61fpA5BJo1gzwmFwoL2tqtLrRg1ebBSMIhwC9WYZ
    Data AY21o4kNpBeV5EQdtByFQXaqheM3kWi6AlUoUMVD4MgPZEjxoZgG8OLmB5R8nA6cLgNonEjV4Bj3GiZgFEW5Yoqm9AaHoXp/tzm/YHZ05NRrb39xRq/FGPc2
    Data LF+yc145w1yc19zXZBXx8/HY3NXVO57+fENd3aDOF1cGFWXa1BVFMrVmA1D+LVrnkTo1ipOJARnla/FJDcJvNoc98nK7vXHzbrABCDe9cTszwKSYM031oBZ4
    Data j374Xvbbn2pn3MyvGmNBhYWQu707Pr3qL6D/O63lFTKeSGyJWXLHr11l+keX/+41wLTJ5GJbfYIZfpD5/d21rajMwlC6vf4eFafUWD3mLQgyaZtFpSAJFGVg
    Data XsdRG5oAKLwAbi5LICS1Km59Q55IQ1ymFSCJ8EJoqdAOBSzrlQquz1bal0TEIiuAp7ZnZMhcJRW/fKLBQIaabUJnfmjJpsKxrQ0KlcwsVJnl3nKb5/TWWhmt
    Data lpeP9xnj1XOz6w766Am4vZja+ZttHwDp/s0K5uq1v33wK0urYez5X73zj5/PeIEi9aaG9W+ev3Bv870jM2aAFqB5+Lyd9ngkdp+j9pkr31n6w3/vVrHbsFd5
    Data syC/VNBVggWBy2E0KKA+QXYbYDwqyZDpCbWI28NQprjq2gZpgBfVL6TbVH9W4eU6GLeR9ZMX5nz790A2w0I3b7o051vfemHZCqCv4em1kbPnju+KHvMYjuy8
    Data 79RS8PWNgH/15z2Pvz94nfng6s+6H9+wGfz97Udm2xrmXf7pJOj91bknL72LctRnoGJ9D15zNVZbkGQz4RDS+kjpU8gbNIBHwKhAABgeeEloAA8DvFuL3ek1
    Data tVRxx0lZrHXRRhRxU7V7c2BVw3e4rhb1y+P1DrlZevYz2UXN+lS51GFS7Hjn/Km9oQ1bc5s6VRG/zJ0YWJ3d+PevFHbx961/6aHTBrFoZO9hmSEsXIErHwa2
    Data RqVcbCmXZkLMheB9d95zZ1hm0UXrlCi5VFW4xfpsbMHdsisntrXU4a9BU+5jzvO+Ci1vhb0UDYbBx9aG+hrYT0GdLZkEByivJOI3zXGh/STGQyoTo5AQL7+x
    Data w1dGc/uiKDTyaIcnT5VkpLq0ozRdz9jYJFLKIazwmV5jAl/b5LeYGhdsv+PI9oqJR3ec2D97rsacItTywEC7qSa37OzoF38B6AFLdsGelpMHB+ae3ry/2SJU
    Data 8QVGs7H65RObV+UOC4Q5u8mW23Txi0ve/Ppv92/7/JceG6nIFSitS+pJhsY/uPr7LzSd/Wjote9++bn2c02GytGOsKRMRndVaVPxjhHPMOTHV6//gYdDlNqx
    Data poKkBoYbq0UqoSmOH5EWLc6mDR6ixg1pg4Rf8RuFRGlUARJ2iMgndD20CKGURgMBCBNeqLZLeblEqSmliDooZAXz4QOPRNOxsKKuZyPQZB7/XO3u5+79SVW8
    Data Z8kMc15J64w1Z2a3VupCapdB5kj+8BdcOXPmxdwExzCjPGqM/+TyE+TLX1tgjuqVrQ0TFiqfXPbgVy4dq46MHP+SnCcBtog6Jox3zY3NizRWahOSa693IDGF
    Data SCd9BLgaEOny0eDOzhOmw3eg2FcBs+18SgVjX3VBlK0M+K0WtYpH8lDfAJKJoxOBUisSKSUliGGlJgnyJuX0PqwHSpSS09zU9IQf37pbW5qApT7m95q68ip/
    Data xNUzXudtmpfbdvWQv6u1foaiJiBzyjsy1dTgiUn/SJsVLtM2Vm6571RPIQ73Z8HtU1uvt82NzJv83P3Hdy339jck5h3etXaxia+UOX0mvq9r5iO9tenbLi7b
    Data EJ+f2PT5vdGlQ80zRvJwZxahsOP6hzwx5E0May3IImG0Y4z6xQI+t2UcVaMOigkQqPpAkZEAyLkwgoeVwgsF2PgynW0xrk6dJgVkxf9Q2ZIPdu6a0OnX/eRf
    Data 8qRdqOEbTmxsGe/v9v03BSyvTs2vHWQ2XisMwHNWuCjZGDjzLmh99ZMLVQxdL4z7d0NbG7FcQZavTiXKvRqVUECRXL/EpQAAKVNAohNOclbxAOcdBCeneFO9
    Data b7b0KZ5uKk1j7GEmE0olR4PEDXmPwNVcSzudxoqagsBJutnfKFCPFSabDekm2WLmym/PRqRHv5+DZZDIZkgsTCuczkr3uZMte39UU7MZUOWqtpjCr55z4Cyg
    Data 5ro0Qzm/0ZzbMPZB4jzj2+6zeraDHvD6iQaHQFVms4m0apl99vbTM5hnmS9MSOeA9nkrfg8+anbBjFr+PLOWaYN4NF3/gHeWaoUeUFUQ1dXCS1RIRIIy5AEe
    Data JyC53hEJ8UAOIHb8lzjBJZOp9o/TwTGasxARvNgx4lY7uORygygH3HL5EY27MDMyb1y4YRHMjqHFry/b02MKur2WBldLi6laHCwceaf1sVF3Vm2Veec1qAOd
    Data 2y7MYH54MDwUK29oPjS+7OkvfXv8m6dSyoqVLZDvwYHaFfm9jefOtVxc3RBs6rgPUAPfeLa5allKnZM6y63Vz3/zC8/3Ma8dis/Li3KIFWGoNr8JWTEH6yko
    Data BvrbWqpz4ZBWo5ChTXukNVOjXpyHiIGjaS7bugCodYFaOzdNjni80j44t02FCPI/4pPGQGlICalRmvcixIpLMeR0wECNfTBPtbBJG/5voRMtAQWQGRqQeALK
    Data oMD59UZNmci4fO/+HdGVbapsrn2+UxSesWbfma6LJ/bWVg7P6P5yp9qc8fc//J8RzYeewklAtvlpPl9tkYg8P2b2rj8Y08R6jlxZ/s9fPDRpicSernr1jz/7
    Data 6uNHY0sXjxx+8xez51/HViEFE4fedxDiXIvVou3Q6ixql+u1NA+rBbXI/7xxuxStzKA8hZMQ8iQBpriGFggAdcNtHjaCKsLJ7sHiyQQ390AYIlBRfcj5Gtdt
    Data A7QCRenppgUxi3l71Zo7n15UpSsUXOc7PnObVycI2Or65EKBSvzS1WeExvKkKqwgDJq66k8xHx4d3mutckl1sCsxUi3A8cfue0kZt8r2dl/52tNgRnltc6ux
    Data UiE3NB5YcMdeAEBfOU7DRWYrJRa5orcxD36eWbHKgFS3ZiDPbhc5r32fd4qNRXGYfap8XqNeKCAA8j2rm52PkmAq3RQ3XpAelZVC0E3xhCY4giDB7YKf/M8h
    Data CRC3BJam3L/LB5Zt3td0+oAhJtCpgOq9luom1ULmK78+HfukwATom8PK7ovXmpd+5YtPn83cpgvKyibA5/Yef6nybqZyt8/s2QRmgNdP1n9SdILceAvm4m/A
    Data XJyD6gUeXLnX6bBa9BBsmkewPhjUAIIXB4DGkYghaG7JjEZjHQCfcbEaL62PumGVy6rxG9YxgZImuYB909IMwgND7S53sZfzObdG3/nkE3d19s90is3akR1d
    Data F0HDP/YxH39lmcWU8Ju7vna2emk4l9NVtm3vY+p7X6y/AvDm3rQuUm5wBCVNfpXGrIk2VR9+aPYrYAKA+HxPhXt1mJjapvEa8i1r75jJVI58c1O/k88XG5oi
    Data 7HbyZTJGWzEnFnjRbkUDW9SFMkHqgzSGs7HXM90kxYujcqCCRRGbg8AN63OIEBjyA/QOeIh5f25CrNfgc/3a7Z8zC/XusCFmlJnPMAcVzspeh5D5Q7XVqjAV
    Data CtQf1+SMQpNUCGmedXYzAx9disxzmw3oZg24NUwI4PVJ4PaHoIz4hK1hgEAH8GLiEEhMQwhcq8yMTGBJDArBH1XMD8Nr6ct+nTMZ/uDQbBQR9sJ/8aVPtjiD
    Data caZ6ppMOhhctTthjGrZiATeOH2DFiaGIid7BpYDPWnztfs5iky/KWQw2QItnesTUH6HFcnOh8KF1bc4kQBYLVdBi8Bg5G1lsRLf2PIB/lUxCH1XAvqBMKijj
    Data NoGVCtY92X2FzA33B3JSGaBDABp2iQOEJT6nxC4Cd4kiERGYEOl8VRLmJ/gvVWK+To7/W6W6RsnlZhWOul6om0Z8fL3n/7Y7DtjtjLiba8Qy/643RnzkPSlD
    Data RYy5HPOa4mTUZ06ieNNOjoOlVDOmxPTPy6VlNDo9ETZlghg42fNj1amKWy6bWj4ECb7ZoU/MdA3i5nRzQklTxoc0PQ0pmzIXnVyfsVZq+Wjx5vrvyBFSCF0y
    Data i2ZRqWQo4LQjPV3UVikboIhYuZwAaGsKUGj4BK+LR6FlW65tTQKCkKAkyp53qXlVUlvemxYIplo0Wvghtz/Am9pJSBfvnCKaakNZGK/nzK+1SgWzD6W3jtpq
    Data KWr+6OHDb7c1JivNadlHNH7tJ53Zge3jVjWZVDTPGe40VwPBLBOM07V2bYfXlqj6bIspHho79uxj9y+cWLVKXW65xmxX6DYuuPPDrq4GmzAxsqXlbohB7/Wr
    Data xJ+pM5gLi2P5gszjDgXdcU/cYYNqmp1GepRotm/nFqZIcqqikDhQvx4HOGczV1KgZKYs9d5RIVXybpb0xdvIWENVvXKjfPXtDmGz35wk9F59vb1yML1yw1NV
    Data yvTYtsY7Dh7q7gS6sfFTcbmne2BslrsXmKqWLk+BAXD/X/o1gfENRzq/OnR6f2z8xZpXX7qwMzTW3+/p3/mPrkcaCksOtl78A7rmHLTvIXjGETRpRXuwTrtR
    Data L5OUUeyktTTXIXA0XsTxGxddb560KvNEaQm6lMRTXAnObcWxA0bu0wIen7m+8cSFy1k14TJW5xvtZiFh0dYSeFf/rH3pSJ23fs0SW7lDmzzwuU8ND4HhwSfu
    Data 3l0xf0/FhvneIY89KFZrG0bzYwd/+vJArEefsBuylbtnz8JwtB3Oi8ITi2IFOIExxGMAq6qMFeKFcNAGNzLlUoqHRUEUTWHqjIAmUiIcQwmIJMh0GcBoHo3x
    Data khhNcwmaI+9099rBB1wfEi9NYFg3Le3BlKJXTMsdNlc1o4Y1N2CBp1yis4ysUA9nl+T1mbLeqvWrn0mrUnB54I6/NT855PY1OIU267Brli3dHA0ZPcJroxO3
    Data H05DSreOj3aba76JdIfOkFuiDm+LL70/++wLl48mVszF+5cFvTD/dP11BfPnJS+9v0+koPLf+P57kNbJkU0Np5jXUKppgid/CZ68GwtjbQUFLHLD3rDTbjLq
    Data NGiuh/w74geAh+oFHgHpnGTXwPE46uqXaIDjpcKZvpEGEoLbT7n15Ln1R0TzDjLWv6bu+B2PVarwCCTDsUskLpG7GiZXeqMuo2X7h3cOL8C7loz3xR7asgaM
    Data DV44fXts2dHInlWjLfoYLsk0GnMN2mjAmPRuH1qA//jE1kfl65Z/FeWgnzFvkZ2QAzXYYEFWk88ko5GAz2rWacpoZFdNBeChBhIPR80AxO0kVmw4U1QKFcQ4
    Data 4BbeeTxonmd6fQw9uFARQKMTVtKO4oyXs/rmHdhi/2lqS97pddAEjupnX017g7NrsVdPRZxNbXxjNN1Usfi+yQM1ydpec5klraELQ20azdjojIIhm9mxeKiv
    Data d2Ek9Jlfq7d98YcbQZnJJ7T4FC6lqf/5x++p2r516cre+jalWKPwN2nUfV+rbmW+IFHKvWt5fOa3zC+Yd2fPROfdce1nZC8phMZUod5IOhUKKuQiIadOfWII
    Data xE3VUCmSlZqnU51Gd4niNm2q1FFHYyiKdQP0Jl6qeKbIjt9x7qWHqlTxoQ0Nx4AuJ1KJDN6AKipKu089xbzTvDis8M5eFL7yhoXiq7o6ULBy8MyVs2bNaPnl
    Data 689WP/vp+/dHl+wRG0UGicis37COKb/21n1+sUioVzllWf9T+OGzc2NKf3ru7vYHz5rM6crxXYgL16EKtUEuNGPtaO4Ad7Yg9fzlqBuCNYNmpEMrHAAyGnEB
    Data TxebjKjgK4UADg6Kc3pu+iLFUfO76PelUTYb2XFEcPaNYpdcfWt/IY3mMIb0sRn3V9lTamsyqvCoy185l9EL4ey0itR5dHW2eVqp7oUBS814dyRk9OtjTUHU
    Data XRjJL6o3ZNuWW4muhxf9YGXrZMCplNo80mAv2IV3//JJGPvnbT7e/Y1h5p2wwvrP2fp0Mr5pfEFyyYXOlAsOSX1qXWrJmsfeLLQhXmSuX8VP897EpDB1xwoy
    Data uSwBJ1RQyiflSQEfZQED1zZHOKA6DpSyGhJuGHvkpTKEKE6PufXKUpWCbJ8GAu/K5NOqgF0fmVmhihpfM/hyCyfhlkzUnKxSClUC2P3K3Tlmz9UFA0Hyt2ba
    Data oIstisusIol9Ts1HTzpCkaYx+9i6HbXbj46YyyS0ExYfnSNvNR86Vx/UYDhz/tqP8DHWpiDmKYjkMoNeFpQHBeysVcHaQwDEba74Iv6DHQS3sM0N09jx65Qt
    Data uLZ0+eqI8VcHV2793JnLsnjIeOrisZ6ERSUwa6O3XjfDdP6AOcUsYj76do62qjxlEZABKtD8/dvDGs6EKbbyIVvrsQauns6kAn6NqozG6kE9il3+uB0neMQ0
    Data W2H4gmxFp3NrSU2xN9wR/w1LiallOFhflrq7XMCeOi/4Bv7b1/YnnTlDKBOWO4sM9ZtSpNajq7WNKQR/mmzyScX5oawmmqpc86F1RnedEw369Qq7oZYPnprX
    Data cyCTv5Wg87fe2fODYeZaWD1rsqnaapJ6XTK5c+ey2NyR2xfOhQnLYvAG1WjDrxby806IiBUpaItZDy9TUIbOUXfTcnjpDhKyuJIPrS3aSuA3+d9ca9TT/2kT
    Data vxV5miPlPLSBdazR6oX5ut5lz637EC8Dpn5NcPk9Ly5iljIPLuT8hv2N+PWrVBLG0VXYJDaK9ozWrF60cNZId1dbS3MjvLOgwmYVCrBVYBXaM0qvW5skBCi2
    Data CDAKE6D8woNf8TIhzpVAMPng+JTGwL18Gr8hwiCFeNPpIZOSiVuqfopKl3a3aPhDpZvAbw06KGQXkxL7VVx3hs0EZSyZpoiqqrvXJUamA5LGo9BlbrOw/jia
    Data UHpUFnm5knzR3RnoPtpfEN7Z17a6uVEXb0npW6cB1GeCuWqZzZc6stTXWGWNK02xCrk3YDvSXgVNhCFrT93aG2OWK3MQeAeKLqCSOzs6u90DXt7fA8P27FBL
    Data zaOOL86bkTDoor1dl5ckiyFMZ6h68o6D6xxNdu+nD1cvYP9Bmdhcu6YJlD8wzLwJkFRFZ0WG4Fn1YrUo9jc11tbkc7Eoe0K9oBfFfncp4Jd6wrdGfB57HpQU
    Data /x+Og91KQO+XzueTIj8bFwm9da1/5uVPxjomdyvNNrGqjFzlbOoMDi9qMqRbkrr2FIcyiv+tSY0DZoANUZgC/gc4O0wBGXV3sH/EnDPo4ucfX5Z+qMPBwqjR
    Data JedMPrkh2MzlghaoCf8B8VJibjRDddh1WoWcT98yQ5X81xlqyuMp7hrQVPGuLA2X/b0JNvtvjuCB7lX1d9z33Va4Mr161dmOO9dtcOK+WcP9baZqPKSo7e/v
    Data Wr7l4cfvP5JY/fyelx87fjxn8YAv3Xu4u2bxgbaL21ZaRNkF++CZrr32fdIIrzKE5dG9ZtmqWDQURPNermrx2wHJwwBB4vYbpDt7pPBsp5puGCbjImSaQpFw
    Data enaKjiYVxm9QNaUKzaagVMUWLrLoI3yGqcZnr6i08C3de5NySCuNSUeaka/IXaRA9NSrj1WpUrM2N5y4jqVCsXmsaGdrtTlS5o/9r4ZdGZm39gHwxucbt20d
    Data dislcG9FrVF63Z3pD5/JI8ETWwy+c2SynZXtt7f0VC/cgxFYP7R/DrQ/iGUhAl0FVThUnUsmQtlwNggrdK2G667F8lELK1ztaLJDwKoGKfdpTUMDkiwW5ajj
    Data LZsKO8VDRC1WnO1la6Zrc+VN9Y0N3FTIAChvtzRVmMwpPkGJJTbdI6OqsJYWahoS6jCJH6jtrZxc/VhamRzd1HAC4EzTxL5DXC0z3GXO43U4IPbl1OVmodRR
    Data XpdiVjJvtz/YurjexFfoDNmVd4Dsma2JxUUd+PGuUkFzkvk9/jaBqlkz5O95iEwC7UbHY3Du5UZ3UrO70XoAzbVPL3hypSwAiM6l3egUarUqkh5vacmBTZXa
    Data ok9rOAC4gTrBBoO0DhcLv01YVHxgUAdG4woPUBoMZBmu43l3b3UtzBNSqb1CLNTYqwcrzi8Iyg3xuNdt9JrA1ccjqqKfunr6upvlTr5a1faXDnXzQKU+w1ea
    Data e2p+wFybDLiVyDodvId9AbSuElmXTpZ79Vo052KtQ2KVwAASAlOHKrn5RjVkXbqow24YbSYTpRUhZDR3AyW78VEaaOqMKn+4vFGf8shcLmPlxjnLHzk1oyGX
    Data SukMMhhiOmNKryuZN9QU4iv272rPP21NKry+xT6pSGzuqXf33L9/9ehEa7aju8Iv5HljqbDMLpe5lnR0dAUW3jXh7s0noHUFeHZenhaeXRfaNEMVm9fltMH9
    Data cCWfxhIgwUcVqQhQ7CniBCrW4eVhNKrZkM08kkulNA2Q1aVEGivdfass3t4Tv2E4A+VOBL5Sq+xOGlezLo6PmlXG3lmzLwbi314nIJRSp6t7eViTcklsojZd
    Data 3lOprGS+XtawzjbUkvICymu3ViWyJx7D/QILX20Sm685ZRIxXEw2XAQAqPc+zXx9m9NrcVugnRGYk1bAU2yeykkN2apoxOlQyGhesR5xiwBGsFtQBNrrTvLY
    Data Jhpqh9/QeuBNtR5KEyTqRt3t9dwiE2h6KnqV7oXAp5IT8XftQFVXwVE94+6t4QW9W30ic9iYWmwU2UVQx86Ly+wGv75R57YqKvJtPZ52548ltno4XZC+q+SF
    Data A5mvaAezHkuqa/63v3/Q07fMPnPJ2tzGtwJKJe3khgI98yfzlxKdjRltsG08sSgPNsgDUonVyhB7QvIOK+J2NTz9jyEqSajXBcl4wGeGK5so76hwzkUBKAZv
    Data grtt9j/YyZnEtdHQZzd12DqkpqAxPt8gcgjNusi8uNzmStmyNbUP3THTLafLxFZ1ebA35onkl98584UX9ozbR1aszU6+4SuZMnbb7oY9JytmfXXd/sKSJnRr
    Data tNNdkfnBN166P7sT2dEMu0+vQzva4fRV0txYnY1UuJ0KOcBQZnK1mXDuj2QgY4gkCabra9zLK805SPZAp1LRzcYhwY40PBppcMaWpoo32yqHjADCeyxCn2oo
    Data rjG57dDiiazSba0wp2slY/c8sluqcaUHCdwt8q7PyOpG+uDIulbjdHePOyVV83c1n9v9O9FtVI9JV7s5+drhlV6zdgqD2ZO7mw49NMP21V8AjVMgx4m4tV3f
    Data 8/dTEx5RRd+K/J6FT/RBVFb4LiNUtgO4wv84rGy+B3kfgMhUwTm9JJOKRUIBq1kh4+7d81T69EKSHU6TEBuSSOIlbLAbsUlzmhV5bBoajEyG0KARD3xRyl2w
    Data tLmhjkF4ENS6QIVVbNFUX142SJO1uQaViCQIAd++9txAPmOtUhJ8/fn34h5n7bCXXVQeyXXcuWG7xioW6H3y6P7z717qqUqYktlaz6LK9PNLczKLlUeIGpl3
    Data MvH1PxTJeOqhXDmoeAlecNW1n5Lroa2tSEW1Npd7LWaOzYZa7p5RjtO3Vi3p/73votzMzUCL/RakKov3jzX9ZweurGt2DAd+LrZDBy63aj67pn9N4Y6dUXNs
    Data dDy0tzXv6TI7W8byt83eOv7f+bEr1x1Y2Q2OKaAfm1tqrv2C+X8PPNx4r5BUWIxGdY1/+6ndk/9wVszdEJuDQcX2l+tvE/dBPDJYDs7jZbmqirDf53HbrYIy
    Data bpKQMQGcXYevBIAiUDMWJ0g8yeZqLI6hW+eoOEYBggJxLhqW4nsxf0G7ufsw4HdueJTi5AoCKFYaDpZcyel48+FjcxIVOpUqaL7ymtmf0FY6OhdU3dVwb6No
    Data 6IRbNzFocFb6ll0q6MNqP9yM7d/bc/4zTQTAL2aSkT6lXiOeuUAs1Kli8E95Zo5X3l5QlkvJtjazXS12yPiKMqMy0LVqS+2ee/cgtbEYzgIX01YsiuahAKsI
    Data edywI62TSVA/GqEQRPYCnO08AnYhAT3jsfu/nqlFBApg6NHFK86JSovRKDIgvygC4nQgO3EefIryGnIEEPrK3FhyVCzga+3PTFiUjlRf89LT6rKRWEtG49/C
    Data /HZTflO2jbn81Iaru/fPOKYiy50yNy4Ezv6OQ78e7tLxRenhIwOPMlJB9IHV55hfbB4IonbpdeYyQDM/JeZ6XiaheWgOpb1lwIXdMN8i5MW0JS/KD/ASCPpi
    Data YQKUe5MuSIvXGf+1SGOovFdvoUdyFeGK8ibUzdsG52nb4G8Ko43JoN9pVylk0xtxAaTuAIFP4Ydx+HHAebDiGjV84OAj4EhTmVa4IT9QIeeEaCKM2KhavL+S
    Data kBDcle4Z2dQV2AIcm2o2ZtvumkJRoLH/ZrXU4o3aW5ZAFJkfpEH2UfAvFhzgReA8teG9PSUgxUxF9ea181wQSHhJ2wFD/oR4FF5THjWl4KPXAfvv6F4vrk/t
    Data wgF3LzNrEEFMoUkgGvAgTYip4anCkye4v3OGOJ7HY6j5zi0yshSg7jYRjkxbYPTO4fqkPjuvzhCoMNnNpL2y3T/r1GBDQp8dLyRUXgUuMrX2dzUaMq5kjaHG
    Data Za6YpzDf8Ia70lFnZ3uGTJSQkfdiUsyJJWFXxqRUCPgEQBHOWUxuMKylWAJw2JfujOeom0qjVDW9S0ek2HtYUcFRFDQK8NPQOji1mfpbAptTiyIK8+iI3i7j
    Data hZuj4ftIzyo6El48dQe/l0mNLjJ6WxmGeWnzxUtVLXWtZwDcO8fuvv4e8VtKDyNPd0GG/vINUpQS6AFc5EmHoUOZ0SXbp2cAOIVUZhJD+oubA1DU1LBLXlqm
    Data LN1Qh1AvDjic3O0Lae5PP5RWU2j0cWkuRpjqu2duiUh6D66vkbqMweOX5JUaV9BDlFVfuXf+4YZ2Z8P2Jl2i40C2shBzuSjcoimQ0fzYgZ/V3bOwpUYaNOU9
    Data 23cMKGSi+Avdb///5r4DsK3qbPucO7WtLWtvyZK3pqcs7733SGI7Tpy9yGYmhDACYYW9KbuUWaCUUrpbKP1K9+IrLdA96KQsK/8990qWJVu24zj9v0UsxbF8
    Data 3/Oc97znHc8D4etHIybe5KsOpb39xrJn1w/0uFxupkRWj7zvG0xMspmxQD9YB45HTRCsWzM02NHW0hSpDPptTIQtkwoFPAr0w34xY5FxFySBncAwcgBCgQoK
    Data IWEFNI/PQzQ4GCkgMUEQ8EUQkHzA+ScBJAXoKx4vPn0oS5xtBGRv20Lh7AvcLkSmo2nugE7cqVgi4tT5e3SeUS4Xd7jJM86J4XS8w8Ptgi4Xuxx+H/ankiqz
    Data pfuaIw9Vi2w6Ktfd/q0LNheuG73TM9Gt83uc3efVtCmVOeXdZqk8zxjpevLpi247URPeUFFrLUPDDg9WqCaP3jfqaKvV5NPilrqZH/qGciV6m7sF2wcf7KzP
    Data 7+7I8nT3G0uYLVF7nUdg1BbvH7C3K/yl+2i52mrPkvFkIgx74oI17Rddf/NIjoKdhJg+mue9JLb5PYw5S+1RAT92SoLJJfqaXLSn1oBm4tvkWhABXWAqKm9r
    Data qYlWlue4rBa9jk9zPWstNrMKp/FAQRaOWkEwgI5KjMDCbPEy3iAeZFAaAjhA11MWuuxMKGpo5whb0B8OPtfb4E7sNnS7Ze3O/TeEDMrO8c9WtehEo3DiCgzn
    Data hNncPsC+MPathhZrlkbnUmWV5mUbvA5r/S6bWmqv0uM8afSGKqMh0NBSkO8a2uU74vGJhIWkgK9U2LNiJ0dq2/cUSCiBMlTWdrhAClWFni6XxmLR9FZabV3P
    Data XBG9fGTnEEOlTGjtE31qk0Xvr7Iy86KRLb2HvvhUON9lOepV0jIDPH7L+IlvVfaVqgpHr/tKNeC4rximYgOD/fVRCwbWjvV21FSHgkWFebluu9GgU0sRmyEB
    Data 1mHrJIiBtwJSPMgAns/j07wg4FF8HuUXQm6SH2IeESHGASJ5YsmxBJDPTwyS0QkGXpLjckLh6ex905f6woW72Gu2LUlRqQknutskJGNOn595n8M7XUQzV1R8
    Data QBOK/UYplium3A6SdvxTTjH3Us9axOPxPvu1e6CoWHKodsSroeXWLJux4ehQU6kqN2vv408P9hcVyFw4X2Goubrzi3u1ynaF77bCsEikzc+Hl0DVPkl5QVWL
    Data 0RE5j2cXaJgbLOwKJr76FcSuneSRQpWpVFtee97tg59+8afQ90C3NkuY7VHI1RVXr3379xhUddmEhNHWABGaf81EBHewEYEb+NlpDC0KDNAJYQVcaY1liWQn
    Data HV2zWQt8th8IV80LEWQLvAfvSAkbUl6QP429+Q0uhMAeY0OI4Y9enPcW+m0FsWbih9Q1oAmMgO6opLWlqtJfnOtRq2iSQpjodWAkDiBEBWOcIAkc7TJIkGj4
    Data nO2NQM+yYLiDMOFMaQxnx2nU6WTLSdKRxCOyVxxua6mUgE3PcP+G85QoQYNd6a4I7Lr/eG9rIFwdEvLFWdZtJTJjXk27tX3DxhsbasoOf/bxsbaW9vU+nVdZ
    Data dEuudWqkvBQWbpt+Qx4pU3jsJba6toDf4o/snXzisqKNzdVrRyurScKvG6jWhk3ZuZds2rQjcv2NV+3ao7vpJ3eXHNgxfMm20XXZHoda6tp3cKi7Y8e+mZ/e
    Data eMX0xAbSqgsc2r234qC0oeGyDROTyKrdxAt4F/kOyAJmoHxOpxXwMV7uCxACIAR2uEDNjUh7jfNTymwxXspL4hdGWqfxz9bXClNfAoCDu0+/TX2GcgMl8DAr
    Data Ox7V5XqHBjraaqtRncbLSLg77XqtQsanEYtKBNA8gkcTQQEk+DyC7wcU4NEUD60tH5AYH83eJLPgstQsOA8xRbukidkSGZdysrjxJDdZMJDwlGxKKo0gisNI
    Data 2gB04h9DMBndd//kG3+E7s+/dPjAN9/4U+ynn5fUVh1/68uf/tT4xJHjv/rKY/ePT+IO27aeZ8/PG35y00WRi8cM/lxrc1Nedo74QK6ko7PvYL5YKnNENm/S
    Data v/cvKPvL95+sOv7852I//ff//uSS48/D3295uf7Uzi1Pf/abv7r+1K4tTz0be7WiqSyvMHfwJJR+Y2++KtdW9fDu3Al9XpjfvM5i7Bg99o3K8UZdcECDVvvn
    Data WANxjPwlyAFhMBRVez3FRZ6wN8x1jHH9+CiDV2aBNJEvw9imT0gDGoIgCQnmTwIdVzwKi/cIoq8SBRm2TZCa36Wf7BuL4HO7xlI6bJT49WWuIF/eWzrmVxbw
    Data ew4V7+gxlNO+7i1lF/40UlHo0/kk/0vHevr27CwhCqQlra31ujK8sl+r6Ag5FKrCtbLyh6O6fHff/9x5Yn/uyOSk3K6Ple+89OnHX6uxCIt6dlQe/zXAwNXg
    Data feKvhBWoQC4ojjJ9yRDYrepcTa5YCFRQhfyIRk9i6PrPEvuXcMT+EpBkaSJhAgUsSxIMWRd8l/iRtjr2AhTKZGKjucQa+2N+wA+/rIzJFnybsJpbik/Hfiy3
    Data ZfMVVugoqg6boVy84JvIad0NjpJW3MDyXYkgYNoKScTxhDy3EZU/S9IcHZ6keJolvUWJVZx1ZEHFO/Vqjxv/uZ8UKj3u/AoDdnHArs3HMK/MECsosJpK4NcB
    Data Ab4Ifku+RMnZ6b1NYBu4MGphnDvYtnXrlonxjvaGemZu1mM1q9xqt0gAlFCJItX+Uiigpgcwgo9ZN0MBtAAaJ3EaOWcKJ1HCQIBTAj8Q8HEBs535BM4n/DxI
    Data oNyCXwQxIUeikTNnEogLT51xcKWwE7hn82ycT2bmpRBBLIs8Jmscb2piUxD0bCCbMmJlwpSJUNflpsJFfhv20x2vNd2+YfKBx77yj9tv3zD+0GNvDN12bJu7
    Data e+uOF//91Ge2bt9QM1yRa7XqfE00YZA/4hZJxbUXGbT5tJrpg/F51j847Wivq9+skNm8gWxllqM9wMzaWfT6KyD8bWfjrf9+9fmnt+64+ZZ/v/rCU9t2XFo0
    Data eKDxxgdf2nfoy6988tL+Q78LaxVbLz84ri9WYqR4TZ1kozFkFZYGRVkGQiTQ5OYpCnS60HR3a7tJba5oEvM0OoHGHrbVdmt2X4awchVxDz5EjgAZuvcLKJy9
    Data 9y986WM5kdmu1jiXFFtjucoYDmCSEltFY99UeNvhPuJ1t6VcgOXmOGshhAROI8/yNn4YP8B27BpBKCpXKiDQZiuMSqOAB+RQjnaWYe4wFxGv5JKokhtnJ7QE
    Data A8HZI1alZJczMb2FwT/F/lMXPPDwcyFFjX/vPW2VJRJva42xWoUf/pDPb78hdOjKYndh843Bg1Pbpz7fUOhdE/sNu+MBcZQsA2aQD5qiOqsFghynJd+al8qR
    Data jToN07ixeRwZFPCgdsIkKQb7my7Bkk2mELhhmzJxZsNXknRusC4jgfaz6Cv0Vxy7G/xf/FV2eq4+Ki0r8RU5HSpllkQkINi7cj6AGEiyEAM2BM5HwRy7xjlz
    Data aIi5MSLIBb5ccm7uEOp89uGUGCnZNkXBvSX+q6fqhMZ13bmhjTVFKqdMrTSJjYLykZL793rX1iuDAUWOQamUurUG/cjYhhvu7a5s2l1/ee1ObI+xTKDSj0xl
    Data GQQ8MV+ozLJFewYKLt8e2LuhQ0PpVbm5g9kCvUFk5nkObR+aHKr5yo37CkdzBwEEd8Jf4d9iLFEHOqLyCFNd9XpMRoMeZQ44vsgAgDiFZvZJnKRQKh+HJEtJ
    Data hZGs8gGIuxaSjK+vkLTT83gyfWqKq1dwfoXm2IeXIKWjsC9t3HPo0Z2NUV3xtnyVuqa1sqxAJGw3FEurRpz9JU1FcreLsUe1S6T15Y4fvaW1ZqwJ/2lkx6GN
    Data 22sqOkpsUWdeMK+7p32gMYvgmyo8EkWxvWtTRalWkZsX1GgZg6hl9qmNOcMt1dMHJloBmtclJrDbqE6WpxyDc3nKIUWx90euJkFTUOcu4as0BadKBdVBa/NV
    Data O4k9WZhIp8q75vs5NK9g43V7UR7vOaaG/XVCCCpQ/RrlQvO8VosuG7FxJlgRfGVujMIwCHABhCyZGoZjrKkxjOuvBhBjB7MS9Wuu48lN4ax9k81ONJf+TK98
    Data pbeg0xhsHy7yW/jNjuwCwvLOYNU2d9AcEq99trpuz5rRd7OsbNHrExorYtswJsZrzFmKih8Lyl6LfZHpaSqBP8Pkr009qvHoPxe78a76S7ffA08p8lCla+b9
    Data /WrTgIPp16116rpyCntbumK/RRApO32Qeo06AtpBF2iLShnOokhhAduDncXNagY6O4ISgp3zRZqROIEONwhxkvUkBADx/lRkCwwC9CcbKRHobOIYNdjLvV2C
    Data M2ZI4yCmEhw7zCE2S/VkTVSD7RhimIBuNeHcOSwc299nqVLoTHa9pWko4lO7rnv7igcurK2oa91SrC3UiJ2KIqUsXxJtqy71ZxdfOTjQdOTZbf8LyT/uN1ki
    Data ZXzflUPrB2AYQseUWao+BXdD9b5/du/4VBGPKt7+3OhDMxf/7frqsrVDnYNMC4BUK8+tC+l50R03PjT4uHHj+h/cecmUtR6Hulx3Kxn7T2xN7OPYnz5PVglQ
    Data avd9LAurxOuBlMEmn5eKzbRTGN6ZwlSKydNIQAEEN8QC8F3QDLTA+5xERBIYCnwwtmyz8MlGINQF3KjGkOLFUGAA2xo14ai2JMdS3V+oy80Wmb1OsX5jv1nt
    Data 9Ln6b95gb9HnF1hJnNR7s31on30Cf4GXUCLgADkv6LUSMTedohWk8tQDEEIDmxh3YlC2RAmISyhEyHCyEkhj8BJ7pWFsyFblHtuiC+fzpPpAU4O1oipri1uo
    Data cRXrAkfhfwzZMu29Lw186ppOJel0C3l8tb/MFAq3qYMTey+pPC/2B4TVH2ENhIaJ8BXADkqjEpXSaFDaVXbEmk2izG+yb5qA8dEmAs6fbYrTHM8p11FO9n6U
    Data CNjV2KU1x7a4ezGsufHARVe9Ul+Y41g/FftrY/vePEWWQNne0FCtLSGwcZ3N3nrtDTddv3b9vbsevvmii23j13xzcKfSW9y7M3LiE4CBveCPxI8J1VIxOZr8
    Data YaNxCErYvlDIsWKgHeSECdfLhuGQsi74LnZCWw2bYu/Hg2+oZoLvWJVy5p8Lvv1HU2txDOZy0XfsTRR9x/4qXvBNgIMfMrOs9dReIActSP89mr1xaniova2h
    Data vqY64C/Ms1nVrMIRusO290O+oCzXixM8BcRQehAKYBjwBbiAj1LypAAn/YCHBuKRIwEURlKcH8EAgSF2SoFAaIsPu/IT4kfM4sDZPDZ0xsndw2wLXLJxNKxG
    Data 5dbkZmP/y0TDGAvKcGgpnGLtjBaF24jtPdawvqhV5Zzx4dMdxQPRQlkOJh3bxexxvyZY5KgdFhi2W2trXKQgN6T2tVmzvbm+hmv2ZTvV+uDHLy2FdQ9St6j2
    Data E32/3M2IW1iqPja+zpPZNQGPxPT4j57QyaQ5m69uPXGJIAsjnrhDoldILLbOpwh9kcZD0VjsgxKRtsjZDFuW3jBo3f5Jb2XXbQCcD7ZFdXt2bUArx7DOl4Rz
    Data 3Dar2aiZ1aZqmYACoQISGGYFAiFPKOAFAZ+xHR9DdQiaz9FcCoUsE05cPYnrXLLFe5lInOv7FWRYtrOPxc58BbEHVhy9feI486W0rDDgg5XLWc8uZh9uZ9ez
    Data CVwDropaLr/sogvP2zO9ccPU2rHBga6O+toow9eY2I9ZzKpONkMBXuLFCFoBMRJtRwkMA7EIF4nRdhSIcIFfCAU8kYDHbEu+iMdPSgzwAU3waYjN3Z0ikQQt
    Data vUQitFHcJhWd2SZls0lsrJa2Re02tEVXIzh9et427uv2NCW3cbhAWYi2cXZRzfZqN5kVKbZ2FOloWzSf2cRM12vZJ8fPPLydj4Xv8FO2tXsL2tZi8eN3S7xa
    Data RUja96SiPEtsJXnspo54O/HclQfIGKBjF+OfJ+5kuakb5nNTFwFAYAQrP4naEP1c0/ECdV3UxhfPdYGQb7ajKcGXjwydFmaEUcV3Vo6OWwXskTOjqMb+tQKK
    Data aoBznMj0d1lO5A1gPJo9sW54cD4zMjqZomnMyAK+IAz4GD+8TI5k3jnjSEYDzsnRWI161TmTsUu9mzbtwU3BiFtg3mZbbQpl7G/O6obBkKUs7/oxY2JVPkP/
    Data KL4qw1H10AJrIkB1jbQ1OYPloM/ZcoDEfZJ1UhS1+stx/q5mvkJdsNamEEeCNfK2VV+Qv3vYC+/AmnAOTXWP9cTX5EvxNdkF1kTVI8MLr0lV6poMQL75/8g2
    Data mfX1HDG0CePu+y7X6q9Q3tF98gKrSBfq8u0+ILbXCPRya77YZtsytdprhW/d4eWJ+KpCmcvcuFussJVUm0tLlAolr4tbNUpKf4Fdtf3gUNQxOLBn945tmzZO
    Data jK8ZHdg/uH/+GkpRFL4wC7wICoCEL5CwgZw4C+NjQsQw/P9tRRW2+D/kipHoS3pue/vqb72vFFPh3PGpvqjFHsmXa83i7ABBrZ+a7o+YbZEcqbZw1bfi25vX
    Data uIbaHG5llqqIkdoaGGS/zu4AgGBXl6IfjPvJfWAsmr1r5/i6hc+vitQ1FcClNyV27p3lUhXUVV9BvHuxkuyq70568RIvu4bkw/RL8TU8D2yLGnZu3zQ9sW6h
    Data Mw/VjJqYdeTP35tAAESYQJSyHVHdl48KCUL439yWIFVcmCLjwR9XdVp9Z/vC/vMHzTUjkyfv8ssKYj8RF/nl7ka/N3/V9+JmvybbP7l/cO0O19YtDp7LFO3Y
    Data XBkdrkE9alPM3ep+SgE2g8NgZzR7357tW4cG+/sqyoK+XI/NyvHfo33YOQABvQVC/m6IwQaIkwRqpychRqL0BkHiqH4PAYlDErBKwyj9zmMlxpnVTL6gINvH
    Data x1+KGH8eLX6Ipf1CHTrJVUc8NYllRyvP1qJ96jTKfKfLzo7jhnxhTh+Lphfm0G++5sKtcxj0Tdl5zcp/xX5/ZcQb6T/ILnqpxukNPa2vcESbclSaSz96dfAH
    Data lUf/crFrDrf+zC1K0/HfbthIv/ycqzO8y7EI1353if9Ikmp/37i3PSr909ay/l1o2VUyvbsfOgi08EUypqvuNbhvuqC6ttiVRsF/iIi9fvxxoR5ibdOfuqsY
    Data AAjWxAKsVkUnaIxK62oqGWX4vFwbU9kVC2k0d49jGIFD66xsBUVQJQtoV5DcWtFcojXocsnloTNVsED3Yfu89Ozxxp3nibqPPLxmmaoWNxEUXj4vmSso+jks
    Data h6Z/f6tx2UIXcPjWtOQvxik/MLepbjDBnEXKNaP9van6D3x0j4rrP/BoXhigrNsZKkEIVl0JAk+7P62KMgSWcmVaPZ0I+OjcyxJn8+/QP2Jt3h+V96VZnIdu
    Data SZzFV2Js/qobG0u7Ha2KsQVpF6JVNPdj6VehuM1/xdp8GxiOygf659u8irN5D6TN/0dAnvH2syorMJzxwrN6a4E5Ml91AI5WhXTRD7I7YTcYiiq3bR0bne9/
    Data KriV4cH/E9tBvkT8uyqL8/piIe8q7pUvLdHPiLGaFp8wJ4QFFKHurDwmruWkLUhUpOS6wHGIh+eLXJDLEblIz4QtIXqBK1Kc9DI0MKhgivdFz4N/jv4Reh7E
    Data 7mGPPw3L7pHhQYjlPEh6DmmpBxGmOcDlPEpovmdjn+fb7POUg4KowOlIPo/BCnHzWS1ORg+0xNNhX8joXJbznE8t5jXQExMy+kF2BaMgH7Ge5nqTmNQS8GyW
    Data cYndvdSy/nqxjbucZ29bYkdCUBJXKmthPKYOgrraaKSsxF+c47aYdUz5nlNwR/eWECBwPk6wStI4H/h5AgzQiSmCpJdk7RP3niJqzvy6HdGxU8sTL0NMDW6u
    Data Qcyf1L3DlXmGgPOvx93kMpXMxPbDF5YaA8ZrL8wxbUIqaDUR/KEcr72KF/vOd4zL1TSb+BsMEm67uRSDAqUEaaJNHgYAY1UqNjO5tVowBTqiuvq68XVjIwN9
    Data nR2tzXVT9VMJ3Qpku7y4bgUNScAnSFaWi2IMSGAUKqGkyFmQZy9ngS+RFzs7vYuHlsqBrYogBvl+psQXzill0N9le23DaE4dzTok9DLYOXVEdBcGBCDCGZUz
    Data qOUrZ6QfK8tX0sB5KSfM8oQ18EhqPYR93kvoH3HPi5iZvbNPyzIzL/6g5PIfNP3YOYMHBWkn0DIftSr9EOKe9sb409aAMBMl5M59WnsOJMyrt7QZD6XlPzv2
    Data dMbzaZlWeDHzEcXZ4x36C6w9mgDDqOjJiXtpX2F+TpOnKWEd5GmK57kYwLqYVbHVUm5l+SaDjUs5kWVariiTlyBYu/2MfjC+axpBKCqpqfYVz/UTFhKu1tZZ
    Data 4qg/k63048VO/WXapX/xgx+C12AekUOMACOaf9Yx/d5owoWdf9Zk8QCB2qQhjE+ywCSzWJLvGroCCAsQMg/IaYQjy0DI9kphsthPlAW6gBUSUGXtKFTFvgKz
    Data DYVavz4Wi72nbys0YF+CzDSkyRr7E/MNpjxV7KuxnxiKDUYoi71nZL5Afs93eoC8gnqZVfKvA21RSWkJo6jnMBtRbzpav5BahBGgOuzHScKCYVyDCoBhRK9T
    Data Aggiuarp/M3M2elCwRuL53CII+GgaaSwinpJkoOlMD7xxFEo4XESi5Ab0OAgsfGpLbX3Pfij7Xt9vvyyfn1uo9/dfSnNu/jiV7797f7mCoNIafGaePjlcjVa
    Data 2J0UJtbZuiL2Q2i5N1OEKXaZPRY7XPbwy/BeKPrFW6MFwminS1dUnLvm4Mex07GZ2Ol7HzcMeLKdFh4lIJh1nflpJN8RuCT2tBYt9kf/M1kz81RFrARgnIoI
    Data cybWg37QElV2dzY3pmqJsDp3rJZIIvdG0OFFVUXoVVMVUaQdoGehMtKZeqCuhuYI8ZeU8zZhyx+xtqyNypvSLInyOx7OkksbkVo1I8rTDuezMGJj2mG9Omb8
    Data a9pZnmLJcdAYlbc0z7dkPmfJBkib/5uQzJwPOwu78jIGAqtiYXLTYnECZ+sHWdRuAA1R5fi6rs75PsDL2ZsH/4vQXepwPAuTQ+Nip+XqmJ1YKq/F6aZwHgPd
    Data R5oak9op7H2E004BJCTDmQRUqNURUEkP4c9eUAWvTPcWq6SwQu5eIAN1N5OP6aSU6MxHDDm+IofdoFPIeRTGMeR4IECEkMiUiC2QQI2IcU3BuEHnaX4lxpKT
    Data 2ZqE3hFrYsZOUoqcHW9GrJG445/PFLSVHh8OKqksLkdjazMH9j7d11epzfeVanhZH1523q6Zx/73qhPrHDk1jXyyBT5STtxcd3eeyCKcm5jJG4+YCIrv8D5Y
    Data 1kJtGj4Nng7K12+49mCORAZh7F0AT98W+z3RQd0CWkEUsSrVRIsLcz12a5YYtMJWGs3eJerCFETUNYArDnNy3kkua5YQyBZkad3l8llhBiKFBzEYSPAn+n1J
    Data /kTkAtmsNs7lrrKGytvqLNGmqdHwvtue7q4ItI+2B+zBSl0k6t92+ZHmikJ3cY6+uKVJz3fqyy9Ym8P878TxDb09NYe36qOa2O8rVef9+qNjkAcNEHuk4sjm
    Data tu9dvqm7a2iCIUz0rLt6g7O9KlhT57F7IvaoT+6vcLTddtmhy0p2lJ/45M1nYo/F/joT+01YzQM4p+BBNbIKHp1gXVTZ3lpXm6rjgXxabYqOB83DeTQeTNfz
    Data 4PHEtsyiHvyzE/VYSKRkpUIfGC9N3GQFyh/EhfP0UBLW7GSt2YqqhZHKVFuy1cJUW+J0eNnCKLyztGHaIN5K7QfPTxvgW4H9sB/Om/lL2C/A2q8fVbj7eqNV
    Data 89EYnWvBCkibeXBpM64mFJP9BJwtkTURN+qKAZnzpQuUuTaRTr/vBw1RJq4hnFVMVJMb7VqBZfEXHyngiQTqfL77sdIaJojJ/tUQE8IQxMG4he+gxKyFx8CB
    Data qKWyYmiwr6ejrbGhtrpirHIs1doS1E+zmIYPXyjGaBx1zaDRAzR1ImBALlzEDwjOzvhLtSSuGNT3LpkwWcFS5G5e4x5sdSazJ+hrLnuCVuJa8p24rxhF9dSB
    Data /ob6+WivWMT+XKfSOXMYS4SVK7Y1nr9YTLkSQ1cuEUEScd+yPW7vYbAtmt3f29HeWJ/uo1GWr21RzIME4gWC+JyV+NwBfvFmvxV7nKvYnr7RiZN3+eRcT5/C
    Data hXr6VmJ8Raa2PeRv3qdwQgjCSMcZ7IwaWltaelp76mrLS0MBH1MFMxk0agVSmWFn3Fr9AOODjigG+Q6IQ5sVw3ACzTUj0sagUIDx+azWJoXF6VHnvOJBDEv2
    Data gnHQX1xnyB10u+evgxqmL8GsHhFRN7X+02WKpBaR1Fm3d6eL0yJ6atQipaTrbym7OGUJ7PWf/HYe/r+webzX9/DhPfhPAgvLFV2YWwGz/V/uMs+xvnGm5bGE
    Data 4f1D+2pOwu8kFY1AZSzA6nTVIhUbAJg/md5WxNDgdpqNHCNBIUDCIKgPEhFUEYg1FZQkeTHYYUEKGRTOlhbJ9IY4thy2fCEv+FhFshnO4xLrw1t3ksvV9bqI
    Data P3j73B44rc/bZzF/+ldL6nwdAjinXkU1supVNWA0qna7opFwKFXDCvnXyriGFSsHCYJpWlaApsU2XkZBK/6ZCVrhC0SxZyRyRaZHrssVvcJumh+vsjb6BtXJ
    Data 2igCuqNKt6u4KNVCCDfBhIUAFU4zD53RMrwzs4w7LTY9I6vckBaPLtsqf04PQlmbEJAKsDZpAkMIN40Nft983JRxVimElJmGC5hm1UCjyBhvnpGR9JlizGVb
    Data C34vY2RJsHaLku/EsdQB+pDlmptKS+ZbLsRZLuHWUw0FYcIFCc8KUEvFLmdkO2hZLGBZvgGXqBpxWluVrNcqBRUoWxEOFuQ7bLpsmgRO6KRZJQeCxEgCQzSe
    Data Yhur44CjE5DVNEgj/l9KZUuxkPTnLYtpbR1Oc0AvLS61RY7Pczvxp/wQ+R1OUQyCHHfyGTlFMQIjwtyzkSnPRiz72fA0j7L4cynSXMgSz4W/kO452KfCA6zn
    Data qAKlaO0ilV5P6tqZXZAwkxA93NktXGafsPhj3p7JCSzxvMSdGbc+wNknP4/d+0FQD4Lo2aNVRYWpz65HgZkwAdgVLuoSu3qJRYb3LLaNlzKBZcksdTtTIcCo
    Data W0E5aAM9UWljA8MS6ivIMxuZ6i8f2aCs2KlH0NZAElZACpVTcQIPx9EOkVwbmYzGKCopq8iGsyl68JyygQZFZBTi0ePIebmMI9rHifkSCZYosariEgGEW6Xz
    Data t07sr7/hoV0N7YODx2+4qWmtxobtPiXR9ztq1ypz+TJlTUWbRtnWU6DwttT6Y78iLugf7S8tL+ppa2+x1ZDOiee/+OWXHr0uuKtr4J7bH3ts2/66oksOhwvc
    Data bQZ7v67Y09oqztZn5Rz1+Biv7yzM5lOCmS/eO7WGoQqt7Q+PnV93PK6URtxPiUE32AO2RB29PRDs2L5l09TkmrHhwZ49vXvS9bgQY0IkoccF+EIJRmPzkw/i
    Data RYS6eOdAqGvJvo1VUfKK7VkqQ7GKWl/w/EyZC5zTAGM1pdEpVRmVoDMqoQSGvLcbkb+RBOBOqIyyYNTyZMEWyvwuSyoMvph2Vi2tHYY/kn5ccc9L5FGd7PMG
    Data UXdYjjvxtMnusIxPSS7zKdNOq+U9YV7aqbX0E8LT6QdX/AmnqQD7hFWovoZOrrkr6uDOLfSgZ7+cmU6u5T3yzzKdYEs/O/ZuxkOMs8EzlJi1QQPoj+rcrprq
    Data SEVJqJjRoXE1uBsS9kD5IX9K6xefT7IhK0kKM6OdXp55lsxvLstK3Uu5imUYayBz7hLZ6iHynfiOqEedX+i8n4sYC3fan+22WOK8X541sF8uduwvwxSbluSR
    Data vhf8nbiZeBNkASPIR9xnMqnDnq2SGmUGAZ/jPgMQcH1TBISsJVwETGjloj8c5HzC6zQyT5BC70K5sH8VlwYVHq3c1hLUBDWwbg6jp1StVVhbguqQRpHTF8T/
    Data qaNUCs+oP0svEBl7S381y+o5815xlH1X31+qyuOr0NM0nm4jAsR9wA18IApao5JgINfrtJtm+8KCbF9YpR8jCXPmtjDJgm1hSWa3hDYTTXOMi2wtOS7DBrnU
    Data m0apmcUDRWNTZO0EhvX2Xn7DXd68njIb6ghzdtYSWM+Rd9oO2IxKdxZqCHsJfloZHRjsMtdiDeLI6EC7oQrDj1rePNZyoO/TDz36yKZdwoO6LF1RkXek7+SD
    Data j8Q+GjFRXluTwcKDHdh3+MyStzxwlWUHgUBw/wU6zWvB2CUAYJwyG9UIGsEo6IlmQzA82NWZrs8mQMyUnD5bolLKT+i0saniuWJtWEKsjV5FsbYFDsyzFHC7
    Data IO0kPVtBN3Jg3jmbsG4nY91B0IQU/lqa023L9tckbIvTYd6iAnjU6tk0/Xg+S3vG/pN2cJ+tQfHPpB/rnD1PUQHGnhOgE6F1fF1b60JoLeQs2jRbSeVnNutq
    Data QzVTMHCWBh7LFCWcraGJGxaJIZDFf0C+wyJ4GrQim09O9HQvZPNczuazluYqeecYxktOQ56l1R9d7Jw9a8url7x/s6p6VCdoBsMoTm9rTSjrsXE6p6zHRusL
    Data yOuRqymvl+4uVkNuDwun+YzV0t8jLpjnO053M+zLJupmkA+iSEG5siLgRwrKKmVcQdkMWdlgiMFw8oxfSEGZRmYtQgBMNInJwwo5UiCLS8zToTmilDY5Gvud
    Data K6Ecq7u4VG+XV+r5wt33eIeeqVbkhD1SF6F2aKos4Z7w1u2fCctDa86vu+FDX75IpB9PSCivGRv/WuzGcj0/IHVnvwvfuQ4aMdvpy91ZWbbxXceavjhww4Hc
    Data /heqnn767uPFm+BrJwaKZK6EinJk4zGAs2i6h3yHRdNmFONOre/rTSCKjXE5RHGR7rmG1FKR8GpADP/DYht41fD2m8U3MseF9mt6hOXsmUasgRvHx4aXYA1M
    Data Y+vhAwHGR5pj/99o6uZrGaw6M8+DaeIIq86uFEoXW8BBKdNHsY4cYfso2lGPVltDzVI9WukdFDTg4TSSzTlnnSvzjY83raxTYm+ajVfSIFGQbkYO4xQ/zoy5
    Data G2yJGrZt3bhhPCMrVUMqzkVQIBSEZ9EuBItSxWH/DfI/LiOaVJmcFchffUKqKd+mAUtTwBPUEZIs29Yea3W2wCql3YNDq05J9a7EqK9c2+C10L4ytd+pCxSr
    Data BYLK7TvjO+EK8s/sThgEG6PZPd1MfL1gP1HDsvqJ2Oz1uWyhzbhEK90cWCDjSqykJ/RDidFQETe2xufU+xPGBtjpnhhNllMKsA5sA1NR9eTEQF9by3x2zIZN
    Data EMfG4/tkDYRkF6QhmTwXSNTe7ufP3SY04CFnlMPtl1U9FWhkd7QOKBCcyw+WtnvOaoto1cX18ryQ0lsglhjKcazEWDO8zsL32uun96bsm7evWOnmcPvVhfIs
    Data T4W57iAtsXidLarCXl3ndNG61vx7sWjKjkFxeDhGEyOEEFSBbmatlG2ttdWlTJ0kz4tiJ7GQOyGaPJCHNxXrcIpXrsQwmoYAI5iF4pE8dqFoHkkjppi4cgyP
    Data N2dX0PS8k2K+jRfdHolajpJGcr7zWr6wn6Yakxmc39hp4GfcGuz2cVlCn4/Vpu+R2F8iSZt1bChc06YZt+ryF94RyjFDewFPl3sjvCV1bwAMTDJR0Q76R6AE
    Data NKGqS3kZ6sUy6mXxWVtWuDicYEpLJ0lLtFFw88hhueKM6dHQ96QPv0xC4dVHvrZMarR/nPJvIZrSZ10eMMJcKPjnzcsmRisP7yU/PX8Sroq5e59PdbIsIw1R
    Data Kaq7ul2sToqER9FoNjPev4QRGCSCrCgj7k+aSZLKREQjMy2/zw39bfqlrxZbbqPbFa98angIezm9i2l4cMlOt6NrxvCvpl/f4lg5wmKlF1Sj+1sqVnJLIWle
    Data Hl7os8FL5un/M0YO/mHm2b+VYIhyZp7149DUQwUYNLWA9qi8uSkYmIsnHqoBFbOdX2cCKt4KQJUxQ3aG8MKpTCmxZeKMeCRj5ovFG/EL+omEb6qt+e/7pvQL
    Data 8pkj7C8p192VeabXjLQu25e8zHJYwp8g30l4pvKy+Z4pDT+o9201PVO6ac4UOjkphlkuXvLS7vWcV3qBfpFByQiojUq6OlNRkt/Gzh7+V/wSCp7d6D+uhJo1
    Data Zx3um+gVoOcGmd5HiDUWicImlMkdI1FdsfXtYrFByZP7u8OHa1eGp6soKSOz6/Qrs5gGSr6Up8suGNxdKhULtdmF4RxVvjaOMOI1imIQ1gbaUN9ROsICNRBQ
    Data 0JoBZ6vorFzuAizVpuxfn/lh+NkCrcgokItNHnGWubdUmUs9crFV77LpJ5d7LP6PQiDIDrc6hgIKOotnt9RtX7NJby3SFlhZ7eE6Bou1DBYrwFpks77espLC
    Data ArfLalEp+TzWZl2QQDajCAoVNgGGgyBg9Q39yb5U9sKQajNkIvns1QrDJdgyObmWBmUvv7xg83VKc14tItt67nK/cplEXXjFEuD8c/VvjkLvHywkhfi3Yu/F
    Data Dty8b7n8XeQfl8YoDvqYfMcJ6mZQDKJgAExHDX4fBO1tZaW+qD9akGcx6bSyLIoExbAY3Zhr9ZDGQyIM0JiVBxFvIR8CmqQBGQQ0zeVYufvxbKXEJoAUNfsK
    Data stdlZ6oScJIx0qdh1mj21obRiWtaijCwFMYb7ZgFmW0Lo4hC1XD55oi2hNddtm/X02Elm37+c+MTQ05PnV1oMQ87xizhxuJ8vUs4Mzp92YkwEZQ3s3lpwqQ1
    Data D4spk1Dd2JQjyrYEau3trzG3ZG+2rmKzquBC/5Z7y595/oFrAtvHsb6teW6xrqHjT9tjf9j84n+Oi+RU5NU3/s1cCNg0dewd/DSpkVvXTJTVm8JeidR0eO1w
    Data hyGCsJ3gnBODFtCDKqlNDSgCZHQDbRo1jwRiKOIjhHNKQknNGYjqqZC9FBPEXIbOpAIpPUfEl6tJwZXifTeijsN2IM65mStWjGyKZY5DpHMzH50FiAGGEEqu
    Data ZxBaB3rBwaitoR7hs763gWH8LSsJ+PJzc1xWYxyldbAOzUZ26KEQb2ZQKsSQ0CcVFkEg5AsBP0hDEsGWJJAxhcIkYPn8JGDFUCCYA1jB4oBFuHS7uJ7YcKJq
    Data QtPhELsMdgbFqoUwnAmuVcXe3O7rs1TyAv+/KvT5Xga6/PJIibUy39nr/uJC6M2A1J9eAyFBZmkdUjdxu7WUAa36BMQcNnPJnw/PPLCpch5w4wy//6FHQDdY
    Data j3LIk2ND/RmZrDNwyNKAR7FJ5P8imyxMTyuvDrlvTJ2WZF5Fztj35uee2VkNCTnCzmpEuRmXqtJg5hmX9JkoCtCAYsXYV2v8J82wZzrbEptOM+DyRzIa0u3D
    Data oZM0szzr21Fed9P05MTYaDrzN5vX5RAqgDw+LxPXMR/QFJve/W+SUWdO9a4KYHMzpn1XEbl3ZUi9JxAcJv/MIrgFTKJuzfq6qkhpSfr8GlsjXBjFPEABHsVq
    Data bKPeC8HqzWtlNP4Zgdqd0cjLR7csY0Y9oSbwRILZfmz0/z6z/epTd284V1zdr6YlAuITqeQ7c6ctS0syT1umoRUZeNWmLeVnMwxXuTrDbwCCnhhNPEEpwAbE
    Data PDM2OtCP6mflpW6nSpmoEtQaIEWi0gCGVOUpgqSCgOIRFFLDpAne3AoBQSQIilILBPxMBQK2m2oxxAWXj1+ago+klgxclsr1L65vfy0T8pYAb26usar9+ynF
    Data g/aJguH29e1bvnXnIsAbWhrB4Rxlge4BgIGGGI1/ihCCclDHrIC6sqKirrIu6M/1mE16rVIhEmLsGlRHIUYDCPBCSKIeC4wmaIzNItDIA5Ao8oUgB7WwJYeX
    Data cDx1eImM3wKSznA+SBNjXukrlQwT/sew7vyCKZoavvrNlotHbc0Y/sVnZlKw6g6EvppYCEPt8DqzwGWpgp44dnlYbv7UC7fe9eGg0WSuv+jFfTvwuXh167w3
    Data H59r8cnCYfwXcQAnIgM1/SLoBuchZYrNmxZQptgAKRr5Tez/iN/El8orrI4n/dISKYZV9K1XLplw4CKEQxQVjxD6o0oUIcz3tqVRCCjkcbFz63HdmVNjZ+R8
    Data w5nzYst1xP9ZJDGGgWkG4RfQP2JstgFx0nR2NDZUR5kH8LmcXKaW5aRphTwz4AHenNIHiREk6wxwVjE8LXGbzCDwl5e4pRJIZ8zGfmviNUrYuNX0khWmaSi8
    Data 5qJvMRncroatkXpbKdPfJ32gQj12w53Xb9hICXa4dm1wdmBYSO23StVl9ZqCgQq73OMod1kbNwtLN7xc+/DVvg34e5mrTp8yoXzu329dIJ/74G2/+8s3ftCk
    Data DXlGJpoNQpGsUBsVKHCCL1au6TFGserpFxu05eE9lHtRxZW2mZ8QU1QAuEAZ6AOtUWlvT1UkHMrPk7OMKTSqQokBhhPQWg5JMwUR/yFK5kDWPROZvbEz6Tgs
    Data mlDSJaPcDOVK9B9iyW7MsGuJ6S3shltffLhM6R86UHcSZleIlCKdO1dZLAo7Tz0Z+2fjpgK5e83Ggpd/Y6L4yo62vj5Xn400lo6N9TbBH2eqTP383Wcqn3n2
    Data 3suLN18q1ot0EpFRe+C8WM7M7+7yikVCrdIuLfc+iZ24Zdyn8IbHj7Y+dIvBGC6dOIL9PPO0Mofwl5jbXQvYCDZEdRPjoyP9fe1t6TgXoehDCPkC/jyQCwAP
    Data CFDq4b8P98x3i1XBO2HMfOlYBbyT92a+07Fov538M4v2AeSxu7uYXri6yook4vnIY8cRP4tzGpCQJtlsBHLZ5wj3Ge1+FsB/O6OxV4x8XaYLH4t74m36CQb3
    Data I6AvquzvW8izl6zQq/NWFebpBcvV8eXvpF5cVgPPr6ff8lgUd5PvsChuAXVRSX1dEr8Uqvam45ckFwEtfRagTbfiWSD1gxTTrRida+fPbEzPvEe8zerjjIO1
    Data UfXoyHxPLEA5nHRI8mkezecFl4SmDfB4Ut4qu2E5ZYtfEF2cMBGqDK0KSD8dLVHa6nKlfKMid32tyVe1GjCdKdDqNFJHS7ktV2W1iHTVox2NFQmsUltYrHaB
    Data jqi0taWyIjW+CCXQSpGQMXNwDmrF58bVghAqgbBsFWq2Xo9+xllA92/pFl0pevEfLmRF7PQLp/9DfoqJmCfABUiZduOG0eH+3q6OlubSsL/Y7TTo+TyMU6aN
    Data FmGAQO2/mHUSCsxAgAvCqIpB8WjEtAwoAiSZltEsGNcNnHzBXbD5XLlIiG6KLhuNzYGygr32sTMlCR5mzvBcoRN1qobjhU/EcKFRJ/4CS1y8/XMmzUJhRcbw
    Data Gv/L+X+66J7na5vkTiNflXXPMzyJ0iDUKezrj5Mee5vZ76Po20KRSu2URiDNsgR//ofc8JhK3FhgKiFMReYqhgVboNKo/LWHc3JztA6hQDVd17wxR5KTvStj
    Data yI29JcyK1b3SqKXFAmW+cA98CyMoeZZDZjbAez7DskF7XaVOMU5lRwydW+tOPaJ7AKpO9mk86/bf0PcoIhPkuXMVBVjslet1IbGcZFgvckbNhurL6snNi0Ti
    Data 2OkYc3aa6W+BBjAOhtFc37q1aK+UhIJ+jztbLeCDBtiA/FSpFfFmWxu5WUqMDifYStjuYj6cpSVPJyZh7/3ICaURkwAFWq3krFTcK6GL+0KEJIzfXGSoUui4
    Data 9pWSuTQkcmiqFjbmm8sIR9DSZfRkaV4YMEYnOovy9V5dUWPeHA6S7ebPZorT8Y5H/tKSwjvixW6Hr/epc3fd+kj/d+tG5eb31mjDQf+hiang5nvaQyzdiCo7
    Data tHn3p38bbcHvyhius7uLKqAUYB+4GByOmi46fOjAzu2bp6cm167p6mhqiFZVlAd8nhy0x1BFtq8M0oQRiuj9fRgQAcgDhBXwUMQuCAIBn/k/PwUJMaRFBMcb
    Data AwBPlL7rElVaYXJpMm60IsQZT1GJ3Na8XUeHrBEs7vpMeHx4wk0tvfUg2qssc0/mrWaXSYZyS90Sh6b2lw4sfdvp/gpl+RpPuFBZgOE+a0VpnkZV3CCWVuQq
    Data M+++2EvMTi1bM/DVw9pFttuucFBA8jVSj8RjfCCatvN2HpTdfAoN2hV23oLhpCgvX5GHUfv9i+0/+A3dZ3DeWgDg6dPMXrNQN4MRsC5qgKC7q62lubGyvDDf
    Data 47ZbjXq1ilnGETiC7mflQcgH0MrpWvIBD/B5aGaZghiNYxB4hBCAeCAgQYEAWlTBXCKgNB4gFB8kKuyJEjvFVdg5onuaWuYe1IVP9t47u9NUObfc6NA/kaVS
    Data 5Pv7pDQpZNxgGZH1M6ZguX2bMrvWMKTNyn5+wFTF7Tytr4HdeSMRtPNatpmZDZZK7APDGLxoL80W3WF/jrdLnbupmWCL7lhj23c6/xJcfMvFtSgJPf0FYAHV
    Data oCWabbNWVqATKw/1OVTbqjmNRuTTvBQkAA8nuBsvzcdwSOLo6xXpVC5Fl7SkGCfsWYrdZDnKje5MwlZxJrdLKDHLBNMXNbldEKRywSSZzURcZzlPiBFYOhtM
    Data guxMkEJ2Ri2fwW4JQy3BdvaXpcy0FOGZJBMDDKdj+jY9wmDHh3Rmi3Pdjrk6szggII6ssUJdz/ntAEthIjetAWA5EDg/vfTPrXw+2xkR5jj8QkV56Rx+6DzH
    Data WE2Ms+Oym/+Qi69n7Ptpz7jU+snSn4/b8wJWH7gClEelYQbRud6Eqi56OjsFCZJAUctZbvLMCZOl1Gj/mDErspxF9WTMMHGru4XtGqgGtagvrqI8FCwqnMs+
    Data iXIgTkCB+G6eu415K+ZqzGiKxRf8oYyGWIqd8s1MWSAYeydGY69RChAERVGB28XNYyFdOpOSZjVUkwXVpOILt23xBaesAuHMWEBwaEktixa5O3ZlZS0AAXz8
    Data 9u+XRWyHc0QpRc/eHf4NLXK3aMGlPxF7fOY379fZ9Wt2Whh0f4d5tkfJ34Is0AwqohI0dVaQl+uxmgUsa5GrskiKE8wj4nA2Rw5ItJNzuKwBhBJoX4iGCfWB
    Data ZlxfboiMWrh0iU0mMyfKYn2saM/O+Qst0MmyTLIfpxoqx1ZH/DI1vfTRP+yXbfhooRWvMvGkFvFjR1NHzdbFFcF/RT+RUNDO9a6WgvaZSS1jT52xtnJ/6sNz
    Data WsoYTQiBE+RGJRAkdy06dVTcLiVSdim+7F26+FZcYsMhO8+8QeGUgrFzEyiLShvqSpl+8ry4teO8vQADBBbEIWdxEiZMjm75UspOLvNodEC5gktZcW9FsEQ1
    Data F8cSzbVLnZeDbqiG6p9fUCw59pqyuFCRX9AxFd574drcTWun7v9j7JNbr/Br6GWs0syTJsk2LAc+BX/5cp/5NaVSam+OlBUr3Fny/L6bYy/EHog9G8v7zkE9
    Data HxCgZOYHxCPs/kSscZOgLqprbJBJGQ6tycbJaFXA581hWCWCsqCAL0AWIyHalnwKQ9dlgocl6NKQxWhaSi+4V/EVEcmif5UQrUOuWsGaOP4vsY6UTfzOYkjB
    Data fpgC9F9cueOCV25+QOrP15+672RXwKQUMPQc6Rv74yeWOMrtqd8fi7V/L3YqtjH28esVtFnp4hXBEqiEjW9cluD6BFjsPobDJUL/iFX5LUEqv057tkYiFrJ8
    Data dA5O5RfxzkKAA1T2BiDeM8PmT6U4G6kuE26Zy7Cx+5aDNOJAxqRPDCwXYXzxYjmc2AMzP8CFVABoQQjkRyV6ncOuC+lD2RoCRxbRAAjMEKLxZYIjjmWNgJxh
    Data 5qonzcKGXBg2MJYpYXL3lTuejb3bMYRgceP9J3rjsCAHMyVC/tD+9dipmadmGizschfCeqiFra/vTSw3us99auY9sphSgB6wCaxBWvPrJ/v7uruiVV6PxWw0
    Data yKQUCXpgjxDdm1t9YhwQ7M0Z4IiplQ95EKXTuVYUig17aHRxSXarc4l0HutRleiJCzDUVIIlbMN5LgmWDhKK5uh80tl1aTVzsVGjrbhAMp34rtxoqL9jvKfO
    Data VD7d+O3fZznNQo2Uc2qNPoszy271VRrDXUOWhnGz+x/TFxRu1PMUEl13tTo3Mn286YamSPNEe+/GWu8nR9MTwDfm5SmLCtqnQtu+Zp15q4IQCRS5yfjCZVFm
    Data 2UMOnYJk2kn8rffEHt6166X+g9Grrwm4xdlKWmjXh2+79qI9gU0+X+nJ8y7uVM3sWyAxjFbkNFPb4FM3g1owDPpRvNnX21BfWV4SyvWqlTwa1MJaFG+G/VYk
    Data zIFKQjiigMYZQLJ1TQrQVDCRKwQQc0MUgyaOC7aryobPS2Ak84XM34USduXcHGt19Wy2CeXXFzQ/9st3Lg/aK3T5JQUyuyrnpVtLtMJGryFEaFzZ1Za1csHv
    Data 9zZ4ssSRoXJ1cah094fm3s4aO8ojaeVWXfXMC+km58MnJ7uuKIkk0hdHsM6fP8HkB9dfcGPX94ZjMwWqsb0NlWZDltshldkv2eobH7lsw7hITpl07jyVPybL
    Data YGGUK1IQQtAKOkB3NLutDYK2jjYmL1heluNWyHk00vhDNi4uhhRei2piaJ6IYlBPgSCNlP4A6Z/l2liAdY1ikY673RJ8cbbozJaGyjD+jFbuMtZfNNAQ8mbp
    Data Q81lxeVsWkiGeKBVfJYHWu6QqLQi26/21iUsGyzd+29Tbwdr2Zl34WucMfKaxsbzuyfv2RXJxPLcVNfcahkb3l9fPmvR7b4NQ0c2rGMsCgcwxLz9Y8Zyv2VO
    Data hwrQBvoQNqsifub81WlVSqmEzwMVsILVaiAhznYAkzSim4EA1YAgOjD8aCiO1bkAIN6+JrQRMMnJYENqhyFn3A0gjU02bZHkzkY8YAuW4wKhNJ4B/NLjVz50
    Data 9/VvQE1O26ClPkftDdq7Hr2z/kh5pVViUg9sm2oemo5cdu/u9ob99V3uKjSteahvtDqNbIC8Ova52Fhs50XN2YRWG5oMqZV8lUasFepVBcZCcQEMQ8vXXnqg
    Data 8tjE8L57Hlufr9LK3VWC1s27f/DxfCVNUMncJu+hfwDMoBAEoiKb1WTUMisv4KHTxMKxQqOSJAbR2Sqczw0WQgBSJMsxWGouEaQZYcxs0LywT8Ov8hqDhMVv
    Data ec0dT89viGDtaU9a0z20swkTwa57mU02edkDYzNfv9nE5QCzqQNpD4PQwHiqI5SaQUMPWBvNRpNP8/GAYrIIh4eFoQB4GI3x6CA/HRKJeI1/lrhY0FetEByf
    Data vJHupFYOjw8uyOCf/jnzHnYjcwKYQRCURyVFhUmUUGymAYUbAINJtz8HJcjrSwk7FYcK8u4JqOBYClSwhewybi529T1r4DejXLMtZL/qwCxe3OmPXtO99bPn
    Data fYjxoKFPnbftjhc2xrbEHtoQB8zMRws/HfIhlIC+FaGG8yHNTf93fEh4odnNFWLlvfljmysHC1xwbBMD1SwLO4eVfBSvz/UnWgLOQiXFkRBLowMuZIlF0IHd
    Data Mf9xFwUIceGCj8QhhDxIqUEWaAMDUabrtramrMTrcTosJm22tE3WRpFs15EKEmzqCdIQ0EFAY5DG/AAjINcxDQmcwQiEsy3TJEQPz2IknRT7bDBDjGmYi1tf
    Data EXNxU+VNhGOnV4QWrLCyzi8zi8SWiarsQrEOXzFUOFTgP2VRUYQy7Z6cuagwssqGSfmQ9CNmSWQkL8HIGIuj4kFrRFNkVLsHK3VVxfmLAgIjcK26cGu53MIY
    Data oaK1JAAARFhgz5hBxOgIQUd7fS2zMGxFL0vCo8AgHGQ7fUiIQTkkcQwVbiHgQbYjG8NZ3ToM6dZBgOEQQ5Nbyfqe0EYlhf/YQjodZ5ZnPoReDBDslZW9kbhZ
    Data 54kMtjA88Et3b6/y95lMUXcTRZfU7im7+GAGcBQWqj1FmNQqLdIp7bKwvaaVarrqriseTYMKefUvLzn+Jq4vCpQQJBV7a/OCENFX5LYfhzifklIys8crtymx
    Data 78ALHrx2Pl4gyzbyKHU38CCuEavFoNNqpFlCAZ9HkRzXCOeLAQTJIwfHE4GvOKEBCGYlADnvwTZCsahh/xfH5lU95SH4wsXZHp+z7Qf76mt7r5JoGlzH1s/C
    Data R1tS8utm8gfZXR6VU5Ejk+UVlNFV3b/bGzsv9sb5HGp0ZVuin8W+8G+AAffpf1N+JjqtBb2gICpqqC8JW0xKOXdT1+WKuEw1wj0OUcICQtR3xBVfNEr2auqS
    Data BcI0hs/OSBBoCREelGxyAkVerJjhHFJW9GjpURdRCF3VXSNDxrIs8eevq9MR+Y0WoT0ytPlQ9OQFdXKl1ChUGmXuHIvrpguqpTSj1z7Ro/dXjpefd6WHhoq0
    Data uGzaanOFLvnfCz+AWZcflsfen/nzB78wNevWPveLf/71p73PU4TWULfvt3ffc2fjnSO9vbAJqh+520q7XBIrw0D29Mvf2fL9j44qKXJ+NMpq6UxSnew0XxN3
    Data 10xXz2HZ08HcphTeIko51DlQyknX5FoVYZztabxZqymD41iAW8s98x7lp1Tsrb4kKkF2TqCTQlnLBDrjNk4CNBHQkWeJ0kyX9rOC6swf02PCVcHqzPGFY0eA
    Data MTeOfELI4HUa9HBM6fMRy2opJAwJeJCG6HaRCbSJWwZ9DpC7oMFXBb4zz6cbfhUB/NHnFzI/51876RcZDG8AvqhodGSufzUNQoxgUYydGx+rWHoi7WygvGaJ
    Data SbRVATb57tKsN5xX/i5FsUwCHcgrb5peyCsXrYcYnXQZbDiziG8+JwjHM0+orQrOhzJOrq2mwy5cjPUJAiVTpbNQKtAE8qKCpobCfIddyCdQTBEpUaMOZitk
    Data a07c9RvDpFw1NDwf23IFqiy55oAbzcjOncfEkWklONeyY+bQW30NQq/MmmVwa3zh12q1BC3JYfArd0/06AJlgdjHscHf3lmko/mjN9afnPiSP0tq3jPQYa1F
    Data OyCSAtH6i//6zNOPt983Cr8Ixy66rFxXpkxg9Iux2E0zptj42w6BTmTb1nTjcx/BVy+fevSw+Zdv/OEAA3YUQc+8wfrfDnTXbG+rrfYXu5xKBXvXjPdyI1Ow
    Data yEuagrtRzEKtaDGsoYwWenTGHjTXYp8ErJpJEJiWAJG1cjB06+ev2mLLFWbVbdu6aazTWBGeC0S3U+82RJfGykX3vc7EmS+9fDLHIuJ7Dt3z+a98efCFkfIk
    Data 6ND00S7mpvoYPQKUwA1KkQZaia8g12HT6+RxdjoH2puA7ftJ9roL56taKUIcN7BFo6Ywippl4Iu3ucvdnBrj/O6f/UTt3nvWvf4rKO010Y2H7l/37W8/v3U7
    Data 1FaRWk3RLbdee6T4pEt39SV3ndoCLel0J986CPnv/7Tr8f8Mno598P5POh8/cBj+5R+PrrHUTT7w472w+xe3PnH/v/BjaQ1BCe23veQIq+UVRs+NOp4SSl5p
    Data z322al7prOzL1Hxzpj3tMhS7huc/KbvC36e/y65wLaiLyiOVJWFfsdeTWGUeio9pSFIkSlMhdbfF1po6g7XO3AG0/EXH3srYDrTM5fdl5jJHKLiL/DOLgmpk
    Data m0RnVAIJrG1YqyT7os5e2jGzXZYFDPibjCZZBkYcGblRCBYrP6d/yGIFqblGETuCJ6eirbItp9xTPhcxNpqkcJTsXxWvgO5HRWF02HDpB5K9MEUw7vUZ4eVD
    Data K7oW1eZIRBK+TD12cEOsi32nLkdMi0PM62XCBociHVNUXVORL8FpXNK1YUM/xr5VNVZuLBKwb4C4KuC9VCuLoTLQCUKIdzjHna4KuApimbN3dPSemjlpcGLe
    Data O8tTBlS2lguUmoKdLSqRTKiM2HjeSHushXtzV6tSKBfKq9g3l4EnpQNDVtn8cAEyFI/hImppby7j3tzyUH5B8k0UGQZOv09/l1KAdrABrGfm3oa7O9tb62rR
    Data rnPaZVmoKw/dfuolEIcdkMDHIUkAyKcIVLMncQJlw9Dkmx9QfJzi+3kwmcVhBxgA24WcNo6Fqk2sbc+Q7CSVckLBgjVBAaqYJVvHT73+2vNbH91Ue9+DP9q+
    Data 95sbn/k2wxCxOcIwRChyWMaTO9Y45SqZFZFMZBkQyUTBxuyWqy/aylJMBC+49q/XmI8j7O6kCF9R19feLFzf79/tEt9w54GX4e1Q9Iu3RguuuGL5jCfdJf4j
    Data CVoJkriBAfPMT3c31MHCTb1NiF6i6EF0LhQyVQwlIQT5oAo0gemorjoabapuqij3Fed5nQ6rGXH+oXk4IVqPOgmG8wAkEGEEziN5XFqSRxJ+AR/j8RJLwJXs
    Data 2YgdAPZiOvsC2PmJ9UjyoSxGI+ECafifZZXAfvfAK282+Atzd++5tf2xG11PvpiRRGL0jpg/Hf74ZXFWiVvv/+zRF584eV2FqXvwkYUJJDrgHWmAh4oEMQp2
    Data +hunf01iTL6uFYyC9qi0ox1VD82mLAlNcXOEwSYNxhUOMQqlnyAg5xSBCOb//CkNSMlUbsBqi99xwrM5XK55hPkypEbNlqg8X4Cl9qKE5cmiAFucx+Yl9mIf
    Data PvhocdhXIK/pOgjVJY+/Un30s3f+qMzftbnXGFHQ2fqqm9c0l2bnqxw6qS34/Z+xnXNZN79QMd2oDeVk2fSyYr3/Rw98xpGW5CM+/80pY7FW0Vw3baIiwa0P
    Data ff3+k5VFI9d+VUZKoKVI5RP6O8Z9k0X1pZqAZObdNraRTrgdy3oUOuoUMrEpUpx3Sft1hhM3YMcXyPQZmQzvH6hOEAB1iGsbbedcr9tpMWdJWK5tLSSwBIsy
    Data DlB9DQ2fpQb0ROKIDjFxvMudvMWwjSOaRBSvnjtugLPLkM5yH87GxMLXcZOSD3Wq3FG/3AUVOh3Bw7JJ99ELHBsieFaWtVAsVDPhfOHdU3kynd+PCdNSdmwk
    Data D99/vEiZiOIdXT2djTI7X6Vs+WObqnGgVFvCVxi7qr4Xm9mb61QQm+drln2D6X/CKAWDwQkwiPjMUlGIzurKOAozAJAmKIKmgmlATPTb8FcTjRkSeKsFyU8+
    Data Sk8qrR4oP6IXnoM9MvMefjchZHDZAhqj0vq6uchEPqCQQ+YsKCmCJChyPjhRJ46UtNMrQWimYvhKYLox3YZnDNOZ0wuZKu4vs+kXGaxuB91R6frJdH9Zug5i
    Data FItW7P+jz1yaOmr1IPvzJbJ2q+hVNyyZumO9LNFLWYAOdCIFjY72hvqaaHmZ16NSsgoaUghBgKtSEhAVqIMJ5oFkCMAyenGFFHpOdi0ltwZWgvD3zy/IFpkE
    Data cpHBK866sMZDPXWASaXZ9V2fdK4A58SzQQVfoA21OvrDcl5kS82uiQ0GSyGTM+v6mHemkAcAQxEV8R2ueocYMBCnX64X6WGRBMuAAQFXIeHoinDUBIKKkMkx
    Data BADSiP4TNP5sM3eyuzRZoOU6HELISGzAilDMGi7xV+l1vU/43YaOiNJb5OiaqHE3TFZc+P5V3o7m2l55Va7ULmsrqaQGr9vrHWkxN1ua60vPv+tUV9TvdnTi
    Data 4nQFm8vaD+VlMUms/eNFk3tfuffaI9vcfXWByRNH9mwy8BVSu8fA93T0P9pdHd5/39YD/vWBQ186VrxlqLF3JFLIVxGfn69lk336P/gHVCcoBc3ohK8oDwdz
    Data 3FqNUEARcTUNjDEcwJErQIZL6m9CNNcyR00jHG98T5oP2SNZ3UagC3DlcJTXTBot/ZjP1iu9BTn12pBL6nDoSw+u2/boqd66ilAoWydlcm7tPoXbEYzoqqL+
    Data 7ZcfaY1gE2kn/FPmoNzt2eTJEomNXbXOrnsv3zU63Vze1lnoFZJuX6hAapVJHZvb2jpyN9w27eyOBAh+2hGfQNavWGStBU1RSVNjKrJ8cWTVQdK8PHTxVgdd
    Data mfvfV4oz7IsZO+LPEnFkIHObPIs9AlIBUMruXWlPd6QyiT50MuWVsVRjy4IgfTYQzNx1f6ZgfDNTH/4KUPnXjFwFEBTGAsR6SgmGQDQqammOVAb8DjtNkshq
    Data FjOGbAUgujcSkMDRUY6AiM7wEGOuxDnObVxyLsAiONc/Hx+5C6lWAlj0l/BYZY65pr9Ql6sVmr1Osb5guiTfpGLgdiYo3T5ND92xwd6izy+wkjip92b7PG0O
    Data TUfkYKuOwVfTGaNz3+WbpwCAIDsWwKcIIWgHkagcAObP9mhVcaHdZjLQJM3moiH6vhS2VzaFxA0tsMctmdl6K0EirKzMsVQjk2WLWJPVTBl4y8RfWeEkb+j2
    Data uZbS+vJGTI5uTXj5uLt5vbOT83ykmqvYIs83OpLm+QYhJFnvh/13z9WlVUtW7AEHl4gIz/bkvW3JMJDzhyyPaCno5zSLUs/i/FaIkdA66wlRWny13eGS4jpn
    Data 6g+vyliKXYFDVC6us7ONuZWLKQXwgSYwElUiZbCiAuZW6DKbEG8OxzAcUSGBHQPEAfKOaCINzrZ7ABIAnIxTtBDsPmd1d5L9HmwXKMvPgc9eZrjbDJunpLir
    Data EHcX5PLtfh+VoYXmofYj09na8370d1nQKlTzddcdbJro6/TsD/g9EDflFmRZJLUF/S8/2vfW4XKvTGHiOT7B5/WR16j41YOxgzPRAVoskDso6Vp4879g8xdo
    Data q7U9y5Blzi4+f9gQUIhMEgN/tPqDOxbu0aCZ+7Sb1LD36XVonqu+jp3X97gddos2W6Xg0yAAA0KU1xBBir1ZY3hYAAFqEKKTM10UheBIEhgLSJpOjMfOoUaD
    Data XGsflyNOeEp/sjGU9vsQ0w3zXVY70hlBRlx4cGjUqNR3j625L9f/+nkCXJFld3RuK1CHHBKLqCU74ipVlMa+xas7zzLUFHLHfOmmg5Tbai4LlF/3acwrMPFV
    Data BrFxxi6ViJlEr+4+CKHq2FOxb11od5ucphksQ2fRxac/JDsYT+kDfaAvKm9pTsUbygOVNUCCnsUcbgU0QYcTwFsEcNBOrwhrziWd5AqAF3tsCf94hjj8cBm3
    Data 4nEm/kbzDAiT/ahrqzo6H5EClO+NI5KH88IsLPkJWGaEI7TzVohExUIN7tj5Z4DEP8/veD8DKOKXLdz/DlksIu/XCNZEs+trWbYqP6skZFYps8R8GoPs/vXi
    Data GKCKIY38H6oU0BBJ2AOaj2BIsmcI1/HMWI/HWI+Tiub6vJOQpGbFgDCKmoPOUDihG7QETlkUmvU1ys1MtT2/fs3V5zfG4XjpZrN58NdyRGD05+GhRXGJYNe/
    Data 7ne1GEtHFHtr5gptHH4DsGF8K0bKWakgWPy1hXGI7OY+/T7r+5rAZNSEPF+0EikI+wqZngw76lQTixisNcEmMWKj1kAegdIJBIkEmDAhBoRBPsQFEAM4azQe
    Data D+1jBhMEwUBOKFwEctxpjBAXXgp9Ts7YjOU5GCZ9n717nd5Y4tdLZPbM0KuHeqS2dHxfFQvBWbzthToK01i0RYWFEwsD77MUa1r8a6Ms/AAGPEx0eDv9BVAP
    Data NoL1UUtjw+TE2rGhge6u9taGjY0bGU3yQI5brURRC87ypJXLIUQ2gwQKCYMMnoCI5olYAiOhGKMJVrKG83zknE40EcY1n6X3kIXCNGCDwmBAwXZGzR2uR85S
    Data pVKypcIwmBUdx5ZkgiLoRtRFtja6t1EXbpBuir38y1uKsq55o4LhLxNZdIENYbndXuq89fqmYz+oqjoMqRxli0/uVa274pbYyaXojyA17lAPVXj1xooDaz8I
    Data 3B3zXOQxuy6CXfDd6+psAiXPwuS7VFLrmotu6o09E/vydNY62Dq5/Vfw40YH03CY81xsT6yFeGsRHqkixlu+RYlBI9gEtkQdzU0QTK0fXzsy1NvT2d60qXlT
    Data Q115WXGR3SaX0iRohI1ZKBYSoVPIKoYAR1IBQWY9JFhcIR6IRMndT7JTq2iJ5oyt8lIIL5CGGDWXDMHtSl0XdPQoudpiPPmbFMdecnXwv2gGyjqitsre2y8o
    Data mOq+wCMyFuhDm/Qiq4gZZp/0S606r7Y+22mWF0Zaulyt9h9KLLXM8mT9S0EW5JbEDEut0Nc1g+UuU6hj/etvXOnq2Wrt37yn4uDvchUK2s4tTdf6vZH7A+31
    Data JZq8lonAxgg8IMvNkpjNMfzSfFmbmSAyMVgldswIqAeDoCWqHOhqa07dJXyUqUzdJUhzjKB5mTYGtWobY35z2NnshHRxrFUBPulPb7Hi8H4fOcLgfRh0ouhg
    Data qKejNR3jKDoo5DDOT2AcGRZnDbsgqKlVBPU8y54limf+lmbdswYtnW7YWbR+l0HrBOiLZiOtnK7OpsZUxKJIIoAQm1Qki+OWD/47Dn0RqquzALArc+Pbqrjw
    Data X2fqi+MQ/Rr5ZwbR68EwYqNcu2ZosKe7pTkd1SIu5kWoFiY9N4j77UUcNr162MYzW/8sQd6VcQXOFu349zN24eFx1L/EoL6fsf9ANHsdY/3uBXEfYnBPI09N
    Data AxohngcEBE+AjkyaRrEfn8LOKfLls7bnmEFJ1vxcT5VKeRbgv3T/+YPmmpHJk3f5ZQWxn4iL/HJ3o9+bvzrQ3+jXZPsn9w+u3eHausXBc5miHZsro8M1yPoI
    Data +9up7Qz2B8EGjot1Yt3IcG8G9JdnRD8QCFDUAng88TkPWhZfh7PcBjS7FqMTJ+/yybm1ULjQWpz1JvhzpmVI7IEfgnowDLaArqhhdKSxYWTL6JaG4cbh+fsg
    Data L+7/eXycJujZE+AcxyuKpdpWz2IDRJdqY12dnfDMsrpcuV3xHaoVNIIxsBu0clzgLc0LRTm53I7gQRqnw7OxzrmOcLBlNMee5T6wLrdr9qw3xlfOoKcWgukY
    Data Tf6UUjAr0xKVD/S1tVRVVjCr4nHrdVwHexGJiiQwXiTBIIm6kADquwY5OISQI59I6higZbGubFOkTNrTmegh8b9dcma7gd1GVca87ANuIeKLrIQJmTuPtdr1
    Data 0orxfyOzh2rK9QrF+BaLOUJLrLkJFsmiMQBBUYwmthNCsAZ0IMv2dqMzuKjQzLKHIssGtAzKlRBDrOoYSqjiSB0wOEsmmkR9UpiRyiTM6DqrHZC0OfbTNEpN
    Data a81UQbRvRZgXaIrk+sobUrhJO6YKx9rEbdWT/7MyeGvKQkpZyaUAAwVMN/hrTF/DOnAYjEaVU+sH+lqaKisK8jVquVQi4qokVaNujCSgdZwVX8N5c6S+SEiQ
    Data nGJdkgJCiPqPkl3gnLx4ksIvbLdxBuash+pMiXbvMIBzUl4UntToQHXU+NEqIZLOijF95g4IOKnc0KApcEb7iyYnhAc2MoMK+Zve3XpplyHP6TbVOZqaDJWi
    Data zTAKS4YGJK5cRZ7A/q16NU+k33bs8ouLd7Qoyyta19tFBb27j9/ccd91x6pLh3s7v9aO12ZskFAZS7x9jzz11dcnXjsVUhTuaGIqgXkD1dsjx+pvvbXpvl2R
    Data /CcxAhItXprPV5kkItcPY8f2XelT+7qufnnbez97eK+pyPdU2Rfe/sk3Hr/Gt2XTyInf/mzN+tNgJ1WwWOdEI3NSv0sFQCtboZVPre/qbGTY3lA/v1wGAdol
    Data BS0GDMegtQ1SZsZikEJLiEMMDxJwVkQFc5NwVvuHYC2MdkCoAJu7AbgOcEW8MYLjtVxgkdgJPBmzRJm7KaDwDpPQoxzyqw1OqzG7aLpc4TQXGsPVkrV3PHo0
    Data S+0ID+KYU+TeVyKtGelhzu9qtd3ZOWGXlK0/0njr0bdE++EfMjVYUF2G7OrDwXdO7HAbNfa4H9p7tOGqh3st3/gZVNsFMgz3m1u1XX85Ne0SFfZsj1y64TM9
    Data Ortzu+eBV1+8t/wiyLTvP05sy6wQAewzb5CnmL1TD8ZBJCppaS4vYzy+Vijg9JZynMgDWRsSLUEENyDBCXEk5zy5u0C6F6dxDuaobONAxl7yIFhkH+Bp7ryh
    Data 4qOcga2HjzfcdIXOJ8hWQuW/myoblBtiX3/zJt9CxwH218w9QXSqUz9630zjlq9/5albSvZn50l50/CVY9e+WHp7rPSox+g6BHvhu9fXLnQ2kH9ajEMTlM38
    Data mNjHYLwZrEezgpMTHe3NjTluk1Gaxc4KVkMIoLUFEmZu0gdDTb2pzGdsW9ryvTpFYxgnsDvL1cL8S/SNaF0yo7ohs3svrWm0Def+VGxl3HuOWf3F3X27ozdc
    Data Umz0jU7kH2uOuDqM9qa1kf1rLsDezYTricV8vaOiM3dHJzwpZ3y9salq5mexvz34SP2dQkJu0utVVd6LTh3d+1d74fgB3zpyIiOwAR5H9oPs/XcDqIxKx9d1
    Data dSbRTSMGMQ7dFFwM2tTqQBukC3CnvT57gBOKFPLcGC/l5aph/PJUGbLCeapkp3/HRPmvxtkR26NKpLqZ47bbzAxrlVBAk1x2uFgNcdIPIY2YiggeS5pPc6T5
    Data XA1vNuGGwdlw0qlERmbO3iSbbhgqaCJ++1KzeZuk2QHy8844DTSWTov4ilOtbX/iM7e19/XbxUbNyMUd98G6vx6PffL1rSZDwGvs+OYtlVsKKiqyS1su6onV
    Data dr9Q+zLEGrvD2UU5OiKtj9WWJ2nwKtVGdXFD5YmH17wEpyH0r3cVOncV4CqxXSXWCN26SNOeG/pjpSOvHeqz8/liXUMRZUrvX8VP//H0PwiM6gQlyH5oFoMh
    Data PCgrLPB6XE6rWcCLz2IYIMaKYZVCSLERIx0GBI7hBBZkBwRQZxyFAzThhiRd/FwImSzbJXpmKDuyHafBw3oOxkmwzgHZ05duUGRPuy29vfW3j5xcFyjMVirz
    Data jC+/Y/QGNKW29qmy2+rurBcNXefMnh7U2Us9W++PagtUXobdqe9Y192fa0AV2cfS+lzvKwkW9Si0anH/lFiYrfSt37e35NrSy6KKnCyipcVoVYltUr6cp1fk
    Data duw8v/rSOy/F8D+m9bkm8fdEAn+NDSvDH7bK+Evb8GeFv3Upe3ul6KtMI4nn0IffRb6TRF9jw5LoWxR4LB3HaqIvzZGuHH3/SbHhmWOP6E1zfQnkvcjYbi3o
    Data jir7ehdCXrALErwE+vD/kvdbehjlrPD4+SW6aVaIUHJm6S5DzmP6KYrF7BAYjcp7e+ZjtroD4vRc3BL/ZdxmbkFcOYQvzdiLuAJX+u9F2V9iF878mniCugV0
    Data gOGosrkpUuEvNhv1WoVMyOfR3B27EkAI7JBg1ft5gEeDICAgul6jTBGPplgRLIBBSACIviQZyGNuliueu0LEwysW5TjG3r8SCWk2SaRAdk5wDSaZBpGV54wE
    Data s3O+NIWtr26v1VVSZFldYddI5+AFpRd8+sqB1lBhKKfn1jsHFbJsjdVkNst8lsrmex7ec/EldUHSZaiqKXNZcbzkPBkt1bd3NUb1IfxxPiP+Fi0hSGHsjdi9
    Data sWdHv14RqGgqzvU8hk3Cqe4qmicUuZxFOTKrGr790pENXbt50vBkzlieS2Eunl6Xo7QERw/VHWewOsXkKu5nYqPN4DC4CuyP6i68YN+e7VuHBvv7KsqCvlyP
    Data zapUSMQ4RFWBvgEI6C0Q8ndDDDZAnGQwK6SF6N5LQowxKvMOiRNBAURc3JAEyLospyMP0jQ7yB5/kWR4FCAzL3taPZFUsttImqK5N1wcT42CpuLsR2H2uqFB
    Data Pyh1Kt7psrMU6yFfGBHKzp8joumpV+4cc8pVUhsaa5cY0Vh7/rS2+ZoLubH2wIXX/n3YYsrOa1b+K/b7KyPeCEMmbxUR/FKN0xt6Wl/hiDblqDSXfvTq4A8q
    Data j/7lYlf9pipmYl7uYSbmZ25Rmo7/dsNG+uXnXJ3hXQ7YlM5oHVzW8Pvd5fvGvcxc9p+2lvXv8srMpEqmd/dDByE0mfRFsiqz5TW4b7qgurbYlTZcf4iIvX78
    Data caEeYm3Tn7qrGI6lR3sEKGPm5ymqE4RBM+gBkwwiTK0t/X0tPa09dbXlpUzXVjFzQzRo1ApWVRZ1bHX5AcYHHVEM8h0QhzYrhuEIGIABBnIKOBYUCbA4hwGF
    Data xe+Qc17xINeaSbKI4FpYFxuhZw6NoNvNXhYTO4zlQFCjaxH72hXfcbOz9c60EJFonFr/6TJFcrpe6qzbu9PFTdc/NWqRUtL1t5RdPGqppqj1oydO/KOlPmiv
    Data /+S37eUDF02YVURQ3rhuuN1YiX9h83iv7+HDe2Kd6cSFPwksPHt/YW4FzPZ/ucvsz1978plP37theudO40zLY1M3ftjRUWcR+of21ZyE34mP4/95obmpmffQ
    Data ZAriLEQ6v329c2fyWJ3fxExeYmyAG87NND7Ajeiu1vTUghO7Kx0bkKT3FJ/lmMAn5Rn6jLNn3mOnVUpBJ5p3bmmeO6fHzjsnpgISxswwGMAZ82ymAzIoUp/h
    Data XMB16ZY782mAmXsWMhaM3TnzA/geYysDMEcFEAmxGqABZ2wkns0QSaEdQ9t4UUWVjQsLpxDCJfVRIFAQe7F3WP0XzXNI9AVjPl4QF30BBdiiOi/wx5kyQZk1
    Data W9BnthNbUPkN6IH6WbUKMp8olEFgQqPCgHvczCEVzM8YGhEXLhLoQDBEXIn9jula0QN7VKBWSbNQ3ztiuZMBDCE3Ue2KW3wRfbqnMjZ+ELdk1JgDJ4gX4BfJ
    Data d0AWkD0r4DNP/Tz3wHAhabUDKXcY4hfpNxIoJi7CS6jtIAsYgScq0GrkMmH8eTSzz5P0U2KcVf0CizYCYM9nKOUTT2cqxSO7xmhsGkwzdi1+IVuFSl84SwAF
    Data Oc8JsDDgjinXrKga+sNBLlToYkmO/D6asXIuV6ZKVPJ8PRsDuxpdyrzWr6eUm/p2bR61NdvLbkW/y37iJfxySgwsIIB0Kg3M5UIuRTc0YIEW5H10ACMojLNO
    Data XNqEXXSSNY9iqebOh5dqziRezdRcCTAgISZwJ9UKlMAD5M9p1Bhkdttz7EbjQLB0jRrTLbfKTFx+JlXi0+8wdRof0QRqkJJ3eVkogDRv3U6b1WxkgCJHvSpC
    Data xMAgw9gTkUZjInwM8IOAz8P4PD9jTIltzmhIXFkYS1EWdrklOLfQyIexTffIQ6f023PqzwsQcipC+M4Dpyw1zd2F0+f5XXbF218JKS1jOdFuX5On894rax68
    Data fhcTatoMxqr9u4uz5nBsX6nAJ3nKvm+9+lLTvV5XtWA3/OAGLNtpUbeUym18KkuUJa5vNJTzYrGPukKhWdrt78JtAYCDUebu/zr9BAiBBjCAprW6u+rrGCa4
    Data gMNu4FjaaXZaKw/SEMZv/QBDEg5BgKpY0I8u/hSkk7MzgB2doSiSsQ9Hh8Bn8/+hZNk2LlLE4Y5egLVCzZkSadFzYEV1A4pK9yS7VITHO/GwsapSU6a8ZuSd
    Data z7x83LP7UMnOMV1eQNkxdT3UB59+sWR639ZHu7uJ8Jrz605c++oLbQTWeMGx8EUEvtvmLnwUq0rxRqdK7vnVJdDzrtugKDK9/bz0khPfvNwp1Cg9tQqNrKZx
    Data o4ksKx574NR1l/U6jtx04YZ2bQDrefnquuZQwZbjh/PEBJZHDKYlqHDQzVh5B3UzcAA/qAX1UbnLGanMz3P6XX6bJVst4KHdm6uAOOo8BiQIc0OFBBF3cKiM
    Data juMJJTyMG+QMhFKkZNCNJaUE6HaRbJwb4KJcZbrxumV62a7LbMJGrzGIa93aWmvpYHjHgSfLFOG1F9bfcOVVne0we+3EKb/M1TmwdszZjdtSc/OGsi3bQnAA
    Data 3vtHRnFm4sDV7d8Yuuly38QLVV948Z5L8tf29bn6Lvlrx6N10c1XNt/3a2L9vJw7qGP2pYXLeSLu+saGMkZB0e2yWlRKPo/jrqcICk0ToSA1CADEAPSn4Iyd
    Data geVwxoVTHMwS/QG4BFMpfRFiPtzCIQuCmzShZ5dunl5+ecHm65TmvNpglkn53OV+Jb9v08EjF+zwjH617saNheuqZUVhtU8gfO5WGPhP+3X/83Ls40dP7uwa
    Data wG9JMdOfq39zFHr/YCGZrag1xN6LHbh5X67U6m7cePGpns8++bdOvUJmKxUYVMX50UJ3X+PxX8d+HPtT7PRlA9X1lHderhP0MTYrZrBUDKKgC2yNGvw+CFpb
    Data ykp9UX+0IM9iQtxlFAmKYTHyaY16SOMhEQZQypjgEagriSZpQAYBO09E02zlKF0NyyaAFDX7CnKdBHMJhOeeaT5NHIZx90ZT3MlH+ubcsqTzakmFquHyzYwH
    Data 43WX7dv1dFgZYjboDX9ufGLI6amzCy3mYceYJdxYnK93CWdGpy87ESaC8uaJ0U5jFQykmPg19UDEm62r2KwquNC/5d7yZ55/4JrA9nGsb2uem0nIdfxpe+wP
    Data m1/8z3GRnIq8+sa/metTcORQ3anYO0TnfExGGL3kDxlMtoExsBdNIg0ONDXWVDMabuGAHykqatQiIYGBNtiG7rUdtTaMT3ggyW+AQIBbgZAShgHJF/BJQXCW
    Data 94gGfJLmQ8wDBFDEvIWCUE9cd4HHWDqhw8pnDxIuSzdHiEIRl9hDnnCJeS6am1TkCqny9Oz9y8aq3LU7tOeVHLyAI+U5ebNQLVVpBWq7rqg5tG6wx73ImJex
    Data /PmjVtPY2vVRbRmWlbIE2LUouSMY+Hjw83kcDc80vBenKDFf6WKoj7Mfge49C89+qXJFtdtjz2cp5e49hCYd79jpn8R+R7TT/wOqQCdYi2Y7qyIlQTSBaDZm
    Data q3lsj1ddISSRKUmUwYM0GoQncNQkA0iKBBSaPwyhQU+28YIxO0kSKGAjCM51oD8cyPI0QrOCtiUsHcERrNVzevQS8WTCwkq7O57ym5fh91S11tk7Nrm1VJG9
    Data oYWvLw43FG66a+8VVcHqbiPPFFbT0aEWtXrtaG9UV15y8aahnu4NRfmfwzek2PVN1YVf+f5ByDN4hCaP3KEw9D33+B1lF12wZUd3bYtCrJZ7G9Sqnm9WNse+
    Data LFHI3HtIfuyXsZ/F/rWmn35rfo2TmbGjr2CwPQq2ctzkGzcMDiDf4XaqETHRKBxluclrjVIckJAbkuXBhBmRw+VOJs434Bg2p6FdSc1e/uXycJjCcQxZTYq8
    Data MQIzxBLRUYRAKJ5z6aBpDA9VYSBR/8dRjMSlo9MLUDSl1dl0lZd3GiNTa4pHTr3zj6/0dpv0fnfdts2XTObEYvdNNvY0mEX8iN83vqvy4qe+VFdTM5jt8Svy
    Data 5DxN6GV5MLT7AFT8ubjGlWVXy68PDEgpT5MnSyf8QapPaSxwyXKs1vrDR+EBKDz0m8tiA7FQ7OaGWlgN3dd8+O/y1z/VGnPGflHvy5ObcJnTffGzO/8I+77T
    Data nFMoJUVik89Uce3M+7G+0W1vvQNjLVKZ2FIbiA0/FPvV7dXNQlpEkW+mxwmIO54Kku+AnWAvOMbNQV580e5dGzeMjXR2tDQ11peVFhWyLPI74U52DvK8PUHk
    Data TKxAACggQFAnmf/zA6FQaOOJMK75l8AxDIv7d8zNp7lVS6rmLEEpz7qaxHIhQV3mm5I1mPRYFu0htElSfzKSvQsFFL5geL5TKiu7/bzAyMJU8wGFS2mS5SiI
    Data F5ztuZ3X9EWFN/a07Gqsz/Y3hbTNc8LgvIpKqcUTunqLp77M7FcYfIUyd67l6tYyCLEvpqws3vHIpTV7luKedw64yb/kDlvLh5qqHrN9ZbI3oMsu7u54YHPw
    Data nvbZKPqJG648z9ZgdT97onKK/YFSsbF6dwPMeXA49lvIGP2h+Xvwc8z58m9mD1aCdtAUlTJ8RiUFDCe9yYjy5zTafWrIKoNykQ8kIeIbhYgMjkwPsmd7xkJo
    Data Z6UQuqF1wudF2clUnYp9z45GWdM918+vrbXJjFm3fK58I8fXZpBf/M+7Tx3LP3BBxaF2ZZFX6gwM7Co/+JevR4/wj+978eGbdGLRCPa5FCsfOyHVIS42xSPQ
    Data Uo+42HKySvJj9+TddeMdNxYw5bHiGgWS9S0rdIq15b6p26UvX3dhUw32DlmbvitO72F46/XMrsgHEdCB8m7NTeVlvuL8PMTJzKPYvJsVEiSAOIGo8yjWS6XQ
    Data ssYLNiJkMilnsjCFYuZZu8X77UJJn5SMpS3yRItSkA2m0w32MdZrqPJYC0tNfFPnsaCM2aFqQzZhRPiVOQiB6MkvfLpMGRo7XHfdaRDK902yQQwbVcN9KVZb
    Data lxV7u+8LBY4Sqbv6QfibL9VfeMGwUyFxmoUqtcLtbA9/+HTkmWfvvdy3CX7n6r2tbBhzWVNX5YZLsRNpUOOYs9bTTyQ42Tra/w9wsqWbbsVMMc+k2O1seWF+
    Data lga6OCPbFPlOgpGtrjadkS2d/2X1GdnSjHXGpC+xFButgOhlzzxMdTDe600GUz5QA7qi8sqK+UwboSSzyzln2UiPDoh9Z07mkmKkMyTQgKVpwMFAFPEYUJMs
    Data V+9QNBup7C7Ml1GSYHDhYTwcCybZMtDQ1DmgzEg/d8+It+X8FCudAVEG0ZeGoUTn+3fBOrCTiejVWzevn0jvfUf2icZ73wGfxw8vv/kd+y81vyvRC3d8G2vU
    Data 56rl/Xbvpk17cFMw4haYt9nOVZs7+ayzumEwZCnLu37MCGY72xtBK7NKrVH5mtG+3vTO9uJ4ZzugaEhTMJjoa2fAKrad++Z29u+V7PomVmEV2tpf5Rtt2kC/
    Data YxAzhhsDCnqVutnJoofVXXUhi6KieO++EnOphp8yAbITDEaVkxMLTYCUx3fBCqY/+Od0A6R3RZyrDXBDWv/DOdsCn03vc0jsgk6W474acdzP3QMo7vQk9sBS
    Data gx3UamI/vZV1FXBfkdaYsEq4xy+f15mAx3H/IIv7i8FIVH340JbNC50AFXHs8+GZef9zD/4l+uLP1WYgNi3WLH/OdsZrizfP49w+Id9h98lWpCW0ccNAf/p5
    Data 4U3slfg2odm85znfLEsu1VlvHuyvi63Kau2kvy2+CImpKJYxpB95q+7O5sbUyREPNzkCKJIK/1cGo9JjpLOeFsHrU8KhVRsP2TU3/EnMPDWCZrAG3f1Gh3u6
    Data 5848ubmZJ0CQgCQAugGKbed+7Gl+vLMqA09fS4t4VmvOiXLOC3kABM9gBnI/9n0gBl7UFcQjgRiKUEtFNoBsA4Ur3vWHof9C4OA6F9B1WWPlpuX/VGoMGAlv
    Data niHgxAx5OfYqLMdrrwIQfB4z8Miz+9n/RD+bejrP6E/92VBHnMQOU3cDNdNJJBLgMNFJBMMg0UnE6R4mdKyKkF9SqzG7bXtJjbGwytrE+DRM6e3zyYhfdq0/
    Data 0NdRpwtpxOq8gZqKboQbN3Ecu5y6GSiYT5CIsfmfwF6PXYmGL67dC75Y10Rgilzmp2Y37kKfQ3zKOPtTS73rD/Z21CG706cJOA2VQMX8dKmEptifzkmCQ8j+
    Data 9HAAZb98CdfJftJt2brEz7U1lueyn8S3l3rRT57zBACDDuIkEWbsIwdOkBOVKxUQ6LIVTqWTJoEcytGukeAQPQ2JEXjCZMj2nLloSmNi0G+d9w7u2Xd836gX
    Data g+rCozaBSmxqjdWlv0OcvPaao3vQr3JwoxH9cmkvAQZziIeJcuqFRX8/9Jshi+AwbhI4S8+QSA5a577jRu/g30j8NkVjQjn7+905+87FVj56h3qB+4VUBYNb
    Data WOMlXu7bbOB+P7Q+2H+gEiiBF5iiAifTfaGUCwWo40mEwZTfikzDAOsdFlg8+KjdkQQGWkBp7NcLLOh/RB4WLslFheQCawzBs6dfJZSUDOSCiqgEAOZPhpkR
    Data 9ZohtnOXAQLIgzjiWQAQgwBNKKBz3g84kgXWJaK2KDXn2+ScgBjTtcse8sm+kLjePIYx/33W7cmusTbb7+1y+AaeVEi0AntLpTZMUflbQpM0vTZyXGe+aH+T
    Data tSncPLC7+fu7If8qsVduruZJOhqOVAZkMldNyyM8odzXlKPl9Y3G7j0NjhdvAhBcHn+WMtARlQQDxUUuJ9KsoymEhBITmqMSYChxRSCuWIgCF5wt5+Jo5ILA
    Data AOGfQ+DOyX7MauczkQnzSEs83+yXKN3FmgSO/CDbUndRSJ+d+XH11vrp4ARNeiNfv2VPTq3DogjBP2f3yrNJEfaV9ROZH3543ZcEQnXA6wwLY1dfmNvc1Rd6
    Data CEDwG/hV7A38EZAFsp4Vor69zyJvk9q1x1X2sXVDUyPW2kBOx8TNIWUu/CBX4LTW37fOVHGeZ88aFNcXn/4n+Rr0gCxgBoVA9YLLqVZRJM70AmIQi/9Ux+wp
    Data iDYS5VrqNXGXzqh2D1bqI8UF6yMzj3GvdFWJV8m/KyltLZdbRGJLRWtJQKDNW/QloMHtp9/n/Y5So3IA8IEGMAI2g4NgR7Rg65axUadDLtuze8vBrQfXT45u
    Data Hts8MtTV0VBXUZaX6/A5fQa9zC63o45EOUJLFiQkEKMQGjw8SImhUEAJGQDNbcGzyURSXID8Oaq/zUa5DrZDLAy4/DANKTyZRU8Ywe3jIluuiwcmz5j4ZnfO
    Data fyflX1hT//3nOjeWiXG7t297weTMJTfuDk84Wva3bPRvXWNvPW//vdW1NTaJOzfPK3WQcqUx0Jw3uenpMqXB35i7dvrlT34+7y38kTn/ZGZozgt/2GLsUhCv
    Data lu/Xua76JPRT7G6rtfHEnVcNuRRWW/PVt564bsfVV+zqUPJtZe1561/45KeBQ5smBywNtwcPT48PWOpvTXud8t1zvwYQXA3ex/5GWIEKONHZLxYCFVShs19B
    Data crEaBLCEjdGAHU+4dhImUu0QwQ6GsJ9oq2MvQKFMJjaaS6yxP+YH/PDLSsJqbik+Hfux3JbNV1iho6g6bIZyMTo+94I/Yj8lVAt+Loq1Wd9dglq4IPu5YXTA
    Data OWFiX7G/AKSu0VbDptj78Y+FauZjY1XKP5pai2Mwl/vU2JvoU2N/ZT+17PS/iPOpvQx2tc8JeFysIErECiKuzTiEoDWLLIqG77VtzsZznM3ri1pVrhmz2+8w
    Data OclN5fvdxXssVR/r0c/dw9zj+ikFmAKT0eyJccRZ2lxdVVnK1NrsVr1WrRTyUU9RjRhSEIM8kBS2QFamAKRYoVgAeHyApqZSXCSfL0BfCwQiAZfDDgfnZK5n
    Data v5wdj5rlgkHwtVFsRY5tK+LOYHv6a7a8wqWjVLO9CFAArxQRKLcdEck38jH0VWDvT37wjUtbqns764MeZ0GVrvTkDfUn1mmU9nJfc3bp+tFbQzKJRuVIvMpa
    Data axtvaXT2W1Ryc7Ro599P+bfAB4x8gZlNfDc2CQx8JfMF9L7y0mjn7n3n7fK6I84Ka0tt61DLLW9BlA+POiwlrYmvftOkU7mNtVqLzlbqCar95eG9AINd2C9Y
    Data ddAoY33eczXDGiEmZiMktnsiMaeHev6QmTiRKOY5VeiJORtwN2F6tpKP7mAwgIyJBl4S9JfcdyXaTinmx5JXXuqirOY19c7efpumNmp7KVs8eJlZz5OLbTBb
    Data t21jbdv1Qp3TqpDoQsxF2VFsd9582ce3frxx4kVvibbAUjZZliez5m2qszT3lKry7z//UpENe/NXV32YV9r4w2tDB/Q8Ye3GKNxdINttql0f+fR0llHi8v3g
    Data q/n+Mmf05Outx7xamYGgrUf33f1mYS3Tv/IB9uANQhVG1GkLIYTY1ti+cTmvSqXKH/giLMbkCKlTAFBjjLW6AfWsEUIxGjO1xht31OwokBqRg3O+1MkiSoH2
    Data Gmu0ENdSGi+0oZgTdfpIXZwir0qqMGHYTDAW3m/kSYT6cHf+xjpBNkZcfF7h5rXXQMOa6Vcf/k+rtryg7WSdrnA4f6JUp9dJ5Z6uWDgYbmptMUbrIsdff/34
    Data 0dcrxsT5+WNaj1fqcbuybJASbz+/85kffumB9zu0Eydjfxj5yVc6jhwqlfIYXFo88hxHlU9VkG8qH9hxtOVhqPpBxyOPPPB+7LexHw6uN6ETtoSp6N5EHQZ5
    Data oBJUAcELVZFQvtKEMzhB9+94oSzl3JahyYpk/ZV9D/rQxXL2SJDCMP6SU6OwlxwoEuAEr7lI4a0NbLn48PCF6x+OfXvbhcf6x4b6r73pkRu7s2e+d+imy5r1
    Data vca1OzYMWxuwExj+8RXfmDSGrzcWyKw8mdR5YNre0rM79q+PY3+LFd1fsnbTS19//pkdu/u0n0y++c1327CLciQ1u27qez72LbyLQlkLNQCkm3gDyIGKwb1S
    Data wQcM7v3IdxXBCIbmIyCDabsbwsNTUizXNuKVCRQW+Ia8MmTpi03HDIVqM/5BuByjRdr6T3TOvJwd+MWfHAEQ3HZ6gPgagxE9gxEyi8GIJixDtwsrci8ydj8E
    Data wzIr99VtRG9nIfZusyw7VDilI2ZuQX8c7CYf8JQERbK8j3hlDlO2rpv83kc56CuEwYbTNcSHlAIxtD/Ls0FmFYogsjP3ERy4SIg8GeRec2hD34HdEIv1HwuX
    Data FuLYDoXFYY0cmnkBvsg3z/wR+97xbB5fqw52fPKVaKkWuxtuPf/rhTmlZaQgYuRlYTPH5NlFRTP7bSK92SZ1lHm9+eh3uQR7hNiPlwMxelYAxWn32q/Xq70u
    Data fKhE7SnAHim2aTxYoV2Tw3Y7vk910/eDHJDHWD/Xo7RhYnYoJt4Cxw1iK2Ao2d8J2edgMzu4L0jBit71hjy3odGUr42GtRHyCJ7vzpPmykiBMDv/kydpmUBr
    Data 0hRqKGwbhLixN4vUuc2GVlvQJBbqzDP9VE2Do1WR3+5dV/1RgKcWqL0CGm/BIML7Dvg5Mp+wAS1wAT+D98Icm0EMEN5RoXjuuMfsmQHV8SK7jaJogjsRXJyQ
    Data 3ulPH2wLTj+23dmydujgQxfe/cSVXc1rhmOftawZatCV+7uba80zp9Aflih6S18GP/dwl6m0KKdj//UTY488MtS1l/kTYu6Q3VBx2G8rcV7kt5Y42nLCzGsA
    Data Tw/EdjO8jZeDemYNZEGEN7Z9nfEt8gieYeiX2YI4d9mZbZMIhOL7lOraIKHM9b2t9RHDD+sau8eCu4+cX+0nLNnhcKnbguGRXjH793URfZ32/vGeNfFvsOnK
    Data yphxYFoz0ObyRLZe+8AQ9uMfvPLELeEDk+3beVLfkL0nz6mwhLp7XJ7SqeO39fx0aOb7sfdQrpH9hqI+e0ee24d2527wMckjjgM36GTwUVhL6xl8yF0RPDk6
    Data FMF8bLu+DQ01IavT1JLfcI0Bt5W05I7eOFwb1JZP1uhyCw1WI2EtbfWOnRqsC2jLJ6IBpVsuXub3Qb6hua+jXlfiCFbpqhzGwkm5cc4bzlJbjRW+tfT3AAhO
    Data gS3E3eRVIMysoksIxYksgJXLw7E5aTu3ufxha3yXMY+oRCkfmvYnvpvoQbstZpjMd7UP+qPCKx9Be/CXd8QOoz/hyycb1q81tMlNdW+z3/fx7djBQrvac3t9
    Data YItR3yd/q9im9mKn2f++vbGsIHe3rmgH+obTAAM7ACAuIQeBAliZVbFogRCL/56QC2nZiDYMQwrut4n/mkR9XnimCfu2XSLKjmZ/8rPuSvyAQoE+/pMvoF+K
    Data HCyLWmJbYxPKXC1fJYEvF4Us8A+Cj8uQs2C9BsDAfgCIY+TI7GcLARb3NpALa9moloYUmfLZn8kLYy/MhLiPxt3dlZ8wgysHkp8N1cxnw5vgPcxnC5SSWC3z
    Data 2TGN4LLkRwOMnVYppxRADtTorFBTWeizkQ+A8WiahmgX8fFtMhn8lLiwUAw/JZPF1rBfXRV7OPYF2IW9qtfmGLAvKpUzNYaIaaZaqcR+jVXP/HXmJeRJe2be
    Data g3ecvh+omPXPQpHFgnOWr6UPTsZ4C86MQtS/TA9TNzNrNhHV7di+aXp83QjDJ93eWl1VVuIr5vqYRYjrrwjycAryeZiVhPGMg5B5yS8hIKDYlAobXeM8Hg6B
    Data i/kCjysDoD8d9GwDMzvKjuaqiIztyxgTNzoztjejLapm8xWcWCyNLIB+soVkm3OY1uZaW6clUuAQ26TFb1C2BXqbh1tzm/T/gb3awpK6wo23pvxdUXudte5E
    Data t66CJMtO3nyZoX4g/3NFpT7N3bHvxP7edWhb/udQ2/P3DkCaFAnFIqO6Jk+1aaG+5//ZH/t4CDMLep944Prw/rS/3PzUnr8SWpknXELwiJhr3bsP9lcyfJon
    Data mjGRIfZ2rPF7/czqfD/2O9LPrM4QUmuBoLuzvhYxiPqK8rzMbJgKDMEhAeKIKIQ8goYkszaJ5nI0RwFK4m3lScURgKbDkC4djV7QtIgTq+KOY4ViIR6IWcaC
    Data hP1xinYgP0LRKd21dtvc7lqcXRjvkLh/6FSBpcCkzpHn22u33vLDY48+NlDrD1eXuy0CnkKocGz7ya0Fboj//sZBU2llXm6eQOep2LA3V2opLNGU9N35s/sm
    Data evRRTex3lSrtSaiD/84SSQVKZ1GewmWBbz84vaXngrV9wzlWfxZPLjSGe47Fbtt2UWH74B8br7ojmqe25Rc1rLWu2Xn+xeVXlXh9M2/HXr8/rOYxtn2dsW0z
    Data dTOqCiLb1tVUR8vLggFPjsNmNauVoBW2ItuWl0IUVeBoQAU1fnO9UARrW5Liml4pxKhLpQ6ZoabEpGXDbNiLzJo4PrkiVPqgXVie6E/EU2k7kDUdQ+J9lzSU
    Data 5ttE0aOVDR27N1Rd+sRMn9lUZL5o+5xe44onszuUjddcXrZtpLON4+qoLRoZaDp2KGHHr8NX3pfKRMKyGrMCwj/eFNp1SfElpx5fE7s+9u2LZ2fwxp+Ef2/6
    Data 0/92bxx6IPZTkVKRy0CV5MVOx/LfQhYE/w+EOVz2
End Function

'$Include:'../utilities/imageassert.bm'
'$Include:'../utilities/base64.bm'
