use std::{sync::Arc, time::Instant};

use futures::{stream::FuturesUnordered, StreamExt};
use languagetool_rust::{check::Level, CheckRequest};
use thiserror::Error;
use typst_syntax::{FileId, Source, VirtualPath};

use crate::{
    preprocessor::{merge_short::merge_short, preprocess},
    problem::Problem,
    word_count::count_words_naive,
};

use super::{
    debug::{
        debug_paragraphs, debug_problems, debug_response, debug_syntax_tree, setup_debug_file,
    },
    metadata::Metadata,
    Spellchecker,
};

impl Spellchecker {
    pub async fn check_file(
        &self,
        file_path: &str,
        file_contents: String,
        debug: bool,
    ) -> Result<(Vec<Problem>, Metadata), Error> {
        if debug {
            setup_debug_file();
        }

        let source = Source::new(
            FileId::new(None, VirtualPath::new(file_path)),
            file_contents,
        );

        if debug {
            debug_syntax_tree(source.root());
        }

        let paragraphs = preprocess(source.root());
        let paragraphs = merge_short(paragraphs, 512);

        if debug {
            debug_paragraphs(&paragraphs);
        }

        let mut tasks: FuturesUnordered<_> = paragraphs
            .iter()
            .map(|paragraph| {
                let (text, contributions) = paragraph.get_text();
                let mut request = CheckRequest::default()
                    .with_text(text.clone())
                    .with_language(self.languagetool_config.language.clone());

                if self.languagetool_config.picky.unwrap_or(false) {
                    request.level = Level::Picky
                }

                request
                    .disabled_rules
                    .clone_from(&self.languagetool_config.disabled_rules);
                request
                    .disabled_categories
                    .clone_from(&self.languagetool_config.disabled_categories);

                let client = Arc::clone(&self.client);

                async move { (client.check(&request).await, paragraph, text, contributions) }
            })
            .collect();

        let mut problems = vec![];

        let req_start = Instant::now();
        while let Some((result, paragraph, text, node_contributions)) = tasks.next().await {
            let response = result?;

            if debug {
                debug_response(&response, paragraph, &text, &node_contributions);
            }

            for lt_match in response.matches {
                let match_text = &text[lt_match.offset..(lt_match.offset + lt_match.length)];

                // Check if match is an ignore word
                if let Some(ignore_words) = self.spellcheck_config.ignore_words.as_ref() {
                    if ignore_words.contains(&match_text.to_string()) {
                        continue;
                    }
                }

                let maybe_problem = Problem::try_from_match(
                    &source,
                    lt_match.clone(),
                    match_text.to_string(),
                    &node_contributions,
                );

                match maybe_problem {
                    Some(problem) => problems.push(problem),
                    None => {
                        log::warn!("Failed to make problem for match:\n{:?}", lt_match);
                    }
                }
            }
        }
        let req_end = Instant::now();

        let text = paragraphs
            .iter()
            .map(|p| p.get_text().0)
            .collect::<Vec<_>>()
            .join(" ");

        let word_count = count_words_naive(&text);

        let metadata = Metadata {
            word_count,
            languagetool_request_time: req_end.duration_since(req_start),
            paragraph_count: paragraphs.len(),
        };

        if debug {
            debug_problems(&problems);
        }

        Ok((problems, metadata))
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to check file with languagetool.\n{0}")]
    LanguageTool(#[from] languagetool_rust::error::Error),
}
