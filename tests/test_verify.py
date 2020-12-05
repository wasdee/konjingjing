import konjingjing

def test_verify_id():
    """

    soft fork. https://github.com/jukbot/thai-citizen-id-validator/blob/master/test/validator.test.js
    """
    assert konjingjing.verify_id('1112034563562')
    assert not konjingjing.verify_id('1101700230705')
    assert not konjingjing.verify_id('110170023073')
    assert not konjingjing.verify_id('11017002070d3')
    assert not konjingjing.verify_id('rytege54fsfsf')
    assert not konjingjing.verify_id('0')
    assert not konjingjing.verify_id('-')
    assert not konjingjing.verify_id('')
    assert not konjingjing.verify_id('blablabla')

    # we don't expect other thing expect str
    # assert not konjingjing.verify_id(None)
