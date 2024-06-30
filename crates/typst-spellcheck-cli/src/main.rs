pub mod args;
mod config_file;
mod output;

use std::{
    fs::{self},
    io::{self},
};

use args::Args;
use clap::Parser;
use config_file::Config;
use output::display_problems;
use owo_colors::OwoColorize;
use thiserror::Error;
use typst_spellcheck::spellchecker::{check_file, Spellchecker};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}: {}", "Error".red().bold(), e);
    }
}

async fn run() -> Result<(), Error> {
    let args = Args::parse();

    let file = args.file.clone();
    let config = Config::from_args_or_file(args)?;

    // check if defined file exists
    if !file.exists() {
        return Err(Error::InvalidFile);
    }

    let spellchecker = Spellchecker::new(config.languagetool_config, config.spellcheck_config);

    let contents = fs::read_to_string(&file).map_err(Error::ReadFile)?;
    let (mut problems, metadata) = spellchecker
        .check_file(&file.to_string_lossy(), contents)
        .await?;
    problems.sort();

    display_problems(&file.to_string_lossy(), problems, metadata).map_err(Error::Display)?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to get Pwd from environment.\n{0}")]
    Pwd(#[source] io::Error),

    #[error("Failed to read config file.\n{0}")]
    ReadConfig(#[source] io::Error),

    #[error("Failed to parse config file.\n{0}")]
    ParseConfig(#[from] toml::de::Error),

    #[error("Without a config file the 'host', 'port', and 'language' options are required.")]
    RequiredOptions,

    #[error("Specified file does not exist.")]
    InvalidFile,

    #[error("Failed to read specified file.\n{0}")]
    ReadFile(#[source] io::Error),

    #[error("Failed to spellcheck file.\n{0}")]
    Spellcheck(#[from] check_file::Error),

    #[error("Failed to display problems.\n{0}")]
    Display(#[source] io::Error),
}
