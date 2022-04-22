use std::borrow::Cow;
use std::slice::Iter;

use lazy_static::lazy_static;
use regex::Regex;
use strum_macros::EnumString;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Language {
    As,
    Bn,
    Ca,
    Cjk,
    Cs,
    Cz,
    De,
    El,
    En,
    Es,
    Et,
    Fi,
    Fr,
    Ga,
    Gu,
    Hi,
    Hu,
    Is,
    It,
    Ja,
    Kn,
    Ko,
    Lt,
    Lv,
    Ml,
    Mni,
    Mr,
    Nl,
    Or,
    Pa,
    Pl,
    Pt,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Ta,
    Te,
    Yue,
    Zh,
}

impl From<Language> for &'static str {
    fn from(language: Language) -> Self {
        match language {
            Language::As => "as",
            Language::Bn => "bn",
            Language::Ca => "ca",
            Language::Cjk => "cjk",
            Language::Cs => "cs",
            Language::Cz => "cz",
            Language::De => "de",
            Language::El => "el",
            Language::En => "en",
            Language::Es => "es",
            Language::Et => "et",
            Language::Fi => "fi",
            Language::Fr => "fr",
            Language::Ga => "ga",
            Language::Gu => "gu",
            Language::Hi => "hi",
            Language::Hu => "hu",
            Language::Is => "is",
            Language::It => "it",
            Language::Ja => "ja",
            Language::Kn => "kn",
            Language::Ko => "ko",
            Language::Lt => "lt",
            Language::Lv => "lv",
            Language::Ml => "ml",
            Language::Mni => "mni",
            Language::Mr => "mr",
            Language::Nl => "nl",
            Language::Or => "or",
            Language::Pa => "pa",
            Language::Pl => "pl",
            Language::Pt => "pt",
            Language::Ro => "ro",
            Language::Ru => "ru",
            Language::Sk => "sk",
            Language::Sl => "sl",
            Language::Sv => "sv",
            Language::Ta => "ta",
            Language::Te => "te",
            Language::Yue => "yue",
            Language::Zh => "zh",
        }
    }
}

impl From<Language> for Vec<&'static str> {
    fn from(language: Language) -> Self {
        let nonbreaking_prefix_file = match language {
            Language::As => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.as"),
            Language::Bn => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.bn"),
            Language::Ca => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.ca"),
            Language::Cs => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.cs"),
            Language::De => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.de"),
            Language::El => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.el"),
            Language::En => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.en"),
            Language::Es => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.es"),
            Language::Et => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.et"),
            Language::Fi => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.fi"),
            Language::Fr => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.fr"),
            Language::Ga => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.ga"),
            Language::Gu => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.gu"),
            Language::Hi => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.hi"),
            Language::Hu => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.hu"),
            Language::Is => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.is"),
            Language::It => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.it"),
            Language::Kn => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.kn"),
            Language::Lt => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.lt"),
            Language::Lv => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.lv"),
            Language::Ml => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.ml"),
            Language::Mni => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.mni"),
            Language::Mr => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.mr"),
            Language::Nl => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.nl"),
            Language::Or => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.or"),
            Language::Pa => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.pa"),
            Language::Pl => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.pl"),
            Language::Pt => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.pt"),
            Language::Ro => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.ro"),
            Language::Ru => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.ru"),
            Language::Sk => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.sk"),
            Language::Sl => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.sl"),
            Language::Sv => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.sv"),
            Language::Ta => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.ta"),
            Language::Te => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.te"),
            Language::Yue => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.yue"),
            Language::Zh => include_str!("../data/nonbreaking_prefixes/nonbreaking_prefix.zh"),
            _ => "",
        };

        nonbreaking_prefix_file
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.starts_with('#') && !line.is_empty())
            .collect()
    }
}

pub enum PerlUniProps {
    IsAlnum,
    Hiragana,
    LineSeparator,
    IsN,
    HangulSyllables,
    IsSc,
    IsLower,
    IsAlphaUnicharsAu,
    Katakana,
    CurrencySymbol,
    IsAlnumUnicharsAu,
    IsUpper,
    Han,
    Number,
    ClosePunctuation,
    IsPi,
    IsPf,
    OpenPunctuation,
    CJKSymbols,
    Punctuation,
    LowercaseLetter,
    Hangul,
    IsAlpha,
    IsSo,
    Separator,
    TitlecaseLetter,
    UppercaseLetter,
    CJK,
    Symbol,
}

impl PerlUniProps {
    fn as_str(&self) -> &'static str {
        match &self {
            PerlUniProps::IsAlnum => include_str!("../data/perluniprops/IsAlnum.txt"),
            PerlUniProps::Hiragana => include_str!("../data/perluniprops/Hiragana.txt"),
            PerlUniProps::LineSeparator => include_str!("../data/perluniprops/Line_Separator.txt"),
            PerlUniProps::IsN => include_str!("../data/perluniprops/IsN.txt"),
            PerlUniProps::HangulSyllables => {
                include_str!("../data/perluniprops/Hangul_Syllables.txt")
            }
            PerlUniProps::IsSc => include_str!("../data/perluniprops/IsSc.txt"),
            PerlUniProps::IsLower => include_str!("../data/perluniprops/IsLower.txt"),
            PerlUniProps::IsAlphaUnicharsAu => {
                include_str!("../data/perluniprops/IsAlpha-unichars-au.txt")
            }
            PerlUniProps::Katakana => include_str!("../data/perluniprops/Katakana.txt"),
            PerlUniProps::CurrencySymbol => {
                include_str!("../data/perluniprops/Currency_Symbol.txt")
            }
            PerlUniProps::IsAlnumUnicharsAu => {
                include_str!("../data/perluniprops/IsAlnum-unichars-au.txt")
            }
            PerlUniProps::IsUpper => include_str!("../data/perluniprops/IsUpper.txt"),
            PerlUniProps::Han => include_str!("../data/perluniprops/Han.txt"),
            PerlUniProps::Number => include_str!("../data/perluniprops/Number.txt"),
            PerlUniProps::ClosePunctuation => {
                include_str!("../data/perluniprops/Close_Punctuation.txt")
            }
            PerlUniProps::IsPi => include_str!("../data/perluniprops/IsPi.txt"),
            PerlUniProps::IsPf => include_str!("../data/perluniprops/IsPf.txt"),
            PerlUniProps::OpenPunctuation => {
                include_str!("../data/perluniprops/Open_Punctuation.txt")
            }
            PerlUniProps::CJKSymbols => include_str!("../data/perluniprops/CJKSymbols.txt"),
            PerlUniProps::Punctuation => include_str!("../data/perluniprops/Punctuation.txt"),
            PerlUniProps::LowercaseLetter => {
                include_str!("../data/perluniprops/Lowercase_Letter.txt")
            }
            PerlUniProps::Hangul => include_str!("../data/perluniprops/Hangul.txt"),
            PerlUniProps::IsAlpha => include_str!("../data/perluniprops/IsAlpha.txt"),
            PerlUniProps::IsSo => include_str!("../data/perluniprops/IsSo.txt"),
            PerlUniProps::Separator => include_str!("../data/perluniprops/Separator.txt"),
            PerlUniProps::TitlecaseLetter => {
                include_str!("../data/perluniprops/Titlecase_Letter.txt")
            }
            PerlUniProps::UppercaseLetter => {
                include_str!("../data/perluniprops/Uppercase_Letter.txt")
            }
            PerlUniProps::CJK => include_str!("../data/perluniprops/CJK.txt"),
            PerlUniProps::Symbol => include_str!("../data/perluniprops/Symbol.txt"),
        }
    }
}

fn apply(text: Cow<str>, mut next: Iter<(Regex, &str)>) -> String {
    if let Some((re, replacement)) = next.next() {
        apply(re.replace_all(text.as_ref(), *replacement), next)
    } else {
        text.into_owned()
    }
}

pub struct MosesPunctNormalizer {
    lang: Language,
    penn: Option<bool>,
    norm_quote_commas: Option<bool>,
    norm_numbers: Option<bool>,
    pre_replace_unicode_punct: Option<bool>,
    post_remove_control_chars: Option<bool>,
}

impl MosesPunctNormalizer {
    pub fn new(lang: Language) -> MosesPunctNormalizer {
        MosesPunctNormalizer::new_with_options(
            lang,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
        )
    }

    pub fn new_with_options(
        lang: Language,
        penn: Option<bool>,
        norm_quote_commas: Option<bool>,
        norm_numbers: Option<bool>,
        pre_replace_unicode_punct: Option<bool>,
        post_remove_control_chars: Option<bool>,
    ) -> MosesPunctNormalizer {
        MosesPunctNormalizer {
            lang,
            penn,
            norm_quote_commas,
            norm_numbers,
            pre_replace_unicode_punct,
            post_remove_control_chars,
        }
    }

    pub fn normalize<T: Into<String>>(&self, text: T) -> String {
        let text = text.into();
        let text = if *self.pre_replace_unicode_punct.as_ref().unwrap_or(&true) {
            self.replace_unicode_punct(&*text)
        } else {
            text
        };

        let text = self.extra_whitespace(&*text);

        let text = if *self.penn.as_ref().unwrap_or(&true) {
            self.handle_penn_substitutions(text)
        } else {
            text
        };

        let text = self.normalize_unicode(&*text);
        let text = self.french_quotes(&*text);
        let text = self.handle_pseudo_spaces(&*text);

        let text = if *self.penn.as_ref().unwrap_or(&true) {
            lazy_static! {
                static ref NORMALIZE_UNICODE_IF_NOT_PENN_0: Regex = Regex::new("`").unwrap();
                static ref NORMALIZE_UNICODE_IF_NOT_PENN_1: Regex = Regex::new("''").unwrap();
            }
            let text = NORMALIZE_UNICODE_IF_NOT_PENN_0.replace_all(&*text, "'");
            let text = NORMALIZE_UNICODE_IF_NOT_PENN_1.replace_all(&*text, " \" ");

            text.into_owned()
        } else {
            text
        };

        let text = if *self.norm_quote_commas.as_ref().unwrap_or(&true) {
            match &self.lang {
                Language::En => self.handle_en_quotation_followed_by_comma(text),
                Language::De | Language::Es | Language::Fr => {
                    self.handle_de_es_fr_quotation_followed_by_comma(text)
                }
                _ => text,
            }
        } else {
            text
        };

        let text = if *self.norm_numbers.as_ref().unwrap_or(&true) {
            match &self.lang {
                Language::De | Language::Es | Language::Fr | Language::Cz | Language::Cs => {
                    self.handle_numbers_comma(text)
                }
                _ => self.handle_numbers_point(text),
            }
        } else {
            text
        };

        if *self.post_remove_control_chars.as_ref().unwrap_or(&false) {
            let remove_control_chars: Regex = Regex::new(r"\p{C}").unwrap();

            remove_control_chars.replace_all(&*text, "").to_string()
        } else {
            text
        }
    }

    fn replace_unicode_punct(&self, text: &str) -> String {
        let text: String = text
            .chars()
            .map(|c| match c {
                '，' => ',',
                '、' => ',',
                '”' => '\'',
                '“' => '\'',
                '∶' => ':',
                '：' => ':',
                '？' => '?',
                '《' => '\'',
                '》' => '\'',
                '）' => ',',
                '！' => '!',
                '（' => '(',
                '；' => ';',
                '１' => '1',
                '」' => '\'',
                '「' => '\'',
                '０' => '0',
                '３' => '3',
                '２' => '2',
                '５' => '5',
                '６' => '6',
                '９' => '9',
                '７' => '7',
                '８' => '8',
                '４' => '4',
                '～' => '~',
                '’' => '\'',
                '━' => '-',
                '〈' => '<',
                '〉' => '>',
                '【' => '[',
                '】' => ']',
                '％' => '%',
                e => e,
            })
            .collect();
        lazy_static! {
            static ref UNICODE_POINT: Regex = Regex::new("…").unwrap();
            static ref UNICODE_CIRCLE_SPACE_AFTER: Regex = Regex::new(r"。\s*").unwrap();
            static ref UNICODE_POINT_SPACE_AFTER: Regex = Regex::new(r"．\s*").unwrap();
        }
        let text = UNICODE_POINT.replace(&*text, "...");
        let text = UNICODE_CIRCLE_SPACE_AFTER.replace_all(&*text, ". ");
        let text = UNICODE_POINT_SPACE_AFTER.replace_all(&*text, ". ");

        text.to_string()
    }

    fn extra_whitespace(&self, text: &str) -> String {
        lazy_static! {
            static ref EXTRA_WHITESPACE: [(Regex, &'static str); 10] = [
                (Regex::new(r"\r").unwrap(), r""),
                (Regex::new(r"\(").unwrap(), r" ("),
                (Regex::new(r"\)").unwrap(), r") "),
                (Regex::new(r" +").unwrap(), r" "),
                (Regex::new(r"\) ([.!:?;,])").unwrap(), r"\)\g$1"),
                (Regex::new(r"\( ").unwrap(), r"("),
                (Regex::new(r" \)").unwrap(), r")"),
                (Regex::new(r"(\d) %").unwrap(), r"\g$1%"),
                (Regex::new(r" :").unwrap(), r":"),
                (Regex::new(r" ;").unwrap(), r";"),
            ];
        }

        apply(Cow::Borrowed(text), EXTRA_WHITESPACE.iter())
    }

    fn normalize_unicode(&self, text: &str) -> String {
        lazy_static! {
            static ref NORMALIZE_UNICODE: [(Regex, &'static str); 15] = [
                (Regex::new(r"„").unwrap(), "\""),
                (Regex::new(r"“").unwrap(), "\""),
                (Regex::new(r"”").unwrap(), "\""),
                (Regex::new(r"–").unwrap(), r"-"),
                (Regex::new(r"—").unwrap(), r" - "),
                (Regex::new(r" +").unwrap(), r" "),
                (Regex::new(r"´").unwrap(), r"'"),
                (Regex::new(r"([a-zA-Z])‘([a-zA-Z])").unwrap(), r"\g$1'\g$2"),
                (Regex::new(r"([a-zA-Z])’([a-zA-Z])").unwrap(), r"\g$1'\g$2"),
                (Regex::new(r"‘").unwrap(), r"'"),
                (Regex::new(r"‚").unwrap(), r"'"),
                (Regex::new(r"’").unwrap(), r"'"),
                (Regex::new(r"''").unwrap(), "\""),
                (Regex::new(r"´´").unwrap(), "\""),
                (Regex::new(r"…").unwrap(), r"..."),
            ];
        }

        apply(Cow::Borrowed(text), NORMALIZE_UNICODE.iter())
    }

    fn french_quotes(&self, text: &str) -> String {
        lazy_static! {
            static ref FRENCH_QUOTES: [(Regex, &'static str); 6] = [
                (Regex::new(r"\u00A0«\u00A0").unwrap(), "\""),
                (Regex::new(r"«\u00A0").unwrap(), "\""),
                (Regex::new(r"«").unwrap(), "\""),
                (Regex::new(r"\u00A0»\u00A0").unwrap(), "\""),
                (Regex::new(r"\u00A0»").unwrap(), "\""),
                (Regex::new(r"»").unwrap(), "\""),
            ];
        }

        apply(Cow::Borrowed(text), FRENCH_QUOTES.iter())
    }

    fn handle_pseudo_spaces(&self, text: &str) -> String {
        lazy_static! {
            static ref HANDLE_PSEUDO_SPACES: [(Regex, &'static str); 10] = [
                (Regex::new(r"\u00A0%").unwrap(), r"%"),
                (Regex::new(r"nº\u00A0").unwrap(), r"nº "),
                (Regex::new(r"\u00A0:").unwrap(), r":"),
                (Regex::new(r"\u00A0ºC").unwrap(), r" ºC"),
                (Regex::new(r"\u00A0cm").unwrap(), r" cm"),
                (Regex::new(r"\u00A0\\?").unwrap(), r"?"),
                (Regex::new(r"\u00A0\\!").unwrap(), r"!"),
                (Regex::new(r"\u00A0;").unwrap(), r";"),
                (Regex::new(r",\u00A0").unwrap(), r", "),
                (Regex::new(r" +").unwrap(), r" "),
            ];
        }

        apply(Cow::Borrowed(text), HANDLE_PSEUDO_SPACES.iter())
    }

    fn handle_penn_substitutions(&self, text: String) -> String {
        let text = Regex::new(r"`").unwrap().replace_all(&*text, "'");
        let text = Regex::new(r"''").unwrap().replace_all(&*text, " \" ");

        text.to_string()
    }

    fn handle_en_quotation_followed_by_comma(&self, text: String) -> String {
        let text = Regex::new("\"([,.]+)")
            .unwrap()
            .replace_all(&*text, "\\g$1\"");

        text.to_string()
    }

    fn handle_de_es_fr_quotation_followed_by_comma(&self, text: String) -> String {
        let text = Regex::new(",\"").unwrap().replace_all(&*text, "\",");
        let text = Regex::new("(\\.+)\"(\\s*[^<])")
            .unwrap()
            .replace_all(&*text, "\"\\g$1\\g$2");

        text.to_string()
    }

    fn handle_numbers_comma(&self, text: String) -> String {
        let text = Regex::new("(\\d)\\u00A0(\\d)")
            .unwrap()
            .replace_all(&*text, "\\g$1,\\g$2");

        text.to_string()
    }

    fn handle_numbers_point(&self, text: String) -> String {
        let text = Regex::new("(\\d)\\u00A0(\\d)")
            .unwrap()
            .replace_all(&*text, "\\g$1.\\g$2");

        text.to_string()
    }
}

const VIRAMAS: &str = "\\u094D\\u09CD\\u0A4D\\u0ACD\\u0B4D\\u0BCD\\u0C4D\\u0CCD\\u0D3B\\u0D3C\\u0D4D\\u0EBA\\u1039\\u1714\\u1BAB\\uA8C4\\uA8F3\\uA8F4\\uA953\\uAAF6\\U00010A3F\\U00011046\\U000110B9\\U00011133\\U000111C0\\U00011235\\U000112EA\\U0001134D\\U00011442\\U000114C2\\U000115BF\\U0001163F\\U000116B6\\U00011839\\U000119E0\\U00011A34\\U00011C3F\\U00011D45\\U00011D97\\u0DCA";
const NUKTAS: &str = "\\u093C\\u09BC\\u0A3C\\u0ABC\\u0AFD\\u0AFE\\u0AFF\\u0B3C\\u0CBC\\u1C37\\U000110BA\\U00011173\\U000111CA\\U00011236\\U000112E9\\U0001133C\\U00011446\\U000114C3\\U000115C0\\U000116B7\\U0001183A\\U00011D42\\U0001E94A";

#[derive(Debug, Clone)]
pub struct MosesTokenizer {
    lang: Language,
    nonbreaking_prefixes: Vec<&'static str>,
    numeric_only_prefixes: Vec<&'static str>,
    pad_not_isalnum: (Regex, &'static str),
    aggressive_hyphen_split: (fancy_regex::Regex, &'static str),
    intratoken_slashes: (fancy_regex::Regex, &'static str),
}

impl MosesTokenizer {
    pub fn new(lang: Language) -> MosesTokenizer {
        let nonbreaking_prefixes: Vec<&'static str> = lang.into();
        let numeric_only_prefixes: Vec<&'static str> = nonbreaking_prefixes
            .iter()
            .filter(|prefix| MosesTokenizer::has_numeric_only(**prefix))
            .map(|prefix| {
                let splits: Vec<&str> = prefix.rsplit(' ').collect();

                *splits.last().unwrap()
            })
            .collect();

        let cjk_chars = String::new();
        match lang {
            Language::Zh | Language::Ja | Language::Ko | Language::Cjk => {
                let mut cjk_chars = String::new();

                match lang {
                    Language::Ko | Language::Cjk => {
                        cjk_chars.push_str(PerlUniProps::Hangul.as_str());
                    }
                    _ => {}
                };
                match lang {
                    Language::Zh | Language::Cjk => {
                        cjk_chars.push_str(PerlUniProps::Han.as_str());
                    }
                    _ => {}
                };
                match lang {
                    Language::Ja | Language::Cjk => {
                        cjk_chars.push_str(PerlUniProps::Hiragana.as_str());
                        cjk_chars.push_str(PerlUniProps::Katakana.as_str());
                        cjk_chars.push_str(PerlUniProps::Han.as_str());
                    }
                    _ => {}
                };
            }
            _ => {}
        }

        let mut is_alpha = String::new();
        is_alpha.push_str(PerlUniProps::IsAlpha.as_str());
        is_alpha.push_str(VIRAMAS);
        is_alpha.push_str(NUKTAS);
        is_alpha.push_str(&*cjk_chars);
        let mut is_alnum = String::new();
        is_alnum.push_str(PerlUniProps::IsAlnum.as_str());
        is_alnum.push_str(VIRAMAS);
        is_alnum.push_str(NUKTAS);
        is_alnum.push_str(&*cjk_chars);

        let pad_not_isalnum = (
            Regex::new(&*format!("([^{}\\s\\.'`,-])", &is_alnum)).unwrap(),
            " $1 ",
        );
        let aggressive_hyphen_split = (
            fancy_regex::Regex::new(&*format!("([{}])\\-(?=[{}])", &is_alnum, &is_alnum)).unwrap(),
            r"$1 @-@ ",
        );
        let intratoken_slashes = (
            fancy_regex::Regex::new(&*format!("([{}])/([{}])", &is_alnum, &is_alnum)).unwrap(),
            r"$1 \@/\@ $2",
        );

        MosesTokenizer {
            lang,
            nonbreaking_prefixes,
            numeric_only_prefixes,
            pad_not_isalnum,
            aggressive_hyphen_split,
            intratoken_slashes,
        }
    }

    fn replace_multidots(&self, text: String) -> String {
        lazy_static! {
            static ref DOTMULTI: Regex = Regex::new(r"\.(\.+)").unwrap();
            static ref MORE: Regex = Regex::new(r"DOTMULTI\.").unwrap();
            static ref R1: Regex = Regex::new(r"DOTMULTI\.([^\.])").unwrap();
            static ref R2: Regex = Regex::new(r"DOTMULTI\.").unwrap();
        }

        let mut text = DOTMULTI.replace_all(&*text, r" DOTMULTI$1").into_owned();
        while MORE.find(&*text).is_some() {
            let ltext = R1.replace_all(&*text, "DOTDOTMULTI $1");
            text = R2.replace_all(ltext.as_ref(), "DOTDOTMULTI").into_owned();
        }

        text
    }

    fn restore_multidots(&self, text: String) -> String {
        lazy_static! {
            static ref DOTDOTMULTI: Regex = Regex::new("DOTDOTMULTI").unwrap();
            static ref DOTMULTI: Regex = Regex::new("DOTMULTI").unwrap();
        }

        let mut text = text;
        while DOTDOTMULTI.find(&*text).is_some() {
            text = DOTDOTMULTI.replace_all(&*text, "DOTMULTI.").into_owned();
        }

        DOTMULTI.replace_all(&*text, ".").into_owned()
    }

    fn is_lower(text: &str) -> bool {
        let lower_chars: &'static str = PerlUniProps::IsLower.as_str(); // TODO lazy const set

        text.chars()
            .all(|tc| lower_chars.chars().any(|lc| lc == tc))
    }

    fn is_any_alpha(text: &str) -> bool {
        let alpha_chars: &'static str = PerlUniProps::IsAlpha.as_str(); // TODO lazy const set

        text.chars()
            .any(|tc| alpha_chars.chars().any(|lc| lc == tc))
    }

    fn has_numeric_only(text: &str) -> bool {
        lazy_static! {
            static ref NUMERIC_ONLY: Regex = Regex::new(r"[\s]+(#NUMERIC_ONLY#)").unwrap();
        }

        NUMERIC_ONLY.find(text).is_some()
    }

    fn handles_nonbreaking_prefixes(&self, text: &str) -> String {
        lazy_static! {
            static ref SPLIT_WS: Regex = Regex::new(r"\s+").unwrap();
            static ref RE_END_WITH_PERIOD: Regex = Regex::new(r"^(\S+)\.$").unwrap();
            static ref RE_NUM: Regex = Regex::new(r"^[0-9]+").unwrap();
        };

        let mut collector = String::new();
        let tokens: Vec<&str> = SPLIT_WS.split(text).collect();
        tokens.iter().enumerate().for_each(|(i, token)| {
            if i != 0 {
                collector.push(' ');
            }
            let token_ends_with_period = RE_END_WITH_PERIOD.captures(token);
            if let Some(token_ends_with_period) = token_ends_with_period {
                let prefix = token_ends_with_period.get(1).unwrap().as_str();
                // Checks for 3 conditions if
                // i.   the prefix contains a fullstop and
                //      any char in the prefix is within the IsAlpha charset
                // ii.  the prefix is in the list of nonbreaking prefixes and
                //      does not contain #NUMERIC_ONLY#
                // iii. the token is not the last token and that the
                //      next token contains all lowercase.
                #[allow(clippy::if_same_then_else)]
                if (prefix.chars().any(|c| c == '.') && MosesTokenizer::is_any_alpha(prefix))
                    || (self.nonbreaking_prefixes.iter().any(|nbp| *nbp == prefix)
                        && self.numeric_only_prefixes.iter().any(|nbp| *nbp == prefix))
                    || (i != tokens.len() - 1
                        && MosesTokenizer::is_lower(tokens.get(i + 1).unwrap()))
                {
                    collector.push_str(*token);
                } else if self.numeric_only_prefixes.iter().any(|nbp| *nbp == prefix)
                    && (i + 1) < tokens.len()
                    && RE_NUM.is_match(token)
                {
                    collector.push_str(*token);
                } else {
                    collector.push_str(prefix);
                    collector.push_str(" .");
                }
            } else {
                collector.push_str(*token);
            }
        });

        collector
    }

    fn escape_xml(&self, text: &str) -> String {
        lazy_static! {
            static ref ESCAPE_XML_REGEXES: [(Regex, &'static str); 8] = [
                (Regex::new(r"&").unwrap(), "&amp;"),
                (Regex::new(r"\|").unwrap(), "&#124;"),
                (Regex::new(r"<").unwrap(), "&lt;"),
                (Regex::new(r">").unwrap(), "&gt;"),
                (Regex::new("\"").unwrap(), "&quot;"),
                (Regex::new(r"'").unwrap(), "&apos;"),
                (Regex::new(r"\[").unwrap(), "&#91;"),
                (Regex::new(r"]").unwrap(), "&#93;"),
            ];
        }

        apply(Cow::Borrowed(text), ESCAPE_XML_REGEXES.iter())
    }

    pub fn penn_tokenize<T: Into<String>>(&self, text: T) -> Tokens {
        lazy_static! {
            static ref MOSES_PENN_REGEXES_1_0: [(Regex, &'static str); 19] = [
                (Regex::new(r"\s+").unwrap(), " "),
                (Regex::new(r"[\x00-\x1F]").unwrap(), ""),
                (Regex::new(r"^``").unwrap(), "`` "),
                (Regex::new("^\"").unwrap(), "`` "),
                (Regex::new("^`([^`])").unwrap(), "` $1"),
                (Regex::new("^'").unwrap(), "`  "),
                (Regex::new("([ (\\[{<])\"").unwrap(), r"$1 `` "),
                (Regex::new(r"([ (\[{<])``").unwrap(), r"$1 `` "),
                (Regex::new(r"([ (\[{<])`([^`])").unwrap(), r"$1 ` $2"),
                (Regex::new(r"([ (\[{<])'").unwrap(), r"$1 ` "),
                (Regex::new(r"\.\.\.").unwrap(), r" _ELLIPSIS_ "),
                (
                    Regex::new(&*format!(
                        r"([^{}])[,]([^{}])",
                        PerlUniProps::IsN.as_str(),
                        PerlUniProps::IsN.as_str()
                    ))
                    .unwrap(),
                    r"$1 , $2"
                ),
                (
                    Regex::new(&*format!(
                        r"([{}])[,]([^{}])",
                        PerlUniProps::IsN.as_str(),
                        PerlUniProps::IsN.as_str()
                    ))
                    .unwrap(),
                    r"$1 , $2"
                ),
                (
                    Regex::new(&*format!(
                        r"([^{}])[,]([^{}])",
                        PerlUniProps::IsN.as_str(),
                        PerlUniProps::IsN.as_str()
                    ))
                    .unwrap(),
                    r"$1 , $2"
                ),
                (
                    Regex::new(&*format!(
                        r"([;:@#\$%&{}{}])",
                        PerlUniProps::IsSc.as_str(),
                        PerlUniProps::IsSo.as_str()
                    ))
                    .unwrap(),
                    r" $1"
                ),
                (
                    Regex::new(&*format!(
                        r"([{}])/([{}])",
                        PerlUniProps::IsAlnum.as_str(),
                        PerlUniProps::IsAlnum.as_str()
                    ))
                    .unwrap(),
                    "$1 @/@ $2"
                ),
                (
                    Regex::new("([^.])([.])([])}>\"']*) ?$").unwrap(),
                    r"$1 $2$3"
                ),
                (Regex::new(r"([?!])").unwrap(), r" $1 "),
                (Regex::new(r"([]\[(){}<>])").unwrap(), r" $1 "),
            ];
            static ref MOSES_PENN_REGEXES_1_1: [(Regex, &'static str); 31] = [
                (Regex::new(r"\(").unwrap(), r"-LRB-"),
                (Regex::new(r"\)").unwrap(), r"-RRB-"),
                (Regex::new(r"\[").unwrap(), r"-LSB-"),
                (Regex::new(r"\]").unwrap(), r"-RSB-"),
                (Regex::new(r"\{").unwrap(), r"-LCB-"),
                (Regex::new(r"\}").unwrap(), r"-RCB-"),
                (Regex::new(r"--").unwrap(), r" -- "),
                (Regex::new(r"^").unwrap(), r" "),
                (Regex::new(r"$").unwrap(), r" "),
                (Regex::new("\"").unwrap(), r" '' "),
                (Regex::new(r"([^'])' ").unwrap(), r"$1 ' "),
                (Regex::new(r"([^'])'").unwrap(), r"$1 ' "),
                (Regex::new(r"'([sSmMdD]) ").unwrap(), r" '$1 "),
                (Regex::new(r"'ll").unwrap(), r" 'll "),
                (Regex::new(r"'re").unwrap(), r" 're "),
                (Regex::new(r"'ve").unwrap(), r" 've "),
                (Regex::new(r"n't").unwrap(), r" n't"),
                (Regex::new(r"'LL").unwrap(), r" 'LL "),
                (Regex::new(r"'RE").unwrap(), r" 'RE "),
                (Regex::new(r"'VE").unwrap(), r" 'VE "),
                (Regex::new(r"N'T").unwrap(), r" N'T "),
                (Regex::new(r" ([Cc])annot ").unwrap(), r" $1an not "),
                (Regex::new(r" ([Dd])'ye ").unwrap(), r" $1' ye "),
                (Regex::new(r" ([Gg])imme ").unwrap(), r" $1im me "),
                (Regex::new(r" ([Gg])onna ").unwrap(), r" $1on na "),
                (Regex::new(r" ([Gg])otta ").unwrap(), r" $1ot ta "),
                (Regex::new(r" ([Ll])emme ").unwrap(), r" $1em me "),
                (Regex::new(r" ([Mm])ore'n ").unwrap(), r" $1ore 'n "),
                (Regex::new(r" '([Tt])is ").unwrap(), r" '$1 is "),
                (Regex::new(r" '([Tt])was ").unwrap(), r" '$1 was "),
                (Regex::new(r" ([Ww])anna ").unwrap(), r" $1an na "),
            ];
            static ref MOSES_PENN_REGEXES_2: [(Regex, &'static str); 10] = [
                (Regex::new(r"_ELLIPSIS_").unwrap(), "..."),
                (Regex::new(r"  *").unwrap(), r" "),
                (Regex::new(r"^ *").unwrap(), r""),
                (Regex::new(r" *$").unwrap(), r""),
                (Regex::new(r"&").unwrap(), "&amp;"),
                (Regex::new(r"\|").unwrap(), "&#124;"),
                (Regex::new(r"<").unwrap(), "&lt;"),
                (Regex::new(r">").unwrap(), "&gt;"),
                (Regex::new("\"").unwrap(), "&quot;"),
                (Regex::new(r"'").unwrap(), "&apos;"),
            ];
        }

        let text = text.into();
        let text = apply(Cow::Owned(text), MOSES_PENN_REGEXES_1_0.iter());
        let text = self
            .intratoken_slashes
            .0
            .replace_all(&*text, self.intratoken_slashes.1);
        let text = apply(text, MOSES_PENN_REGEXES_1_1.iter());
        let text = self.handles_nonbreaking_prefixes(&*text);
        let text = apply(Cow::Owned(text), MOSES_PENN_REGEXES_2.iter());

        Tokens { text }
    }

    pub fn tokenize<T: Into<String>>(&self, text: T, escape: Option<bool>) -> Tokens {
        let text = text.into();
        lazy_static! {
            static ref SPACE: Regex = Regex::new(r"\s+").unwrap();
            static ref SPACE_BEGIN: Regex = Regex::new(r"^\s*").unwrap();
            static ref SPACE_END: Regex = Regex::new(r"\s*$").unwrap();
            static ref ASCII_JUNK: Regex = Regex::new(r"[\x00-\x1F]").unwrap();
        }
        let text = SPACE.replace_all(&*text, " ");
        let text = ASCII_JUNK.replace_all(text.as_ref(), "");

        // TODO Protected Patterns

        let text = SPACE_BEGIN.replace(text.as_ref(), "");
        let text = SPACE_END.replace(text.as_ref(), "");

        // TODO Finnish and Swedish ["fi", "sv"] https://github.com/alvations/sacremoses/blob/b36fedeb15fb403ccc4f4ae61cbcd5c3d60e2883/sacremoses/tokenize.py#L469

        let (regexp, substitution) = &self.pad_not_isalnum;
        let text = regexp.replace_all(text.as_ref(), *substitution);

        // TODO aggressive dash splits as arg

        let (regexp, substitution) = &self.aggressive_hyphen_split;
        let text = regexp.replace_all(text.as_ref(), *substitution);

        let text = self.replace_multidots(text.into_owned());

        lazy_static! {
            static ref COMMA_SEPARATE_1: Regex =
                Regex::new(&*format!("([^{}])[,]", PerlUniProps::IsN.as_str())).unwrap();
            static ref COMMA_SEPARATE_2: Regex =
                Regex::new(&*format!("[,]([^{}])", PerlUniProps::IsN.as_str())).unwrap();
            static ref COMMA_SEPARATE_3: Regex =
                Regex::new(&*format!("([{}])[,]$", PerlUniProps::IsN.as_str())).unwrap();
        }

        let text = COMMA_SEPARATE_1.replace_all(&*text, "$1 , ");
        let text = COMMA_SEPARATE_2.replace_all(&*text, "$1 , ");
        let text = COMMA_SEPARATE_3.replace_all(&*text, "$1 , ");

        let text = if self.lang == Language::En {
            lazy_static! {
                static ref ENGLISH_SPECIFIC_APOSTROPHE: [(Regex, &'static str); 5] = [
                    (
                        Regex::new(&*format!(
                            r"([^{}])[']([^{}])",
                            PerlUniProps::IsAlpha.as_str(),
                            PerlUniProps::IsAlpha.as_str()
                        ))
                        .unwrap(),
                        r"$1 ' $2"
                    ),
                    (
                        Regex::new(&*format!(
                            r"([^{}{}])[']([{}])",
                            PerlUniProps::IsAlpha.as_str(),
                            PerlUniProps::IsN.as_str(),
                            PerlUniProps::IsAlpha.as_str()
                        ))
                        .unwrap(),
                        r"$1 ' $2"
                    ),
                    (
                        Regex::new(&*format!(
                            r"([{}])[']([^{}])",
                            PerlUniProps::IsAlpha.as_str(),
                            PerlUniProps::IsAlpha.as_str()
                        ))
                        .unwrap(),
                        r"$1 ' $2"
                    ),
                    (
                        Regex::new(&*format!(
                            r"([{}])[']([{}])",
                            PerlUniProps::IsAlpha.as_str(),
                            PerlUniProps::IsAlpha.as_str()
                        ))
                        .unwrap(),
                        r"$1 '$2"
                    ),
                    (
                        Regex::new(&*format!(r"([{}])[']([s])", PerlUniProps::IsN.as_str()))
                            .unwrap(),
                        r"$1 '$2"
                    ),
                ];
            }
            apply(text, ENGLISH_SPECIFIC_APOSTROPHE.iter())
        } else if self.lang == Language::Fr || self.lang == Language::It {
            lazy_static! {
                static ref FR_IT_SPECIFIC_APOSTROPHE: [(Regex, &'static str); 4] = [
                    (
                        Regex::new(&*format!(
                            r"([^{}])[']([^{}])",
                            PerlUniProps::IsAlpha.as_str(),
                            PerlUniProps::IsAlpha.as_str()
                        ))
                        .unwrap(),
                        r"$1 ' $2"
                    ),
                    (
                        Regex::new(&*format!(
                            r"([^{}])[']([{}])",
                            PerlUniProps::IsAlpha.as_str(),
                            PerlUniProps::IsAlpha.as_str()
                        ))
                        .unwrap(),
                        r"$1 ' $2"
                    ),
                    (
                        Regex::new(&*format!(
                            r"([{}])[']([^{}])",
                            PerlUniProps::IsAlpha.as_str(),
                            PerlUniProps::IsAlpha.as_str()
                        ))
                        .unwrap(),
                        r"$1 ' $2"
                    ),
                    (
                        Regex::new(&*format!(
                            r"([{}])[']([{}])",
                            PerlUniProps::IsAlpha.as_str(),
                            PerlUniProps::IsAlpha.as_str()
                        ))
                        .unwrap(),
                        r"$1' $2"
                    ),
                ];
            }
            apply(text, FR_IT_SPECIFIC_APOSTROPHE.iter())
        } else {
            text.into_owned()
        };

        let text = self.handles_nonbreaking_prefixes(&*text);

        let text: Cow<str> = SPACE.replace_all(&*text, " ");
        let text: &str = text.trim();

        lazy_static! {
            static ref TRAILING_DOT_APOSTROPHE: Regex = Regex::new(r"\.' ?$").unwrap();
        }
        let text = TRAILING_DOT_APOSTROPHE.replace_all(text, " . ' ");

        // TODO restore protected patterns

        let text = self.restore_multidots(text.into_owned());

        let text = if let Some(escape) = escape {
            if escape {
                self.escape_xml(&*text)
            } else {
                text
            }
        } else {
            // Default: Do escape
            self.escape_xml(&*text)
        };

        let text: Cow<str> = SPACE.replace_all(&*text, " ");
        let text: &str = text.trim();
        let text: String = text.into();

        Tokens { text }
    }
}

pub struct Tokens {
    pub text: String,
}

impl Tokens {
    pub fn tokens(&self) -> Vec<&str> {
        lazy_static! {
            static ref SPACE: Regex = Regex::new(r"\s+").unwrap();
        }

        SPACE.split(&*self.text).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Language, MosesTokenizer};

    #[test]
    fn test_1() {
        let tokenizer = MosesTokenizer::new(Language::En);

        let tkns = tokenizer.tokenize("Machine Learning is great, isn\'t it?", Option::None);

        assert_eq!(
            tkns.tokens(),
            ["Machine", "Learning", "is", "great", ",", "isn", "&apos;t", "it", "?"]
        );
    }

    #[test]
    fn test_2() {
        let tokenizer = MosesTokenizer::new(Language::En);

        let tkns = tokenizer.tokenize("abc def.", Option::None);

        assert_eq!(tkns.tokens(), ["abc", "def", "."]);
    }

    #[test]
    fn test_3() {
        let tokenizer = MosesTokenizer::new(Language::En);

        let tkns = tokenizer.tokenize("2016, pp.", Option::None);

        assert_eq!(tkns.tokens(), ["2016", ",", "pp", "."]);
    }

    #[test]
    fn test_4() {
        let tokenizer = MosesTokenizer::new(Language::En);

        let tkns = tokenizer.tokenize("this 'is' the thing", Option::None);

        assert_eq!(
            tkns.tokens(),
            ["this", "&apos;", "is", "&apos;", "the", "thing"]
        );
    }

    #[test]
    fn test_5() {
        let tokenizer = MosesTokenizer::new(Language::En);

        let tkns = tokenizer.tokenize("foo-bar", Option::None);

        assert_eq!(tkns.tokens(), ["foo", "@-@", "bar"]);
    }

    #[test]
    fn test_escape_xml() {
        let text = "This ain't funny. It's actually hillarious, yet double Ls. | [] < > [ ] & You're gonna shake it off? Don't?";
        let tokenizer = MosesTokenizer::new(Language::En);
        let tkns = tokenizer.tokenize(text, Option::None);
        assert_eq!(
            tkns.tokens(),
            [
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
        );
    }

    #[test]
    fn test_opening_brackets() {
        let text = "By the mid 1990s a version of the game became a Latvian television series (with a parliamentary setting, and played by Latvian celebrities).";
        let tokenizer = MosesTokenizer::new(Language::En);
        let tkns = tokenizer.tokenize(text, Option::None);
        assert_eq!(
            tkns.tokens(),
            [
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
        );
    }

    #[test]
    fn test_dot_splitting() {
        let text = "The meeting will take place at 11:00 a.m. Tuesday.";
        let tokenizer = MosesTokenizer::new(Language::En);
        let tkns = tokenizer.tokenize(text, Option::None);
        assert_eq!(
            tkns.tokens(),
            [
                "The", "meeting", "will", "take", "place", "at", "11", ":", "00", "a.m.",
                "Tuesday", "."
            ]
        );
    }

    #[test]
    fn test_trailing_dot_apostrophe() {
        let text = "'Hello.'";
        let tokenizer = MosesTokenizer::new(Language::En);
        let tkns = tokenizer.tokenize(text, Option::None);
        assert_eq!(tkns.tokens(), ["&apos;Hello", ".", "&apos;"]);
    }

    #[test]
    fn test_final_dot_unconditionally() {
        let text = "'So am I.";
        let tokenizer = MosesTokenizer::new(Language::En);
        let tkns = tokenizer.tokenize(text, Option::None);
        assert_eq!(tkns.tokens(), ["&apos;So", "am", "I", "."]);

        let text = "Des gens admirent une œuvre d'art.";
        let tokenizer = MosesTokenizer::new(Language::Fr);
        let tkns = tokenizer.tokenize(text, Option::Some(false));
        assert_eq!(
            tkns.tokens(),
            ["Des", "gens", "admirent", "une", "œuvre", "d'", "art", "."]
        );

        let text = "...schwer wie ein iPhone 5.";
        let tokenizer = MosesTokenizer::new(Language::De);
        let tkns = tokenizer.tokenize(text, Option::None);
        assert_eq!(
            tkns.tokens(),
            ["...", "schwer", "wie", "ein", "iPhone", "5", "."]
        );

        let text = "Dvě děti, které běží bez bot.";
        let tokenizer = MosesTokenizer::new(Language::Cz);
        let tkns = tokenizer.tokenize(text, Option::None);
        assert_eq!(
            tkns.tokens(),
            ["Dvě", "děti", ",", "které", "běží", "bez", "bot", "."]
        );
    }
}
