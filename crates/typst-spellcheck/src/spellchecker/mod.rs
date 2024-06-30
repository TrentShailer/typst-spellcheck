pub mod check_file;
pub mod debug;
pub mod metadata;

use std::sync::Arc;

use languagetool_rust::ServerClient;

use crate::{LanguageToolConfig, SpellcheckConfig};

pub struct Spellchecker {
    pub languagetool_config: LanguageToolConfig,
    pub spellcheck_config: SpellcheckConfig,
    pub client: Arc<ServerClient>,
}

impl Spellchecker {
    pub fn new(
        languagetool_config: LanguageToolConfig,
        spellcheck_config: SpellcheckConfig,
    ) -> Self {
        let client = Arc::new(
            ServerClient::new(
                &languagetool_config.host,
                &languagetool_config.port.to_string(),
            )
            .with_max_suggestions(5),
        );

        Self {
            languagetool_config,
            spellcheck_config,
            client,
        }
    }
}
