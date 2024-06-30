use languagetool_rust::check::Match;
use typst_syntax::Source;

use crate::{
    preprocessor::paragraph::NodeContribution,
    range::{Position, Range},
};

/// A problem reported by languagetool
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Problem {
    pub range: Range,
    pub match_string: String,
    pub context: String,
    pub short_message: String,
    pub message: String,
    pub replacements: Vec<String>,
    pub rule_category: String,
    pub rule_id: String,
}

impl Problem {
    pub fn try_from_match(
        source: &Source,
        lt_match: Match,
        match_string: String,
        node_contributions: &[NodeContribution],
    ) -> Option<Self> {
        let match_start = lt_match.offset;

        // Find the contribution that contains the start of the match
        let start_contribution = node_contributions.iter().find(|contribution| {
            contribution.length != 0
                && match_start >= contribution.offset
                && match_start < contribution.offset + contribution.length
        })?;

        // Number of bytes from the start of the node that the match starts
        let match_start_offset = match_start - start_contribution.offset;

        // Find the range of that node in document space
        let node_start = source.range(start_contribution.span)?.start;
        // because a match may start part way through a node, the start range needs to be offset by the amount
        let doc_match_start = node_start + match_start_offset;
        let doc_match_end = doc_match_start + lt_match.length;

        let range_start = Position {
            column: source.byte_to_column(doc_match_start)? + 1,
            line: source.byte_to_line(doc_match_start)? + 1,
        };

        let range_end = Position {
            column: source.byte_to_column(doc_match_end)? + 1,
            line: source.byte_to_line(doc_match_end)? + 1,
        };

        let range = Range {
            start: range_start,
            end: range_end,
        };

        let corrected_context = lt_match
            .context
            .text
            .chars()
            .filter(|c| c != &'\r')
            .collect::<String>();

        Some(Self {
            range,
            match_string,
            context: corrected_context,
            short_message: lt_match.short_message,
            message: lt_match.message,
            replacements: lt_match.replacements.into_iter().map(|v| v.value).collect(),
            rule_category: lt_match.rule.category.id,
            rule_id: lt_match.rule.id,
        })
    }
}
