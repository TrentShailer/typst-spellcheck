use std::{ops::RangeInclusive, path::PathBuf};

use clap::Parser;

const AFTER_HELP: &str = r#"Host, Port, and Language are required options if not defined in typst-spellcheck.toml.

Options will override config file values if defined.

typst-spellcheck.toml:
| host      String
| port      u16
| language  String
| picky     Option<bool>
|
| disabled_rules        Option<Vec<String>>
| disabled_categories   Option<Vec<String>>
| ignore_words          Option<Vec<String>>
"#;

/// Spellcheck a typst file with a selfhosted languagetool server.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None, after_help = Some(AFTER_HELP))]
pub struct Args {
    /// Languagetool server URL
    #[arg(short = 'H', long)]
    pub host: Option<String>,

    /// Languagetool server port
    #[arg(short, long, value_parser = port_in_range)]
    pub port: Option<u16>,

    /// Language for spellcheck
    #[arg(short, long)]
    pub language: Option<String>,

    /// List of disabled languagetool rule IDs
    #[arg(long)]
    pub disabed_rules: Option<Vec<String>>,

    /// List of disabled languagetool rule categories
    #[arg(long)]
    pub disabed_categories: Option<Vec<String>>,

    /// List of words to ignore problems with (case sensitive)
    #[arg(long)]
    pub ignore_words: Option<Vec<String>>,

    /// Path to config file (checks pwd by default)
    #[arg(long, value_hint = clap::ValueHint::DirPath)]
    pub config_file: Option<PathBuf>,

    /// Enable picky mode
    #[arg(long)]
    pub picky: Option<bool>,

    /// Typst file to spellcheck
    #[arg(value_hint = clap::ValueHint::DirPath)]
    pub file: PathBuf,
}

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;
fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}
