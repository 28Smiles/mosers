use mosers::Language;
use pyo3::prelude::*;
use std::str::FromStr;
use strum::ParseError;

#[pyclass]
struct MosesTokenizer(mosers::MosesTokenizer);

#[pymethods]
impl MosesTokenizer {
    #[new]
    fn new(lang: String) -> PyResult<Self> {
        let language: Result<Language, ParseError> = Language::from_str(&*lang);
        let language = language.map_err(|_| pyo3::exceptions::PyValueError::new_err(""));
        Ok(MosesTokenizer(mosers::MosesTokenizer::new(language?)))
    }

    fn tokenize(
        self_: PyRefMut<'_, Self>,
        text: String,
        escape: Option<bool>,
    ) -> PyResult<Vec<String>> {
        let tokens = self_.0.tokenize(text, escape);
        let tokens: Vec<String> = tokens
            .tokens()
            .iter()
            .map(|token| (*token).into())
            .collect();

        Ok(tokens)
    }

    fn penn_tokenize(self_: PyRefMut<'_, Self>, text: String) -> PyResult<Vec<String>> {
        let tokens = self_.0.penn_tokenize(text);
        let tokens: Vec<String> = tokens
            .tokens()
            .iter()
            .map(|token| (*token).into())
            .collect();

        Ok(tokens)
    }
}

#[pyclass]
struct MosesPunctNormalizer(mosers::MosesPunctNormalizer);

#[pymethods]
impl MosesPunctNormalizer {
    #[new]
    fn new(
        lang: String,
        penn: Option<bool>,
        norm_quote_commas: Option<bool>,
        norm_numbers: Option<bool>,
        pre_replace_unicode_punct: Option<bool>,
        post_remove_control_chars: Option<bool>,
    ) -> PyResult<Self> {
        let language: Result<Language, ParseError> = Language::from_str(&*lang);
        let language = language.map_err(|_| pyo3::exceptions::PyValueError::new_err(""));
        Ok(MosesPunctNormalizer(
            mosers::MosesPunctNormalizer::new_with_options(
                language?,
                penn,
                norm_quote_commas,
                norm_numbers,
                pre_replace_unicode_punct,
                post_remove_control_chars,
            ),
        ))
    }

    fn normalize(self_: PyRefMut<'_, Self>, text: String) -> PyResult<String> {
        let tokens = self_.0.normalize(text);

        Ok(tokens)
    }
}

#[pymodule]
fn mosers(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MosesTokenizer>()?;
    m.add_class::<MosesPunctNormalizer>()?;
    Ok(())
}
