use convert_case::{Case, Casing};
use regex::Regex;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub struct CaseConverter {
    special_words: HashMap<String, String>,
    word_patterns: Vec<(Regex, String)>,
}

pub static CONVERTER: LazyLock<Mutex<CaseConverter>> = LazyLock::new(|| {
    let mut converter = CaseConverter::new();

    let pattern = "x(?:_?c)(?:_?o)(?:_?m)(?:_?p)(?:_?o)(?:_?n)(?:_?e)(?:_?n)(?:_?t)";
    converter.add_pattern(&format!(r"(?i){}", pattern), "XCOMPONENT");

    Mutex::new(converter)
});

impl CaseConverter {
    pub fn new() -> Self {
        CaseConverter {
            special_words: HashMap::new(),
            word_patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: &str, replacement: &str) {
        if let Ok(regex) = Regex::new(pattern) {
            self.word_patterns.push((regex, replacement.to_string()));
        }
    }

    pub fn to_screaming_snake_case(&self, input: &str, case: Case) -> String {
        let mut result = input.to_string().to_case(case);

        for (pattern, replacement) in &self.word_patterns {
            result = pattern.replace_all(&result, replacement).to_string();
        }

        let words: Vec<&str> = result.split('_').collect();
        let processed_words: Vec<String> = words
            .iter()
            .map(|word| {
                let word_lower = word.to_lowercase();
                if let Some(special) = self.special_words.get(&word_lower) {
                    special.clone()
                } else {
                    word.to_uppercase()
                }
            })
            .collect();

        processed_words.join("_")
    }
}

pub fn convert_case(input: &str, case: Case) -> String {
    if let Ok(converter) = CONVERTER.lock() {
        converter.to_screaming_snake_case(input, case)
    } else {
        input.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcomponent_conversion() {
        assert_eq!(
            convert_case("xcomponent", Case::ScreamingSnake),
            "XCOMPONENT"
        );
        assert_eq!(
            convert_case("XComponent", Case::ScreamingSnake),
            "XCOMPONENT"
        );
        assert_eq!(
            convert_case("xComponent", Case::ScreamingSnake),
            "XCOMPONENT"
        );
        assert_eq!(
            convert_case("XCOMPonent", Case::ScreamingSnake),
            "XCOMPONENT"
        );
    }

    #[test]
    fn test_mixed_case_conversion() {
        assert_eq!(
            convert_case("myXComponentTest", Case::ScreamingSnake),
            "MY_XCOMPONENT_TEST"
        );
        assert_eq!(
            convert_case("TestXComponentExample", Case::ScreamingSnake),
            "TEST_XCOMPONENT_EXAMPLE"
        );
    }
}
