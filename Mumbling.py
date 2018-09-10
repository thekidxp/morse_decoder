def accum(s):
    returnValue = ''

    for i, c in enumerate(s):
        addedLetters = ''

        for num in range(i + 1):
            addedLetters = addedLetters + c

        addedLetters = addedLetters.capitalize()

        if i == 0:
            returnValue = returnValue + addedLetters
        else:
            returnValue = returnValue + '-' + addedLetters

    return returnValue

Test.describe("accum")
Test.it("Basic tests")
Test.assert_equals(accum("ZpglnRxqenU"), "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu")
Test.assert_equals(accum("NyffsGeyylB"), "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb")
Test.assert_equals(accum("MjtkuBovqrU"), "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu")
Test.assert_equals(accum("EvidjUnokmM"), "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm")
Test.assert_equals(accum("HbideVbxncC"), "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc")
Test.assert_equals(accum("VwhvtHtrxfE"), "V-Ww-Hhh-Vvvv-Ttttt-Hhhhhh-Ttttttt-Rrrrrrrr-Xxxxxxxxx-Ffffffffff-Eeeeeeeeeee")
Test.assert_equals(accum("KurgiKmkphY"), "K-Uu-Rrr-Gggg-Iiiii-Kkkkkk-Mmmmmmm-Kkkkkkkk-Ppppppppp-Hhhhhhhhhh-Yyyyyyyyyyy")
Test.assert_equals(accum("NctlfBlnmfH"), "N-Cc-Ttt-Llll-Fffff-Bbbbbb-Lllllll-Nnnnnnnn-Mmmmmmmmm-Ffffffffff-Hhhhhhhhhhh")
Test.assert_equals(accum("WegunHvbdmV"), "W-Ee-Ggg-Uuuu-Nnnnn-Hhhhhh-Vvvvvvv-Bbbbbbbb-Ddddddddd-Mmmmmmmmmm-Vvvvvvvvvvv")
Test.assert_equals(accum("VoywwSpqidE"), "V-Oo-Yyy-Wwww-Wwwww-Ssssss-Ppppppp-Qqqqqqqq-Iiiiiiiii-Dddddddddd-Eeeeeeeeeee")
Test.assert_equals(accum("VbaixFpxdcO"), "V-Bb-Aaa-Iiii-Xxxxx-Ffffff-Ppppppp-Xxxxxxxx-Ddddddddd-Cccccccccc-Ooooooooooo")
Test.assert_equals(accum("OlyqvYwkuzF"), "O-Ll-Yyy-Qqqq-Vvvvv-Yyyyyy-Wwwwwww-Kkkkkkkk-Uuuuuuuuu-Zzzzzzzzzz-Fffffffffff")
Test.assert_equals(accum("JrhfdMtchiH"), "J-Rr-Hhh-Ffff-Ddddd-Mmmmmm-Ttttttt-Cccccccc-Hhhhhhhhh-Iiiiiiiiii-Hhhhhhhhhhh")
Test.assert_equals(accum("JiwpcSwslvW"), "J-Ii-Www-Pppp-Ccccc-Ssssss-Wwwwwww-Ssssssss-Lllllllll-Vvvvvvvvvv-Wwwwwwwwwww")
Test.assert_equals(accum("EagpiEvmabJ"), "E-Aa-Ggg-Pppp-Iiiii-Eeeeee-Vvvvvvv-Mmmmmmmm-Aaaaaaaaa-Bbbbbbbbbb-Jjjjjjjjjjj")
Test.assert_equals(accum("RznlcEmuxxP"), "R-Zz-Nnn-Llll-Ccccc-Eeeeee-Mmmmmmm-Uuuuuuuu-Xxxxxxxxx-Xxxxxxxxxx-Ppppppppppp")
Test.assert_equals(accum("OrggaExarzP"), "O-Rr-Ggg-Gggg-Aaaaa-Eeeeee-Xxxxxxx-Aaaaaaaa-Rrrrrrrrr-Zzzzzzzzzz-Ppppppppppp")
Test.assert_equals(accum("DriraMtedfB"), "D-Rr-Iii-Rrrr-Aaaaa-Mmmmmm-Ttttttt-Eeeeeeee-Ddddddddd-Ffffffffff-Bbbbbbbbbbb")
Test.assert_equals(accum("BjxseRxgtjT"), "B-Jj-Xxx-Ssss-Eeeee-Rrrrrr-Xxxxxxx-Gggggggg-Ttttttttt-Jjjjjjjjjj-Ttttttttttt")
Test.assert_equals(accum("EquhxOswchE"), "E-Qq-Uuu-Hhhh-Xxxxx-Oooooo-Sssssss-Wwwwwwww-Ccccccccc-Hhhhhhhhhh-Eeeeeeeeeee")

from random import randint

def accum_sol6792(s):
    a = list(s)
    res = ""
    for i, c in enumerate(a):
        res += c * (i + 1) + "-"
    return res.strip("-").title()

def do_ex():
    i = 0
    res = ""
    while (i < 11):
        if (i % 5 == 0):
            n = randint(65, 90)
        else:
            n = randint(97, 122)
        res += chr(n)
        i += 1
    return res

def randomTests():
    print("100 random tests ****************** ")
    for _ in range(0, 100):
        s1 = do_ex()
        Test.assert_equals(accum(s1), accum_sol6792(s1))

randomTests()
