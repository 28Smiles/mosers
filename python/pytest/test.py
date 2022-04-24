import mosers


def test_1():
    tokenizer = mosers.MosesTokenizer("en")

    tkns = tokenizer.tokenize("Machine Learning is great, isn\'t it?")
    expected_tkns = ["Machine", "Learning", "is", "great", ",", "isn", "&apos;t", "it", "?"]
    assert expected_tkns == tkns


def test_2():
    tokenizer = mosers.MosesTokenizer("en")

    tkns = tokenizer.tokenize("abc def.")
    expected_tkns = ["abc", "def", "."]
    assert expected_tkns == tkns


def test_3():
    tokenizer = mosers.MosesTokenizer("en")

    tkns = tokenizer.tokenize("2016, pp.")
    expected_tkns = ["2016", ",", "pp", "."]
    assert expected_tkns == tkns


def test_4():
    tokenizer = mosers.MosesTokenizer("en")

    tkns = tokenizer.tokenize("this 'is' the thing")
    expected_tkns = ["this", "&apos;", "is", "&apos;", "the", "thing"]
    assert expected_tkns == tkns


def test_5():
    tokenizer = mosers.MosesTokenizer("en")

    tkns = tokenizer.tokenize("foo-bar")
    expected_tkns = ["foo", "@-@", "bar"]
    assert expected_tkns == tkns


def test_escape_xml():
    tokenizer = mosers.MosesTokenizer("en")

    tkns = tokenizer.tokenize("This ain't funny. It's actually hillarious, yet double Ls. | [] < > [ ] & You're gonna shake it off? Don't?")
    expected_tkns = [
        "This",
        "ain",
        "&apos;t",
        "funny",
        ".",
        "It",
        "&apos;s",
        "actually",
        "hillarious",
        ",",
        "yet",
        "double",
        "Ls",
        ".",
        "&#124;",
        "&#91;",
        "&#93;",
        "&lt;",
        "&gt;",
        "&#91;",
        "&#93;",
        "&amp;",
        "You",
        "&apos;re",
        "gonna",
        "shake",
        "it",
        "off",
        "?",
        "Don",
        "&apos;t",
        "?",
    ]
    assert expected_tkns == tkns


def test_opening_brackets():
    tokenizer = mosers.MosesTokenizer("en")

    tkns = tokenizer.tokenize("By the mid 1990s a version of the game became a Latvian television series (with a parliamentary setting, and played by Latvian celebrities).")
    expected_tkns = [
        "By",
        "the",
        "mid",
        "1990s",
        "a",
        "version",
        "of",
        "the",
        "game",
        "became",
        "a",
        "Latvian",
        "television",
        "series",
        "(",
        "with",
        "a",
        "parliamentary",
        "setting",
        ",",
        "and",
        "played",
        "by",
        "Latvian",
        "celebrities",
        ")",
        "."
    ]
    assert expected_tkns == tkns


def test_dot_splitting():
    tokenizer = mosers.MosesTokenizer("en")

    tkns = tokenizer.tokenize("The meeting will take place at 11:00 a.m. Tuesday.")
    expected_tkns = [
        "The",
        "meeting",
        "will",
        "take",
        "place",
        "at",
        "11",
        ":",
        "00",
        "a.m.",
        "Tuesday",
        "."
    ]
    assert expected_tkns == tkns


def test_trailing_dot_apostrophe():
    tokenizer = mosers.MosesTokenizer("en")

    tkns = tokenizer.tokenize("'Hello.'")
    expected_tkns = ["&apos;Hello", ".", "&apos;"]
    assert expected_tkns == tkns


def test_final_dot_unconditionally():
    tokenizer = mosers.MosesTokenizer("en")

    tkns = tokenizer.tokenize("'So am I.")
    expected_tkns = ["&apos;So", "am", "I", "."]
    assert expected_tkns == tkns

    tokenizer = mosers.MosesTokenizer("fr")

    tkns = tokenizer.tokenize("Des gens admirent une œuvre d'art.", False)
    expected_tkns = ["Des", "gens", "admirent", "une", "œuvre", "d'", "art", "."]
    assert expected_tkns == tkns

    tokenizer = mosers.MosesTokenizer("de")

    tkns = tokenizer.tokenize("...schwer wie ein iPhone 5.")
    expected_tkns = ["...", "schwer", "wie", "ein", "iPhone", "5", "."]
    assert expected_tkns == tkns

    tokenizer = mosers.MosesTokenizer("cz")

    tkns = tokenizer.tokenize("Dvě děti, které běží bez bot.")
    expected_tkns = ["Dvě", "děti", ",", "které", "běží", "bez", "bot", "."]
    assert expected_tkns == tkns
