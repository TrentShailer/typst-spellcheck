use std::{env, fs};

use crate::Error;
use serde::{Deserialize, Serialize};
use typst_spellcheck::{LanguageToolConfig, SpellcheckConfig};

use crate::args::Args;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(flatten)]
    pub languagetool_config: LanguageToolConfig,
    #[serde(flatten)]
    pub spellcheck_config: SpellcheckConfig,
}

const DEFAULT_DISABLED_RULES: [&str; 1] = ["WHITESPACE_RULE"];

impl Config {
    pub fn from_args_or_file(args: Args) -> Result<Config, Error> {
        let config_path = match args.config_file {
            Some(v) => v,
            None => env::current_dir()
                .map_err(Error::Pwd)?
                .join("typst-spellcheck.toml"),
        };

        // check if config file exists
        let config_file = if config_path.exists() {
            let contents = fs::read_to_string(&config_path).map_err(Error::ReadConfig)?;
            let config: Config = toml::from_str(&contents)?;
            config
        } else {
            // if no config file, then make sure host, port, and language are defined.
            if args.host.is_none() || args.port.is_none() || args.language.is_none() {
                return Err(Error::RequiredOptions);
            }

            Config::default()
        };

        let mut disabled_rules = args
            .disabed_rules
            .or(config_file.languagetool_config.disabled_rules);

        if let Some(no_default_disabled_rules) = args.no_default_disabled_rules {
            if !no_default_disabled_rules {
                let mut default_disabled_rules: Vec<String> = DEFAULT_DISABLED_RULES
                    .iter()
                    .map(|r| r.to_string())
                    .collect();

                if let Some(specified_rules) = disabled_rules.as_mut() {
                    specified_rules.append(&mut default_disabled_rules);
                } else {
                    disabled_rules = Some(default_disabled_rules)
                }
            }
        }

        // Build configs from args and file
        let languagetool_config = LanguageToolConfig {
            host: args.host.unwrap_or(config_file.languagetool_config.host),
            port: args.port.unwrap_or(config_file.languagetool_config.port),
            language: args
                .language
                .unwrap_or(config_file.languagetool_config.language),

            disabled_categories: args
                .disabed_categories
                .or(config_file.languagetool_config.disabled_categories),
            disabled_rules,
            picky: args.picky.or(config_file.languagetool_config.picky),
        };

        let spellcheck_config = SpellcheckConfig {
            ignore_words: args
                .ignore_words
                .or(config_file.spellcheck_config.ignore_words),
        };

        Ok(Config {
            languagetool_config,
            spellcheck_config,
        })
    }
}
