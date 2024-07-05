pub mod preprocessor;
pub mod problem;
pub mod range;
pub mod spellchecker;
pub mod word_count;

/// Languagetool specific config
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LanguageToolConfig {
    pub host: String,
    pub port: u16,
    pub language: String,
    pub disabled_rules: Option<Vec<String>>,
    pub disabled_categories: Option<Vec<String>>,
    pub picky: Option<bool>,
}

/// typst-spellcheck specific config
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellcheckConfig {
    pub ignore_words: Option<Vec<String>>,
    // pub ignore_headings: bool,
}
